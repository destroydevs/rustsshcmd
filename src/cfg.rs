use std::fs::File;
use std::io::{stdin, Write};
use std::thread;
use std::time::Duration;

pub struct ConfigYaml {
    server_ip: String,
    server_user: String,
    server_password: String
}

fn has_config() -> bool {
    File::open("config.yaml").is_ok()
}

pub fn schedule_create_config() {

    if has_config() {
        return;
    }

    let mut user = Default::default();
    let mut password = Default::default();
    let mut ip = Default::default();

    println!(" > Write user:");
    stdin().read_line(&mut user).unwrap();
    println!(" > Write password:");
    stdin().read_line(&mut password).unwrap();
    println!(" > Write IP:");
    stdin().read_line(&mut ip).unwrap();
    println!("Done! Config created! Wait...");
    thread::sleep(Duration::from_secs(1));

    let mut cfg = File::create("config.yaml").unwrap();
    let _ = &cfg.write(format!("user: {}password: {}ip: {}",user,password,ip).as_bytes()).unwrap();
}

impl ConfigYaml {
    pub fn new(server_ip: String, server_user: String, server_password: String) -> ConfigYaml {
        ConfigYaml{server_ip, server_user, server_password}
    }

    pub fn get_server_ip(&self) -> &String {
        &self.server_ip
    }
    pub fn get_server_user(&self) -> &String {
        &self.server_user
    }

    pub fn get_server_password(&self) -> &String {
        &self.server_password
    }



}