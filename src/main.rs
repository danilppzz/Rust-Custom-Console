pub mod Logger;

use std::{io::{self, Write}};

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

            if command == "stop" {
                println!("Press Enter to close...");
                ConsoleThread::stop_console();
                break;
            } else if command == "done" {
                Logger::success("Done")
            } else {
                Logger::error("Invalid Command")
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