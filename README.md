# tellydb-rs 🦀

A high-performance, memory-safe, in-memory key-value database written in Rust. 
This project is a complete rewrite of the original [tellydb](https://github.com/aloima/tellydb), aiming to leverage Rust's modern systems programming capabilities.

## 🎯 Project Goal
The objective is to reimplement the core functionality of `tellydb` in Rust, replacing C's manual memory management and `epoll` with Rust's ownership model and the `mio` event loop for superior safety and performance.

## ✨ Key Features
- **Redis Compatibility**: Fully implements the RESP (Redis Serialization Protocol) to ensure compatibility with all standard Redis clients.
- **Pipeline Architecture**: Uses a decoupled design (**Network $\rightarrow$ Parser $\rightarrow$ Executor**) with internal command queues to maximize throughput.
- **Diverse Data Types**: Support for integers, strings, lists, and hash tables implemented via Rust's powerful `enum` system.
- **Persistence**: Implements a one-file binary storage system using a length-prefix pattern with atomic saving capabilities.
- **Security**: Integrated `rustls` for encrypted communication and a robust authorization system.
- **High Concurrency**: Non-blocking I/O powered by `mio` and thread-safe communication via `std::sync::mpsc`.

## 🏗 Architecture
The system is designed as a pipeline to avoid lock contention and ensure efficiency:
- **I/O Layer**: Manages TCP connections and raw byte streams (Non-blocking).
- **Parser Layer**: Transforms raw bytes into structured RESP commands.
- **Execution Layer**: A dedicated worker thread that manages the in-memory `HashMap` and handles data persistence.

## 🛠 Tech Stack
- **Language:** Rust
- **I/O Engine:** `mio`
- **Security:** `rustls`
- **Concurrency:** `std::sync::mpsc`
- **Storage:** `std::collections::HashMap`

## 📦 Workspace Structure
- `telly-proto`: RESP protocol implementation.
- `telly-core`: Storage engine and persistence logic.
- `telly-common`: Shared utilities and error handling.
- `telly-server`: The main binary and network orchestrator.
- `telly-cli`: Command-line interface for server interaction.

## 🚧 Current Status: WIP
- [x] Workspace structure setup
- [ ] RESP Parser implementation
- [ ] Basic TCP Server (Mio)
- [ ] Core Storage Engine & Persistence
- [ ] TLS Integration
- [ ] CLI Development

## 📜 License
Licensed under the **BSD-3-Clause License**.

## 🙏 Acknowledgments
Original project by [aloima](https://github.com/aloima).
