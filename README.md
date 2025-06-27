# macOS SMC Prometheus Exporter

A high-performance Prometheus exporter for macOS System Management Controller (SMC) metrics, built in Rust. Monitor your Mac's hardware sensors including CPU temperatures, power consumption, battery status, fan speeds, and more.

## 🚀 Features

- **CPU Monitoring**: Core temperatures, power consumption, and per-core temperature tracking
- **Battery Analytics**: Detailed battery information including cycles, capacity, voltage, and charging status
- **Fan Control**: Real-time fan speed monitoring with RPM and mode detection
- **GPU Temperatures**: Graphics processor thermal monitoring
- **System Sensors**: Memory, mainboard, and other component temperatures
- **Prometheus Integration**: Native Prometheus metrics format with proper labels
- **High Performance**: Built in Rust for minimal resource usage
- **Real-time Updates**: 10-second collection intervals for up-to-date metrics

## ⚠️ Compatibility

**This exporter only works on Intel-based Apple Macs with SMC (System Management Controller).**

- ✅ **Supported**: Intel MacBook Pro, MacBook Air, iMac, Mac mini, Mac Pro (2006-2020)
- ❌ **Not Supported**: Apple Silicon Macs (M1, M2, M3 series)
- ❌ **Not Supported**: Non-Apple hardware

The SMC is a hardware component specific to Intel-based Macs that manages power, thermal, and other low-level system functions. Apple Silicon Macs use different hardware management systems.

## 📊 Available Metrics

### CPU Metrics
- `cpu_temperature{unit, temp_type}` - Overall CPU temperatures (die, graphics, proximity, system_agent)
- `cpu_power{unit, power_type}` - CPU power consumption (core, dram, gfx, rail, total)
- `cpu_core_temperature{unit, temp_type, core_id}` - Individual CPU core temperatures

### Battery Metrics
- `battery_detail{unit, detail_type, battery_id}` - Detailed battery information (cycles, capacity, voltage, etc.)
- `battery_info{unit, info_type, battery_id}` - General battery status (charging, AC present, health)

### System Metrics
- `fan_speed{unit, speed_type, fan_id}` - Fan speeds and modes (actual, min, max, target, mode)
- `gpu_temperature{unit, temp_type}` - GPU temperatures (proximity, die)
- `other_temperature{unit, temp_type}` - Other system temperatures (memory, mainboard, etc.)

## 🛠️ Installation

### Prerequisites
- Intel-based Mac running macOS
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Build from Source
```bash
git clone https://github.com/yourusername/macsmc-exporter.git
cd macsmc-exporter
cargo build --release
```

### Run
```bash
cargo run --release
```

The exporter will start and expose metrics on `http://localhost:9090/metrics`.

## 🔧 Configuration

The exporter runs with sensible defaults:
- **Port**: 9090
- **Metrics endpoint**: `/metrics`
- **Collection interval**: 10 seconds
- **Log level**: INFO

## 📈 Prometheus Configuration

Add this job to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'macsmc-exporter'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 15s
```

## 📋 Example Metrics Output

```prometheus
# HELP cpu_temperature Metric for CPU temperatures
# TYPE cpu_temperature gauge
cpu_temperature{temp_type="die",unit="celsius"} 68.1171875
cpu_temperature{temp_type="graphics",unit="celsius"} 66
cpu_temperature{temp_type="proximity",unit="celsius"} 54.375

# HELP cpu_power Metric for CPU power consumption
# TYPE cpu_power gauge
cpu_power{power_type="core",unit="watts"} 11.2734375
cpu_power{power_type="total",unit="watts"} 14.3359375

# HELP battery_detail Metric for battery detailed information
# TYPE battery_detail gauge
battery_detail{battery_id="0",detail_type="cycles",unit="count"} 342
battery_detail{battery_id="0",detail_type="current_capacity",unit="mah"} 4956
battery_detail{battery_id="0",detail_type="full_capacity",unit="mah"} 5103

# HELP fan_speed Metric for fan speed information
# TYPE fan_speed gauge
fan_speed{fan_id="0",speed_type="actual",unit="rpm"} 1801
fan_speed{fan_id="0",speed_type="max",unit="rpm"} 5500
```

## 🏗️ Architecture

The exporter is built with a modular architecture:

```
src/
├── main.rs                    # Main application loop
└── libs/
    ├── metrics.rs            # Prometheus metrics definitions
    ├── prometheus.rs         # Prometheus registry setup
    ├── webserver.rs          # HTTP server for /metrics endpoint
    ├── cpu_temperature.rs    # CPU temperature collection
    ├── cpu_power.rs          # CPU power monitoring
    ├── cpu_core_temperatures.rs # Per-core temperature tracking
    ├── battery_detail.rs     # Detailed battery metrics
    ├── battery_info.rs       # General battery information
    ├── fan_speed.rs          # Fan speed monitoring
    ├── gpu_temperature.rs    # GPU temperature collection
    ├── other_temperatures.rs # Other system sensors
    └── logger.rs             # Logging configuration
```

## 🙏 Acknowledgments

This project is built on top of the excellent [**macsmc**](https://docs.rs/macsmc/latest/macsmc/index.html) crate by [@knutwalker](https://github.com/knutwalker). The macsmc crate provides the low-level SMC interface that makes this exporter possible.

**Special thanks to:**
- [@knutwalker](https://github.com/knutwalker) for creating and maintaining the [macsmc crate](https://docs.rs/macsmc/latest/macsmc/index.html)
- The Rust community for the amazing ecosystem
- The Prometheus project for the metrics standard

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.

### Development Setup
```bash
git clone https://github.com/yourusername/macsmc-exporter.git
cd macsmc-exporter
cargo build
cargo run
```

### Running Tests
```bash
cargo test
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Related Projects

- [macsmc](https://docs.rs/macsmc/latest/macsmc/index.html) - The underlying SMC client library
- [Prometheus](https://prometheus.io/) - Monitoring and alerting toolkit
- [Node Exporter](https://github.com/prometheus/node_exporter) - Similar exporter for Linux systems

## 📞 Support

If you encounter issues:
1. Check that you're running on an Intel-based Mac
2. Ensure you have the necessary permissions to access SMC
3. Check the logs for detailed error messages
4. Open an issue with your system information and error details

---

**Made with ❤️ for the macOS and Prometheus communities** 
