import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';

async function loadSettings() {
    try {
        const config = await invoke('get_config');
        document.getElementById('display-name').value = config.display_name || '';
        document.getElementById('group-code').value = config.group_code || '';
        document.getElementById('shared-folder').value = config.shared_folder || '';
        document.getElementById('discovery-port').value = config.discovery_port || 19999;
        document.getElementById('max-connections').value = config.max_parallel_connections || 8;
    } catch (e) {
        console.error('Failed to load settings:', e);
    }
}

document.addEventListener('DOMContentLoaded', () => {
    const browseBtn = document.getElementById('browse-folder');
    if (browseBtn) {
        browseBtn.addEventListener('click', async () => {
            try {
                const selected = await open({ directory: true });
                if (selected) {
                    document.getElementById('shared-folder').value = selected;
                }
            } catch (e) {
                console.error('Folder selection failed:', e);
            }
        });
    }

    const form = document.getElementById('settings-form');
    if (form) {
        form.addEventListener('submit', async (e) => {
            e.preventDefault();

            const config = {
                display_name: document.getElementById('display-name').value,
                group_code: document.getElementById('group-code').value,
                shared_folder: document.getElementById('shared-folder').value,
                discovery_port: parseInt(document.getElementById('discovery-port').value) || 19999,
                transfer_port: 20000,
                discovery_broadcast_addr: '255.255.255.255',
                max_parallel_connections: parseInt(document.getElementById('max-connections').value) || 8,
                chunk_size_mb: 1,
                auto_discover: true,
            };

            try {
                await invoke('update_config', { config: config });
                alert('تم حفظ الإعدادات');
            } catch (e) {
                console.error('Failed to save settings:', e);
                alert('فشل حفظ الإعدادات: ' + e);
            }
        });
    }

    loadSettings();
});