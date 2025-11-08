# Polyglot Playground

![Build](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Contributions](https://img.shields.io/badge/contributions-welcome-orange)

## Overview
Polyglot Playground is a multi-language project showcasing REST APIs in Python, JavaScript, Go, and Rust. Perfect for learning and comparing languages.

## Features
- `/hello` endpoint returns a greeting.
- `/fibonacci?n=10` returns Fibonacci sequence.

## Quick Start
```bash
git clone https://github.com/yourusername/polyglot-playground.git
cd polyglot-playground
make up
```

Access APIs:
- Python: http://localhost:5000/hello
- JavaScript: http://localhost:5001/hello
- Go: http://localhost:5002/hello
- Rust: http://localhost:5003/hello

For Fibonacci:
```
http://localhost:5000/fibonacci?n=10
http://localhost:5001/fibonacci?n=10
http://localhost:5002/fibonacci?n=10
http://localhost:5003/fibonacci?n=10
```

Stop services:
```bash
make down
```

### Run Individually Without Docker
- **Python**
```bash
cd python
pip install -r requirements.txt
python app.py
```
- **JavaScript**
```bash
cd javascript
npm install
node app.js
```
- **Go**
```bash
cd go
go run main.go
```
- **Rust**
```bash
cd rust
cargo run
```

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
