myRustApps
==========
This repository dedicated to myOWN [Rust][rust] programs I use for fun or in real projects. ;)  
I have no real opinion about [Rust][rust] language yet.  

*"Hundreds of companies around the world are using Rust in production today for fast, low-resource, cross-platform solutions. Software you know and love, like Firefox, Dropbox, and Cloudflare, uses Rust. From startups to large corporations, from embedded devices to scalable web services, Rust is a great fit."*  

These code examples may be reusable for other [Rust][rust] programs or projects.  

Codes description
-----------------

### myRustApps files

* `hello_from_rust.rs`: "Hello from Rust" just a most simple Rust example! Compile it with `rustc -O hello_from_rust.rs` command.  

### myRustApps folders

* `hello_cargo`: This is [Rust][rust] `hello world` program.  

* `rust_cli`: This is [Rust][rust] command line execution program. Just wrapper around command line execution.  

* `my_bluepill`: This is a [MCU][mcu] programming example of [Rust][rust] language use for micro controller unit ([MCU][mcu]) programming. This code was tested by me on real hardware `STM32F103C8T6` ([Blue Pill][bluepill] board). The code power on the onboard [LED][led] (PC13).  
   **Note :** Setup environment instruction for [Rust][rust] micro controller programming is detailed explained in official books like [THIS][rust_embedded_book] and [THIS][rust_embedded_debug]. The [MCU][mcu] programming by [Rust][rust] can be separated here into a separate section in the future.  
   
* `All Applications`:  
   **Note :** The [Rust][rust] uses **four SPACEs** in code. Pay attention when you copy and paste ;). The `Cargo` **MUST BE** preinstalled before use. [Rust][rust] book can be found [HERE][rustbook].  
   ***Requires :*** [Rust][rust] correctly pre-installed on your OS platform (see [Rust][rustwin] Windows installation manual).  
   ***Important:*** To compile I use [Makefile][makefile] which invoke Rust `Cargo`.  

* `To be continued...`  

### License  

This code has been written by ©2019 DimiG  

[rust]:https://www.rust-lang.org
[makefile]:https://en.wikipedia.org/wiki/Makefile
[rustwin]:https://github.com/rust-lang/rustup.rs/blob/master/README.md#working-with-rust-on-windows
[rustbook]:https://doc.rust-lang.org
[mcu]:https://en.wikipedia.org/wiki/Microcontroller
[bluepill]:https://wiki.stm32duino.com/index.php?title=Blue_Pill
[led]:https://en.wikipedia.org/wiki/Light-emitting_diode
[rust_embedded_book]:https://rust-embedded.github.io/book/
[rust_embedded_debug]:https://rust-embedded.github.io/debugonomicon/overview.html
