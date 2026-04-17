# 📱 Android Device Info CLI

Ultra-fast Rust CLI for querying Android device properties via ADB.

## Install
```bash
cargo install --git https://github.com/OutrageousStorm/android-device-info-cli
```

## Usage
```bash
adi info                    # full device report
adi model                   # just model name
adi android-version         # Android version
adi properties              # all getprop values
adi properties --filter cpu # filter by keyword
adi json                    # JSON output
```

*Much faster than shell equivalents due to Rust performance.*
