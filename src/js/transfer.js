import { invoke } from '@tauri-apps/api/tauri';

async function monitorTransfer(transferId) {
    const interval = setInterval(async () => {
        try {
            const progress = await invoke('get_transfer_progress', { transferId: transferId });

            if (progress) {
                updateTransferUI(transferId, progress);

                if (progress.status === 'Completed' || progress.status === 'Failed' || progress.status === 'Cancelled') {
                    clearInterval(interval);
                }
            }
        } catch (e) {
            console.error('Progress check failed:', e);
            clearInterval(interval);
        }
    }, 500);
}

function updateTransferUI(transferId, progress) {
    const list = document.getElementById('transfers-list');
    let item = document.getElementById(`transfer-${transferId}`);

    if (!item) {
        item = document.createElement('div');
        item.id = `transfer-${transferId}`;
        item.className = 'transfer-item';
        list.prepend(item);
    }

    const percent = progress.total_size > 0
        ? ((progress.transferred / progress.total_size) * 100).toFixed(1)
        : 0;

    const isDone = progress.status === 'Completed' || progress.status === 'Failed' || progress.status === 'Cancelled';

    item.innerHTML = `
        <div class="transfer-header">
            <span class="transfer-name">${progress.file_name}</span>
            <button class="cancel-btn" data-transfer-id="${transferId}" ${isDone ? 'disabled' : ''}>إلغاء</button>
        </div>
        <div class="transfer-size">${formatSizeTransfer(progress.transferred)} / ${formatSizeTransfer(progress.total_size)}</div>
        <div class="transfer-speed">${progress.speed_mbps.toFixed(1)} Mbps</div>
        <div class="progress-bar">
            <div class="progress-fill" style="width: ${percent}%"></div>
        </div>
        <div class="transfer-status">${getStatusText(progress.status)}</div>
    `;

    item.querySelector('.cancel-btn')?.addEventListener('click', async () => {
        try {
            await invoke('cancel_transfer', { transferId: transferId });
        } catch (e) {
            console.error('Cancel failed:', e);
        }
    });
}

function getStatusText(status) {
    const statusMap = {
        'Pending': 'قيد الانتظار',
        'Transferring': 'جاري النقل',
        'Completed': 'مكتمل',
        'Failed': 'فشل',
        'Cancelled': 'ملغي'
    };
    return statusMap[status] || status;
}

function formatSizeTransfer(bytes) {
    if (bytes === 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
}

window.monitorTransfer = monitorTransfer;