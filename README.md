# Polyglot Playground

![License](https://img.shields.io/badge/license-MIT-blue)

## Overview
Polyglot Playground is a multi-language project showcasing REST APIs in Python, JavaScript, Go, and Rust. Perfect for learning and comparing languages.
## Features
Add these to end of the url:
- `/hello` endpoint returns a greeting.
- `/fibonacci?n=10` returns Fibonacci sequence.


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
http://localhost:5000/fibonacci?n=10
```

## Troubleshooting
### Rust Installation (Optional but Recommended)
If you want to build Rust locally or check Cargo version:
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

### Go Troubleshooting
Ensure `go.mod` exists (included in this project). If missing, create:
```
module polyglot-playground-go
go 1.21
```

### Docker Desktop Tips
- **View in Docker Desktop**: See containers visually.
- **View Config**: Inspect Compose configuration.
- **Enable Watch**: Auto-rebuild on file changes.

## GitHub Upload Guide
1. Create `.gitignore`:
```
__pycache__/
node_modules/
target/
bin/
.DS_Store
```
2. Initialize Git:
```bash
git init
git add .
git commit -m "Initial commit"
```
3. Push to GitHub:
```bash
git branch -M main
git remote add origin https://github.com/yourusername/polyglot-playground.git
git push -u origin main
```

## Release Instructions
- Tag version: `v1.0.0`
- Title: `Polyglot Playground v1.0.0`
- Description:
```
Initial release of Polyglot Playground:
- Multi-language REST APIs (Python, JavaScript, Go, Rust)
- Docker Compose setup
- Makefile for easy commands
- README with instructions
```
Attach ZIP file and publish.

## License
MIT
