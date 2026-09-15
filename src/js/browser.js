import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';

async function refreshMyFiles() {
    try {
        const files = await invoke('get_shared_files');
        const config = await invoke('get_config');
        document.getElementById('shared-path').textContent = config.shared_folder;

        const list = document.getElementById('files-list');
        list.innerHTML = files.map(file => `
            <div class="file-item">
                <div class="file-icon">${file.is_directory ? '📁' : getFileIcon(file.name)}</div>
                <div class="file-info">
                    <div class="file-name">${file.name}</div>
                    <div class="file-size">${file.is_directory ? 'مجلد' : formatSize(file.size)}</div>
                </div>
            </div>
        `).join('');
    } catch (e) {
        console.error('Failed to load files:', e);
    }
}

function getFileIcon(name) {
    const ext = name.split('.').pop()?.toLowerCase();
    const icons = {
        pdf: '📄', doc: '📝', docx: '📝',
        jpg: '🖼️', jpeg: '🖼️', png: '🖼️', gif: '🖼️',
        mp4: '🎬', mp3: '🎵', zip: '📦', rar: '📦',
        exe: '⚙️', msi: '⚙️',
        js: '📜', rs: '📜', py: '📜', html: '📜', css: '📜',
    };
    return icons[ext] || '📄';
}

function formatSize(bytes) {
    if (bytes === 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
}

async function sendToPeer(peerDeviceId, peerName) {
    const popup = document.createElement('div');
    popup.className = 'send-popup';
    popup.innerHTML = `
        <div class="popup-content">
            <h3>إرسال إلى ${peerName}</h3>
            <button id="send-file-btn">📄 إرسال ملف</button>
            <button id="send-folder-btn">📁 إرسال مجلد</button>
            <button id="cancel-send-btn">إلغاء</button>
        </div>
    `;
    document.body.appendChild(popup);

    document.getElementById('send-file-btn').addEventListener('click', async () => {
        popup.remove();
        const selected = await open({ 
            multiple: true,
            filters: [{ name: 'All Files', extensions: ['*'] }]
        });
        if (selected) {
            const files = Array.isArray(selected) ? selected : [selected];
            for (const filePath of files) {
                await startTransfer(peerDeviceId, filePath);
            }
        }
    });

    document.getElementById('send-folder-btn').addEventListener('click', async () => {
        popup.remove();
        const selected = await open({ directory: true });
        if (selected) {
            await startTransfer(peerDeviceId, selected);
        }
    });

    document.getElementById('cancel-send-btn').addEventListener('click', () => {
        popup.remove();
    });
}

async function startTransfer(peerDeviceId, filePath) {
    try {
        const isFolder = filePath.endsWith('\\') || filePath.endsWith('/') || 
                         !filePath.includes('.');
        const command = isFolder ? 'send_folder' : 'send_file';
        
        const transferId = await invoke(command, {
            peerDeviceId: peerDeviceId,
            filePath: filePath,
        });
        
        document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
        document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
        document.querySelector('[data-tab="transfers"]').classList.add('active');
        document.getElementById('transfers-section').classList.add('active');
        
        if (window.monitorTransfer) {
            window.monitorTransfer(transferId);
        }
    } catch (e) {
        console.error('Failed to send:', e);
        alert('فشل الإرسال: ' + e);
    }
}

window.sendToPeer = sendToPeer;

document.addEventListener('DOMContentLoaded', () => {
    const btn = document.getElementById('refresh-files');
    if (btn) {
        btn.addEventListener('click', refreshMyFiles);
        refreshMyFiles();
    }
});