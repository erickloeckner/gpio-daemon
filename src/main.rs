use std::{env, process};
use std::fs;
use std::process::Command;
use std::time::{Duration, Instant};
use std::thread::sleep;

use gpio_cdev::{Chip, EventRequestFlags, EventType, LineRequestFlags};

use serde::Deserialize;

#[derive(Clone, Deserialize)]
struct Config {
    main: Main,
    gpio: Gpio,
    action: Action,
}

#[derive(Clone, Deserialize)]
struct Main {
    debug: bool,
    active_high: bool,
    debounce_time: f32,
}

#[derive(Clone, Deserialize)]
struct Gpio {
    chip: String,
    pin: u32,
}

#[derive(Clone, Deserialize)]
struct Action {
    command: String,
    cooldown: f32,
}

fn main() -> std::result::Result<(), gpio_cdev::Error> {
    let config_path = env::args().nth(1).unwrap_or_else(|| {
        println!("no config file specified");
        process::exit(1);
    });
    let config_raw = fs::read_to_string(&config_path).unwrap_or_else(|err| {
        println!("error reading config: {}", err);
        process::exit(1);
    });
    let config: Config = toml::from_str(&config_raw).unwrap_or_else(|err| {
        println!("error parsing config: {}", err);
        process::exit(1);
    });

    let mut chip = Chip::new(&config.gpio.chip)?;
    let pin = chip
        .get_line(config.gpio.pin)?;
    let mut last_event = 0;
    let debounce_time = (config.main.debounce_time.max(0.0) * 1000000000.0) as u64;
    let event_type = match config.main.active_high {
        true => EventType::RisingEdge,
        false => EventType::FallingEdge,
    };

    for event in pin.events(
        LineRequestFlags::INPUT,
        EventRequestFlags::BOTH_EDGES,
        "gpioevents",
    )? {
        match event {
            Ok(e) => {
                if config.main.debug { println!("[ event data ] {:?}", e) }

                if e.timestamp() - last_event < ((config.action.cooldown * 1000000000.0) as u64) { continue }

                if e.event_type() == event_type && e.timestamp() - last_event > debounce_time {
                    if config.main.debug { println!("[ input active ]") }
                    last_event = e.timestamp();
                    let trigger_start = Instant::now();
                    if let Ok(mut child) = Command::new("sh").arg("-c").arg(&config.action.command).spawn() {
                        if config.main.debug { println!("[ executing command '{}' ]", &config.action.command) }
                        let _ = child.wait();
                    }

                    let cooldown_remaining = config.action.cooldown - trigger_start.elapsed().as_secs_f32();
                    if cooldown_remaining > 0.0 {
                        if config.main.debug { println!("[ cooling down for {} seconds ]", cooldown_remaining) }
                        sleep(Duration::from_secs_f32(cooldown_remaining));
                    }
                }
            }
            Err(_) => (),
        }
    }
    
    Ok(())
}
