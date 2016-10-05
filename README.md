# Rusty DAM - Digital Asset Management system written in [Rust-lang]

Currently has **[WIP]** status. And developed only for education purposes.
If you are find some bugs or way to improve code quality, please make pull request or create issue.

## Modules 

### 1. bworker (Background Worker) - Crate which allow to easily create Windows Services

[![Build status](https://ci.appveyor.com/api/projects/status/12xtxvwwf3qf3coj?svg=true)](https://ci.appveyor.com/project/AlexNav73/rusty-dam) (Windows)
[![Build Status](https://travis-ci.org/AlexNav73/rusty_dam.svg?branch=master)](https://travis-ci.org/AlexNav73/rusty_dam) (Linux)

> Unix daemons currently not supported!

```rust
extern crate bworker;

use bworker::Service;

use std::sync::mpsc::{ channel, Receiver, Sender };
use std::sync::Arc;
use std::thread;
use std::io::Write;
use std::fs::OpenOptions;

unsafe impl Send for TestService {}
unsafe impl Sync for TestService {}

struct TestService {
    recver: Arc<Receiver<()>>,
    sender: Arc<Sender<()>>,
}

impl TestService {
    fn new() -> TestService {
        let (s, r) = channel();
        TestService {
            recver: Arc::new(r),
            sender: Arc::new(s),
        }
    }
}

impl Service for TestService {
    fn start(&self, args: &[String]) {
        let mut file = OpenOptions::new().append(true).open("D:\\absolute\\path\\to\\log\\file.rs").unwrap();
        file.write(b"Service start func\n");

        for arg in args {
            file.write(arg.as_bytes());
            file.write(b"\n");
        }

        loop { 
            // Some business logic here ...

            if self.recver.try_recv().is_ok() { break; }
            ::std::thread::sleep(::std::time::Duration::new(1, 0));
        }
    }

    fn stop(&self) {
        let mut file = OpenOptions::new().append(true).open("D:\\absolute\\path\\to\\log\\file.rs").unwrap();
        file.write(b"Service stop func\n");
        self.sender.send(());
    }
}

fn main() {
    bworker::spawn(TestService::new());
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
