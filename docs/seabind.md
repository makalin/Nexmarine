# Seabind Protocol Specification

![Seabind Logo](https://raw.githubusercontent.com/makalin/Nexmarine/main/assets/seabind_logo.png)

**Version:** 0.1.0 (Pre release)
**Project:** [Nexmarine](https://github.com/makalin/Nexmarine)

---

## 🌊 Overview

**Seabind** is a lightweight, efficient, and extensible protocol designed for marine data communication. It bridges sensors, navigation systems, and applications with minimal overhead while maintaining clarity and flexibility.

Seabind supports both **binary** and **JSON** representations, making it ideal for embedded systems, real-time streaming, and human-readable diagnostics.

---

## ⚙️ Core Principles

- **Simplicity**: Easy to parse, minimal fields
- **Efficiency**: Compact data for low-bandwidth environments
- **Extensibility**: Custom data paths without breaking compatibility
- **Unit Awareness**: Every value includes implicit or explicit units
- **Time-Stamped**: Synchronization-ready data points
- **Protocol Agnostic Transport**: Works over WebSockets, TCP, UDP, Serial, etc.

---

## 📐 Data Structure

### JSON Representation Example
```json
{
  "timestamp": "2025-04-22T12:34:56Z",
  "path": "nav.position",
  "value": {
    "latitude": 37.7749,
    "longitude": -122.4194
  },
  "source": "GPS-01",
  "unit": "degrees"
}
```

### Binary Format (WIP)
| Field       | Type     | Size      | Description            |
|-------------|----------|-----------|------------------------|
| Header      | Bytes    | 2 Bytes   | `0xSB` (Seabind marker)|
| Timestamp   | UNIX     | 8 Bytes   | Epoch ms               |
| Path ID     | UInt16   | 2 Bytes   | Predefined path table  |
| Value       | Varied   | Dynamic   | Depends on data type   |
| Unit ID     | UInt8    | 1 Byte    | Unit from unit table   |
| CRC         | UInt16   | 2 Bytes   | Integrity check        |

> Full binary spec will be detailed in future versions.

---

## 📊 Standard Data Paths

| Path                | Description                  | Unit     |
|---------------------|------------------------------|----------|
| `nav.position`      | GPS Position                 | degrees  |
| `nav.course`        | Course Over Ground           | degrees  |
| `nav.speed`         | Speed Over Ground            | knots    |
| `env.wind.speed`    | Apparent Wind Speed          | m/s      |
| `env.wind.angle`    | Apparent Wind Angle          | degrees  |
| `env.depth`         | Depth Below Transducer       | meters   |
| `engine.rpm`        | Engine Revolutions Per Minute| RPM      |
| `tank.fuel.level`   | Fuel Level                   | percent  |

*Custom paths allowed under `ext.*` namespace.*

---

## 🚀 Transport Layers

Seabind can operate over multiple transports:

- **WebSockets** (default for Nexmarine)
- **TCP/UDP**
- **Serial Ports (RS232/RS422)**
- **MQTT** *(Planned)*

---

## 🔒 Security

- Encrypted communication via **WSS** or **TLS**
- Optional message signing for critical data streams
- Device authentication roadmap (v1.0)

---

## 📅 Roadmap

- [ ] Finalize Binary Encoding
- [ ] Define Path & Unit Registries
- [ ] Compression Support (e.g., LZ4 for batch data)
- [ ] Multi-Stream Synchronization
- [ ] Seabind SDKs for Rust, Python, and C

---

## 📖 License

This protocol is open and free to use under the **MIT License**.

---

## 📬 Contact

For suggestions, contributions, or questions:

**Mehmet Turgay AKALIN**  
[GitHub: makalin](https://github.com/makalin)
