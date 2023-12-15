pub mod logger;

use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

struct ConsoleThread;

impl ConsoleThread {
    fn stop_console() {
        STOP.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn run() {
        logger::title("1.0");
        while !STOP.load(std::sync::atomic::Ordering::Relaxed) {
            let mut input = String::new();
            print!("\x1b[37m>\x1b[0m ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let command = input.trim();

            if command == "cls" || command == "clear" {
                logger::close("Press Enter to close...");
                ConsoleThread::stop_console();
                break;
            } else if command == "cloud" {
                logger::info("Target /cloud/GET/ is disabled due the version.", false);
            } else if command == "help" {
                logger::info("   ┣━ cloud ( Access command to the public cloud. )", false);
                logger::info("   ┣━ close ( Stop the console without exit. )", false);
                logger::info("   ┗━ ping $ip ( Make a ping with the ip you want. )", false);
            } else if command.starts_with("ping") {
                let parts: Vec<&str> = command.split_whitespace().collect();
                if parts.len() < 2 {
                    logger::error("Invalid args, 2 arg should not be Null.", true);
                } else {
                    let ip = parts[1];
                    // Logger::info(&format!("Pinging IP: {}", ip), true);

                    let mut ping_process = Command::new("ping")
                        .arg(ip)
                        .stdout(Stdio::piped())
                        .spawn()
                        .expect("Failed to execute command");

                    if let Some(stdout) = ping_process.stdout.take() {
                        let reader = io::BufReader::new(stdout);

                        for line in reader.lines() {
                            if let Ok(line_content) = line {
                                logger::info(&line_content, false);
                            }
                        }
                    }

                    ping_process
                        .wait()
                        .expect("Failed to wait for command completion");
                }
            } else {
                logger::error("Invalid Command", true);
            }
        }

        // Wait for Enter to close the console
        let mut enter_input = String::new();
        io::stdin().read_line(&mut enter_input).unwrap();
    }
}

static STOP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() {
    if cfg!(target_os = "windows") && !std::path::Path::new("C:/Users/daniz/Desktop/Rust-Context/main.exe").exists() {
        // Install the app if it's not already installed
        Command::new("cargo")
            .arg("install")
            .arg("--path")
            .arg(".")
            .spawn()
            .expect("Failed to install the app.");
    }

    // Your app-specific logic to run when already installed
    ConsoleThread::run();
}