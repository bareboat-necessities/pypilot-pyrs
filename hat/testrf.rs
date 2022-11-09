use std::*;
use std::collections::HashMap;

const spi: _ = spidev.SpiDev();
spi.open(0, 1);
spi.max_speed_hz = 5000;
const i: _ = 0;
while true {
const x: _ = spi.xfer(vec![0, 0, 0, 0, 0, 0]);
i += 1;
println!("{:?} {:?} ",i, x);
time.sleep(0.1);
}