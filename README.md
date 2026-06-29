## This is my repository for learn Rust programming, written on 29 july 2026, by GhenAyari.

--- 
### How to write "Hello world in Rust"
<br>

```

fn main(){
    println!("Hello, world!");
}

```
above is how to write hello world in rust and how to run it, can type "cargo run" and below the result

![img.png](img.png)

---

### a brief introduction to cargo in rust
Cargo is package manager default and build system in Rust.<br>
example of use cargo below: <br>
1. for make a new project in rust, we can write 

```

cargo new name_file

```
cargo will make with project structure below

belajar_rust/<br>
├── Cargo.toml<br>
└── src/<br>
└── main.rs

2. can run program as shown below

```
cargo run
```

3. Running test

```
cargo test
```

4. if wanna measure performance or release an application

```
cargo build --release
```
or

```
cargo run --release
```

Most rust programmer i think, spend 90% of their time using cargo run, then switch to
cargo build --release once the application is ready for development 
or performance testing
---
### Unit test
In Rust one project only can use one main function. i gonna use alternative methods is that "unit test"
<br>
a unit test is a code specifically dedicated to testing.

```
#[test]
fn testing(){
    println!(my name's ghen and i currently learn rust);
}
```
this is output

![img_1.png](img_1.png)