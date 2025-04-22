# Seabind Protocol

## Overview

Seabind is a lightweight, efficient protocol designed specifically for marine data transmission. It combines the benefits of binary and JSON formats to provide a flexible, type-safe, and performant solution for marine data exchange.

## Message Format

### Basic Structure

```json
{
  "path": "nav.position.latitude",
  "timestamp": 1648372981234,
  "value": 45.5,
  "unit": "degrees",
  "source": "GPS"
}
```

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `path` | String | Hierarchical path identifying the data point (e.g., "nav.position.latitude") |
| `timestamp` | i64 | Unix timestamp in milliseconds |
| `value` | SeabindValue | The actual data value (see Value Types below) |
| `unit` | String (optional) | Unit of measurement (e.g., "degrees", "knots", "meters") |
| `source` | String (optional) | Source of the data (e.g., "GPS", "AIS", "Depth Sounder") |

### Value Types

Seabind supports multiple value types through the `SeabindValue` enum:

```rust
enum SeabindValue {
    Number(f64),        // For numeric values
    Text(String),       // For string values
    Boolean(bool),      // For boolean values
    Object(HashMap<String, SeabindValue>),  // For complex objects
    Array(Vec<SeabindValue>)  // For arrays of values
}
```

## Path Structure

Seabind uses a hierarchical path structure to organize data:

```
<domain>.<category>.<measurement>
```

### Common Paths

#### Navigation Data
- `nav.position.latitude` - Vessel latitude
- `nav.position.longitude` - Vessel longitude
- `nav.speed.knots` - Speed over ground in knots
- `nav.course` - Course over ground in degrees
- `nav.heading` - Vessel heading in degrees

#### Environmental Data
- `env.wind.speed` - Wind speed
- `env.wind.direction` - Wind direction
- `env.temperature.air` - Air temperature
- `env.temperature.water` - Water temperature
- `env.pressure` - Atmospheric pressure

#### AIS Data
- `ais.target.<mmsi>.position` - Position of AIS target
- `ais.target.<mmsi>.speed` - Speed of AIS target
- `ais.target.<mmsi>.course` - Course of AIS target

#### System Data
- `system.battery.voltage` - Battery voltage
- `system.battery.current` - Battery current
- `system.temperature.cpu` - CPU temperature
- `system.memory.usage` - Memory usage percentage

## Units

Seabind supports standard marine units:

| Measurement | Unit | Description |
|------------|------|-------------|
| Position | degrees | Latitude/Longitude in decimal degrees |
| Speed | knots | Speed in nautical miles per hour |
| Distance | meters | Distance in meters |
| Depth | meters | Water depth in meters |
| Temperature | celsius | Temperature in degrees Celsius |
| Pressure | hPa | Pressure in hectopascals |
| Angle | degrees | Angles in degrees (0-360) |

## Protocol Features

### 1. Type Safety
- Strong typing through the `SeabindValue` enum
- Validation of numeric ranges and units
- Structured data paths for easy validation

### 2. Efficiency
- Binary + JSON hybrid format
- Minimal overhead
- Efficient serialization/deserialization

### 3. Extensibility
- Support for custom paths
- Flexible value types
- Optional fields for backward compatibility

### 4. Security
- Optional message signing
- Source verification
- Timestamp validation

## Example Messages

### Position Update
```json
{
  "path": "nav.position.latitude",
  "timestamp": 1648372981234,
  "value": 45.5,
  "unit": "degrees",
  "source": "GPS"
}
```

### Environmental Data
```json
{
  "path": "env.wind.speed",
  "timestamp": 1648372981234,
  "value": 15.2,
  "unit": "knots",
  "source": "Wind Sensor"
}
```

### AIS Target
```json
{
  "path": "ais.target.123456789.position",
  "timestamp": 1648372981234,
  "value": {
    "latitude": 45.5,
    "longitude": -122.5
  },
  "unit": "degrees",
  "source": "AIS"
}
```

## Implementation Notes

### Rust Implementation
The protocol is implemented in Rust using:
- `serde` for serialization/deserialization
- `chrono` for timestamp handling
- Custom traits for data conversion

### NMEA Integration
Seabind includes built-in support for NMEA 0183/2000 conversion:
- Automatic parsing of NMEA sentences
- Conversion to Seabind format
- Support for common NMEA message types

### WebSocket Transport
The protocol is designed to work efficiently over WebSocket:
- JSON format for WebSocket messages
- Binary format for high-frequency data
- Automatic reconnection handling

## Best Practices

1. **Path Naming**
   - Use consistent path structures
   - Follow the domain.category.measurement pattern
   - Use lowercase with dots for separation

2. **Value Types**
   - Use appropriate value types for data
   - Prefer Number for numeric values
   - Use Object for complex data structures

3. **Units**
   - Always specify units when applicable
   - Use standard marine units
   - Document custom units

4. **Timestamps**
   - Use UTC timestamps
   - Include milliseconds for precision
   - Validate timestamps on receipt

5. **Error Handling**
   - Validate message structure
   - Handle missing optional fields
   - Log protocol errors 