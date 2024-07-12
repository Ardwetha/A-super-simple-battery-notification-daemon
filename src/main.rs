pub mod config;

use std::{i32, process::Command, time::Duration};

fn main() {

    let config = match config::ConfigData::load() {
        Err(e) => {
            println!("Error reading config");
            println!("{}",e);
            return;
        },
        Ok(n) => n,
    };
    let base_notify_level = config.alerts.len() as i32;
    let mut notify_level = config.alerts.len() as i32;
    loop {
        let res = Command::new("sh").arg("-c").arg(format!("cat /sys/class/power_supply/{}/capacity", config.battery_name)).output().unwrap();
        let mut bat_str = String::from_utf8(res.stdout).unwrap();
        bat_str.pop();
        let bat_val:u16 = bat_str.parse().unwrap();
        for (i, val) in config.alerts.iter().enumerate() {
            if val.capacity_level >= bat_val && notify_level == base_notify_level - i as i32 {
                notify_level -= 1;
                Command::new("sh").arg("-c").arg(format!("notify-send -u {} {}",val.warning_level, val.warning_text)).output().unwrap();
            }
        } 
        std::thread::sleep(Duration::from_secs(10));
        
    }
}
