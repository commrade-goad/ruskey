use std::env;
use std::process;

fn get_args() -> Vec<String> {
    return env::args().collect();
}

fn parse_args(mut args: Vec<String>) -> Vec<i32> {
    let mut return_value: Vec<i32> = Vec::new();
    match &args[1][..] {
        "volume" => return_value.push(1),
        "brightness" => return_value.push(2),
        _ => return_value.push(0),
    }
    match &args[2][..] {
        "up" => {
            return_value.push(1);
        }
        "down" => {
            return_value.push(2);
        }
        "toggle" => {
            return_value.push(3);
        }
        "set" => {
            return_value.push(4);
        }
        _ => {
            return_value.push(0);
        }
    }
    match return_value[1] {
        3 => {
            return return_value;
        }
        4 => {
            let value = args[3].parse().expect("ERROR : Failed to parse the second value");
            return_value.push(value);
        }
        _ => {
            args.push("5".to_string());
            let value = args[3].parse().unwrap_or(5);
            return_value.push(value);
        }
    }
    return return_value;
}

fn reader(parsed_args: Vec<i32>) -> i32 {
    for n in 0..parsed_args.len() {
        if parsed_args[n] == 0 {
            return 1;
        }
    }

    // let time_sleep = 150;

    let mut label: Vec<String> = Vec::new();
    if parsed_args[0] == 1 {
        match parsed_args[1] {
            1 => {
                let mut command_status = process::Command::new("/usr/bin/pactl")
                    .arg("set-sink-volume")
                    .arg("@DEFAULT_SINK@")
                    .arg(format!("+{}%", parsed_args[2]))
                    .spawn()
                    .expect("Failed!");
                let _result = command_status.wait().unwrap();
                label.push("[ 🔊 ]".to_string());
            }
            2 => {
                let mut command_status = process::Command::new("/usr/bin/pactl")
                    .arg("set-sink-volume")
                    .arg("@DEFAULT_SINK@")
                    .arg(format!("-{}%", parsed_args[2]))
                    .spawn()
                    .expect("Failed!");
                let _result = command_status.wait().unwrap();
                label.push("[ 🔉 ]".to_string());
            }
            3 => {
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
            4 => {
                let mut command_status = process::Command::new("/usr/bin/pamixer")
                    .arg("--set-volume")
                    .arg(format!("{}", parsed_args[2]))
                    .spawn()
                    .expect("Failed!");
                let _result = command_status.wait().unwrap();
                label.push("[ 🔊 ]".to_string());
            }
            _ => return 1,
        };
        // std::thread::sleep(std::time::Duration::from_millis(time_sleep));
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
    };
    if parsed_args[0] == 2 {
        match parsed_args[1] {
            1 => {
                let mut command_status = process::Command::new("/usr/bin/brightnessctl")
                    .arg("set")
                    .arg(format!("{}%+",parsed_args[2]))
                    .spawn()
                    .expect("Failed!");
                let _result = command_status.wait().unwrap();
                label.push("[ ☼ ]".to_string());
            }
            2 => {
                let mut command_status = process::Command::new("/usr/bin/brightnessctl")
                    .arg("set")
                    .arg(format!("{}%-",parsed_args[2]))
                    .spawn()
                    .expect("Failed!");
                let _result = command_status.wait().unwrap();
                label.push("[  ]".to_string());
            }
            4 => {
                let mut command_status = process::Command::new("/usr/bin/brightnessctl")
                    .arg("set")
                    .arg(format!("{}%",parsed_args[2]))
                    .spawn()
                    .expect("Failed!");
                let _result = command_status.wait().unwrap();
                label.push("[  ]".to_string());
            }
            _ => return 1,
        };
        // std::thread::sleep(std::time::Duration::from_millis(time_sleep));
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
    };
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
    let parsed_args: Vec<i32> = parse_args(args);
    match reader(parsed_args) {
        1 => {
            process::exit(1);
        }
        _ => {}
    };
}
