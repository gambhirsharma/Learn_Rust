# Day 01
```
fn main(){
        println!("Hello, World!");
    }
```
use `rustc` to make the executable file.
use `rustup` to linstall the latest statble version of Rust.
use `rustup docs --book` to open the offline version of the Rust Book.

- fn main()
The main() function is special: it is always the first code that runs in every executable Rust Program. 

`rustfmt` is used to format rust code something similar to Prettier.

- println!
This is calls a Rust macro. 
If it had called function insted, it would be entered as println (without the !). ! means that you're calling a macro insted of a normal function and that macros don't always follow tha same rules as functions.

# Hello, Cargo
cargo is Rust's build system and package manager. 

### Create a Project with Cargo
```
cargo new hello_cargo
cd hello_cargo
```
File Structure.
```
hello_cargo
 |- Cargo.toml
 |- src/
   |- main.rs

```

Cargo.toml ->

```
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

TOML(Tom's Obvious, Minimal Language) format is used as Cargo's configuration format.

main ->
```
fn main() {
    println!("Hello, world!");
}
```

### Build and Running a Cargo Project
- `cargo build`
This command creates an executable file in target/debug/hello_cargo.

file Structure
```
hello_cargo
 |- Cargo.lock
 |- Cargo.toml
 |- src/
  |- main.rs
 |- target/
  |- CACHEDIR.TAG 
  |- degug/
   |- hello_cargo
   |- hello_cargo.d
   |- build/
   |- deps/
    |- ...
   |- examples/
   |- incremental/
    |- ...
```

use `cargo run` to run the executable file from 'debug/hello_cargo'.
In place of using 'cargo build' and then 'cargo run' you can directly use 'cargo run' and it will build it before running the executable file. 


Cargo also provides a command called cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable.

---

Let’s recap what we’ve learned so far about Cargo:

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

### Building for Release
When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug.

```
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```
