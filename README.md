# 🧠 A minimalistic CLI agent for local LLM server

A lightweight and efficient command-line interface (CLI) agent designed to interact with any **OpenAI-compatible API local server**.

## ✨ Features
* ⚡ Fast CLI interaction with OpenAI-compatible APIs
* ⚡ Works with any local server implementing the OpenAI API spec (e.g. llama.cpp, vLLM, etc.)
* ⚡ Color console
* ⚡ Code syntax highlighting

## 🚀 Usage examples
### Code highlighting
<img width="1133" height="1050" alt="code" src="https://github.com/user-attachments/assets/defd145e-dc9a-4305-8313-e13531e04483" />

### Table rendering
<img width="1474" height="958" alt="table" src="https://github.com/user-attachments/assets/6702f7ac-b492-454a-917a-402b34defce6" />

## 📦 Installation
```bash
git clone https://github.com/hardglitch/llm-client.git
cd llm-client
cargo build --release
```

## ⚙️ Configuration
```bash
-p, --port - server port (8080 by default)
-l, --log-file - log file (e.g. -l "log.log" by default)
--log-size - the log file size (e.g. --log-file 104857600 by default)
--show-stat - show token stat (e.g. "[tokens: 97/ total: 543/ context: 16384]"). Disabled by default

exit - exit from programm
```

## 📜 License
MIT
