use std::*;
use std::collections::HashMap;

const station: _ = network.WLAN(network.STA_IF);
station.wifi_ps(network.WIFI_PS_MAX_MODEM);
use page::{gettime};
const connected: _ = (false, gettime());
const enabled: _ = true;
const address: _ = "";
fn connect()  {
println!("{:?} {:?} ","wifi timeout, reconnecting wifi", (gettime() - connected[1]));
//global address
let config = config_esp32.read_config();
let essid = config["essid"];
let psk = String::from(config["psk"]);
address = config["address"];
println!("{:?} {:?} ","wifi connecting to", essid);
station.active(false);
station.active(true);
if psk {
println!("{:?} {:?} {:?} ","connect to", essid, psk);
station.connect(essid, psk);
} else {
station.connect(essid);
}
}
connect();
fn enable()  {
//global enabled
if !connected[0] {
connect();
}
enabled = true;
}
fn disable()  {
//global enabled
station.active(false);
enabled = false;
}
fn poll<T0, RT>(client: T0) -> RT {
//global connected
let isconnected = station.isconnected();
if connected[0] == isconnected {
if enabled&&!isconnected&&(gettime() - connected[1]) > 8 {
connected = (isconnected, gettime());
connect();
}
return isconnected;
}
connected = (isconnected, gettime());
if isconnected {
if address {
host = address;
} else {
let addrs = station.ifconfig();
println!("{:?} {:?} ","wifi connection success", addrs);
host = addrs[3];
}
if client.host != host {
println!("{:?} {:?} ","wifi connecting to pypilot at", host);
client.disconnect();
}
client.host = host;
} else {
println!("{:?} ","wifi disconnected");
client.disconnect();
}
return connected;
}