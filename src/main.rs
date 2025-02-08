mod cfg;

use ::config::{Config, File};
use expectrl::{spawn, Regex};
use std::io::{self, BufRead, Read, Write};
use std::thread;

fn main() {

    cfg::schedule_create_config();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    stdout.flush().unwrap();

    loop {
        stdout.flush().unwrap();

        let settings = Config::builder()
            .add_source(File::with_name("config.yaml"))
            .build()
            .unwrap();

        let ip = settings.get::<String>("ip").unwrap();
        let user = settings.get::<String>("user").unwrap();
        let password = settings.get::<String>("password").unwrap();

        let cfg = cfg::ConfigYaml::new(ip.clone(), user.clone(), password.clone());

        let input = format!("{} {} {}", cfg.get_server_user(), cfg.get_server_ip(),cfg.get_server_password());
        let args: Vec<&str> = input.split_whitespace().collect();

        let ssh_command = format!("ssh {}@{}", args[0], args[1]);
        let password = args[2].to_string();

        match spawn(&ssh_command) {
            Ok(mut session) => {
                println!("Connecting...");
                session.expect(Regex("password:")).unwrap();
                session.send_line(&password.trim()).unwrap();

                let mut reader = session.get_stream().try_clone().unwrap();
                thread::spawn(move || {
                    let mut buffer = [0u8; 1024];
                    loop {
                        let n = reader.read(&mut buffer).unwrap();
                        if n == 0 { break; }
                        let out = String::from_utf8_lossy(&buffer[..n]);
                        print!("\x1b[93m{}", out);
                        io::stdout().flush().unwrap();
                    }
                });

                loop {
                    let mut user_input = String::new();
                    stdin.lock().read_line(&mut user_input).unwrap();
                    session.send_line(user_input.trim()).unwrap();
                }
            }
            Err(e) => eprintln!("SSH Error: {}", e),
        }
    }
}