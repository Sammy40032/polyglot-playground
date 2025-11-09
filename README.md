# Polyglot Playground

![License](https://img.shields.io/badge/license-MIT-blue)

## Overview
Polyglot Playground is a multi-language project showcasing REST APIs in Python, JavaScript, Go, and Rust. You can use it to compare, and learn languages of these 4!
## Features
Add these to end of the url:
- `/hello` endpoint returns greeting with the name of the language.
- `/fibonacci?n=10` returns Fibonacci sequence. (Replace the 10 with the number of your choice)

## Quick Start
### Windows Users
- Ensure Docker Desktop is installed and running.
- Navigate to the project folder and run:
```powershell
docker compose up --build
```
To stop:
```powershell
docker compose down
```
If `make` is not available, skip it and use the above commands.

### Linux/macOS
```bash
make up
```
To stop:
```bash
make down
```

## Endpoints
- Python: http://localhost:5000/hello
- JavaScript: http://localhost:5001/hello
- Go: http://localhost:5002/hello
- Rust: http://localhost:5003/hello

Fibonacci example:
```
http://localhost:5000/fibonacci?n=10 (PYTHON PORT)
```

## Troubleshooting
### Rust Installation
1. Download Rust installer from https://rustup.rs
2. Install and restart PowerShell.
3. Verify installation:
```powershell
rustup --version
cargo --version
rustc --version
```
4. Build locally (optional):
```powershell
cd rust
cargo build
```
This creates `Cargo.lock` and speeds up Docker builds.

### Python Instalation
1. Download Python Installer from https://www.python.org/downloads/
2. Restart Powershell
3. Verify Installation:
'''powershell
python --version
'''
### Docker Desktop Tips
- **View in Docker Desktop**: See containers visually.
- **View Config**: Inspect Compose configuration.
- **Enable Watch**: Auto-rebuild on file changes.
Attach ZIP file and publish.

## License
MIT
