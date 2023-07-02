use std::env;
use std::process;

enum Mode {
    Up,
    Down,
    Toggle,
    Set,
}

enum Capability {
    Volume,
    Brightness,
}

struct Bundle {
    mode: Option<Mode>,
    capability: Option<Capability>,
    value: Option<i32>,
}

impl Bundle {
    fn set_mode(&mut self, new_mode: Mode) {
        return self.mode = Some(new_mode);
    }
    fn set_capability(&mut self, new_capability: Capability) {
        return self.capability = Some(new_capability);
    }
    fn set_value(&mut self, new_value: i32) {
        return self.value = Some(new_value);
    }
    fn default_value() -> Bundle {
        return Bundle {
            mode: None,
            capability: None,
            value: None,
        };
    }
}

fn get_args() -> Vec<String> {
    return env::args().collect();
}

fn parse_args(mut args: Vec<String>) -> Bundle {
    let mut return_value: Bundle = Bundle::default_value();
    match &args[1][..] {
        "volume" => return_value.set_capability(Capability::Volume),
        "brightness" => return_value.set_capability(Capability::Brightness),
        _ => {}
    }
    match &args[2][..] {
        "up" => return_value.set_mode(Mode::Up),
        "down" => return_value.set_mode(Mode::Down),
        "toggle" => return_value.set_mode(Mode::Toggle),
        "set" => return_value.set_mode(Mode::Set),
        _ => {}
    }
    match return_value.mode {
        Some(Mode::Toggle) => {
            return return_value;
        }
        Some(Mode::Set) => {
            if args.len() < 4 {
                println!("ERROR : Missing set value");
                process::exit(1);
            }
            let value = args[3]
                .parse()
                .expect("ERROR : Failed to parse the second value");
            return_value.set_value(value);
        }
        _ => {
            args.push("5".to_string());
            let value = args[3].parse().unwrap_or(5);
            return_value.set_value(value);
        }
    }
    return return_value;
}

fn reader(parsed_args: Bundle) -> i32 {
    if parsed_args.capability.is_none() || parsed_args.mode.is_none() {
        return 1;
    }

    let mut label: Vec<String> = Vec::new();
    match parsed_args.capability {
        Some(Capability::Volume) => {
            match parsed_args.mode.unwrap() {
                Mode::Up => {
                    let mut command_status = process::Command::new("/usr/bin/pactl")
                        .arg("set-sink-volume")
                        .arg("@DEFAULT_SINK@")
                        .arg(format!("+{}%", parsed_args.value.unwrap()))
                        .spawn()
                        .expect("Failed!");
                    let _result = command_status.wait().unwrap();
                    label.push("[ 🔊 ]".to_string());
                }
                Mode::Down => {
                    let mut command_status = process::Command::new("/usr/bin/pactl")
                        .arg("set-sink-volume")
                        .arg("@DEFAULT_SINK@")
                        .arg(format!("-{}%", parsed_args.value.unwrap()))
                        .spawn()
                        .expect("Failed!");
                    let _result = command_status.wait().unwrap();
                    label.push("[ 🔉 ]".to_string());
                }
                Mode::Toggle => {
                    process::Command::new("/usr/bin/pamixer")
                        .arg("-t")
                        .spawn()
                        .expect("Failed!");
                    let value = match process::Command::new("/usr/bin/pamixer")
                        .arg("--get-mute")
                        .output()
                    {
                        Ok(v) => v,
                        Err(_e) => return 1,
                    };
                    let value_converted_bool = String::from_utf8_lossy(&value.stdout)
                        .trim()
                        .parse::<bool>()
                        .unwrap_or(false);
                    match value_converted_bool {
                        false => {
                            label.push("[ 🔇 ]".to_string());
                            let value_converted = 0;
                            spawn_notif(
                                format!("{} {value_converted}%", label[0]).to_string(),
                                value_converted,
                            );
                            return 0;
                        }
                        true => {
                            label.push("[ 🔊 ]".to_string());
                        }
                    };
                }
                Mode::Set => {
                    let mut command_status = process::Command::new("/usr/bin/pamixer")
                        .arg("--set-volume")
                        .arg(format!("{}", parsed_args.value.unwrap()))
                        .spawn()
                        .expect("Failed!");
                    let _result = command_status.wait().unwrap();
                    label.push("[ 🔊 ]".to_string());
                }
            };
            let value = match process::Command::new("/usr/bin/pamixer")
                .arg("--get-volume")
                .output()
            {
                Ok(v) => v,
                Err(_e) => return 1,
            };
            let value_converted = String::from_utf8_lossy(&value.stdout)
                .trim()
                .parse::<i32>()
                .unwrap_or(0);
            spawn_notif(
                format!("{} {value_converted}%", label[0]).to_string(),
                value_converted,
            );
            return 0;
        }
        Some(Capability::Brightness) => {
            match parsed_args.mode.unwrap() {
                Mode::Up => {
                    let mut command_status = process::Command::new("/usr/bin/brightnessctl")
                        .arg("set")
                        .arg(format!("{}%+", parsed_args.value.unwrap()))
                        .spawn()
                        .expect("Failed!");
                    let _result = command_status.wait().unwrap();
                    label.push("[ ☼ ]".to_string());
                }
                Mode::Down => {
                    let mut command_status = process::Command::new("/usr/bin/brightnessctl")
                        .arg("set")
                        .arg(format!("{}%-", parsed_args.value.unwrap()))
                        .spawn()
                        .expect("Failed!");
                    let _result = command_status.wait().unwrap();
                    label.push("[  ]".to_string());
                }
                Mode::Set => {
                    let mut command_status = process::Command::new("/usr/bin/brightnessctl")
                        .arg("set")
                        .arg(format!("{}%", parsed_args.value.unwrap()))
                        .spawn()
                        .expect("Failed!");
                    let _result = command_status.wait().unwrap();
                    label.push("[  ]".to_string());
                }
                _ => return 1,
            };
            let value = match process::Command::new("/usr/bin/brightnessctl")
                .arg("get")
                .output()
            {
                Ok(v) => v,
                Err(_e) => return 1,
            };
            let value_converted = String::from_utf8_lossy(&value.stdout)
                .trim()
                .parse::<i32>()
                .unwrap_or(0);
            let value_percentage = value_converted * 100 / 255;
            spawn_notif(
                format!("{} {value_percentage}%", label[0]).to_string(),
                value_percentage,
            );
            return 0;
        }
        None => {}
    }
    return 1;
}

fn spawn_notif(string: String, progress_bar_value: i32) {
    match &progress_bar_value {
        0 => {
            process::Command::new("/usr/bin/dunstify")
                .arg("--appname=ruskey")
                .arg("-r")
                .arg("2593")
                .arg(&format!("{string}"))
                .arg("-t")
                .arg("1000")
                .spawn()
                .expect("Failed!");
        }
        _ => {
            process::Command::new("/usr/bin/dunstify")
                .arg("--appname=ruskey")
                .arg("-r")
                .arg("2593")
                .arg("-h")
                .arg(&format!("int:value:{}", progress_bar_value))
                .arg(&format!("{string}"))
                .arg("-t")
                .arg("1000")
                .spawn()
                .expect("Failed!");
        }
    }
}

fn main() {
    let args: Vec<String> = get_args();
    if args.len() < 3 {
        process::exit(1);
    }
    let parsed_args: Bundle = parse_args(args);
    match reader(parsed_args) {
        1 => {
            process::exit(1);
        }
        _ => {}
    };
}
