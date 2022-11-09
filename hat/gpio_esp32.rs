use std::*;
use std::collections::HashMap;

use machine::{Pin, TouchPad};
use time::{sleep};
if gc.mem_free() > 1000000.0 {
keypad_pin_numbers = vec![34, 36, 26, 33, 0, 35, 39, -1, 37, 38];
keypad_pullup = vec![37, 38, 39];
keypad_touch = vec![];
power_down_pin_number = None;
} else {
keypad_pin_numbers = vec![33, 25, 12, 13, 27, 15, 32, -1, 0, 35];
keypad_pullup = vec![0, 35];
keypad_touch = vec![];
power_down_pin_number = 26;
}
const noisr: _ = false;
fn make_pin<T0, T1, T2, RT>(pin: T0, i: T1, lcd: T2) -> RT {
//global noisr
if keypad_pullup.iter().any(|&x| x == pin) {
pin = Pin(pin, Pin::IN, Pin::PULL_UP);
} else {
pin = Pin(pin, Pin::IN, Pin::PULL_DOWN);
}
fn cbr<T0>(pin: T0)  {
handle_pin(pin, i, lcd);
}
if !noisr {
let try_dummy = { //unsupported
pin.irq(cbr, (Pin::IRQ_FALLING | Pin::IRQ_RISING));
};
let except!() = { //unsupported
println!("{:?} ","no Pin.irq!! keypresses will lag");
noisr = true;
};
}
return pin;
}
fn handle_pin<T0, T1, T2>(pin: T0, i: T1, lcd: T2)  {
if pin == -5 {
let mut key = lcd.keypad[i];
lcd.keypress = true;
let mut v = 1;
key.update(v);
} else {
if pin == -10 {
key = lcd.keypad[i];
lcd.keypress = true;
v = 0;
key.update(v);
} else {
key = lcd.keypad[i];
v = pin();
if keypad_pullup.iter().any(|&x| x == keypad_pin_numbers[i]) {
v = !v;
}
if !v {
lcd.keypress = true;
}
key.update(v);
}
}
}
let keypad_pins = vec![];
let keypad_pins_wake = vec![];
let index_pins_for_touch = vec![];
let Threshold = vec![];
let Threshold_ratio = vec![];
fn powerdown()  {
if power_down_pin_number {
Pin(power_down_pin_number, Pin::OUT).off();
}
}
fn init<T0>(lcd: T0)  {
//global keypad_pins
//global Threshold
//global Threshold_ratio
if power_down_pin_number {
Pin(power_down_pin_number, Pin::IN, Pin::PULL_UP);
}
for i in (0..keypad_pin_numbers.len()) {
let pini = keypad_pin_numbers[i];
if pini >= 0 {
if !keypad_touch.iter().any(|&x| x == pini) {
let pin = make_pin(pini, i, lcd);
keypad_pins.push(pin);
if !keypad_pullup.iter().any(|&x| x == pini) {
keypad_pins_wake.push(pin);
}
} else {
index_pins_for_touch.push(i);
}
}
}
for i in (0..index_pins_for_touch.len()) {
let touch = TouchPad(Pin(keypad_pin_numbers[index_pins_for_touch[i]]));
for x in (0..12) {
Threshold.push(touch.read());
sleep(0.1);
}
Threshold_ratio.push((Threshold.iter().sum() / Threshold.len()));
Threshold = vec![];
}
}
fn poll<T0>(lcd: T0)  {
if noisr {
for i in (0..keypad_pins.len()) {
handle_pin(keypad_pins[i], i, lcd);
}
}
for i in (0..index_pins_for_touch.len()) {
let touch = TouchPad(Pin(keypad_pin_numbers[index_pins_for_touch[i]]));
let mut ratio = (touch.read()/Threshold_ratio[i]);
if 0.1 < ratio {
handle_pin(-5, index_pins_for_touch[i], lcd);
while 0.1 < ratio {
ratio = (touch.read()/Threshold_ratio[i]);
sleep(0.1);
}
handle_pin(-10, index_pins_for_touch[i], lcd);
}
}
}