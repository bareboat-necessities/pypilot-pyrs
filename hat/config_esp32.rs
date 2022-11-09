use std::collections::HashMap;
use std::*;

const filename: _ = "config";

fn config<T0, T1>(key: T0, value: T1) {
    let config = read_config();
    config[key] = value;
    write_config(config);
}

fn read_config<RT>() -> RT {
    let mut config = HashMap::new();
    let mut failed = false;
    let try_dummy = {
        //unsupported
        let f = open(filename);
        while true {
            let line = f.readline().rstrip();
            let try_dummy = {
                //unsupported
                if !line {
                    break;
                }
                let (key, value) = line.split("=", 1);
                let v = value.strip();
                config[key.strip()] = v;
            };
            let except!(Exception) = {
                //unsupported
                println!("{:?} {:?} {:?} ", "failed parsing line", line, e);
                failed = true;
                break;
            };
        }
        f.close();
    };
    let except!(Exception) = {
        //unsupported
        println!("{:?} {:?} ", "failed to load config", e);
        failed = true;
    };
    if failed {
        config = [("essid", "pypilot"), ("psk", ""), ("address", "")]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();
        write_config(config);
    }
    for k in vec![
        "smallstep",
        "hue",
        "bigstep",
        "backlight",
        "contrast",
        "buzzer",
    ] {
        if config.iter().any(|&x| x == k) {
            config[k] = i32::from(float(config[k]));
        }
    }
    for k in vec!["invert", "flip"] {
        if config.iter().any(|&x| x == k) {
            config[k] = config[k] == "True";
        }
    }
    return config;
}

fn write_config<T0>(config: T0) {
    let f = open(filename, "w");
    for key in config {
        f.write(key);
        f.write(" = ");
        f.write(String::from(config[key]));
        f.write(
            "
",
        );
    }
    f.close();
}
