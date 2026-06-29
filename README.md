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

this is output, we can run with "cargo test name_test_function -- --exact" or can also "cargo test name_test_funciton -- --nocapture"
<br> but, first step jus run all unit test and won't show the output. so i often use second step

![img_1.png](img_1.png)

---

### Variable
A variable is used to store data values, to create or declare a variable in rust, we can use "let" keyword.
examples of its usage is shown below:

```
#[test]
fn variable(){
    let my_name = "Ghendida";
    println!("Hallo {} ", my_name);
}
```
and the output:

```
PS D:\Rust\basic_rust> cargo test variable -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.48s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Hallo Ghendida 
test variable ... ok
```

---
<b>In Rust, we cannot change a variable that has already been assigned, which is usually called immutable. However, 
Rust allows us to create variables that can be changed, known as mutable, and the keyword is let mut. </b>

<br>
examples for mutable variable is showns below

```
#[test]
fn variable_mutable(){
    let mut age_in_2025: i8 = 18;
    println!("my age in 2025 is {} ", age_in_2025);

    age_in_2025 = 19;
    println!("my age in 2026 is {} ", age_in_2025);
}
```

and the output:

```
PS D:\Rust\basic_rust> cargo test variable_mutable -- --nocapture
Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.49s                                                                                                           
Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
my age in 2025 is 18
my age in 2026 is 19
test variable_mutable ... ok
```

---

Rust is a statically typed language, meaning every time you create a variable with a specific data type, 
its type can't be changed to another. 
Unlike JavaScript and PHP, this is not possible for example, changing from a string to an integer will not work in Rust

<br>
example for can't change data type

```
#[test]
fn static_type(){
    let mut my_github = "GhenAyari";
    println!("My github is {}", my_github);

    my_github = 1;
    println!("My github is {}", my_github);
}
```
 
and the output will be

```

PS D:\Rust\basic_rust> cargo test static_type -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
error[E0308]: mismatched types                                                                                                                                                     
  --> src\main.rs:30:17
   |
27 |     let mut my_github = "GhenAyari";
   |                         ----------- expected due to this value
...
30 |     my_github = 1;
   |                 ^ expected `&str`, found integer

For more information about this error, try `rustc --explain E0308`.                                                                                                                
error: could not compile `basic_rust` (bin "basic_rust" test) due to 1 previous error  

```

---

In Rust, we can create variables with the same name, but when we do, the previous variable will be covered, 
or what is called shadowing. this practice is not ideal, but it is still allowed in Rust

<br>
example for shadowing

```
#[test]
fn shadowing(){
    let name = "Ghendida";
    println!("Hallo {} ", name);

    let name = 10;
    println!("it's the {}th now ", name);

    let name = 2026;
    println!("this is {} year ", name);
}
```

and output will be

```
PS D:\Rust\basic_rust> cargo test shadowing -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.52s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Hallo Ghendida 
it's the 10th now 
this is 2026 year 
test shadowing ... ok

```

As seen above, if we create a variable with the same name but a different value and type, 
the previous variable will be shadowed and become inaccessible


---
Every variable in Rust has a data type, grouped into two types: scalar and compound. a scalar type represents a single value, for example: strings, integers, floats, 
booleans, and chars. meanwhile, compound types represent multiple values, which are tuples and arrays
<br>
In Rust, when creating a variable, there is no need to mention the data type explicitly because Rust will 
automatically recognize the data type used. However, it is still possible if you want to mention the data type explicitly when creating a variable with the colon (:) keyword
<br>

example an explicit variable
```
#[test]
fn explicit_variable(){
    let age: i8 = 19;
    println!("My age is {} ", age);

    let weight: f32 = 51.5;
    println!("my body weight is {} ", weight);
}
```

output:

```
PS D:\Rust\basic_rust> cargo test explicit_variable -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.48s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
My age is 19 
my body weight is 51.5 
test explicit_variable ... ok
```

---

Here's integer and float type

![img_2.png](img_2.png)

![img_3.png](img_3.png)

If you make a variable implicitly or dont mention the data type, 
Rust will automatically give i32 for integers and f64 for decimals

---

Type data conversion

Rust can perform data type conversions from smaller to larger types, and vice versa. However, there is something to keep in mind: converting a larger type to a smaller one can cause an integer overflow. 
For example, trying to convert the value 100,000 from an i32 to an i8 will trigger an integer overflow
<br>

first, example from smaller to larger types

```
#[test]
fn conversion(){
    let a: i8 = 19;
    println!("my number {} ", a);

    let b: i16 = a as i16;
    println!("his number is {} ", b);

    let c : i32 = a as i32;
    println!("my number {} ", c); 
}
```

the output will be

```
PS D:\Rust\basic_rust> cargo test conversion -- --nocapture       
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.51s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
my number 19 
his number is 19 
my number 19 
test conversion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s
```

---

and an example for large to small

```
#[test]
fn conversion_to_large(){
    let a: i64 = 1000000;
    println!("number {} ", a);

    let b: i8 = a as i8;
    println!("number {} ", b);

}
```

the output

```
PS D:\Rust\basic_rust> cargo test conversion_to_large -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.42s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
number 1000000 
number 64 
test conversion_to_large ... ok
```






