# Rusty DAM - Digital Asset Management system written in [Rust-lang]

Currently has **[WIP]** status. And developed only for education purposes.
If you are find some bugs or way to improve code quality, please make pull request or create issue.

## Modules 

### 1. bworker (background worker) - crate which allow to easily create Windows Services 

Status: almost done, some issues left.

```rust
extern crate bworker;

use bworker::{ Service, ServiceBuilder };

use std::sync::mpsc::{ channel, Receiver, Sender };
use std::sync::Arc;
use std::io::Write;
use std::fs::OpenOptions;

unsafe impl Send for TestService {}
unsafe impl Sync for TestService {}

struct TestService {
    recver: Arc<Receiver<()>>,
    sender: Arc<Sender<()>>
}

impl TestService {
    fn new() -> TestService {
        let (s, r) = channel(); // Will be used as Cansellation Token
        TestService {
            recver: Arc::new(r),
            sender: Arc::new(s)
        }
    }
}

impl Service for TestService {
    fn start(&self, args: &[String]) {
        let mut file = OpenOptions::new().append(true).open("absolute\\path\\to\\log\\file\\out.txt").unwrap();
        file.write(b"Service start func\n");
        loop { 
            if self.recver.try_recv().is_ok() { break; }
            // Buisiness logic ...
        }
    }

    fn stop(&self) {
        let mut file = OpenOptions::new().append(true).open("absolute\\path\\to\\log\\file\\out.txt").unwrap();
        // Cleaning ...
        self.sender.send(());
    }
}

fn main() {
    ServiceBuilder::new().run(TestService::new());
}
```

[Rust-lang]: https://www.rust-lang.org 
