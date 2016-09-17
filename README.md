# Rusty DAM - Digital Asset Management system written in [Rust-lang]
---

Currently has **[WIP]** status. And developed only for education purposes.
If you are find some bugs or way to improve code quality, please make pull request or create issue.

## Modules 
---

### 1. bworker (background worker) - crate which allow to easily create Windows Services 

```rust
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
```

> Note: Windows Service logging all his activity into EventLog. For doing that, it needed to make [Message Text File]
> and put it aside your binary. Name for this binary should returned by your service implementation.

[Rust-lang]: https://www.rust-lang.org 
[Message Text File]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd996906(v=vs.85).aspx
