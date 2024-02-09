use crate::fan::set_fan;
use crate::temperature::get_temperature;
use crate::{config::parse_json, fan::reset_fan};
use calculate::calculate_curve_value;
use core::time;
use fan::init_fan;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::process::exit;
use std::thread;

mod calculate;
mod config;
mod fan;
mod temperature;

fn main() {
    let signals = Signals::new(&[SIGINT]);
    let mut signalss = match signals {
        Err(_e) => {
            eprintln!("Error gettings signals");
            exit(1);
        }
        Ok(v) => v,
    };

    thread::spawn(move || {
        for sig in signalss.forever() {
            println!("Received signal {:?}", sig);
            reset_fan();
            exit(0);
        }
    });

    let raw_config = parse_json();
    let config = match raw_config {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            exit(1);
        }
    };

    let timeout = time::Duration::from_secs(1);
    let mut last_value = -1;

    let init_fan_res = init_fan();
    if !init_fan_res {
        eprintln!("Couldn't enter custom fan mode for gpu");
        exit(1);
    }
    loop {
        let temp = match get_temperature() {
            None => {
                eprintln!("Error reading values");
                reset_fan();
                exit(1);
            }
            Some(v) => v,
        };

        let val = calculate_curve_value(&config, temp);
        if val != last_value {
            println!("Setting fan to: {}, temp: {}", val, temp);
            set_fan(val);
            last_value = val;
        }

        thread::sleep(timeout);
    }
}
