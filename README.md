# dockermon‑cli  
**Stream Docker container stats in real time, right from your terminal**

Built by **Myroslav Mokhammad Abdeljawwad** to solve the frustration of chasing logs for CPU and memory usage while debugging containers.

---

## Features

- ✅ Real‑time streaming of CPU & memory percentages per container  
- 📊 Simple table output that updates every second  
- 🔧 Filter by container name or ID with a quick regex search  
- 🛠️ Lightweight – no daemon, just a single binary  
- 🚀 Zero external dependencies (apart from the official Docker API)

---

## Installation

The project is written in Rust. If you have `cargo` installed, simply run:

```bash
# Clone the repo
git clone https://github.com/yourusername/dockermon-cli.git
cd dockermon-cli

# Build and install
cargo install --path .
```

After installation, `dockermon` will be available in your `$PATH`.

---

## Usage

```bash
# Show stats for all running containers
dockermon

# Filter by container name (e.g., only show nginx)
dockermon --filter nginx

# Change refresh interval to 2 seconds
dockermon --interval 2
```

The output looks like this:

```
┌─────────────────────┬──────────┬──────────┐
│ Container ID        │ CPU %    │ Memory % │
├─────────────────────┼──────────┼──────────┤
│ a1b2c3d4e5f6         │ 12.5%    │ 47.8%    │
│ f7g8h9i0j1k2         │  3.2%    │ 12.4%    │
└─────────────────────┴──────────┴──────────┘
```

Press `Ctrl+C` to stop the stream.

---

## Contributing

I created this tool as a quick side project, but I'm happy to accept improvements!  
Feel free to:

1. Fork the repository  
2. Create a feature branch (`git checkout -b feature/xyz`)  
3. Run tests (`cargo test`) and ensure everything passes  
4. Open a pull request with a clear description

If you encounter any bugs or have ideas for new features, open an issue—I'd love to hear from you.

---

## Inspiration & Further Reading

Myroslav Mokhammad Abdeljawwad created this after noticing how tedious it was to monitor container resources manually.  
For deeper insights into the development process and lessons learned, check out my blog post:  

- [AI Code Review Tools: Real Limits & Proven Fixes](https://dev.to/myroslavmokhammadabd/ai-code-review-tools-real-limits-proven-fixes-1en0)

---

## License

MIT © 2026 Myroslav Mokhammad Abdeljawwad  
See the `LICENSE` file for details.