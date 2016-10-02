# Rusty DAM - Digital Asset Management system written in [Rust-lang]

Currently has **[WIP]** status. And developed only for education purposes.
If you are find some bugs or way to improve code quality, please make pull request or create issue.

## Modules 


### 1. [![Build Status](https://travis-ci.org/AlexNav73/rusty_dam.svg?branch=master)](https://travis-ci.org/AlexNav73/rusty_dam) bworker (Background Worker) - Crate which allow to easily create Windows Services

> Unix daemons currently not supported!

```rust
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
        let (s, r) = channel(); // Will be used as Cansellation Token
        TestService {
            recver: Arc::new(r),
            sender: Arc::new(s)
        }
    }
}

impl Service for TestService {
    fn start(&self, args: &[String]) {
        loop { 
            // Buisiness logic ...

            if self.recver.try_recv().is_ok() { break; }
            ::std::thread::sleep(::std::time::Duration::new(1, 0));
        }
    }

    fn stop(&self) {
        self.sender.send(());
        // Cleaning ...
    }
}

fn main() {
    ServiceBuilder::new().run(TestService::new());
}
```

Service Installation:

```
cargo build
sc.exe create "rusty_dam" binPath="absolute\\path\\to\\service\\binnary\\rusty_dam.exe"
```

> To launch service, open Services window, find your service by name, and click "Start" button

Service uninstall:
```
sc.exe delete "rusty_dam"
```

[Rust-lang]: https://www.rust-lang.org 
