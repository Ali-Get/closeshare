
# CloseShare

**Fast, Secure, Local Network File Sharing**

⚡ Transfer files at **120+ MB/s** on Gigabit networks  
📁 Support for **1TB+ folders** with parallel streaming  
🔒 **End-to-end encryption** with AES-256-GCM  

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-1.6-blue.svg)](https://tauri.app)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-lightgrey.svg)]()

</div>

---

## ✨ Features

- **Automatic Device Discovery** — UDP broadcast finds all CloseShare devices on your local network instantly
- **Group Isolation** — Optional group codes keep your network private in crowded environments
- **Parallel Transfers** — Up to 8 simultaneous TCP connections per file for maximum throughput
- **On-the-fly Compression** — zstd compression during transfer boosts effective speed by 30-200%
- **End-to-End Encryption** — AES-256-GCM encryption for all data transfers
- **Cross-Platform** — Native binaries for Linux and Windows
- **Clean Dark Theme GUI** — Simple and intuitive Tauri-based interface
- **Persistent Identity** — UUID-based device identification survives IP changes
- **Resume Support** — Failed chunks are retransmitted individually, not the entire file

---

## 🔧 How It Works
Discovery Phase (UDP):
Device A ──broadcast──→ Device B
Device B ──response───→ Device A

Exchange Phase (TCP):
Device A ──request file list──→ Device B
Device B ──file metadata──────→ Device A

Transfer Phase (TCP Parallel):
Device A ══8 connections══ Device B
Chunks compressed with zstd
Integrity verified with Blake3

---

## 🚀 Performance

| Network | Speed | 1TB Transfer Time |
|---------|-------|-------------------|
| **1 Gbps** | ~112 MB/s | ~2.5 hours |
| **2.5 Gbps** | ~250 MB/s | ~1 hour |
| **10 Gbps** | ~800-1100 MB/s | ~15-18 minutes |

> Note: Actual speeds depend on disk I/O and file compressibility.

---

## 📥 Installation

### Download Pre-built Binaries

Go to [GitHub Releases](https://github.com/shadow-601/closeshare/releases) and download:

- **Windows:** `.msi` or `.exe` installer
- **Linux:** `.deb` or `.AppImage`

### Build from Source

```bash
# Prerequisites
# Install Rust: https://rustup.rs
# Install Node.js 18+: https://nodejs.org
# Linux: sudo apt install libwebkit2gtk-4.0-dev build-essential libssl-dev

git clone https://github.com/shadow-601/closeshare.git
cd closeshare
npm install
npm run tauri dev      # Development
npm run tauri build    # Production build

💻 Usage
First Time Setup
Open CloseShare

Set a display name for your device

Optionally set a group code to isolate your devices

Choose a shared folder

Sending Files
Method 1: Go to "My Files" tab → select file → Send

Method 2: Click a peer in "Devices" tab → request file

Method 3: Drag & drop files onto a peer card

Monitoring Transfers
Click the "Transfers" tab to see live progress, speed, and ETA

🔒 Security
AES-256-GCM authenticated encryption

Blake3 checksums for data integrity

Group codes prevent unauthorized discovery

UUID-based persistent device identification

📁 Project Structure
closeshare/
├── src-tauri/src/
│   ├── protocol/        # Network message definitions
│   ├── discovery/       # UDP device discovery
│   ├── exchange/        # File metadata exchange
│   ├── transfer/        # Parallel file transfer engine
│   ├── security/        # Encryption & authentication
│   ├── core/            # Configuration & state
│   └── commands/        # Tauri backend commands
├── src/
│   ├── index.html       # Main UI
│   ├── styles/          # CSS stylesheets
│   └── js/              # Frontend JavaScript
├── package.json
└── vite.config.js

⚙️ Configuration

Configuration stored in:

Windows: %APPDATA%\closeshare\config.json

Linux: ~/.config/closeshare/config.json

Setting	Default	Description
display_name	Hostname	Name shown to peers
group_code	""	Shared secret for isolation
shared_folder	Home dir	Root folder for sharing
discovery_port	19999	UDP discovery port
transfer_port	20000	TCP transfer port
max_parallel_connections	8	Parallel TCP streams
chunk_size_mb	1	Size per transfer chunk

🤝 Contributing
Contributions are welcome!

Fork the repository

Create a feature branch

Commit your changes

Open a Pull Request

📄 License
This project is licensed under the MIT License — see the LICENSE file for details.

🙏 Acknowledgments
Built with:

Rust — Language

Tokio — Async runtime

Tauri — Desktop framework

Blake3 — Hashing

zstd — Compression

Inspired by: Magic Wormhole, LAN Share, Snapdrop, LocalSend

<div align="center">
Made for fast, simple, secure local file sharing.

Report Bug · Request Feature

</div> ```

