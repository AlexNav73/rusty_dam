
extern crate bworker;

use bworker::{ Service, spawn };

use std::sync::mpsc::{ channel, Receiver, Sender };
use std::sync::Arc;
use std::thread;

unsafe impl Send for TestService {}
unsafe impl Sync for TestService {}

struct TestService {
    recver: Arc<Receiver<()>>,
    sender: Arc<Sender<()>>,
    name: String
}

impl TestService {
    fn new(name: String) -> TestService {
        let (s, r) = channel();
        TestService {
            recver: Arc::new(r),
            sender: Arc::new(s),
            name: name
        }
    }
}

impl Service for TestService {
    fn start(&self, args: &[String]) {
        use std::io::Write;
        use std::fs::OpenOptions;

        let mut file = OpenOptions::new().append(true).open(self.name.to_owned()).unwrap();
        file.write(b"Service start func\n");

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

    fn stop(&self) {
        use std::io::Write;
        use std::fs::OpenOptions;

        let mut file = OpenOptions::new().append(true).open(self.name.to_owned()).unwrap();
        file.write(b"Service stop func\n");
        file.write(&format!("{:?}\n", thread::current()).as_bytes());
        self.sender.send(());
    }
}

fn main() {
    spawn(&[ 
          TestService::new("D:\\Programms\\rusty_dam\\target\\debug\\out.txt".into()),
          TestService::new("D:\\Programms\\rusty_dam\\target\\debug\\out2.txt".into()),
    ]);
}
