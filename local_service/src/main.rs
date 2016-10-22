
extern crate bworker;

use bworker::{ Service, Builder };

use std::sync::mpsc::{ channel, Receiver, Sender };
use std::sync::Arc;
use std::thread;

unsafe impl Send for Service1 {}
unsafe impl Sync for Service1 {}

struct Service1 {
    recver: Arc<Receiver<()>>,
    sender: Arc<Sender<()>>
}

impl Service1 {
    fn new() -> Service1 {
        let (s, r) = channel();
        Service1 {
            recver: Arc::new(r),
            sender: Arc::new(s),
        }
    }
}

impl Service for Service1 {
    fn name(&self) -> &str { "Service1" }

    fn start(&self, args: &[String]) {
        use std::io::Write;
        use std::fs::OpenOptions;

        let mut file = OpenOptions::new().append(true).open("D:\\Programms\\rusty_dam\\target\\debug\\out.txt").unwrap();

        for arg in args {
            file.write(arg.as_bytes());
            file.write(b"\n");
        }

        loop { 
            if self.recver.try_recv().is_ok() { break; }
            file.write(&format!("{:?}\n", thread::current()).as_bytes());
            ::std::thread::sleep(::std::time::Duration::new(1, 0));
        }
    }

    fn stop(&self) { self.sender.send(()); }
}

unsafe impl Send for Service2 {}
unsafe impl Sync for Service2 {}

struct Service2 {
    recver: Arc<Receiver<()>>,
    sender: Arc<Sender<()>>,
}

impl Service2 {
    fn new() -> Service2 {
        let (s, r) = channel();
        Service2 {
            recver: Arc::new(r),
            sender: Arc::new(s)
        }
    }
}

impl Service for Service2 {
    fn name(&self) -> &str { "Service2" }

    fn start(&self, args: &[String]) {
        use std::io::Write;
        use std::fs::OpenOptions;

        let mut file = OpenOptions::new().append(true).open("D:\\Programms\\rusty_dam\\target\\debug\\out2.txt").unwrap();

        for arg in args {
            file.write(arg.as_bytes());
            file.write(b"\n");
        }

        loop { 
            if self.recver.try_recv().is_ok() { break; }
            file.write(&format!("{:?}\n", thread::current()).as_bytes());
            ::std::thread::sleep(::std::time::Duration::new(1, 0));
        }
    }

    fn stop(&self) { self.sender.send(()); }
}

fn main() {
    let s1 = Service1::new();
    let s2 = Service2::new();

    let mut b = Builder::new()
        .service(&s1)
        .service(&s2)
        .spawn();
}
