use picogpiod::client::Client;

use std::{env, fs};

use toml::{Table, Value};

const CONFIG_FILE: &str = "/etc/picogpiod.toml";

fn main() {
    let mut args = env::args();
    let binary_name = args.next().unwrap();
    let help = || {
        println!("Usage: {binary_name} [command...]");
        println!();
        println!("Modes:");
        println!("  help -> Display this");
        println!("  float [pin]          -> cease control of [pin]");
        println!("  <high/low> [pin]     -> set [pin] high or low");
        println!("  getpdn [pin]         -> read [pin] with pulldn,");
        println!("                          print HIGH or LOW");
        println!("  getpup [pin]         -> read [pin] with pullup,");
        println!("                          print HIGH or LOW");
        println!("  getana [pin]         -> read [pin] with ADC,");
        println!("                          result is usually 10bit");
    };
    let Some(command) = args.next() else {
        help();
        return;
    };
    if command == "help" {
        help();
        return;
    }

    let config = fs::read_to_string(CONFIG_FILE).expect("No config file");
    let config: Table = toml::from_str(&config).expect("Invalid config file");

    let socket = config
        .get("socket")
        .and_then(Value::as_table)
        .expect("missing socket table in config");
    let sockfile = socket
        .get("file")
        .and_then(Value::as_str)
        .unwrap_or("/run/picogpiod");

    // let sockfile = "/run/picogpiod";

    let Some(pin) = args.next().and_then(|x| x.parse::<u16>().ok()) else {
        help();
        return;
    };

    let mut client = Client::new(sockfile).unwrap();

    match command.as_str() {
        "float" => client.float(pin).unwrap(),
        "high" => client.write_digital(pin, true).unwrap(),
        "low" => client.write_digital(pin, false).unwrap(),
        "getpdn" => {
            if client.read_pulldn(pin).unwrap() {
                println!("HIGH")
            } else {
                println!("LOW")
            }
        }
        "getpup" => {
            if client.read_pullup(pin).unwrap() {
                println!("HIGH")
            } else {
                println!("LOW")
            }
        }
        "getana" => println!("{}", client.read_analog(pin).unwrap()),

        _ => {
            help();
            client.disconnect();
            return;
        }
    }
    client.disconnect();
}
