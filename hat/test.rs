use std::collections::HashMap;
use std::*;

use arduino::arduino;
use pypilot::servo::Servo;
use pypilot::{client, server};
fn main() {
    for i in (0..sys.argv.len()) {
        if sys.argv[i] == "-t" {
            if sys.argv.len() < (i + 2) {
                println!("{:?} ", (_("device needed for option") + " -t"));
                exit(1);
            }
            test(sys.argv[(i + 1)]);
        }
    }
    println!("{:?} ", "pypilot Servo");
    use server::pypilotServer;
    let server = pypilotServer();
    use client::pypilotClient;
    let client = pypilotClient(server);
    use sensors::Sensors;
    let sensors = Sensors(client);
    let servo = Servo(client, sensors);
    println!("{:?} ", "initializing arduino");
    let config = [
        ("host", "localhost"),
        (
            "hat",
            [(
                "arduino",
                [("device", "/dev/spidev0.1"), ("resetpin", "16")]
                    .iter()
                    .cloned()
                    .collect::<HashMap<_, _>>(),
            )]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>(),
        ),
        ("arduino.nmea.baud", 4800),
        ("arduino.nmea.in", false),
        ("arduino.nmea.out", false),
        ("arduino.ir", true),
        ("arduino.debug", true),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
    let a = arduino(config);
    let mut dt = 0;
    let period = 0.0;
    let start = time.monotonic();
    while true {
        let events = a.poll();
        if events {
            println!("{:?} ", events);
            for (key, count) in events {
                if key != "voltage" {
                    if count {
                        servo.command.set(1);
                    } else {
                        servo.command.set(0);
                    }
                }
            }
        }
        servo.poll();
        sensors.poll();
        client::poll();
        server::poll();
        dt = ((period - time.monotonic()) + lastt);
        if dt > 0 && dt < period {
            time.sleep(dt);
            lastt += period;
        } else {
            lastt = time.monotonic();
        }
    }
}
fn main() {
    main();
}
