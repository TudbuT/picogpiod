use picogpiod::daemon::start;

use std::fs;

use toml::{Table, Value};

const CONFIG_FILE: &str = "/etc/picogpiod.toml";

fn main() {
    let config = fs::read_to_string(CONFIG_FILE).expect("No config file");
    let config: Table = toml::from_str(&config).expect("Invalid config file");
    let port = config
        .get("port")
        .and_then(Value::as_table)
        .expect("missing port table in config");

    let file = port
        .get("file")
        .and_then(Value::as_str)
        .unwrap_or("/dev/ttyACM0");
    let baud = port
        .get("baud")
        .and_then(Value::as_integer)
        .unwrap_or(2_000_000);

    let socket = config
        .get("socket")
        .and_then(Value::as_table)
        .expect("missing socket table in config");
    let sockfile = socket
        .get("file")
        .and_then(Value::as_str)
        .unwrap_or("/run/picogpiod");

    start(file, baud as u32, sockfile);
}
