# 📱 Android Device Info CLI

Ultra-fast Rust CLI to gather Android device information via ADB. Single binary, no dependencies.

## Install
```bash
cargo install --path .
# or download pre-built binary from releases
```

## Usage
```bash
# Full device report
android-device-info

# JSON output for scripting
android-device-info --json > device.json

# Specific info only
android-device-info --battery
android-device-info --storage
android-device-info --network
```

## Performance
- Rust binary: ~50ms startup
- Parallel info gathering
- Minimal memory footprint
