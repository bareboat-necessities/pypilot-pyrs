use std::collections::HashMap;
use std::*;

const DEFAULT_PORT: _ = 23322;
use version::strversion;
const retries: _ = 0;
const zerosocket: _ = socket.socket(socket.AF_INET, socket.SOCK_DGRAM);
fn get_local_addresses<RT>() -> RT {
    //global retries
    let mut addresses = vec![];
    if retries {
        let try_dummy = {
            //unsupported
            use netifaces::{ifaddresses, interfaces};
            for interface in interfaces() {
                let addrs = ifaddresses(interface);
                for i in addrs {
                    if i.iter().any(|&x| x == "addr") {
                        addresses.push(i["addr"]);
                    }
                }
            }
            return addresses;
        };
        let except!(Exception) = {
            //unsupported
            retries -= 1;
        };
    }
    let interfaces = os.listdir("/sys/class/net");
    for interface in interfaces {
        let try_dummy = {
            //unsupported
            addresses.push(socket.inet_ntoa(
                fcntl.ioctl(
                    zerosocket.fileno(),
                    35093,
                    struct_.pack("256s", bytes(interface[..15], "utf-8")),
                )[20..24],
            ));
        };
        let except!() = { //unsupported
             /*pass*/
        };
    }
    return addresses;
}
struct zeroconf {}

impl zeroconf {
    fn run(&self) {
        while true {
            let try_dummy = {
                //unsupported
                use zeroconf::{IPVersion, ServiceInfo, Zeroconf};
            };
            let except!() = {
                //unsupported
                time.sleep(10);
                continue;
            };
            break;
        }
        let mut addresses = vec![];
        let mut zeroconf = HashMap::new();
        let mut info = HashMap::new();
        let ip_version = IPVersion.V4Only;
        while true {
            let t = time.time();
            let i = get_local_addresses();
            if i != addresses {
                println!("{:?} {:?} {:?} ", "zeroconf addresses", i, i.len());
                for address in zeroconf {
                    zeroconf[address].unregister_service(info[address]);
                    zeroconf[address].close();
                }
                zeroconf = HashMap::new();
                info = HashMap::new();
                addresses = i;
                for address in addresses {
                    info[address] = ServiceInfo(
                        "_pypilot._tcp.local.",
                        "pypilot._pypilot._tcp.local.",
                        vec![socket.inet_aton(address)],
                        DEFAULT_PORT,
                        [("version", strversion)]
                            .iter()
                            .cloned()
                            .collect::<HashMap<_, _>>(),
                    );
                    zeroconf[address] = Zeroconf(ip_version, vec![address]);
                    let try_dummy = {
                        //unsupported
                        zeroconf[address].register_service(info[address]);
                    };
                    let except!(Exception) = {
                        //unsupported
                        println!("{:?} {:?} ", "zeroconf exception:", e);
                    };
                }
            }
            time.sleep(60);
        }
    }
}
fn main() {
    let zc = zeroconf();
    zc.run();
}
