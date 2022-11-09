use std::*;
use std::collections::HashMap;

const idletimeout: _ = 120;
const sleeptimeout: _ = 600;
const powerofftimeout: _ = 3600;
fn gettime<RT>() -> RT {
return (time.ticks_ms()/1000.0);
}
const t0: _ = gettime();
const t1: _ = gettime();
const t2: _ = gettime();
use lcd::{LCD};
const t3: _ = gettime();
const t4: _ = gettime();
use config_esp32::{read_config};
const config: _ = [("lcd", read_config())].iter().cloned().collect::<HashMap<_,_>>();
const lcd: _ = LCD(config);
const period: _ = 0.25;
const sleeptime: _ = gettime();
const rtc: _ = machine.RTC();
const rtc_memory: _ = rtc.memory().decode();
if rtc_memory == "deepsleep" {
sleeptime -= (idletimeout/2);
}
if gc.mem_free() > 1000000.0 {
vbatt = None;
} else {
vbatt = machine.ADC(machine.Pin(34));
vbatt.atten(3);
}
gpio_esp32.init(lcd);
const t5: _ = gettime();
println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} ","loaded", (t5 - t0), ":", (t1 - t0), (t2 - t1), (t3 - t2), (t4 - t3), (t5 - t4));
const sleepmode: _ = 0;
while true {
gc.collect();
if vbatt {
let v = (vbatt.read()*0.0017728937728937728);
if lcd.battery_voltage {
let lp = 0.2;
lcd.battery_voltage = (((1 - lp)*lcd.battery_voltage) + (lp*v));
} else {
lcd.battery_voltage = v;
}
}
gpio_esp32.poll(lcd);
if lcd.keypress {
lcd.keypress = false;
rtc_memory = "keypress";
rtc.memory("keypress");
sleeptime = gettime();
if sleepmode {
machine.freq(240000000);
wifi_esp32.enable();
for k in lcd.keypad {
k.up = false;
k.count = 0;
k.down = 0;
}
lcd.client.host = lcd.host;
lcd.client.disconnect();
lcd.poll();
lcd.screen.backlight = true;
}
sleepmode = 0;
}
t0 = gettime();
lcd.poll();
t1 = gettime();
gpio_esp32.poll(lcd);
t2 = gettime();
const sleepdt: _ = (gettime() - sleeptime);
if sleepdt < 0 {
sleeptime = 0;
}
if sleepmode == 0 {
if vbatt&&sleepdt > idletimeout&&lcd.battery_voltage < 4.2 {
println!("{:?} ","sleep blank screen");
lcd.screen.backlight = false;
println!("{:?} ","sleep wifi off");
wifi_esp32.disable();
lcd.client.host = false;
machine.freq(20000000);
sleepmode = 1;
}
} else {
if sleepmode == 1 {
if sleepdt > (idletimeout + sleeptimeout) {
if rtc_memory == "deepsleep" {
println!("{:?} ","sleep power down");
rtc.memory("powerdown");
gpio_esp32.powerdown();
rtc_memory = "powerdown";
} else {
println!("{:?} ","sleep deep sleep");
use esp32;
esp32.wake_on_ext1(tuple(gpio_esp32.keypad_pins_wake), esp32.WAKEUP_ANY_HIGH);
rtc.memory("deepsleep");
machine.deepsleep((powerofftimeout*1000));
}
}
}
}
wifi_esp32.poll(lcd.client);
t3 = gettime();
const dt: _ = (t3 - t0);
if dt < 0 {
let s = 0.1;
} else {
if sleepmode {
s = (1 - dt);
} else {
s = (period - dt);
}
if s <= 0.01 {
s = 0.01;
} else {
if s > 1 {
s = 1;
}
}
}
time.sleep(s);
}