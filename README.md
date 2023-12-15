# Rust Custom Console
First you need to install [rust](https://www.rust-lang.org/tools/install) in your computer, then cone this repo from the command or download the zip package. If you only want to use the console I'll create a release for all updates, and you only have to download the executable of the console.
```
git clone https://github.com/soizx/Discord-Soizx
```

# Commands
At the moment i only have the ` ping $ip ` command with flags but im working on a cloud command for download content that you want.
```
ping localhost
```
- /t	Specifies ping continue sending echo Request messages to the destination until interrupted. To interrupt and display statistics, press CTRL+ENTER. To interrupt and quit this command, press CTRL+C.
- /n <count>	Specifies the number of echo Request messages be sent. The default is 4.
- /a	Specifies reverse name resolution be performed on the destination IP address. If this operation is successful, ping displays the corresponding host name.
- /l <size>	Specifies the length, in bytes, of the Data field in the echo Request messages. The default is 32. The maximum size is 65,500.

# Example Logger File
```rs
pub fn success(message: &str, prefix: bool) {
    let timeat = current_time();
    if prefix {
        println!("\x1b[32m[{}] [SUCCESS] {}\x1b[0m", timeat, message);
    } else {
        println!("\x1b[32m{}\x1b[0m", message);
    }
}
```
