# Rusty DAM - Digital Asset Management system written in [Rust-lang]

Currently has **[WIP]** status. And developed only for education purposes.
If you are find some bugs or way to improve code quality, please make pull request or create issue.

## Modules 

### 1. bworker (background worker) - crate which allow to easily create Windows Services 

Status: almost done, some issues left.

```rust
extern crate bworker;

use bworker::service::{ Service, ServiceBuilder };

struct TestService;

impl Service for TestService {
    fn start(&mut self, args: &[String]) {
        /* Insert code here ... */ 
    }

    fn stop(&mut self) {
        /* Insert code here ... */ 
    }
}

fn main() {
   ServiceBuilder::new().run(TestService);
}
```

> Note: Windows Service logging all his activity into EventLog. For doing that, it needed to make [Message Text File]
> and put it aside your binary. Name for this binary should returned by your service implementation.

[Rust-lang]: https://www.rust-lang.org 
[Message Text File]: https://msdn.microsoft.com/en-us/library/windows/desktop/dd996906(v=vs.85).aspx
