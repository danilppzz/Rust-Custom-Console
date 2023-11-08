pub mod Logger;

use std::{io::{self, BufRead, Write}};
use std::process::{Command, Stdio};

struct ConsoleThread;

impl ConsoleThread {
    fn stop_console() {
        STOP.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn run() {
        println!("soizx@1.0");
        while !STOP.load(std::sync::atomic::Ordering::Relaxed) {
            let mut input = String::new();
            print!("\x1b[37m>\x1b[0m ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let command = input.trim();

            if command == "cls" || command == "clear" {
                Logger::warn("Press Enter to close...", true);
                ConsoleThread::stop_console();
                break;
            } else if command == "help" {
                Logger::warn("Console Commands", true);
                Logger::info("- help", false);
                Logger::info("- stop", false);
                Logger::info("- ping $ip", false);
            } else if command.starts_with("ping") {
                let parts: Vec<&str> = command.split_whitespace().collect();
                if parts.len() < 2 {
                    Logger::error("Invalid ping command. Usage: ping [IP]", true);
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
                                Logger::info(&line_content, false);
                            }
                        }
                    }
            
                    ping_process.wait().expect("Failed to wait for command completion");
                }
            } else {
                Logger::error("Invalid Command", true);
            }
        }

        // Wait for Enter to close the console
        let mut enter_input = String::new();
        io::stdin().read_line(&mut enter_input).unwrap();
    }
}

static STOP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn main() {
    ConsoleThread::run();
}