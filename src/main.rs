
extern crate bworker;

use bworker::{ Service, ServiceBuilder };

struct TestService;

impl Service for TestService {
    fn start(&self, args: &[String]) {
        use std::io::Write;
        use std::fs::OpenOptions;

        let mut file = OpenOptions::new().append(true).open("D:\\Programms\\rusty_dam\\target\\debug\\out.txt").unwrap();
        file.write(b"Service start func\n");
        // for arg in args {
        //     let _ = file.write(arg.as_bytes());
        // }
    }

    fn stop(&self) {
        use std::io::Write;
        use std::fs::OpenOptions;

        let mut file = OpenOptions::new().append(true).open("D:\\Programms\\rusty_dam\\target\\debug\\out.txt").unwrap();
        file.write(b"Service stop func\n");
    }
}

fn main() {
    ServiceBuilder::new().run(TestService);
}
