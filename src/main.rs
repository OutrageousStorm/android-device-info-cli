use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process::Command;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Info,
    Model,
    AndroidVersion,
    Properties { #[arg(long)] filter: Option<String> },
    Json,
}

fn adb(cmd: &str) -> String {
    let output = Command::new("adb")
        .args(&["shell", cmd])
        .output()
        .expect("Failed to run adb");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn getprop(key: &str) -> String {
    adb(&format!("getprop {}", key))
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Info => {
            println!("{}", "📱 Android Device Info".cyan().bold());
            println!("{}", "─".repeat(40).dimmed());
            println!("{:<20} {}", "Model:".bold(), getprop("ro.product.model"));
            println!("{:<20} {}", "Brand:".bold(), getprop("ro.product.brand"));
            println!("{:<20} {}", "Android:".bold(), getprop("ro.build.version.release"));
            println!("{:<20} {}", "API Level:".bold(), getprop("ro.build.version.sdk"));
            println!("{:<20} {}", "Codename:".bold(), getprop("ro.product.device"));
            println!("{:<20} {}", "CPU ABI:".bold(), getprop("ro.product.cpu.abi"));
            println!("{:<20} {}", "Fingerprint:".bold(), getprop("ro.build.fingerprint"));
        }
        Commands::Model => println!("{}", getprop("ro.product.model")),
        Commands::AndroidVersion => println!("{}", getprop("ro.build.version.release")),
        Commands::Properties { filter } => {
            let props = adb("getprop");
            for line in props.lines() {
                if let Some(ref f) = filter {
                    if line.to_lowercase().contains(&f.to_lowercase()) {
                        println!("{}", line.dimmed());
                    }
                } else {
                    println!("{}", line.dimmed());
                }
            }
        }
        Commands::Json => {
            let model = getprop("ro.product.model");
            let android = getprop("ro.build.version.release");
            let json = format!(
                r#"{{"model":"{}","android":"{}","api":"{}","brand":"{}"}}"#,
                model, android, getprop("ro.build.version.sdk"), getprop("ro.product.brand")
            );
            println!("{}", json);
        }
    }
}
