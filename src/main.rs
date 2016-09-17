
extern crate bworker;

use bworker::service::{ Service, launch };

struct TestService;

impl Service for TestService {
    fn start(&mut self, args: &[String]) {
        use std::io::Write;
        use std::fs::File;

        let mut file = File::create("C:\\Users\\Aliaksandr\\Desktop\\out.txt").expect("Could not open file");
        println!("{:?}", file.write(b"Service started!"));
        for arg in args {
            let _ = file.write(arg.as_bytes());
        }
    }

    fn stop(&mut self) {
        use std::io::Write;
        use std::fs::File;

        let mut file = File::open("C:\\Users\\Aliaksandr\\Desktop\\out.txt").expect("Could not open file");
        let _ = file.write(b"Service stopped!");
    }
}

fn main() {
    launch(TestService);
}
