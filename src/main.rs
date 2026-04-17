use std::process::Command;
use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Parser, Debug)]
#[command(name = "android-device-info")]
#[command(about = "Fast Android device info via ADB", long_about = None)]
struct Args {
    #[arg(short, long)]
    json: bool,
    #[arg(short, long)]
    battery: bool,
    #[arg(short, long)]
    storage: bool,
    #[arg(short, long)]
    network: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeviceInfo {
    model: String,
    android_version: String,
    api_level: String,
    serial: String,
    battery_level: u32,
    battery_temp: u32,
    storage_total: String,
    storage_available: String,
    ram_total: String,
    cpu_cores: String,
}

fn adb(cmd: &str) -> String {
    Command::new("adb")
        .args(&["shell", cmd])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn get_prop(prop: &str) -> String {
    adb(&format!("getprop {}", prop))
}

fn main() {
    let args = Args::parse();

    let info = DeviceInfo {
        model: get_prop("ro.product.model"),
        android_version: get_prop("ro.build.version.release"),
        api_level: get_prop("ro.build.version.sdk"),
        serial: adb("getprop ro.serialno"),
        battery_level: adb("dumpsys battery | grep level")
            .split('=')
            .last()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0),
        battery_temp: adb("dumpsys battery | grep temperature")
            .split('=')
            .last()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0),
        storage_total: adb("df /data | tail -1 | awk '{print $2}'"),
        storage_available: adb("df /data | tail -1 | awk '{print $4}'"),
        ram_total: adb("cat /proc/meminfo | grep MemTotal | awk '{print $2}'"),
        cpu_cores: adb("nproc"),
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&info).unwrap());
    } else {
        println!("\n📱 Android Device Info");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  Model:      {}", info.model);
        println!("  Android:    {}", info.android_version);
        println!("  API:        {}", info.api_level);
        println!("  Serial:     {}", info.serial);
        println!("  Battery:    {}% ({}°C)", info.battery_level, info.battery_temp);
        println!("  Storage:    {} / {}", info.storage_available, info.storage_total);
        println!("  RAM:        {}", info.ram_total);
        println!("  CPU cores:  {}", info.cpu_cores);
        println!();
    }
}
