import { invoke } from '@tauri-apps/api/tauri';

window.invoke = invoke;

document.addEventListener('DOMContentLoaded', () => {
    document.querySelectorAll('.tab').forEach(tab => {
        tab.addEventListener('click', () => {
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));

            tab.classList.add('active');
            const section = document.getElementById(`${tab.dataset.tab}-section`);
            if (section) section.classList.add('active');
        });
    });

    async function updateStatus() {
        try {
            const peers = await invoke('get_peers');
            document.getElementById('status').textContent = `${peers.length} أجهزة متصلة`;
        } catch (e) {
            document.getElementById('status').textContent = 'غير متصل';
        }
    }

    setInterval(updateStatus, 3000);
    updateStatus();
});