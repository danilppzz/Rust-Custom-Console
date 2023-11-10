use std::time::{SystemTime, UNIX_EPOCH};

fn current_time() -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let hours = (since_the_epoch.as_secs() / 3600) % 24;
    let minutes = (since_the_epoch.as_secs() % 3600) / 60;
    let seconds = since_the_epoch.as_secs() % 60;

    format!("{:02}:{:02}:{:02}", hours+1, minutes, seconds)
}


pub fn info(message: &str, prefix: bool) {
    let timeat = current_time();
    if prefix {
        println!("\x1b[37m[{}] [INFO] {}\x1b[0m", timeat, message);
    } else {
        println!("\x1b[37m{}\x1b[0m", message);
    }
    
}

pub fn warn(message: &str, prefix: bool) {
    let timeat = current_time();
    if prefix {
        println!("\x1b[33m[{}] [WARNING] {}\x1b[0m", timeat, message);
    } else {
        println!("\x1b[33m{}\x1b[0m", message); 
    }
    
}

pub fn error(message: &str, prefix: bool) {
    let timeat = current_time();
    if prefix {
        println!("\x1b[31m[{}] [ERROR] {}\x1b[0m", timeat, message);
    } else {
        println!("\x1b[31m{}\x1b[0m", message);
    }
    
}

pub fn success(message: &str, prefix: bool) {
    let timeat = current_time();
    if prefix {
        println!("\x1b[32m[{}] [SUCCESS] {}\x1b[0m", timeat, message);
    } else {
        println!("\x1b[32m{}\x1b[0m", message);
    }
}

pub fn close(message: &str) {
    println!("\x1b[33m{}\x1b[0m", message);
}