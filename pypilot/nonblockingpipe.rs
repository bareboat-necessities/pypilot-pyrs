use std::*;
use std::collections::HashMap;

struct NonBlockingPipeEnd {
    pipe: ST0,
    pollin: ST1,
    pollout: ST2,
    name: ST3,
    sendfailcount: ST4,
    failcountmsg: ST5,
    recvfailok: ST6,
    sendfailok: ST7,
}

impl NonBlockingPipeEnd {
    fn __init__<T0, T1, T2, T3>(&self, pipe: T0, name: T1, recvfailok: T2, sendfailok: T3) {
        self.pipe = pipe;
        self.pollin = select.poll();
        self.pollin.register(self.pipe, select.POLLIN);
        self.pollout = select.poll();
        self.pollout.register(self.pipe, select.POLLOUT);
        self.name = name;
        self.sendfailcount = 0;
        self.failcountmsg = 1;
        self.recvfailok = recvfailok;
        self.sendfailok = sendfailok;
    }
    fn fileno<RT>(&self) -> RT {
        return self.pipe.fileno();
    }
    fn flush(&self) {
        /*pass*/
    }
    fn close(&self) {
        self.pipe.close();
    }
    fn recv<T0, RT>(&self, timeout: T0) -> RT {
        let try_dummy = { //unsupported
            if self.pollin.poll(0) {
                return self.pipe.recv();
            }
            if !self.recvfailok {
                println!("{:?} {:?} ",_("error pipe block on recv!"), self.name);
            }
        };
        let except!() = { //unsupported
            println!("{:?} {:?} ",_("failed to recv nonblocking pipe!"), self.name);
        };
        return false;
    }
    fn recvdata<RT>(&self) -> RT {
        return self.pollin.poll(0);
    }
    fn readline<RT>(&self) -> RT {
        return self.recv();
    }
    fn write<T0, T1>(&self, value: T0, udp: T1) {
        self.send(value);
    }
    fn send<T0, T1, RT>(&self, value: T0, block: T1) -> RT {
        let t0 = time.time();
        if block || self.pollout.poll(0) {
            let t1 = time.time();
            self.pipe.send(value);
            let t2 = time.time();
            if (t2 - t0) > 0.001 {
                println!("{:?} {:?} {:?} ", "nonblocking pipe end too long!", (t2 - t0), self.name);
            }
            return true;
        }
        if self.sendfailok {
            return false;
        }
        self.sendfailcount += 1;
        if self.sendfailcount == self.failcountmsg {
            println!("{:?} {:?} {:?} ",(_("pipe full") + (" (%d)" % self.sendfailcount)), self.name, _("cannot send"));
            self.failcountmsg *= 10;
        }
        return false;
    }
}

use bufferedsocket::{LineBufferedNonBlockingSocket};

struct SocketNonBlockingPipeEnd {
    name: ST0,
}

impl SocketNonBlockingPipeEnd {
    fn __init__<T0, T1, T2, T3>(&self, socket: T0, name: T1, recvfailok: T2, sendfailok: T3) {
        self.name = name;
        super(SocketNonBlockingPipeEnd, self).__init__(socket, name);
    }
    fn recv<T0, RT>(&self, timeout: T0) -> RT {
        self.recvdata();
        let line = super(SocketNonBlockingPipeEnd, self).readline();
        if !line {
            return;
        }
        let try_dummy = { //unsupported
            let d = pyjson.loads(line.rstrip());
            return d;
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} ",_("failed to decode data socket!"), self.name, e);
            println!("{:?} {:?} ", "line", line);
        };
        return false;
    }
    fn send<T0, T1, RT>(&self, value: T0, block: T1) -> RT {
        let t0 = time.time();
        let try_dummy = { //unsupported
            let data = pyjson.dumps(value);
            self.write((data + "
"));
            let t1 = time.time();
            if (t1 - t0) > 0.02 {
                println!("{:?} {:?} {:?} {:?} ", "too long", (t1 - t0), self.name, data.len());
            }
            return true;
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} ",_("failed to encode data socket!"), self.name, e);
            return false;
        };
    }
}
let try_dummy = { //unsupported
use pypilot::linebuffer::{linebuffer};
};
let except!() = { //unsupported
};
struct PipeNonBlockingPipeEnd {
    name: ST0,
    b: ST1,
    pollout: ST2,
    recvfailok: ST3,
    sendfailok: ST4,
}

