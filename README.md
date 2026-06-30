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

--- 

### Operators

Operators numeric

![img_4.png](img_4.png)

below for example operators numeric use case studies trapezoid area formula

```
#[test]
fn operators_numeric(){

    let height = 3.0;

    let a = 5.0;

    let b = 8.0;

    let l = 0.5;

    let result = l * (a + b) * height;

    println!("result = {}, ({} + {}), X {}, = {} ", l, a, b, height, result);

}
```

and the result is:

```
PS D:\Rust\basic_rust> cargo test operators_numeric -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.50s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
result = 0.5, (5 + 8), X 3, = 19.5 
test operators_numeric ... ok
```

--- 

comparison operators<br>

Comparison operators are special symbols in programming used to compare two values or expressions to determine the relationship between them. The result of a comparison operation is 
always a boolean value—either True or False—which is commonly used in decision-making structures like if statements or loops

![img_5.png](img_5.png)

example for comparison operators

```
#[test]
fn comparison_operators(){

    let a = 15 > 10;
    let b = 10 >= 10;
    let c = 15 < 10;
    let d = 10 == 10;

    println!("is the number 15 than 10? = {}", a);
    println!("is the number 10 than same as 10? = {}", b);
    println!("is the number 15 less than 10? = {}", c);
    println!("is the number 10 same as 10? = {}", d);

}
```

and the output:

```
PS D:\Rust\basic_rust> cargo test comparison_operators -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.57s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
is the number 15 than 10? = true
is the number 10 than same as 10? = true
is the number 15 less than 10? = false
is the number 10 same as 10? = true
test comparison_operators ... ok
```

---

boolean operators

Operator boolean adalah operator logika yang digunakan untuk membandingkan nilai atau mengevaluasi ekspresi, menghasilkan nilai akhir berupa benar (true) atau salah (false).
Operator ini berfungsi sebagai dasar pengendalian alur program dan penyaringan informasi dalam berbagai sistem digital

![img_6.png](img_6.png)

![img_7.png](img_7.png)

![img_8.png](img_8.png)

![img_9.png](img_9.png)

an example for boolean operators

```
#[test]
fn boolean_operators(){

    let age = 20;
    let height = 170;

    let category = 18 <= age;
    let height = 165 <= height;

    let result = category && height;

    println!("is he an adult man? {}", result);

}
```

and the output: 

```
PS D:\Rust\basic_rust> cargo test boolean_operators -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.55s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
is he an adult man? true
test boolean_operators ... ok
```

---

### Compound data type

Tuple,A tuple is a data type that groups together a collection of data types. 
The number of elements in a tuple is final and can't be modified, decreased, or increased. to create a tuple,can use parentheses ()

an example for tuple

```
#[test]
fn tuple(){
    let a: (i32, f64, &str) = (500, 6.4, "Hello");

    println!("Here is tuple = {:?} ", a);

    let tuple1 = a.0;
    let tuple2 = a.1;
    let tuple3 = a.2;

    println!("{}, {}, {} ", tuple1, tuple2, tuple3);

    // or we can also do Destructing tuple
    let (a, b, _) = a; // use _ if don't wanna ose one of them
    println!("Use desctructing tuple = {}, {}",a, b );
}
```

and the output:

```
S D:\Rust\basic_rust> cargo test tuple -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.52s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Here is tuple = (500, 6.4, "Hello") 
500, 6.4, Hello 
500, 6.4
test tuple ... ok
```

--- 

Mutable Tuple<br>
Technically, we can still modify the contents of a tuple by making it a mutable tuple. You just need to add the mut keyword

an example for mutable tuple 

```
#[test]
fn mutable_tuple(){
    let mut about_me: (&str, i8, &str) = ("Ghen", 19, "Mulawarman University");

    let (a, b, c) = about_me;

    println!("{}, {}, {}", a, b, c);

    about_me.0 = "Ghendida";
    about_me.1 = 20;
    about_me.2 = "From mulawarman university";

    println!("{:?}", about_me);

}
```

```
PS D:\Rust\basic_rust> cargo test mutable_tuple -- --nocapture 
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.48s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Ghen, 19, Mulawarman University
("Ghendida", 20, "From mulawarman university")
test mutable_tuple ... ok
```








