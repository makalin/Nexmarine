# Nexmarine

![Nexmarine Logo](https://raw.githubusercontent.com/makalin/Nexmarine/main/assets/nexmarine_logo.png)

**Nexmarine** is a blazing-fast, cross-platform marine data server written in Rust. Designed as a modern alternative to bulky middleware solutions, Nexmarine leverages the lightweight and efficient **Seabind** protocol to handle vessel data with precision, reliability, and speed.

## 🚀 Features

- ⚡ **Rust-Powered Performance** — Safe, concurrent, and memory-efficient
- 🌊 **Seabind Protocol** — Custom, lightweight, and extensible marine data protocol
- 📡 **Real-Time Data Streaming** — WebSocket & REST API support
- 🌐 **Cross-Platform** — Runs on Linux, macOS, Windows, Raspberry Pi, and embedded systems
- 🔌 **Modular & Extensible** — Plugin-ready architecture without the bloat
- ⚙️ **Supports NMEA 0183/2000**, GPS, AIS, environmental sensors, and more
- 📝 **Simple Configuration** — Human-readable TOML config files
- 📊 **Dashboard Ready** — Modern web interface for real-time monitoring

## 📦 Installation

### Using Cargo

```bash
cargo install nexmarine
```

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/makalin/Nexmarine.git
cd Nexmarine
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
./target/release/nexmarine --config nexmarine.toml
```

## 🛠️ Configuration

Create a `nexmarine.toml` file:

```toml
[input]
source = "/dev/ttyUSB0"  # Serial port for NMEA data
format = "nmea0183"      # Input format (nmea0183 or nmea2000)

[output]
websocket = true         # Enable WebSocket server
rest_api = true         # Enable REST API
seabind_log = "./logs/vessel_data.sbnd"  # Optional log file
```

## 🌐 Web Dashboard

Nexmarine includes a modern web dashboard for real-time monitoring:

1. Access the dashboard at `http://localhost:3000`
2. Features include:
   - Real-time position, speed, and navigation data
   - Environmental sensor readings
   - AIS target tracking
   - System status monitoring
   - Customizable layouts

## 🔌 API Reference

### WebSocket API

Connect to `ws://localhost:8000/ws` to receive real-time data streams.

Example message format:
```json
{
  "path": "nav.position.latitude",
  "timestamp": 1648372981234,
  "value": 45.5,
  "unit": "degrees",
  "source": "GPS"
}
```

### REST API

- `GET /api/status` - Server status
- `GET /api/data` - Latest data snapshot
- `POST /api/data` - Send data to server

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

For integration tests:

```bash
cargo test --features integration-tests
```

## 📖 Documentation

Generate and view the documentation:

```bash
cargo doc --no-deps --open
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Mehmet Turgay AKALIN**  
🔗 [github.com/makalin](https://github.com/makalin)  
🚀 Passionate about Rust, marine tech, and open-source innovation.
