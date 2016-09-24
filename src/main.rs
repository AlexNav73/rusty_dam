
extern crate bworker;

use bworker::{ Service, ServiceBuilder };

use std::sync::mpsc::{ channel, Receiver, Sender };
use std::sync::Arc;

unsafe impl Send for TestService {}
unsafe impl Sync for TestService {}

struct TestService {
    recver: Arc<Receiver<()>>,
    sender: Arc<Sender<()>>
}

impl TestService {
    fn new() -> TestService {
        let (s, r) = channel();
        TestService {
            recver: Arc::new(r),
            sender: Arc::new(s)
        }
    }
}

impl Service for TestService {
    fn start(&self, args: &[String]) {
        use std::io::Write;
        use std::fs::OpenOptions;

        let mut file = OpenOptions::new().append(true).open("D:\\Programms\\rusty_dam\\target\\debug\\out.txt").unwrap();
        file.write(b"Service start func\n");
        loop { 
            if self.recver.try_recv().is_ok() { break; }
            file.write(b"Service loop\n");
            ::std::thread::sleep(::std::time::Duration::new(1, 0));
        }
    }

    fn stop(&self) {
        use std::io::Write;
        use std::fs::OpenOptions;

        let mut file = OpenOptions::new().append(true).open("D:\\Programms\\rusty_dam\\target\\debug\\out.txt").unwrap();
        file.write(b"Service stop func\n");
        self.sender.send(());
    }
}

fn main() {
    ServiceBuilder::new().run(TestService::new());
}