impl PipeNonBlockingPipeEnd {
    fn __init__<T0, T1, T2, T3, T4>(&self, r: T0, w: T1, name: T2, recvfailok: T3, sendfailok: T4) {
        self.name = name;
        let (self.r, self.w) = (r, w);
        os.set_blocking(r, false);
        os.set_blocking(w, false);
        self.b = linebuffer::LineBuffer(r);
        self.pollout = select.poll();
        self.pollout.register(self.w, select.POLLOUT);
        self.recvfailok = recvfailok;
        self.sendfailok = sendfailok;
    }
    fn fileno<RT>(&self) -> RT {
        return self.r;
    }
    fn close(&self) {
        os.close(self.r);
        os.close(self.w);
    }
    fn recvdata<RT>(&self) -> RT {
        return self.b.recv();
    }
    fn readline<RT>(&self) -> RT {
        return self.b.line();
    }
    fn recv<T0, RT>(&self, timeout: T0) -> RT {
        self.recvdata();
        let line = self.b.line();
        if !line {
            return;
        }
        let try_dummy = { //unsupported
            let d = pyjson.loads(line.rstrip());
            return d;
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} ",_("failed to decode data socket!"), self.name, e);
            println!("{:?} {:?} ", "line", line);
        };
        return false;
    }
    fn flush(&self) {
        /*pass*/
    }
    fn write<T0, T1>(&self, data: T0, udp: T1) {
        if !self.pollout.poll(0) {
            if !self.sendfailok {
                println!("{:?} {:?} ",_("failed write"), self.name);
            }
        }
        let t0 = time.time();
        os.write(self.w, data.encode());
        let t1 = time.time();
        if (t1 - t0) > 0.04 {
            println!("{:?} {:?} {:?} {:?} ", "too long write pipe", (t1 - t0), self.name, data.len());
        }
    }
    fn send<T0, T1, T2, RT>(&self, value: T0, block: T1, maxdt: T2) -> RT {
        if 0 {
            if !self.pollout.poll(0) {
                if !self.sendfailok {
                    println!("{:?} {:?} ", "failed poll send", self.name);
                }
            }
        }
        let t0 = time.monotonic();
        let try_dummy = { //unsupported
            let mut data = (pyjson.dumps(value) + "
");
            data = data.encode();
            let t1 = time.monotonic();
            os.write(self.w, data);
            let t2 = time.monotonic();
            if (t2 - t0) > maxdt {
                println!("{:?} {:?} {:?} {:?} {:?} ", "too long send nonblocking pipe", (t1 - t0), (t2 - t1), self.name, data.len());
            }
            return true;
        };
        let except!(Exception) = { //unsupported
            println!("{:?} {:?} {:?} {:?} ", "failed send ex", t0, time.monotonic(), e);
            if !self.sendfailok {
                println!("{:?} {:?} {:?} ", "failed to encode data pipe!", self.name, e);
            }
            return false;
        };
    }
}

struct NoMPLineBufferedPipeEnd {
    name: ST0,
    lines: Vec<_>,
}

impl NoMPLineBufferedPipeEnd {
    fn __init__<T0>(&self, name: T0) {
        self.name = name;
        self.lines = vec![];
    }
    fn fileno<RT>(&self) -> RT {
        return 0;
    }
    fn flush(&self) {
        /*pass*/
    }
    fn close(&self) {
        /*pass*/
    }
    fn write<T0, T1>(&self, data: T0, udp: T1) {
        self.send(data);
    }
    fn recv<T0, RT>(&self, timeout: T0) -> RT {
        return self.readline();
    }
    fn readline<RT>(&self) -> RT {
        if !self.lines {
            return false;
        }
        let ret = self.lines[0];
        self.lines = self.lines[1..];
        return ret;
    }
    fn send<T0, RT>(&self, value: T0) -> RT {
        if self.remote.lines.len() >= 1000 {
            return false;
        }
        self.remote.lines.append(value);
        return true;
    }
}

fn NonBlockingPipe<T0, T1, T2, T3, RT>(name: T0, use_multiprocessing: T1, recvfailok: T2, sendfailok: T3) -> RT {
    if use_multiprocessing {
        if 1 {
            let (r0, w0) = os.pipe();
            let (r1, w1) = os.pipe();
            return (PipeNonBlockingPipeEnd(r0, w1, (name + "[0]"), recvfailok, sendfailok), PipeNonBlockingPipeEnd(r1, w0, (name + "[1]"), recvfailok, sendfailok));
        } else {
            if 1 {
                use multiprocessing;
                let mut pipe = multiprocessing.Pipe();
                return (NonBlockingPipeEnd(pipe[0], (name + "[0]"), recvfailok, sendfailok), NonBlockingPipeEnd(pipe[1], (name + "[1]"), recvfailok, sendfailok));
            } else {
                use socket;
                let socket = socket.socketpair();
                return (SocketNonBlockingPipeEnd(socket[0], (name + "[0]"), recvfailok, sendfailok), SocketNonBlockingPipeEnd(socket[1], (name + "[1]"), recvfailok, sendfailok));
            }
        }
    }
    let mut pipe = (NoMPLineBufferedPipeEnd((name + "[0]")), NoMPLineBufferedPipeEnd((name + "[1]")));
    pipe[0].remote = pipe[1];
    pipe[1].remote = pipe[0];
    return pipe;
}