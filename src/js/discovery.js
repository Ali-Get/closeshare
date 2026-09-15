import { invoke } from '@tauri-apps/api/tauri';

async function refreshPeers() {
    try {
        const peers = await invoke('get_peers');
        const list = document.getElementById('peers-list');
        document.getElementById('peers-count').textContent = `${peers.length} أجهزة متصلة`;

        list.innerHTML = peers.map(peer => `
            <div class="peer-card" data-device-id="${peer.device_id}">
                <span class="status ${peer.status === 'Online' ? 'online' : 'offline'}"></span>
                <div class="name">${peer.display_name}</div>
                <div class="hostname">${peer.hostname}</div>
                <div class="ip">${peer.socket_addr}</div>
            </div>
        `).join('');

        list.querySelectorAll('.peer-card').forEach(card => {
            card.addEventListener('click', () => {
                const deviceId = card.dataset.deviceId;
                const peerName = card.querySelector('.name').textContent;
                if (window.sendToPeer) {
                    window.sendToPeer(deviceId, peerName);
                }
            });
        });
    } catch (e) {
        console.error('Failed to refresh peers:', e);
    }
}

document.addEventListener('DOMContentLoaded', () => {
    const btn = document.getElementById('refresh-peers');
    if (btn) {
        btn.addEventListener('click', refreshPeers);
        refreshPeers();
        setInterval(refreshPeers, 5000);
    }
});