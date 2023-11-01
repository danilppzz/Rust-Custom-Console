fn main() {    
    Logger::info("This is an informational message");
    Logger::warn("This is a warning message");
    Logger::error("This is an error message");
    Logger::success("This is a success message");
    Logger::new(4);
}

struct Logger;

impl Logger {
    fn new(times: u8) {
        for _ in 0..times {
            println!("\x1b[37m\x1b[0m");
        }
    }
    fn info(message: &str) {
        println!("\x1b[37m [INFO] {}\x1b[0m", message);
    }

    fn warn(message: &str) {
        println!("\x1b[33m [WARNING] {}\x1b[0m", message);
    }

    fn error(message: &str) {
        println!("\x1b[31m [ERROR] {}\x1b[0m", message);
    }

    fn success(message: &str) {
        println!("\x1b[32m [SUCCESS] {}\x1b[0m", message);
    }
}