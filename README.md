# This is my repository for learn Rust programming, written on 29 july 2026, by GhenAyari.

🇬🇧 English | [🇮🇩 Bahasa Indonesia](README.id.md)

---

## How to write "Hello world in Rust"
<br>

```rust
fn main(){
    println!("Hello, world!");
}

```
above is how to write hello world in rust and how to run it, can type "cargo run" and below the result

![img.png](img.png)

---

## a brief introduction to cargo in rust
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
## Unit test
In Rust one project only can use one main function. i gonna use alternative methods is that "unit test"
<br>
a unit test is a code specifically dedicated to testing.

```rust
#[test]
fn testing(){
    println!(my name's ghen and i currently learn rust);
}
```

this is output, we can run with "cargo test name_test_function -- --exact" or can also "cargo test name_test_funciton -- --nocapture"
<br> but, first step jus run all unit test and won't show the output. so i often use second step

![img_1.png](img_1.png)

---

## Variable
A variable is used to store data values, to create or declare a variable in rust, we can use "let" keyword.
examples of its usage is shown below:

```rust
#[test]
fn variable(){
    let my_name = "Ghendida";
    println!("Hallo {} ", my_name);
}
```
and the output:

```terminaloutput
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

```rust
#[test]
fn variable_mutable(){
    let mut age_in_2025: i8 = 18;
    println!("my age in 2025 is {} ", age_in_2025);

    age_in_2025 = 19;
    println!("my age in 2026 is {} ", age_in_2025);
}
```

and the output:

```terminaloutput
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

```rust
#[test]
fn static_type(){
    let mut my_github = "GhenAyari";
    println!("My github is {}", my_github);

    my_github = 1;
    println!("My github is {}", my_github);
}
```
 
and the output will be

```terminaloutput
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

```rust
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

```terminaloutput
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
```rust
#[test]
fn explicit_variable(){
    let age: i8 = 19;
    println!("My age is {} ", age);

    let weight: f32 = 51.5;
    println!("my body weight is {} ", weight);
}
```

output:

```terminaloutput
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

```rust
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

```terminaloutput
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

```rust
#[test]
fn conversion_to_large(){
    let a: i64 = 1000000;
    println!("number {} ", a);

    let b: i8 = a as i8;
    println!("number {} ", b);

}
```

the output

```terminaloutput
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

## Operators

Operators numeric

![img_4.png](img_4.png)

below for example operators numeric use case studies trapezoid area formula

```rust
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

```terminaloutput
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

```rust
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

```terminaloutput
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

```rust
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

```terminaloutput
PS D:\Rust\basic_rust> cargo test boolean_operators -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.55s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
is he an adult man? true
test boolean_operators ... ok
```

---

## Compound data type

Tuple,A tuple is a data type that groups together a collection of data types. 
The number of elements in a tuple is final and can't be modified, decreased, or increased. to create a tuple,can use parentheses ()

an example for tuple

```rust
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

```terminaloutput
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

```rust
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

```terminaloutput
PS D:\Rust\basic_rust> cargo test mutable_tuple -- --nocapture 
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.48s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Ghen, 19, Mulawarman University
("Ghendida", 20, "From mulawarman university")
test mutable_tuple ... ok
```

--- 
Array <br>

An array is a data type that contains a collection of data just like a tuple. The difference is in an 
array you can only use one data type, different from a tuple which can use many data types. To make an array, use []

example code below:

```rust
#[test]
fn array(){

    let array_list: [i8; 3] = [10, 20, 30];
    println!("here are some array = {:?}", array_list);

    let a = array_list[0];
    let b = array_list[1];
    let c = array_list[2];

    println!("{}, {}, {}", a, b, c);


}
```

the output result below:

```terminaloutput
PS D:\Rust\basic_rust> cargo test array -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.58s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
here are some array = [10, 20, 30]
10, 20, 30
test array ... ok
```

--- 
Mutable Array<br>
we can change contain of array with use "mut".

example code below

```rust
#[test]
fn mutable_array(){

    let mut array_can_change: [&str; 3] = ["Ramli", "Ruger", "Razi"];

    println!("{:?}", array_can_change);

    array_can_change[0] = "Rizal";
    array_can_change[1] = "Raditya";
    array_can_change[2] = "Roslan";

    println!("{}, {}, {}" , array_can_change[0], array_can_change[1], array_can_change[2]);

}
```

the output result below

```terminaloutput
PS D:\Rust\basic_rust> cargo test mutable_array -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.59s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
["Ramli", "Ruger", "Razi"]
Rizal, Raditya, Roslan
test mutable_array ... ok
```

--- 
two demonsional array <br>
we can create an array inside an array, which is commonly referred to as a two-dimensional array

example code below

```rust
#[test]
fn two_dimensional_arrays(){

    let array: [[i32; 3];3] = [
        [13, 16, 6],
        [10, 08, 09],
        [10, 06, 30]

    ];

    println!("{:?}", array);

    println!("{}", array[1][1]);
    println!("{}", array[0][1]);
    println!("{}", array[0][0]);

}
```

and the output result below

```terminaloutput
PS D:\Rust\basic_rust> cargo test two_dimensional_array -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.45s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
[[13, 16, 6], [10, 8, 9], [10, 6, 30]]
8
16
13
test two_dimensional_arrays ... ok
```

--- 
## Constant

A constant is an immutable variable that uses the const keyword. The difference 
between const and let is that constants cannot be made mutable, and you must explicitly state the data type when creating a constant

example code below

```rust
const MAXIMUM: i16 = 37;
#[test]
fn const_variable() {
    const MINIMUM: i16 = 33;
    println!("Use constant variable {}", MINIMUM);

    println!("We can use variable out of scope {}", MAXIMUM);


}
```

an output will be bellow

```terminaloutput
PS D:\Rust\basic_rust> cargo test const_variable -- --nocapture       
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.52s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Use constant variable 33
We can use variable out of scope 37
test const_variable ... ok
```

---

## Scope 
Variable scope defines the area where a variable can be used. A variable can 
be used inside the scope where the variable is located and in the inner scope, but it can't be used in the outer scope

example code below 

```rust
const UNIV_NAME: &str = "Mulawarman University"; // This variable can be used because it is located in the outermost scope so any function can access it
#[test]
fn scope() {
    // variable name can't used in here
    let name = "Ghendida"; // variable name can used start here
    println!("he's name is {}", name);

    { // inner scope
        println!("he's name middle name is Gantari and first name {}", name);
        let age: i8 = 19;
        println!("he's {} years old and from {} ", age, UNIV_NAME);
    }

    // println!("{}", age); // error bc in outer scope
}
```

the output below

```terminaloutput
PS D:\Rust\basic_rust> cargo test scope -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.57s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
he's name is Ghendida
he's name middle name is Gantari and first name Ghendida
he's 19 years old and from Mulawarman University 
test scope ... ok
```

--- 

## Management Memory 

Memory management is how a programming language manages memory (RAM) usage while a program is running. Every time a program creates data, the computer must allocate space in memory to store it. When that data is no longer needed, the memory space must be freed
so it can be reused by other data. The main challenge is determining when memory should be released and who is responsible for doing so<br>

In languages like C, the programmer is fully responsible for memory management. Programmers must manually request memory when needed and return it once they are finished. This approach offers immense control and high performance, but it is also highly error-prone. If a programmer forgets to free the memory, a 
memory leak occurs. If memory is freed more than once or used after being released, the program can crash or exhibit undefined behavior<br>

Languages like Java, Kotlin, Python, JavaScript, and Go take a different approach. They use a Garbage Collector, which is a system that automatically finds and cleans up memory that is no longer in use. This approach makes development easier because programmers don't have to worry about when to free memory. However, this cleanup process requires extra resources and can sometimes cause performance drops or brief pauses while the program is running.

Rust attempts to combine the strengths of both approaches. It does not force the programmer to manage memory manually like C, nor does it use a Garbage Collector like Java or Python. Instead, Rust utilizes a system of ownership, borrowing, and lifetimes to ensure every piece of data has clear ownership rules. The Rust compiler checks these rules during compilation. 
If there is any potential memory management error, the program will not compile until the issue is resolved<br>

As a result, Rust can manage memory automatically without needing a Garbage Collector at runtime. Many errors that are typically only caught during runtime in other languages can be detected early by the Rust compiler. This is why Rust is often described as a language that
offers a combination of high performance like C/C++ and better memory safety than many other systems programming languages<br>

To summarize in a single sentence: C and C++ entrust memory management to the programmer, 
Java and Python entrust it to the Garbage Collector, while Rust entrusts it to the compiler through the ownership system

When a Rust program runs, it stores data in RAM (Random Access Memory).
<br>

Most data is stored in one of two places:

```text
RAM
├── Stack
└── Heap
```

Understanding Stack and Heap is important because Rust's Ownership system was designed around them.

---

### Stack

The Stack is a fast and organized memory region.

Think of it like a stack of plates:

```text
Top
┌─────┐
│  3  │
├─────┤
│  2  │
├─────┤
│  1  │
└─────┘
Bottom
```

You can only add or remove items from the top.

Because of this structure:

- Very fast
- Automatically managed
- Predictable memory access

#### Common Stack Data

- Integers (`i32`, `u64`)
- Floats (`f32`, `f64`)
- Booleans (`bool`)
- Characters (`char`)
- Fixed-size arrays

---

### Heap

The Heap is a larger and more flexible memory region.

Think of it like a warehouse:

```text
┌─────────┐
│ Box A   │
├─────────┤
│ Box B   │
├─────────┤
│ Box C   │
└─────────┘
```

Unlike the Stack, the operating system must search for available space before storing data.

Because of this:

- More flexible
- Can store dynamic data
- Slower than Stack

### Common Heap Data

- String
- Vec<T>
- HashMap<K, V>
- Dynamic collections

---

### Stack vs Heap

| Feature | Stack | Heap |
|----------|----------|----------|
| Speed | Very Fast | Slower |
| Allocation | Automatic | Dynamic |
| Memory Size | Smaller | Larger |
| Structure | Ordered | Flexible |
| Access Cost | Low | Higher |
| Common Data | Numbers, Booleans | Strings, Vectors |

---

#### How String Uses Stack and Heap

A Rust String uses both memory regions.

```text
Stack
┌──────────┐
│ Pointer  │ ─────────────┐
│ Length   │              │
│ Capacity │              │
└──────────┘              │
                          ▼
Heap
┌─────────────────────┐
│ G | h | e | n       │
└─────────────────────┘
```

#### Stack Stores

- Pointer
- Length
- Capacity

#### Heap Stores

- Actual text data

For example:

```text
"Ghen"
```

The characters themselves live on the Heap.

---

### Why Ownership Exists

Heap memory is powerful but dangerous.

Without proper management, programs may suffer from:

- Memory Leaks
- Double Free Errors
- Dangling Pointers
- Undefined Behavior

Rust prevents these issues using:

- Ownership
- Borrowing
- Lifetimes

The Rust compiler checks these rules before the program runs.

---

### Memory Management Comparison

| Language | Memory Management |
|-----------|------------------|
| C | Manual |
| C++ | Mostly Manual |
| Java | Garbage Collector |
| Kotlin | Garbage Collector |
| Python | Garbage Collector + Reference Counting |
| JavaScript | Garbage Collector |
| Go | Garbage Collector |
| Rust | Ownership System |

Rust is unique because it provides memory safety without requiring a Garbage Collector.

example code below

```rust
#[test]
fn memory_management() {

    // When function_a() is called,
    // Rust creates a stack frame for function_a
    function_a();

    // After function_a() finishes,
    // its stack frame is removed

    // Then Rust creates a new stack frame
    // for function_b
    function_b();
}

fn function_a(){

    // age is an i32
    // its size is fixed (4 bytes)
    // stored directly on the STACK
    let age = 19;

    // The variable year_of_birth itself is stored on the STACK
    //
    // However, the actual String data ("2006")
    // is stored on the HEAP
    //
    // Stack:
    // Pointer
    // Length
    // Capacity
    //
    // Heap:
    // "2" "0" "0" "6"
    let year_of_birth: String = String::from("2006");

    // year is an i32
    // the result of parsing the String
    // stored on the STACK
    let year: i32 = year_of_birth.parse().unwrap();

    println!(
        "Ghen is {} years old and born in {}",
        age,
        year
    );

    // function_a finishes here

    // age and year are removed from the STACK

    // year_of_birth is also removed from the STACK

    // Before it is removed,
    // Rust automatically frees
    // the "2006" data stored on the HEAP
}

fn function_b(){

    // The variable name is stored on the STACK
    //
    // The actual String data "Ghendida"
    // is stored on the HEAP
    let name: String = String::from("Ghendida");

    // i32 value
    // stored on the STACK
    let entry_year = 2024;

    println!(
        "my name is {} and i entered this university in {}",
        name,
        entry_year
    );

    // function_b finishes here

    // entry_year is removed from the STACK

    // name is removed from the STACK

    // The "Ghendida" data stored on the HEAP
    // is automatically cleaned up by Rust
}
```

### Memory Layout During `function_a()`

When `function_a()` is running, the stack contains the local variables `age`, `year`, and the metadata of `year_of_birth`.

The actual string data `"2006"` is stored on the heap.

```text
STACK
┌─────────────────────┐
│ age = 19            │
├─────────────────────┤
│ year = 2006         │
├─────────────────────┤
│ year_of_birth       │
│ Pointer ───────────────┐
│ Length = 4         │   │
│ Capacity = 4       │   │
└─────────────────────┘   │
                          ▼
HEAP
┌─────────────────────┐
│ 2 │ 0 │ 0 │ 6       │
└─────────────────────┘
```

---

### after `function_a()` finishes

When the function scope ends:

- `age` is removed from the stack.
- `year` is removed from the stack.
- `year_of_birth` is dropped.
- The heap memory containing `"2006"` is automatically freed by Rust.

```text
STACK
┌─────────────────────┐
│ (empty)             │
└─────────────────────┘

HEAP
┌─────────────────────┐
│ "2006" freed        │
└─────────────────────┘
```

---

### memory layout during `function_b()`

When `function_b()` is running:

- `entry_year` is stored directly on the stack.
- `name` stores String metadata on the stack.
- The actual text `"Ghendida"` is stored on the heap.

```text
STACK
┌─────────────────────┐
│ entry_year = 2024   │
├─────────────────────┤
│ name                │
│ Pointer ───────────────┐
│ Length = 8         │   │
│ Capacity = 8       │   │
└─────────────────────┘   │
                          ▼
HEAP
┌─────────────────────┐
│ G h e n d i d a     │
└─────────────────────┘
```

---

### key Observation

A `String` in Rust does **not** store its text directly on the stack.

The stack only stores:

- Pointer
- Length
- Capacity

The actual text data lives on the heap.

This is why Rust's ownership system is especially important for heap-allocated data such as:

- `String`
- `Vec<T>`
- `HashMap<K, V>`

When the owner goes out of scope, Rust automatically frees the associated heap memory.

---

## &str and String

- Rust features two string types: &str (string slice), which has a fixed size, and String, which is growable<br>
- &str is fixed-size, it goes onto the stack, while String is allocated on the heap due to its dynamic size<br>

example code for &str is below

```rust
#[test]
fn string_slice() {

    // name is a string slice (&str)
    //
    // STACK:
    // - Pointer
    // - Length
    //
    // READ-ONLY MEMORY:
    // - "  Ghendida  "
    //
    // &str does not own the text.
    let mut name: &str = "  Ghendida  ";

    println!("{}", name);

    // trim()
    //
    // Removes leading and trailing whitespace.
    //
    // IMPORTANT:
    // trim() does NOT create a new String.
    //
    // It returns another &str that points
    // to a portion of the original text.
    //
    // STACK:
    // - New Pointer
    // - New Length
    //
    // READ-ONLY MEMORY:
    // - Still points to the same text
    //
    // No HEAP allocation occurs.
    let delete_space: &str = name.trim();

    println!("{}", delete_space);
}
```

and the output

```terminaloutput
PS D:\Rust\basic_rust> cargo test string_slice -- --nocapture         
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.50s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
  Ghendida  
Ghendida
test string_slice ... ok
```

example code for String is below

```rust
#[test]
fn string_not_fixed_size() {

    let name: String = String::from("ghendida ayari");
    println!("{}\n",name.to_lowercase());

    let mut list_name: (String, String, String) = (String::new(), String::from("satrio"), String::from("Rusman"));
    println!("{:?}", list_name);

    list_name.0.push_str("Akmal");

    println!("\n{}, {}, {} ", list_name.0.to_uppercase() , list_name.1.to_uppercase(), list_name.2.replace("Rusman", "Ramli").to_uppercase());


}
```

the output will be

```terminaloutput
PS D:\Rust\basic_rust> cargo test string_not_fixed_size -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
ghendida ayari

("", "satrio", "Rusman")

AKMAL, SATRIO, RAMLI 
test string_not_fixed_size ... ok
```

---

## understanding Ownership in Rust

A beginner-friendly guide to one of the most important concepts in Rust.

---

### Table of Contents

- [Introduction](#introduction)
- [What is Ownership?](#what-is-ownership)
- [Why Does Rust Need Ownership?](#why-does-rust-need-ownership)
- [The Three Rules of Ownership](#the-three-rules-of-ownership)
- [Ownership and Scope](#ownership-and-scope)
- [Ownership and Heap Memory](#ownership-and-heap-memory)
- [Why Ownership Makes Rust Safe](#why-ownership-makes-rust-safe)
- [Comparison with Other Languages](#comparison-with-other-languages)
- [Key Takeaways](#key-takeaways)

---

### Introduction

One of the biggest challenges in programming is managing memory safely and efficiently.

Many programming languages solve this problem in different ways:

- C uses manual memory management.
- C++ mostly uses manual memory management with additional tools.
- Java, Kotlin, Python, and JavaScript use a Garbage Collector.
- Rust uses a unique system called **Ownership**.

Ownership allows Rust to provide memory safety without requiring a Garbage Collector.

---

### What is Ownership?

Ownership is Rust's system for managing memory.

Every value in Rust has an **owner**.

The owner is responsible for the value and determines when that value should be removed from memory.

Think of ownership like owning a house.

```text
House
  ↑
Owner
```

Every house has an owner.

When ownership of the house is transferred to someone else, the previous owner no longer owns it.

Rust applies the same concept to data stored in memory.

---

### Why Does Rust Need Ownership?

Without proper memory management, programs can suffer from problems such as:

- Memory Leaks
- Double Free Errors
- Use After Free
- Dangling Pointers

These issues can cause crashes, unexpected behavior, and security vulnerabilities.

Ownership helps Rust prevent these problems at compile time before the program is executed.

---

### The Three Rules of Ownership

Rust's ownership system is built on three simple rules.

---

### Each Value Has One Owner

Every value in Rust has a variable that owns it.

```text
Data
 ↑
Owner
```

A value cannot exist without an owner.

Rust always knows who owns a piece of data.

---

### There Can Only Be One Owner at a Time

Ownership can be transferred from one variable to another.

When ownership is transferred:

```text
Old Owner ❌
      ↓
New Owner ✅
```

The previous owner loses access to the data.

This prevents multiple owners from trying to free the same memory.

---

### When the Owner Goes Out of Scope, the Value is Dropped

When the owner leaves its scope, Rust automatically cleans up the associated memory.

```text
Scope Ends
     ↓
Owner Removed
     ↓
Drop
     ↓
Memory Freed
```

This process happens automatically.

No manual cleanup is required.

---

### Ownership and Scope

Scope determines how long a variable is valid.

A variable exists only inside the block where it was created.

```text
Scope Starts
     ↓
Variable Exists
     ↓
Scope Ends
     ↓
Variable Dropped
```

When the scope ends, Rust automatically releases any resources owned by that variable.

---

### Ownership and Heap Memory

Ownership becomes especially important when working with heap-allocated data.

Examples include:

- String
- Vec<T>
- HashMap<K, V>

For example, a String stores its actual text on the heap.

```text
STACK
┌──────────────┐
│ Pointer      │──────────────┐
│ Length       │              │
│ Capacity     │              │
└──────────────┘              │
                              ▼
HEAP
┌──────────────┐
│ H e l l o    │
└──────────────┘
```

The owner is responsible for this heap memory.

When the owner is dropped, Rust automatically frees the heap allocation.

---

#### Why Ownership Makes Rust Safe

Ownership allows Rust to prevent many common memory bugs.

#### Prevents Memory Leaks

Unused memory is automatically released when the owner is dropped.

#### Prevents Double Free

Only one owner exists at a time.

Therefore, memory cannot be freed twice.

#### Prevents Use After Free

Rust ensures that data cannot be accessed after it has been dropped.

#### Improves Concurrency Safety

Ownership rules help prevent data races in multithreaded programs.

---

### Comparison with Other Languages

| Language | Memory Management |
|-----------|------------------|
| C | Manual |
| C++ | Mostly Manual |
| Java | Garbage Collector |
| Kotlin | Garbage Collector |
| Python | Garbage Collector + Reference Counting |
| JavaScript | Garbage Collector |
| Go | Garbage Collector |
| Rust | Ownership System |

Rust is unique because it provides memory safety without using a Garbage Collector.

---

### Key Takeaways

✅ Every value in Rust has an owner.

✅ A value can only have one owner at a time.

✅ When the owner goes out of scope, the value is automatically dropped.

✅ Ownership prevents many memory-related bugs.

✅ Rust achieves memory safety without a Garbage Collector.

✅ Ownership is the foundation of Rust's memory management system.

---

#### Ownership scope 

Scope is the area where a variable is valid and can be accessed. Ownership scope refers to the period during which an owner remains within that scope and retains its data. 
Once the owner goes out of scope, Rust automatically triggers the drop process and frees the memory associated with that owner

example code below 

```rust
#[test]
fn ownership_scope() {

    // let a variable can't access in here because out of scope and hasn't been created in this line
    let a = 10; // can access start here

    {
        let name = "ghen";
        println!("{}", name);
    } // variable name can't accesss start here because already out of scope

    println!("{}", a); // can access in here because still in scope

} // variable a can't access or ended in here bcs already out of scope
```

and output below

```terminaloutput
PS D:\Rust\basic_rust> cargo test ownership_scope -- --nocapture      
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.53s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
ghen
10
test ownership_scope ... ok
```

---

#### Data Copy

example code below

```rust
#[test]
fn data_copy() {
    let a = 16;
    let b = a;

    println!("{}, {}", a, b);
    
}
```

output below

```terminaloutput
C:/Users/Asus/.cargo/bin/cargo.exe test --color=always --package basic_rust --bin basic_rust data_copy --profile test --no-fail-fast --config "target.x86_64-pc-windows-gnu.runner=['C:\Program Files\JetBrains\RustRover 2025.3.4\bin\native-helper\intellij-rust-native-helper.exe']" -- --format=json --exact -Z unstable-options --show-output
Testing started at 7:05 PM ...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)
16, 16
```

explanation

```text
STACK
┌──────────┐
│ a = 16   │
├──────────┤
│ b = 16   │
└──────────┘

HEAP
┌──────────┐
│ (empty)  │
└──────────┘
```

- `i32` is stored on the stack.
- `b = a` creates a copy of the value.
- Both `a` and `b` remain valid.
- No ownership transfer occurs.
- No heap allocation is involved.

---

#### Ownership Movement

However, "Data Copy" does not occur for data types stored on the Heap.

Following the Ownership rules, a value can only have one owner at a time.

Therefore, when we create a new variable (a new owner) from an existing variable (the old owner), the data is not copied. Instead, the ownership is transferred from the old owner to the new owner.

Once the transfer is complete, the old owner is automatically considered invalid and can no longer be used.

example code below

```rust
#[test]
fn ownership_movement() {

    let name: String = String::from("Ghendida");

    // ownership name has move to name_2
    let name_2 = name;
    // name variable has can't access because ownership move to name_2

    println!("{}", name_2);

}
```

and output below

```terminaloutput
PS D:\Rust\basic_rust> cargo test ownership_movement -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.52s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Ghendida
test ownership_movement ... ok
```

---

#### Clone

Now we know that data on the Stack will be copied, while data on the Heap will have its ownership moved.

So, what if we also want to copy data that is stored on the Heap?

To do that, we must use "Clone".

Cloning means creating an exact duplicate of the original data.

String has a .clone() method specifically for this purpose.

When we call the .clone() method, it will copy the String data into a completely new String.

In Rust, almost all data types stored on the Heap implement the .clone() method

example code is below

```rust
#[test]
fn clone() {

    let name: String = String::from("Ghendida");

    let name2 = name.clone();

    println!("{}, {}", name, name2); // This is known as a clone.
    // If the string data is 10 MB, Rust will perform a clone of the same size, which is 10 MB

}
```

and the output

```terminaloutput
PS D:\Rust\basic_rust> cargo test clone -- --nocapture             
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.48s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Ghendida, Ghendida
test clone ... ok
```

## If Expression

Rust also has if expressions. An if expression is used to branch your code depending on conditions. 
If the condition is met, the if code block will be executed, but if it is not met, it will not be executed

example code below

```rust
#[test]
fn if_expression() {

    let a = 8;

    if a >= 9 {
        println!("Cool!");
    } else if a >= 8 {
        println!("Not bad")
    } else if a >= 5 {
        println!("is bad")
    } else {
        println!("shit")
    }

}
```

and the output

```terminaloutput
PS D:\Rust\basic_rust> cargo test if_expression -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.55s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Not bad
test if_expression ... ok

```

---

## Let Statement

In Rust, if is an expression, meaning it can return a value and can be used with a let statement to 
assign data to a variable. This is very useful because we don't need to separate the variable declaration from assigning its value

example code below

```rust
#[test]
fn let_statement () {

    let value = 80;
    let result: &str;

    if value >= 80{
        result = "Good";
    } else if value >= 70 {
        result = "Not Bad";
    } else if value >= 60 {
        result = "Bad";
    } else {
        result = "Shit";
    }

    println!("{}", result);

}
```

and output is below

```terminaloutput
PS D:\Rust\basic_rust> cargo test let_statement -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.54s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Good
test let_statement ... ok
```

--- 

## Loop

In Rust, the `loop` keyword is used to create an **infinite loop**. Unlike `while` or `for`, a `loop` does not have a built-in exit condition at the start; it will execute the block of code repeatedly forever until it is explicitly told to stop.

### Key Concepts

*   **`break`**: Acts as an emergency stop. It immediately terminates the loop and exits the block.
*   **`continue`**: Skips the remaining code in the current iteration and immediately jumps back to the top of the loop for the next turn.
*   **Loop as an Expression**: In Rust, a `loop` can return a value. You can pass a value directly after the `break` keyword, which can then be assigned to a variable using a `let` statement.

### Code Example

```rust
#[test]
fn loop_expression() {
    let mut counter = 0; // dimulai dari 0

    loop{
        counter += 1; // anggka akan ditambah 1 terus menerus

        if counter == 11 { // jika counternya sudah mencapai 11
            break; // hentikan
        } else if counter % 2 == 1 { // jika counter dimodulus 2 hasilnya 1 maka buang atau tidak perlu tampilkan
            continue; // continue akan mengabaikan atau skip dan langsung ke perulangan berikutnya
        }

        println!("Counter: {}", counter);
    }
}
```

and output is below

```terminaloutput
/home/ghen/.cargo/bin/cargo test --color=always --package basic_rust --bin basic_rust --profile test --no-fail-fast --config target.x86_64-unknown-linux-gnu.runner=['/home/ghen/.local/share/JetBrains/Toolbox/apps/rustrover/bin/native-helper/intellij-rust-native-helper'] -- loop_expression --format=json --exact -Z unstable-options --show-output
Testing started at 7:11 PM ...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/main.rs (target/debug/deps/basic_rust-bf48ffa96a14829d)
Counter: 2
Counter: 4
Counter: 6
Counter: 8
Counter: 10
```

another example

```rust
#[test]
fn loop_return_value() {
    let mut counter = 0; // angkanya adalah  0

    let result = loop {
        counter += 1; // perulangan akan dilakukan dan akan ditambah 1 terus
        if counter >= 10 { // jika lebih dari sama dengan 10
            break counter * 3; //break atau hentikan lalu dikali 3. jadi 3x10
        }
    };
    println!("{}", result); // memanggil perulangan

}
```

the output

```terminaloutput
#[test]
fn loop_return_value() {
    let mut counter = 0; // angkanya adalah  0

    let result = loop {
        counter += 1; // perulangan akan dilakukan dan akan ditambah 1 terus
        if counter >= 10 { // jika lebih dari sama dengan 10
            break counter * 3; //break atau hentikan lalu dikali 3. jadi 3x10
        }
    };
    println!("{}", result); // memanggil perulangan

}
```

--- 
### Loop Label

This section demonstrates how to use nested loops (a loop inside another loop) and **Loop Labels** (`'label:`) in Rust to create a multiplication table.

Code Structure

```rust
#[test]
fn loop_label() {
    let mut number = 1; // Left-hand side multiplier (Outer Variable)

    'luar: loop {
        let mut i = 1; // Right-hand side multiplier (Inner Variable)

        loop {
            if number >= 10 {
                break 'luar; // Terminate the ENTIRE nested loop structure
            }
            
            println!("{} X {} = {}", number, i, number * i);
            i += 1;
            
            if i > 10 {
                break; // Terminate only the INNER loop
            }
        }

        number += 1; // Incremented only after the inner loop finishes 10 cycles
    }
}
```

and output below

```terminaloutput
/home/ghen/.cargo/bin/cargo test --color=always --package basic_rust --bin basic_rust --profile test --no-fail-fast --config target.x86_64-unknown-linux-gnu.runner=['/home/ghen/.local/share/JetBrains/Toolbox/apps/rustrover/bin/native-helper/intellij-rust-native-helper'] -- loop_label --format=json --exact -Z unstable-options --show-output
Testing started at 3:17 PM ...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src/main.rs (target/debug/deps/basic_rust-bf48ffa96a14829d)
1 X 1 = 1
1 X 2 = 2
1 X 3 = 3
1 X 4 = 4
1 X 5 = 5
1 X 6 = 6
1 X 7 = 7
1 X 8 = 8
1 X 9 = 9
1 X 10 = 10
2 X 1 = 2
2 X 2 = 4
2 X 3 = 6
2 X 4 = 8
2 X 5 = 10
2 X 6 = 12
2 X 7 = 14
2 X 8 = 16
2 X 9 = 18
2 X 10 = 20
3 X 1 = 3
3 X 2 = 6
3 X 3 = 9
3 X 4 = 12
3 X 5 = 15
3 X 6 = 18
3 X 7 = 21
3 X 8 = 24
3 X 9 = 27
3 X 10 = 30
4 X 1 = 4
4 X 2 = 8
4 X 3 = 12
4 X 4 = 16
4 X 5 = 20
4 X 6 = 24
4 X 7 = 28
4 X 8 = 32
4 X 9 = 36
4 X 10 = 40
5 X 1 = 5
5 X 2 = 10
5 X 3 = 15
5 X 4 = 20
5 X 5 = 25
5 X 6 = 30
5 X 7 = 35
5 X 8 = 40
5 X 9 = 45
5 X 10 = 50
6 X 1 = 6
6 X 2 = 12
6 X 3 = 18
6 X 4 = 24
6 X 5 = 30
6 X 6 = 36
6 X 7 = 42
6 X 8 = 48
6 X 9 = 54
6 X 10 = 60
7 X 1 = 7
7 X 2 = 14
7 X 3 = 21
7 X 4 = 28
7 X 5 = 35
7 X 6 = 42
7 X 7 = 49
7 X 8 = 56
7 X 9 = 63
7 X 10 = 70
8 X 1 = 8
8 X 2 = 16
8 X 3 = 24
8 X 4 = 32
8 X 5 = 40
8 X 6 = 48
8 X 7 = 56
8 X 8 = 64
8 X 9 = 72
8 X 10 = 80
9 X 1 = 9
9 X 2 = 18
9 X 3 = 27
9 X 4 = 36
9 X 5 = 45
9 X 6 = 54
9 X 7 = 63
9 X 8 = 72
9 X 9 = 81
9 X 10 = 90
10 X 1 = 10
10 X 2 = 20
10 X 3 = 30
10 X 4 = 40
10 X 5 = 50
10 X 6 = 60
10 X 7 = 70
10 X 8 = 80
10 X 9 = 90
10 X 10 = 100
```

#### Visualizing the Lifecycle (How It Works)

To easily understand how the variables move under the hood, here is a step-by-step breakdown of their execution flow:

 1. Scope and Placement
* **`number` (Outer Variable)** is declared at the top-level scope. The program evaluates this line **only once**.
* **`i` (Inner Variable)** is declared within the outer loop's scope. Every single time the outer loop completes a cycle and restarts, this line is re-evaluated, resetting `i` back to `1`.

2. The Execution Timeline

| Outer Loop (`number`) | Inner Loop (`i`) | What Happens Behind the Scenes? |
| :--- | :--- | :--- |
| **`number = 1`** | `i = 1` | The inner loop starts. Prints `1 X 1 = 1`. `i` increments to 2. |
| `number = 1` | `i = 2` | Stays in the inner loop. Prints `1 X 2 = 2`. `i` increments to 3. |
| *... (cycles)* | *... (cycles)* | *Continues until `i` reaches 10 and prints `1 X 10 = 10`.* |
| `number = 1` | `i = 11` | Condition `if i > 10` triggers `break;`. The inner loop is destroyed. |
| **`number += 1` $\rightarrow$ `2`**| *Dropped* | Flow moves down, increments `number` to 2, and loops back to the top of `'luar`. |
| `number = 2` | **`i = 1`** | **`let mut i = 1;` is read again.** A brand new `i` is born at 1. |
| `number = 2` | `i = 2` | The inner loop cycles through 1 to 10 all over again for multiplication 2. |

3. The Grand Finale
This system continues until `number` hits `10`. At that exact moment, the inner loop's condition `if number >= 10` evaluates to true, triggering `break 'luar;`. This bypasses the normal flow and immediately terminates both loops simultaneously.


--- 
## While loop

Rust provides several ways to handle repetition. The two most fundamental conditional control flows are `loop` and `while`. Understanding when to use which is essential for writing clean and idiomatic Rust code.

### Summary Table

| Feature | `loop` | `while` |
| :--- | :--- | :--- |
| **Concept** | Infinite loop by default. | Conditional loop. |
| **Exit Condition** | None at the entry point. Requires manual `break` inside the block. | Evaluated at the entry point before each iteration. |
| **Use Case** | Continuous processes (e.g., servers, game loops) or dynamic breaking logic. | Iterating through collection bounds or waiting for a specific flag to change. |
| **As an Expression** | **Supported.** Can return a value via `break value;`. | **Not supported.** Always evaluates to the unit type `()`. |

---

### Detailed Mechanics

#### 1. The `loop` Keyword
A `loop` tells the compiler to re-execute a block of code forever until it explicitly hits a `break` statement. Because Rust knows a `loop` can only be exited when you decide to break, it allows you to return a value directly from it.

```rust
fn while_loop() {

    let mut number = 0; // number dimulai dari 0

    while number <= 15 { // selama number kurang dari sama degan 15 maka jalankan terus
        if number % 2 == 1 { // jika number dimodulus 1 menghasilkan 1 (agar memunculkan yang ganjil aja)
            println!("{}", number); // munculkan number
        }
        number += 1; // number akan ditambah 1 terus
    }
}

#[test]
fn while_array() {

    let arrayy: [&str; 5] = ["Ambatukam", "Rusman", "Rusdiyansah", "Marlan", "Zuki"];
    let mut index = 0;

    while index < arrayy.len() {
        println!("{} ", arrayy[index]);
        index += 1;
    }
}
```

--- 
## For

The `for` loop is the most common, secure, and idiomatic way to handle repetition in Rust. While `while` loops iterate based on a *condition*, a `for` loop iterates over a **collection** or a **range of values** with a predefined boundary.

### Key Advantages
* **Memory Safety**: Eliminates the risk of "out-of-bounds" errors when traversing arrays.
* **No Infinite Loop Trap**: You don't need to manually increment a counter (e.g., `i += 1`), preventing accidental infinite loops.

---

### Code Examples

#### 1. Iterating Over Ranges (`..` and `..=`)
Rust uses ranges to generate numbers sequentially.

```rust
#[test]
fn test_for_ranges() {
    // Exclusive Range (1 to 4): upper bound is NOT included
    for i in 1..5 {
        println!("Exclusive: {}", i); // Prints 1, 2, 3, 4
    }

    // Inclusive Range (1 to 5): upper bound IS included
    for j in 1..=5 {
        println!("Inclusive: {}", j); // Prints 1, 2, 3, 4, 5
    }
}
```

---
## Function

Functions are the core building blocks in Rust. They allow us to write reusable, organized, and modular code. This section covers everything from basic functions to recursive algorithms.

### 1. Basic Functions
A function is declared using the `fn` keyword. Here is a simple function that unpacks (destructures) a tuple and prints its values.

```rust
fn test_function() {
    let absen: (&str, i32, i8) = ("Ghen", 2006, 16);

    // Destructuring the tuple into 3 separate variables
    let (tuple1, tuple2, tuple3) = absen;
    
    println!("Name: {}", tuple1);
    println!("Birth Year: {}", tuple2);
    println!("Birth Date: {}", tuple3);
}

#[test]
fn call_test_function() {
    test_function();
}
```

### 2. Functions with Parameters
Parameters allow functions to accept input data dynamically. You must explicitly define the data type for each parameter.

```rust
// This function requires two string slice (&str) parameters
fn say_hello(first_name: &str, last_name: &str) { 
    println!("Hello {} {}!", first_name, last_name);
}

#[test]
fn call_say_hello() {
    // Calling the function by providing the exact arguments
    say_hello("Ghendida", "Aditya"); 
    say_hello("Nolan", "Alerick");
}
```

### 3. Functions with Return Values
If a function produces a result, we must declare the return type using an arrow `->`. Rust uses **Implicit Return**, meaning the last expression without a semicolon (`;`) is automatically returned.

```rust
// Calculates the factorial of 'a' using a standard for-loop
fn factorial_loop(a: i32) -> i32 { 
    // Guard clause: if 'a' is less than 1, return 0 immediately
    if a < 1 {
        return 0; 
    } 

    let mut result = 1; 
    
    // Iterates from 1 up to 'a'
    for i in 1..=a { 
       result *= i; 
    }
    
    // Implicit Return: sends the final 'result' out of the function
    result 
}

#[test]
fn call_factorial_loop() {
    // We must store the returned value in a variable to use it
    let hasil = factorial_loop(3); 
    println!("Result: {}", hasil); // Output: 6
}
```

### 4. Recursive Functions
A recursive function is a function that calls itself to solve smaller instances of the same problem. It must have a **Base Case** (a stopping condition) to prevent infinite loops (Stack Overflow).

**Example A: Recursion with Return Value (Math)**
```rust
fn factorial_recursive(a: i32) -> i32 { 
    // 1. BASE CASE: Stops recursion when 'a' reaches 1 or less
    if a <= 1 { 
        return 1; 
    }
    
    // 2. RECURSIVE STEP: Multiplies 'a' by the result of the function with 'a - 1'
    // E.g., for a = 3, it stacks: 3 * (2 * (1)) = 6
    a * factorial_recursive(a - 1) 
}

#[test]
fn call_factorial_recursive() {
    let hasil = factorial_recursive(3);
    println!("Recursive Result: {}", hasil);
}
```

**Example B: Recursion without Return Value (Action)**
```rust
// Prints a name multiple times using recursion instead of a loop
fn function_recursive(name: String, times: u32) { 
    // Base Case: Stop when times hit 0
    if times == 0 {
        return;
    } else {
        println!("{}", name);
    }
    
    // Call itself, reducing the 'times' counter by 1
    function_recursive(name, times - 1);
}

#[test]
fn call_function_recursive() {
    function_recursive(String::from("Baskoro"), 3);
}
```

---
## Ownership function

In Rust, memory safety is guaranteed through a system called **Ownership**. When passing variables into a function, Rust behaves differently depending on where the data is stored in memory: **Stack** or **Heap**.

### The Core Concept: Copy vs Move

#### 1. The Stack (Copy Trait)
Data types with a fixed size known at compile time (like `i8`, `i16`, `bool`, `char`) are stored on the **Stack**.
When you pass these variables to a function, Rust simply **copies** the value. The original variable remains valid and can still be used.

#### 2. The Heap (Move Semantics)
Dynamic data types that can grow or shrink (like `String`) are stored on the **Heap**.
When you pass a `String` to a function, Rust **moves** the ownership of that data to the function. The original variable loses its "ownership" and is immediately destroyed to prevent memory leaks.

---

### Visual Illustration

```text
============================================================
1. COPY (Stack - Fixed Size Data)
============================================================
let number = 16;

[ Main Function ]                     [ number_function ]
  number (16)       ---- COPIES --->    number (16)
 (STILL VALID)                        (Works independently)

============================================================
2. MOVE (Heap - Dynamic Data)
============================================================
let nama = String::from("Rusdi");

[ Main Function ]                     [ name function ]
  nama ("Rusdi")    ---- MOVES ---->    name ("Rusdi")
 (DESTROYED/DROP)                     (Takes full ownership)
============================================================
```

Code Example
Below is the implementation demonstrating both concepts, as well as the difference between printing directly and returning a formatted string.


```
// 1. Stack Data Function (Copy)
fn number_function(number: i16) {
    println!("umur = {}", number);
}

// 2. Heap Data Function (Move)
// This function promises to return a String type
fn name(name: String) -> String { 
    // format! works exactly like println! but instead of printing to the terminal,
    // it builds and stores the text into a new String variable.
    format!("nama {}", name) 
}

#[test]
fn show_name_number() {
    // --- STACK (COPY) ---
    let number = 16; // 16 is copied because it has a fixed size
    number_function(number); 
    // This is the same as number_function(10) because 'number' is stored in the stack.
    // In the stack, there is no ownership transfer; the data is merely copied.

    // --- HEAP (MOVE) ---
    let nama = String::from("Rusdi"); // "Rusdi" is stored in the heap because it's a String.
    name(nama); 
    // 'nama' has now transferred its ownership to the 'name' function.
    // Therefore, 'nama' can no longer be called here because it belongs to 'name' now.
    
    // println!("nama {}", nama); // ERROR: Value borrowed after move.

    // Calling the function directly inside println!
    // We don't need {:?} because the 'name' function uses format! to return a clean String.
    println!("{}", name(String::from("Amba"))); 
}
```

![Screenshot From 2026-07-09 22-52-03.png](../../Pictures/Screenshots/Screenshot%20From%202026-07-09%2022-52-03.png)

--- 

## Returning Ownership (Tuple Workaround) in Rust

Because Rust's memory management strictly enforces the **Move** semantic for Heap data (like `String`), variables sent into a function are usually destroyed (dropped) after the function finishes.

However, we can "rescue" our original variables by making the function **return their ownership** back to us, usually by packing them into a **Tuple** alongside the newly created data.

### Code Example

```rust
fn full_name_return_function(first_name: String, last_name: String) -> (String, String, String) {
    // 1. The variables 'first_name' and 'last_name' take ownership of the data from the outside.
    // 2. A new text is assembled in the Heap and stored in the 'full_name' variable.
    let full_name = format!("{} {}", first_name, last_name);

    // 3. RETURNING OWNERSHIP VIA TUPLE:
    // Instead of letting 'first_name' and 'last_name' die (drop) inside this function,
    // we pack them back into a Tuple along with the new 'full_name',
    // and return them to give ownership back to the caller function.
    (first_name, last_name, full_name)
}

#[test]
fn show_full_name_return_function() {
    // 1. We create two Strings, "Ezra" and "Arden".
    // 2. We Move them into 'full_name_return_function'.
    // 3. The function returns a Tuple containing 3 Strings.
    // 4. We use Destructuring 'let (a, b, c)' to capture and assign 
    //    the ownership of those 3 Strings back to our local scope.
    let (a, b, c) = full_name_return_function(String::from("Ezra"), String::from("Arden"));
    
    // Now variables a, b, and c legally own their respective String data,
    // so all three can be printed safely without any ownership errors!
    println!("{} ", a); // Prints: Ezra
    println!("{} ", b); // Prints: Arden
    println!("{} ", c); // Prints: Ezra Arden
}
```

### Visualizing the Workflow (The Factory Analogy)
To easily understand how the memory moves, imagine the function as an Assembly Factory and variables as Boxes of Materials

```

=========================================================================
VISUALIZING THE OWNERSHIP RETURN FLOW
=========================================================================

LOCATION 1: show_full_name_return_function (HOMETOWN)
---------------------------------------------------
STEP 1: You create 2 new boxes of materials.
📦 String 1 = "Ezra"
📦 String 2 = "Arden"

STEP 2: You send both boxes via courier (MOVE) to the Factory.
(Since the goods are sent, your HOMETOWN is now EMPTY. You no longer
own "Ezra" and "Arden").
|
| Sending... 🚚💨
v

LOCATION 2: full_name_return_function (ASSEMBLY FACTORY)
---------------------------------------------------
STEP 3: The factory receives your boxes.
The items now legally belong to the factory with new labels:
📦 first_name = "Ezra"
📦 last_name = "Arden"

STEP 4: The factory builds a new item using your materials.
📦 full_name = "Ezra Arden" (This is the 3rd, newly created item)

STEP 5: The factory packs ALL THREE items into one big cardboard box (Tuple).
📦 Cardboard Package = ("Ezra", "Arden", "Ezra Arden")
Then, the package is shipped back to you (RETURN).
|
| Shipping back... 🚚💨
v

BACK TO LOCATION 1: show_full_name_return_function (HOMETOWN)
---------------------------------------------------
STEP 6: The package arrives! You tear it open (Destructuring let (a, b, c)).
You distribute the contents to new owners in your town:
📦 a receives "Ezra"
📦 b receives "Arden"
📦 c receives "Ezra Arden"

DONE! Because the items are back in your town (owned by a, b, and c),
you can now print them safely.
```

---
## References And Borrowing
The "Returning Ownership" method (using Tuples) works, but it becomes exhausting if you have many variables. This is where **References** (`&`) come to the rescue!

In Rust, using a reference is called **Borrowing**. A reference allows a function to read or use data **without taking ownership** of it.

### The Book Analogy


* **Without Reference (Move/Take Ownership):** You give your notebook to a friend. The notebook is now theirs. You no longer own it (your variable is dropped). If you want it back, they have to physically mail it back to you.
* **With Reference (Borrowing):** You lend your notebook to a friend for a moment (`&`). You say, *"You can read this, but it's still mine!"* Once your friend finishes reading (the function ends), the notebook automatically returns to your desk. You never lost ownership.

### Code Example

By adding an ampersand (`&`), we pass a reference to the data instead of the data itself.

```rust
// The function parameters use '&String', meaning this function ONLY BORROWS the data.
// It does NOT take ownership.
fn full_name_references(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn show_full_name_references() {
    // 1. Create original String data in the Heap. 
    // 'first_name' and 'last_name' are the LEGAL OWNERS of this data.
    let first_name = String::from("Caleum");
    let last_name = String::from("Lucien");

    // 2. THE BORROWING PROCESS:
    // By adding the '&' symbol, we DO NOT move ownership.
    // We only give "read access" (borrow) to the full_name_references function. 
    // The function builds a new String and returns it to the 'name' variable.
    let name = full_name_references(&first_name, &last_name);

    // 3. OWNERSHIP PROOF:
    // Because the data was ONLY BORROWED, after the function finishes its job,
    // 'first_name' and 'last_name' remain the legal owners and are not dropped.
    // As a result, we can print all three safely without any errors!
    println!("{}", first_name); // Prints: Caleum (Safe!)
    println!("{} ", last_name); // Prints: Lucien (Safe!)
    println!("{} ", name);      // Prints: Caleum Lucien (New assembled string)
}
```

![Screenshot From 2026-07-10 11-23-01.png](../../Pictures/Screenshots/Screenshot%20From%202026-07-10%2011-23-01.png)

In Rust, **References** and **Borrowing** are the core features that guarantee memory safety without needing a garbage collector.
* **Reference (The Noun/Type):** The pointer type that allows you to refer to some value without taking ownership of it (e.g., `&String`, `&mut i32`).
* **Borrowing (The Verb/Action):** The act of creating a reference and passing it to a function or variable (e.g., `&buku`).

### The Golden Rules of Borrowing
Rust enforces strict rules at compile time to prevent "Data Races" (memory collisions):
1. At any given time, you can have **EITHER**:
    * One mutable reference (`&mut T`).
    * **OR** any number of immutable references (`&T`).
2. References must always be valid (no dangling pointers).

### Immutable vs Mutable
* **Immutable Borrowing (`&`)**: Use this when a function only needs to **read** the data. Multiple functions/variables can read the data simultaneously.
* **Mutable Borrowing (`&mut`)**: Use this when a function needs to **modify** the original data directly. Only ONE mutable borrow is allowed at a time. For primitive data types (like `i32`), you must use the dereference operator (`*`) to modify the value.

---

### Code Examples

#### 1. Immutable Borrowing (Read-Only)
```rust
// 1. REFERENCE (Data Type)
// The parameter 'teks' requests an IMMUTABLE REFERENCE of type '&String'.
// It can only read the data, not modify it.
fn baca_buku(teks: &String) {
    println!("Membaca: {}", teks);
}

#[test]
fn test_perbedaan() {
    let buku = String::from("Pemrograman Rust");

    // 2. BORROWING (The Action)
    // By adding '&' in front of the 'buku' variable, 
    // we are BORROWING the data (giving read-only access).
    baca_buku(&buku);
}
```
#### 2 Mutable Borrowing (String Modification)

```rust
// The parameter uses '&mut String', meaning it requests a VIP access to modify the data.
fn change_value(value: &mut String) {
    // String has built-in methods, so it automatically dereferences under the hood.
    value.push_str(" Testing");
}

#[test]
fn test_change_value() {
    // The original variable MUST also be declared as 'mut'
    let mut value = String::from("Rusdiyansah");
    
    // We pass the mutable reference using '&mut'
    change_value(&mut value);
    
    println!("{}", value); // Output: Rusdiyansah Testing
}
```

#### 3. Mutable Borrowing (Multiple Types & Dereferencing)
```rust
// The parameter uses '&mut String', meaning it requests a VIP access to modify the data.
fn change_value(value: &mut String) {
// String has built-in methods, so it automatically dereferences under the hood.
value.push_str(" Testing");
}

#[test]
fn test_change_value() {
// The original variable MUST also be declared as 'mut'
let mut value = String::from("Rusdiyansah");

    // We pass the mutable reference using '&mut'
    change_value(&mut value);
    
    println!("{}", value); // Output: Rusdiyansah Testing
}
```

---

## Understanding Slices and String Slices in Rust

### What is a Slice?
In Rust, a **slice** is a reference to a contiguous sequence of elements in a collection (like an array or a string) rather than the whole collection.

When you create a slice, you are not copying any data or allocating new memory. Instead, a slice acts as a "window" that merely looks at existing data. Under the hood, a slice stores only two pieces of information:
1. **Pointer:** The exact memory address where the slice begins.
2. **Length:** The number of elements the slice encompasses.

## What is a String Slice (`&str`)?
A `String` in Rust is essentially a growable array of bytes (`Vec<u8>`) encoded in UTF-8, stored on the Heap memory. A **string slice (`&str`)** is just a reference (a "window") to a portion of that UTF-8 data. Because it's a reference, it is incredibly fast and memory-efficient.

## Code Examples

```rust
#[test]
fn slice_references() {
    // The array is stored in a contiguous memory block with a fixed size (6 elements).
    let angka: [i16; 6] = [1, 2, 3, 4, 5, 6];

    // The `..` syntax means "take everything from start to finish".
    // slice1 references all elements in the array.
    let slice1: &[i16] = &angka[..];
    println!("slice1: {:?}", slice1); // Output: [1, 2, 3, 4, 5, 6]

    // `0..6` means "start from index 0, up to before index 6".
    // The upper bound is exclusive.
    let slice2: &[i16] = &angka[0..6];
    println!("slice2: {:?}", slice2); // Output: [1, 2, 3, 4, 5, 6]

    // `2..` means "start from index 2, and continue to the very end".
    let slice3: &[i16] = &angka[2..];
    println!("slice3: {:?}", slice3); // Output: [3, 4, 5, 6]

    // `..5` means "start from the beginning, stop before index 5".
    let slice4: &[i16] = &angka[..5];
    println!("slice4: {:?}", slice4); // Output: [1, 2, 3, 4, 5]
}

#[test]
fn string_slice_references() {
    // The `name` variable is a String type. Its actual data is allocated on the Heap
    // because text size can dynamically grow or shrink.
    let name: String = String::from("Ghen Ayari");
    
    // &str (String slice) directly peeks into the Heap memory owned by `name`.
    // `..4` takes the first 4 bytes (indices 0, 1, 2, 3).
    let first_name: &str = &name[..4];
    println!("{}", first_name); // Output: Ghen

    // `5..` skips the first 5 bytes (discarding "Ghen " including the space),
    // and takes the remainder of the string.
    let last_name: &str = &name[5..];
    println!("{}", last_name); // Output: Ayari
}
```
![Screenshot From 2026-07-10 17-16-47.png](../../Pictures/Screenshots/Screenshot%20From%202026-07-10%2017-16-47.png)

---
## Struct
In Rust, a **Struct** (short for structure) is a custom data type that lets you package together and name multiple related values that make up a meaningful group. While they may look like Classes in Object-Oriented Programming (OOP), Rust strictly separates **Data** (Struct) from **Behavior** (which is done later using `impl`).

### Core Terminologies
1. **Struct (The Blueprint):** The template or blueprint that defines what data is required.
2. **Field (The Data):** The specific attributes or variables defined inside the Struct blueprint (e.g., `first_name`, `age`).
3. **Instance (The Object):** The actual, physical realization of the Struct in memory. When you assign the Struct to a variable and fill in all the fields, it becomes an Instance.

### Three Types of Structs
1. **Classic C-like Structs:** Has named fields. Best for complex data.
2. **Tuple Structs:** Looks like a tuple. Has unnamed fields, accessed by index (`.0`, `.1`). Great for coordinates or RGB colors.
3. **Unit-like Structs:** Has no fields at all. Useful for traits and marker types.

### Code Examples & Features

```rust
// 1. CLASSIC STRUCT
struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8
}

#[test]
fn struct_person() {
    // Creating an instance of Person. All fields MUST be populated.
    let person: Person = Person {
        first_name: String::from("Ghendida"),
        last_name: String::from("Ayari"),
        middle_name: String::from("Gantari"),
        age: 20
    };

    println!("{}", person.first_name);
    // ... etc
}

// Using References (&) so this function only BORROWS the Struct, 
// without taking its ownership.
fn print_person(person: &Person) { 
    println!("Nama depan = {}", person.first_name);
    println!("Usia = {}", person.age);
}

#[test]
fn struct_init_shorthand() {
    let first_name: String = String::from("Ghendida");
    let last_name: String = String::from("Ayari");
    let age: u8 = 21;

    let person: Person = Person {
        // FIELD INIT SHORTHAND: Because the variable name and field name match, we can write it once.
        // EFFECT: The 'first_name' and 'last_name' variables are MOVED from the outer scope,
        // their ownership is now fully transferred into the 'person' struct.
        first_name, 
        middle_name: String::from("Gantari"),
        last_name, 
        age // Because u8 is a Stack (Copy) type, 'age' is just copied. The original variable is not dropped.
    };

    // println!("{}", first_name); // CORRECT! This will cause an error because ownership has moved.

    print_person(&person);

    // STRUCT UPDATE SYNTAX (..)
    let person2: Person = Person {
        // We use CLONE because String lives on the Heap.
        // If we DON'T clone, the '..person' syntax below will take ownership of 
        // the remaining String fields from the old 'person', causing a Partial Move.
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        
        // Tells Rust: "For the remaining fields (i.e., age), please copy them from the old 'person'"
        ..person
    };

    print_person(&person2);
    // This can STILL BE CALLED safely because we smartly avoided 
    // moving the Strings by using .clone() above!
    print_person(&person); 
}

// 2. TUPLE STRUCT
// Unnamed fields, order matters. Perfect for mathematical coordinates.
struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-656.73, 314.431);
    // Accessed using index (.0, .1) just like a normal Tuple
    println!("lat = {} ", geo_point.0);
    println!("let = {} ", geo_point.1);
}

// 3. UNIT STRUCT
// Has no fields. Useful for advanced logical markers later on.
struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1 = Nothing;
    let _nothing2 = Nothing{}; // Using empty curly braces is also valid
}
```

---

## Impl and Methods in Rust

In Rust, data and behavior are strictly separated. While `struct` defines the **Data** (the blueprint), the `impl` (Implementation) block defines the **Behavior** (the actions).

### The Analogy: The Factory and the Motorcycle
* **Struct:** The blueprint or design of the motorcycle (defines what it has: name, color, fuel).
* **Impl Block:** The workshop manual or the factory itself. It contains all the instructions on what the motorcycle can do.
* **Associated Function:** The factory machine. It doesn't need an existing motorcycle to work; its job is to *produce* a new motorcycle.
* **Method:** The actions performed by the *physical* motorcycle after it's built (e.g., checking status, refueling, or sending it to the scrapyard).

### Rules of `impl` (What You Can and Cannot Do)
* **✅ CAN DO:** You can create more than one `impl` block for the same Struct. Rust will automatically merge them during compilation.
* **❌ CANNOT DO:** You cannot define new variables or data fields inside an `impl` block. This space is strictly reserved for functions and constants.

### The Power of `self` (Ownership in Methods)
The first parameter of a function inside an `impl` block determines its type:
1. **No `self` (Associated Function):** Acts as a constructor. Called using double colons (`StructName::function_name()`).
2. **`&self` (Immutable Borrow):** The method only needs to **read** the data.
3. **`&mut self` (Mutable Borrow):** The method needs to **read and modify** the data.
4. **`self` (Take Ownership):** The method **consumes** the instance. Once this method finishes, the instance is dropped from memory and can no longer be used.

---

### Code Example

```rust
struct Motor {
    nama: String,
    warna: String,
    no_rangka: u32,
    bensin: u8
}

impl Motor {
    // 1. ASSOCIATED FUNCTION
    // Does not take 'self'. Acts as a factory to produce a Motor instance.
    fn motor_baru(nama_merek: String, warna_motor: String) -> Motor { 
        Motor {
            nama: nama_merek,
            warna: warna_motor,
            bensin: 0,
            no_rangka: 18765,
        }
    }

    // 2. METHOD: IMMUTABLE BORROW (&self)
    // Uses '&self' to read data only. Cannot modify fields.
    fn keluar_pabrik_cek_status(&self) { 
        println!("Motor dengan nomor rangka {} keluar pabrik dan sekarang bensinnya {} ", self.no_rangka, self.bensin);
    }

    // 3. METHOD: MUTABLE BORROW (&mut self)
    // Uses '&mut self' to modify data (e.g., adding fuel).
    fn isi_bensin(&mut self, tambah_bensin: u8) { 
        self.bensin += tambah_bensin; 
        println!("Motor dengan nomor rangka {} dan warna {} sudah isi bensin menjadi {} liter ", self.no_rangka, self.warna, self.bensin);
    }

    // 4. METHOD: TAKE OWNERSHIP (self)
    // Consumes the instance. The entire Motor ownership is moved into this function.
    fn hancurkan_motor(self) { 
        println!("Hancurkan motor menjadi rongsokan {} ", self.nama);
        // Motor is destroyed from memory after this block ends.
    }
}

#[test]
fn show_motor() {
    // Calling an associated function using '::'
    let mut motor_saya = Motor::motor_baru(String::from("Yamaha"), String::from("Hitam")); 

    // Calling methods using dot notation ('.') on the instance
    motor_saya.keluar_pabrik_cek_status(); 

    motor_saya.isi_bensin(3);

    // Taking ownership. 'motor_saya' is now consumed.
    motor_saya.hancurkan_motor(); 

    // motor_saya.keluar_pabrik_cek_status(); // ERROR! Cannot be called anymore because 'motor_saya' has been destroyed.
}
```


---
## Enum
**Enum** (short for Enumeration) is a custom data type that allows you to define a type by enumerating its possible *variants*. While a `struct` uses the **AND** concept (a Car has wheels AND an engine), an `enum` uses the **OR** concept (a Traffic Light is Red OR Yellow OR Green).

### When to Use Enum vs. Struct?
*   **Use `struct`** when you need to group related pieces of data together simultaneously. (e.g., A `User` has a `name`, `email`, and `age` all at once).
*   **Use `enum`** when a value can only be exactly *one* of several possible states or choices. (e.g., A `Connection` state is either `Connected` OR `Disconnected` OR `Connecting`).

### The Rules (Do's and Don'ts)

**Enum:**
*   ✅ **CAN** store different types and amounts of data inside each variant (e.g., `VariantA`, `VariantB(String)`, `VariantC(i32, i32)`).
*   ✅ **CAN** have its own behavior using `impl` blocks (Associated Functions and Methods).
*   ❌ **CANNOT** extract its inner data directly using dot notation (e.g., `my_enum.0`). You *must* use Pattern Matching to unlock the data.

**Pattern Matching (`match`):**
*   ✅ **CAN** extract inner data and bind it to a new local variable instantly.
*   ❌ **CANNOT** leave out any variant. Rust enforces **Exhaustive Matching**, meaning you must handle every single possible variant.
*   ❌ **CANNOT** "fall-through". Unlike `switch` in other languages, once a match is found, it executes the block and exits automatically. No `break` keyword is needed.

### Writing Enum and Match Variations

```rust
enum Example {
    Empty,               // Unit-like: No data
    Text(String),        // Tuple-like: 1 data
    Position(i32, i32),  // Tuple-like: 2 data
}

fn handle_example(ex: Example) {
    match ex {
        Example::Empty => println!("Nothing here"),
        Example::Text(msg) => println!("Message: {}", msg),
        Example::Position(x, y) => println!("At {}, {}", x, y),
        // _ => println!("Catch-all pattern if you want to ignore the rest"),
    }
}
```

### Example code

```rust
// 1. DEFINING THE ENUM
enum MesinKopi {
    Mati,             // Variant with no data
    Menyala(u32),     // Variant holding a 'u32' data representing coffee stock
}

// 2. IMPLEMENTING BEHAVIOR
impl MesinKopi {
    // Associated Function (Factory/Constructor)
    fn mesin_baru() -> MesinKopi {
        MesinKopi::Mati
    }

    // Method with &self (Immutable Borrow - Reading data only)
    fn cek_status(&self) {
        match self {
            MesinKopi::Mati => {
                println!("Mesin kopi mati");
            }
            // Binding the inner 'u32' to a variable named 'stok'
            MesinKopi::Menyala(stok) => {
                println!("Mesin kopi siap, sisa stok {} gelas", stok);
            }
        }
    }

    // Method with &mut self (Mutable Borrow - Modifying enum state/data)
    fn isi_kopi(&mut self, tambahan: u32) {
        match self {
            MesinKopi::Mati => {
                // Mutating the enum from 'Mati' to 'Menyala' state
                // We use '*' (dereference) to overwrite the actual borrowed value
                *self = MesinKopi::Menyala(tambahan);
                println!("Mesin otomatis dihidupkan dan diisi dengan tambahan {} gelas kopi", tambahan);
            }
            MesinKopi::Menyala(stok) => {
                // Mutating the inner value of the current variant
                *stok += tambahan;
                println!("Stok ditambah sekarang mesin memiliki stok {} gelas kopi", stok);
            }
        }
    }

    // Method with self (Takes Ownership - Consumes the instance)
    fn hancurkan_mesin(self) {
        match self {
            MesinKopi::Mati => {
                println!("Mesin kopi mati dan dibuang ke rongsokan");
            }
            MesinKopi::Menyala(stok) => {
                println!("Sayang sekali mesin dihancurkan padahal masih menyala dan memiliki stok {} gelas kopi di dalamnya", stok);
            }
        }
        // After this function ends, the 'MesinKopi' instance is dropped from memory.
    }
}

#[test]
fn test_mesin_kopi() {
    let mut mesin_kantor = MesinKopi::mesin_baru();

    mesin_kantor.cek_status();      // Output: Mesin kopi mati
    mesin_kantor.isi_kopi(15);      // Mutates state to Menyala(15)
    mesin_kantor.cek_status();      // Output: Mesin kopi siap...
    mesin_kantor.isi_kopi(13);      // Mutates inner value to 28
    mesin_kantor.cek_status();      // Output: Mesin kopi siap, sisa stok 28 gelas
    mesin_kantor.hancurkan_mesin(); // Consumes the machine
}
```

---
## Pattern Matching in Rust

In Rust, `match` is not just a simple `switch-case` statement. It is a powerful **expression** that can destructure complex data types, bind variables on the fly, and apply conditional logic.

Pattern matching in Rust operates on the **"Mirror Principle"**: the way you destructure (unpack) the data in the `match` arm must exactly mirror the way the data was constructed.

### 🛠️ How It Works
When you pass a value into a `match` block, Rust evaluates it against a series of patterns from top to bottom. As soon as it finds the **first matching pattern**, it executes the corresponding code block and exits immediately.

### ⚖️ The Rules: What You CAN and CANNOT Do

**✅ What you CAN do:**
*   **Destructure multiple types of data:** You can unpack Tuples, Structs, Enums, and even nested combinations of them.
*   **Bind variables on the fly:** You can capture unknown values into new local variables (like `n` or `p`) to use them inside the match arm.
*   **Use Conditional Guards (`if`):** You can attach an `if` statement to a pattern to add mathematical or logical checks before accepting the match.
*   **Use the Catch-All (`_`):** You can use `_` to explicitly ignore a specific value in a tuple/struct, or to catch all remaining unhandled cases.

**❌ What you CANNOT do:**
*   **CANNOT be non-exhaustive:** You must handle *every* possible case. If a variable can be anything, you must provide a catch-all (`_` or a variable name) at the end.
*   **CANNOT fall-through:** Unlike C or Java, Rust does not require a `break` keyword. Once a match is found, it will never execute the blocks below it.
*   **CANNOT mix brackets:** You cannot destructure a Tuple using `{}` or a Struct using `()`. You must mirror the original syntax.
*   **CANNOT return different types:** If the `match` is used as an expression to return a value, all arms must return the exact same data type.

---

### 🚀 Examples in Action

Below are various types of pattern matching implemented in Rust.

### 1. Destructuring Tuples
Tuples rely on **positional order**. You unpack them using parentheses `()`. You can check exact values, bind values to variables, or ignore them completely.

```rust
#[test]
fn match_security() {
    // (Brings ID?, Brings Bag?)
    let pengunjung = (true, true); 

    match pengunjung {
        // Pattern 1: Exact match for both values
        (true, false) => {
            println!("Silahkan masuk");
        }
        // Pattern 2: Exact match for both values
        (true, true) => {
            println!("Silahkan masuk tapi tasnya kami lihat dulu");
        }
        // Pattern 3: Catching false on the first position. 
        // The '_' ignores the second position (we don't care if they have a bag or not)
        (false, _) => { 
            println!("Maaf tidak boleh masuk karena tidak membawa ID card");
        }
    }
}

#[test]
fn match_player() {
    // (Health, Ammo)
    let player = (50, 70); 

    match player {
        // If health is 0, ignore the ammo count. Game over.
        (0, _) => {
            println!("Nyawa anda telah tidak ada");
        }
        // Exact match for full stats.
        (100, 100) => {
            println!("Kondisi prime siap bertempur");
        }
        // Health is greater than 0, but ammo is exactly 0.
        // We bind the current health to the variable 'n' to print it.
        (n, 0) => {
            println!("Nyawa masih {} tapi pelurumu ga ada bjir", n);
        }
        // Catch-all for any other combinations.
        // Binds both values to new variables 'n' and 'p'.
        (n, p) => {
            println!("Terus berjuang dengan sisa nyawa {} dan sisa peluru {}", n, p);
        }
    }
}
```

### 2. Destructuring Structs
Structs rely on named fields. You unpack them using curly braces {}. The order of fields doesn't matter, and you can use .. to ignore the rest of the fields you don't need.

```rust
struct Karyawan {
    nama: String,
    divisi: String,
}

#[test]
fn match_karyawan() {
    let budi = Karyawan {
        nama: String::from("Budi"),
        divisi: String::from("IT"),
    };

    match budi {
        // Destructures both 'nama' and 'divisi'. 
        // Also applies a Match Guard (if) to check a specific condition.
        Karyawan { nama, divisi } if divisi == "IT" => {
            println!("Tolong codingkan le {}", nama);
        }
        // The '..' (Rest pattern) tells Rust to ignore any other fields in the struct.
        // We only care about capturing the 'nama'.
        Karyawan { nama, .. } => {
            println!("Selamat bekerja {}", nama);
        }
    }
}
```

### 3. Match Guards (if inside match)
Match guards allow you to add complex logic (like greater than, less than, or logical AND/OR) right after destructuring the data, but before executing the block.

```rust
enum Kendaraan {
    Motor(u32),
    Mobil(u32),
    Truk(u32)
}

fn cek_tilang(target: Kendaraan) {
    match target {
        // Matches a Mobil, extracts the speed, THEN checks if it's over 100.
        Kendaraan::Mobil(kecepatan) if kecepatan > 100 => {
            println!("Kilat! Mobil melaju dengan kecepatan {} km/jam! Tilang!", kecepatan);
        }
        // Matches a Motor, extracts the speed, THEN checks if it's over 70.
        Kendaraan::Motor(kecepatan) if kecepatan > 70 => {
            println!("Anak Amor! Tilang, karena membahayakan. Berjalan dengan kecepatan {} km/jam", kecepatan);
        }
        // Matches a Truk, extracts the speed, THEN checks if it's over 60.
        Kendaraan::Truk(kecepatan) if kecepatan > 60 => {
            println!("Anak Amor! Tilang, karena membahayakan. Berjalan dengan kecepatan {} km/jam", kecepatan);
        }
        // Catch-all for any vehicle that didn't violate the speed limits above.
        _ => {
            println!("Kecepatan aman silahkan jalan");
        }
    }
}

#[test]
fn test_kamera_tol() {
    let kendaraan_1 = Kendaraan::Mobil(110); // Will trigger the > 100 guard
    let kendaraan_2 = Kendaraan::Mobil(60);  // Will fall to the '_' safe catch-all
    let kendaraan_3 = Kendaraan::Motor(75);  // Will trigger the > 70 guard
    let kendaraan_4 = Kendaraan::Truk(50);   // Will fall to the '_' safe catch-all

    cek_tilang(kendaraan_1);
    cek_tilang(kendaraan_2);
    cek_tilang(kendaraan_3);
    cek_tilang(kendaraan_4);
}
```

---
## Type Aliases in Rust (`type`)

### 📖 What is a Type Alias?
A **Type Alias** in Rust allows you to give a new, custom name (a nickname) to an existing data type. It does **not** create a brand new data type; it simply provides an alternative name that the compiler treats exactly the same as the original type.

You declare it using the `type` keyword.

### ⚙️ How It Works (`type` vs `let`)
It is important to understand the difference between creating a variable and creating a type:
*   Use `let` to store **Data/Values** (e.g., `let speed = 100;`).
*   Use `type` to name a **Data Blueprint/Type** (e.g., `type Speed = u32;`).

### ⚖️ The Rules: What You CAN and CANNOT Do

**✅ What you CAN do:**
*   **Mix and match:** Because an alias is just a nickname, you can safely mix the alias with the original type. A `u32` value can be passed into a function expecting your custom `type Kilometer = u32`.
*   **Shorten complex types:** You can alias long Tuples or deeply nested types (like `Result<String, std::io::Error>`) into a single, clean word.
*   **Clarify intent (Domain Modeling):** You can use it to give real-world context to primitive numbers, so other developers understand what the number represents.

**❌ What you CANNOT do:**
*   **CANNOT enforce strict type safety:** Because an alias is identical to its original type, the compiler will not stop you from accidentally adding `Kilometer` (u32) and `Kilogram` (u32) together. (If you need strict isolation, you must use a Tuple Struct, e.g., `struct Kilometer(u32);`).
*   **CANNOT store actual values:** `type` is only for defining the shape of the data, not for holding the data itself.

---

### 🚀 Common Use Cases & Code Examples

### 1. Shortening Tuples
Instead of writing `(f64, f64)` everywhere in your functions, you can create a single alias. This makes your function signatures much cleaner.

```rust
// When I say 'TitikKordinat', Rust knows I mean a tuple containing two f64 values.
type TitikKordinat = (f64, f64); 

// The parameter is much easier to read now
fn cetak_lokasi(lokasi: TitikKordinat) {
    println!("Berada di titik kordinat {} dan {}", lokasi.0, lokasi.1);
}

#[test]
fn test_titik_kordinat() {
    let rumah: TitikKordinat = (15.73, 30.13);
    cetak_lokasi(rumah);
}
```

### 2. Domain Modeling (Giving Context to Primitives)
Using aliases gives semantic meaning to your numbers. Without aliases, a function taking two u32 arguments might confuse a developer. With aliases, it reads like human language.

```rust
type JumlahBarang = u32;
type Duit = u64;

// The function signature clearly tells a story about what data it needs and returns.
fn hitung_omset(terjual: JumlahBarang, harga_satuan: Duit) -> Duit {
    // We must cast 'terjual' (u32) to 'Duit' (u64) so they share the same memory size before multiplying.
    let total = (terjual as Duit) * harga_satuan;
    total // No semicolon here so the value is returned
}

#[test]
fn test_omset_ukm() {
    let kopi_terjual: JumlahBarang = 150;
    // Using underscore '_' as a thousands separator makes the code highly readable.
    let harga_kopi: Duit = 25_000; 
    let total_pendapatan = hitung_omset(kopi_terjual, harga_kopi);
    
    println!("Total pendapatan ukm ini adalah Rp{}", total_pendapatan);

    let kopi_terjual_2: JumlahBarang = 75;
    let harga_kopi_2: Duit = 30_000;
    let total_pendapatan_2 = hitung_omset(kopi_terjual_2, harga_kopi_2);
    
    println!("Total pendapatan ukm ini adalah Rp{}", total_pendapatan_2);
}
```

### 3. Simplifying Complex Pattern Matching
When dealing with long Tuples that hold various states, an alias combined with pattern matching creates an incredibly robust and readable system.
```rust
// Defining a 4-element Tuple representing: Object Name, X coordinate, Y coordinate, Is Threat?
type DataRadar = (String, f64, f64, bool);

fn proses_radar(target: DataRadar) {
    // Destructuring the Tuple alias using pattern matching
    match target {
        // Pattern 1: Exact match when the boolean is true (Threat detected)
        (nama, x, y, true) => {
            println!("AWAS! Objek {} terdeteksi di kordinat {}, {}. Ini adalah ancaman!", nama, x, y);
        }
        // Pattern 2: Exact match when the boolean is false (Safe object)
        (nama, x, y, false) => {
            println!("Aman. Objek {} terpantau lewat di kordinat {} dan {}", nama, x, y);
        }
        // Notice we don't need a catch-all ('_') because a boolean can only be true or false.
        // Rust knows this match is 100% exhaustive!
    }
}

#[test]
fn test_sistem_radar() {
    let target1: DataRadar = (String::from("Ambatukam"), 10.5, 3.3, true);
    let target2: DataRadar = (String::from("Rusdiyansah"), 16.5, 3.3, false);

    proses_radar(target1);
    proses_radar(target2);
}
```

---
## Rust Module

### What is a Module?
In Rust, a **Module** (declared with the `mod` keyword) is a way to organize your code into separate namespaces. You can think of modules as **folders on your computer**, but for your code. They allow you to group related functions, structs, enums, and traits together, keeping your project structured and preventing name collisions.

### When to Use Modules?
*   **When your code gets too long:** If you have to scroll endlessly to find a function in `main.rs`, it's time to split it into modules.
*   **Domain Separation:** When building complex systems, you can separate different logic (e.g., `mod database`, `mod user_interface`, `mod security`).
*   **Team Collaboration:** Modules help prevent team members from accidentally modifying or relying on internal, unfinished code.

### ⚙️ How Does It Work?
The golden rule of Rust modules is **"Private by Default"**.
Everything you put inside a module (functions, structs, fields) is strictly secret. Code from outside the module cannot see or use it unless you explicitly grant permission by adding the `pub` (public) keyword.

### What You CAN and CANNOT Do

**✅ What you CAN do:**
*   **Hide Implementation Details (Encapsulation):** You can keep helper functions or sensitive data fields private while exposing a safe `pub` function to interact with them.
*   **Prevent Name Clashes:** You can have a `hitung()` function in `mod pajak` and a `hitung()` function in `mod diskon` without any errors.
*   **Nest Modules:** You can create modules inside modules (like folders inside folders).

**❌ What you CANNOT do:**
*   **You CANNOT instantiate a module:** Unlike Object-Oriented Programming (OOP) classes, you cannot create an "object" out of a module. It is purely an organizational container.
*   **You CANNOT access private fields:** Even if a `struct` is `pub`, its internal fields remain private unless they also have the `pub` keyword.
*   **You CANNOT use short names automatically:** Just declaring `mod name;` doesn't bring its contents into your current scope. You must use the `use` keyword to shorten the path.

---

### 1. Inline Module (Same File)
Inline modules are defined using `mod name { ... }` within the same file. This is useful for short groupings or unit tests.

```rust
// Defining an inline module named 'ekspedisi'
mod ekspedisi {
    // Creating public Type Aliases
    pub type NomorResi = String;
    pub type BeratKg = f64;

    // A public Enum for shipping status
    pub enum StatusPengiriman {
        Packing,
        Dijalann(String), // Carries the courier's name
        Terkirim,
        Nyasar
    }

    // A public Struct with mixed privacy fields
    pub struct Paket {
        pub resi: NomorResi,       // Public: Anyone can see the receipt number
        pub tujuan: String,        // Public: Anyone can see the destination
        berat: BeratKg,            // Private: Internal data, cannot be accessed directly
        status: StatusPengiriman   // Private: Can only be changed via official methods
    }

    impl Paket {
        // Associated function (Constructor) to create a new Package
        pub fn terima_paket(resi: NomorResi, tujuan: String, berat: BeratKg) -> Paket {
            Paket {
                resi: resi,
                tujuan: tujuan,
                berat: berat,
                status: StatusPengiriman::Packing // Initial status is always 'Packing'
            }
        }

        // Method to update the private 'status' field
        pub fn update_status(&mut self, status_baru: StatusPengiriman) {
            self.status = status_baru;
        }

        // Method to read data using borrowing (&self) to avoid moving ownership
        pub fn lacak(&self) {
            // Using &self.status to peek at the data without stealing it
            match &self.status {
                StatusPengiriman::Packing => {
                    println!("Paket {} tujuan {} sedang dipacking dengan berat {} kg", self.resi, self.tujuan, self.berat)
                }
                StatusPengiriman::Dijalann(nama_kurir) => {
                    println!("Paket {} sedang dibawa oleh kurir {}", self.resi, nama_kurir)
                }
                StatusPengiriman::Terkirim => {
                    println!("Mantap paket {} sudah datang, terima kasih sudah menggunakan jasa Ambarusdi", self.resi)
                }
                StatusPengiriman::Nyasar => {
                    println!("paket {} nyasar hehehe", self.resi)
                }
            }
        }
    }
}

// Importing the struct with an alias to make it shorter
use ekspedisi::Paket as Barang;
// Importing the enum from the crate root
use crate::ekspedisi::StatusPengiriman; 

#[test]
fn test_amba_rusdi_express() {
    // Creating a mutable instance so we can change its status later
    let mut paket_baru = Barang::terima_paket(String::from("Jmk33"), String::from("Ngawi"), 33.1);
    
    paket_baru.lacak();
    paket_baru.update_status(StatusPengiriman::Dijalann(String::from("Mas Amba")));
    paket_baru.lacak();
    paket_baru.update_status(StatusPengiriman::Nyasar);
    paket_baru.lacak();
    paket_baru.update_status(StatusPengiriman::Terkirim);
    paket_baru.lacak();
}
```

### 2. File Module (Different Files)
   When your code grows, you can move modules into separate files. The file name automatically becomes the module name. You do not need to wrap the code in mod { } inside the new file.

### File 1: scanner.rs
(Place this file in the src directory, alongside main.rs)

```rust
// In a separate file, everything written here belongs to the 'scanner' module automatically.

pub enum KatergoriMalware {
    Ransomware,
    Spyware,
    Aman
}

pub struct LogJaringan {
    pub nama_file: String,
    pub ip_sumber: String,
    status: KatergoriMalware // Private field to prevent unauthorized tampering
}

impl LogJaringan {
    // Constructor function
    pub fn analisis_file(nama: String, ip: String, status: KatergoriMalware) -> LogJaringan {
        LogJaringan {
            nama_file: nama,
            ip_sumber: ip,
            status: status
        }
    }
    
    // Method to check status using borrowing (&self)
    pub fn cetak_peringatan(&self) {
        match &self.status {
            KatergoriMalware::Ransomware => {
                println!("Bahaya! {} dari {} terdeteksi sebagai ransomware. Segera blokir jaringan ", self.nama_file, self.ip_sumber);
            }
            KatergoriMalware::Spyware => {
                println!("awas aktivitas mencurigakan dari file {} dengan ip {} terindikasi spyware ", self.nama_file, self.ip_sumber);
            }
            KatergoriMalware::Aman => {
                println!("Jaringan aman!, file {} bersih ", self.nama_file);
            }
        }
    }
}
```

### File 2: main.rs

```rust
// 1. Register the module. This tells Rust to look for a file named 'scanner.rs'
mod scanner;

// 2. Import the specific items we need so we don't have to type 'scanner::' everywhere
use scanner::LogJaringan;
use scanner::KatergoriMalware;

#[test]
fn test_keamanan_jaringan() {
    let file_1 = LogJaringan::analisis_file(String::from("update_palu.exe"), String::from("192.167.93"), KatergoriMalware::Ransomware);
    file_1.cetak_peringatan();
    
    let file_2 = LogJaringan::analisis_file(String::from("Ambacong di desa oncong.mp3"), String::from("192.163.93"), KatergoriMalware::Aman);
    file_2.cetak_peringatan();
    
    let file_3 = LogJaringan::analisis_file(String::from("Ambalabu.mkv"), String::from("191.163.91"), KatergoriMalware::Spyware);
    file_3.cetak_peringatan();
}
```

---
## Rust Traits: Defining Shared Behavior

### What is a Trait?
In Rust, a **Trait** is a collection of methods defined for an unknown type: `Self`. You can think of it as a "contract" or an "interface" (if you are familiar with OOP languages like Java or C#). If a struct wants to have a certain ability, it must implement the trait and fulfill the contract by writing the logic for those methods.

---

### What Traits CAN Do
*   **Define Shared Behavior:** You can define a standard set of actions that multiple different structs must be able to perform.
*   **Default Implementations:** You can provide a default body for a method inside the trait. If a struct implements the trait, it can use the default behavior or override it.
*   **Trait Inheritance (Supertraits):** You can force a trait to depend on another trait. (e.g., To be an Advanced Wizard, you MUST first be a Basic Wizard).
*   **Act as Type Bounds:** You can restrict Generic functions to only accept types that have specific abilities (e.g., `fn attack<T: Weapon>(item: T)`).
*   **Polymorphism (Dynamic Dispatch):** Using `Box<dyn Trait>`, you can store different structs in the same collection as long as they implement the same trait.

### What Traits CANNOT Do
*   **Cannot Store Data (State):** Traits cannot have fields or attributes. They only define *behavior* (methods). You must use a `struct` or `enum` to hold data.
*   **Cannot Violate the Orphan Rule:** You cannot implement an external trait (like `Display` from standard library) on an external type (like `String`). Either the trait or the struct must be created by you in your own code.

---

### Types of Traits (Concepts)
1.  **Basic Traits:** A simple list of methods a struct must implement.
2.  **Supertraits:** A trait that requires another trait to be implemented first (used in your code below).
3.  **Generic Traits:** Traits that accept a generic type parameter (e.g., `trait CanEat<T>`).
4.  **Marker Traits:** Traits that have no methods at all (like `Send`, `Sync`, or `Copy`). They just act as a "tag" to tell the Rust compiler that a type has a certain property.

---

### Code Example: The Magic Academy 🪄

Below is a demonstration of Basic Traits, Supertraits, and Trait as Function Parameters (`impl Trait`).

```rust
// 1. BASIC TRAIT
// Defines a fundamental ability that any wizard must have.
pub trait SihirDasar {
    fn keluarkan_cahaya(&self);
}

// 2. SUPERTRAIT (Trait Inheritance)
// To implement 'SihirTingkatLanjut', a struct MUST ALSO implement 'SihirDasar'.
// You cannot cast advanced magic if you don't know basic magic!
pub trait SihirTingkatLanjut: SihirDasar {
    fn panggil_meteor(&self);
}

// 3. THE STRUCT (Data/State)
// This holds the actual data. Traits cannot hold data, only structs can.
pub struct PenyihirSakti {
    pub nama: String,
    pub level: i32,
}

// 4. IMPLEMENTING THE BASIC TRAIT
// Fulfilling the contract for 'SihirDasar'
impl SihirDasar for PenyihirSakti {
    fn keluarkan_cahaya(&self) {
        println!("Penyihir {} mengeluarkan bola cahaya dari tongkatnya dan levelnya sekarang {}", self.nama, self.level);
    }
}

// 5. IMPLEMENTING THE SUPERTRAIT
// Fulfilling the contract for 'SihirTingkatLanjut'.
// Rust allows this because PenyihirSakti already implemented SihirDasar above.
impl SihirTingkatLanjut for PenyihirSakti {
    fn panggil_meteor(&self) {
        println!("Penyihir {} berhasil memanggil meteor dan levelnya sekarang {}", self.nama, self.level);
    }
}

// 6. TRAIT AS A PARAMETER (impl Trait)
// This function accepts ANY struct, as long as it has the 'SihirTingkatLanjut' trait.
// Because it requires the Supertrait, we can safely call BOTH basic and advanced methods.
pub fn jalankan_ujian_elite(lulus: &impl SihirTingkatLanjut) {
    println!("========= Ujian dimulai ===========");
    
    // Calling the basic magic method (inherited)
    lulus.keluarkan_cahaya();
    
    // Calling the advanced magic method
    lulus.panggil_meteor();
    
    println!("Penyihir lulus dan kini menjadi penyihir sakti");
}

// 7. MAIN EXECUTION
fn show_akademi_sihir() {
    // Instantiating the struct
    let penyihir = PenyihirSakti {
        nama: String::from("Mas Fuad"),
        level: 36,
    };
    
    // Passing the struct by reference to the function that requires the trait
    jalankan_ujian_elite(&penyihir);
}
```

---

## Rust Generics: The Ultimate Shape-Shifter 

### What are Generics?
In Rust, **Generics** are a way to write code that can handle multiple data types without having to copy-paste or duplicate the logic for each type. Instead of hardcoding a specific type like `i32` or `String`, we use a placeholder (usually `<T>`, `<U>`, etc.).

Rust replaces these placeholders with the actual data types at compile time. This process is called **Monomorphization**, which means using Generics in Rust has **Zero-Cost Abstraction** (it doesn't slow down your program at all).

---

### Types of Generics
1.  **Generic Structs:** Structs that can store fields of any data type.
2.  **Generic Functions:** Functions that can accept arguments or return values of any data type.
3.  **Generic Enums:** Enums that can hold variants with flexible data types (like `Option<T>` and `Result<T, E>`).
4.  **Generic Methods (`impl`):** Methods attached to generic structs that can even change the generic type during execution.
5.  **Generic Type Bounds (Trait Bounds):** Restricting a generic `<T>` so it MUST have a specific ability (Trait) using `:` or `where`.
6.  **Generic Traits:** Traits that accept a generic parameter, allowing a struct to implement the same trait multiple times for different targets.

---

### What Generics CAN Do
*   **Prevent Code Duplication:** Write a function or struct once, use it for strings, integers, floats, or custom structs.
*   **Ensure Type Safety:** If you declare a generic collection of `<String>`, the compiler will immediately block you if you try to insert an `i32`.
*   **Flexible APIs:** Create highly adaptable systems (like network payloads or database connections) that work with any data format.

### What Generics CANNOT Do
*   **Perform Operations Without Bounds:** You **cannot** use operators like `+`, `-`, `>`, `==`, or call methods on a raw `<T>`. Why? Because Rust doesn't know if `<T>` is a number, a string, or a table! You *must* use Trait Bounds (like `<T: PartialOrd>`) to unlock those operations.
*   **Mix Types in a Single Variable:** If you create a `Vec<T>`, and the first item is `i32`, the entire Vector is now locked to `i32`. You cannot suddenly add a `String` to it. (To mix types, you need Trait Objects like `Box<dyn Trait>`, not Generics).

---

### Code Examples (Categorized & Commented)

### 1. Generic Structs
*Using placeholders to allow a struct to hold various data types.*

```rust
#[derive(Debug)]
#[allow(dead_code)]
// CATEGORY: Generic Struct
// We use 3 different placeholders: T, Ttahun, and Tstatus.
// This means each field can be a completely different data type.
struct KapsulWaktu <T, Tstatus, Ttahun,> {
    isi: T,
    tahun_dibuka: Ttahun,
    diserahkan: Tstatus
}

#[test]
fn test_kapsul_waktu() {
    // Rust automatically detects that T=String, Ttahun=i32, and Tstatus=&str
    let kapsul_rusdi = KapsulWaktu {
        isi: String::from("Uget uget boyolali"),
        tahun_dibuka: 2026,
        diserahkan: "Y"
    };
    println!("{:?}", kapsul_rusdi);
}
```

### 2. Generic Functions
Functions that accept and return flexible types

```rust
// CATEGORY: Generic Function
// Accepts two generic parameters (T and U).
// Returns a tuple where their positions (and types) are swapped.
fn tukar_posisi<T,U>(kiri: T, kanan: U) -> (U,T) {
    (kanan, kiri)
}

#[test]
fn test_tukar_posisi() {
    // T is String, U is i32. The return will be (i32, String).
    let posisi = tukar_posisi(String::from("Rusdiyansah tukar posisi ke kordinat"), 30 );
    println!("{:?}", posisi);
}
```

### 3. Generic Enums
Enums that can hold different types of data in their variants

```rust
#[allow(dead_code)]
// CATEGORY: Generic Enum (Single Generic)
// This is exactly how Rust's built-in 'Option' works!
enum Value<T> {
    NONE,
    VALUE(T)
}

#[test]
fn test_value() {
    let value: Value<i32> = Value::VALUE(30);

    match value {
        Value::NONE => println!("NONE"),
        Value::VALUE(val) => println!("VALUE = {}", val),
    }
}

// CATEGORY: Generic Enum (Multiple Generics)
// Aman holds type T, Diserang holds type U, Maintenance holds nothing.
enum StatusServer<T, U> {
    Aman(T),
    Diserang(U),
    Maintenance
}

// Function accepting a specific realization of the Generic Enum
fn sinyal_server(s: StatusServer<String, i32>) {
    match s {
        StatusServer::Aman(teks) => println!("Status server saat ini {}", teks),
        StatusServer::Diserang(ip) => println!("Status server diserang, ping saat ini {}", ip),
        StatusServer::Maintenance => println!("Sedang maintenance")
    }
}

#[test]
fn test_status_server() {
    let status_server: StatusServer<String, i32> = StatusServer::Aman(String::from("Aman"));
    let status_server2: StatusServer<String, i32> = StatusServer::Diserang(300);
    let status_server3: StatusServer<String, i32> = StatusServer::Maintenance;
    
    sinyal_server(status_server);
    sinyal_server(status_server2);
    sinyal_server(status_server3);
}
```

### 4. Generic Methods (Typestate Pattern)
Implementing methods for a generic struct, and using generics to change the struct's type.

```rust
struct PaketJaringan<T>{
    payload: T
}

// CATEGORY: Generic Methods
// We must declare <T> after impl so Rust knows T is a generic placeholder.
impl<T> PaketJaringan<T> {
    fn baca_payload(&self) -> &T {
        &self.payload
    }
    
    // CATEGORY: Method-Level Generic (Changing State)
    // This method takes ownership of the current struct (self),
    // and returns a BRAND NEW struct with a DIFFERENT generic type <U>.
    fn ganti_protokol<U>(self, payload_baru: U) -> PaketJaringan<U> {
        PaketJaringan {
            payload: payload_baru
        }
    }
}

#[test]
fn test_paket_jaringan() {
    let paket_awal = PaketJaringan { payload: String::from("GET /admin_panel") }; // T is String
    println!("Payload awal: {}", paket_awal.baca_payload());

    // paket_awal is consumed. paket_enkripsi is born where U is i32.
    let paket_enkripsi = paket_awal.ganti_protokol(8080); 
    println!("Payload setelah enkripsi: {}", paket_enkripsi.baca_payload());
}
```

### 5. Generic Type Bounds (where clause)
Restricting generics so they can use specific operators or methods.

```rust
// CATEGORY: Generic Type Bounds (Using 'where')
// Without bounds, we cannot use '>' or println!("{}", ...).
// We restrict T: it MUST be printable (Display) AND comparable (PartialOrd).
fn cetak_yang_tertinggi<T>(sensor_a: T, sensor_b: T)
    where T: std::fmt::Display + std::cmp::PartialOrd 
{
    if sensor_a > sensor_b {
        println!("Peringatan! nilai tertinggi! {} ", sensor_a)
    } else {
        println!("Peringatan! nilai tertinggi {}", sensor_b)
    }
}
```

### 6. Generic Type Bounds (Direct Syntax)
Using your own custom traits to restrict a generic function.

```rust
pub trait BisaMenyala {
    fn hidupkan(&self);
}

pub struct Lampu;
pub struct KipasAngin;

impl BisaMenyala for Lampu {
    fn hidupkan(&self) { println!("Lampu menyala terang") }
}
impl BisaMenyala for KipasAngin {
    fn hidupkan(&self) { println!("Kipas Angin berputar mantap") }
}

// CATEGORY: Generic Type Bounds (Direct Syntax: <T: Trait>)
// T can be anything, AS LONG AS it has the 'BisaMenyala' trait.
pub fn tombol_pintar<T: BisaMenyala>(alat: T) {
    alat.hidupkan(); 
}
```

### 7. Generic Traits
A trait that takes a generic parameter, allowing a struct to implement it multiple times.

```rust
// CATEGORY: Generic Trait
// The Trait itself has a placeholder <T>.
pub trait KirimPesan<T> {
    fn kirim(&self, tujuan: T, pesan: String);
}

pub struct SistemKeamanan;
pub struct Email { pub email: String }
pub struct Sms { pub nomor: String }

// Implementing the trait where T = Email
impl KirimPesan<Email> for SistemKeamanan {
    fn kirim(&self, tujuan: Email, pesan: String ){
        println!("Mengirim email ke {} dengan pesan {}", tujuan.email, pesan);
    }
}

// Implementing the EXACT SAME trait where T = Sms
impl KirimPesan<Sms> for SistemKeamanan {
     fn kirim(&self, tujuan: Sms, pesan: String) {
        print!("Mengirim sms darurat ke nomor {} dengan pesan {}", tujuan.nomor, pesan);
    }
}
```

---

## Rust Overloadable Operators 

### What are Overloadable Operators?
In Rust, you cannot just use math symbols like `+`, `-`, or `*` on your own custom `struct` or `enum` right out of the box. The compiler won't know how to add or multiply them.

**Operator Overloading** is the feature that allows you to define custom behavior for these standard operators. In Rust, this is simply done by implementing specific **Generic Traits** provided in the `std::ops` and `std::cmp` modules.

---

### What You CAN Do
*   **Use Math Symbols on Custom Structs:** You can make your code much cleaner by typing `struct_a * struct_b` instead of `struct_a.multiply(struct_b)`.
*   **Cross-Type Operations:** You can multiply/add two *different* types together (e.g., multiplying a `Salary` struct by a `Days` struct).
*   **Define Custom Return Types:** By using the `type Output` inside the trait, you get to decide exactly what type of data is produced after the operation.

### What You CANNOT Do
*   **Create New Operator Symbols:** You cannot invent new symbols like `$$`, `<->`, or `@`. You can only overload existing Rust operators.
*   **Change Operator Precedence:** You cannot change the rule of mathematics. Multiplication (`*`) will always be evaluated before Addition (`+`), even on your custom structs.
*   **Overload Short-Circuiting Operators:** You cannot overload `&&` (AND) and `||` (OR).
*   **Violate the Orphan Rule:** You cannot redefine how `i32 + i32` works, or how `String == String` works. You can only overload operators if the `struct` or the `trait` belongs to you.

---

### Types of Overloadable Operators
Here are the most common traits you can import from `std::ops` (and `std::cmp`):

| Operator Symbol | Trait Name | Module | Example Usage |
| :--- | :--- | :--- | :--- |
| `+` | `Add` | `std::ops::Add` | `a + b` |
| `-` | `Sub` | `std::ops::Sub` | `a - b` |
| `*` | `Mul` | `std::ops::Mul` | `a * b` |
| `/` | `Div` | `std::ops::Div` | `a / b` |
| `%` | `Rem` | `std::ops::Rem` | `a % b` (Remainder/Modulo) |
| `==` / `!=` | `PartialEq` | `std::cmp::PartialEq`| `a == b` |
| `+=` | `AddAssign`| `std::ops::AddAssign` | `a += b` |

---

### Code Example: Salary Calculation System

```rust
// 1. IMPORT THE TRAIT
// We need the 'Mul' (Multiply) trait to unlock the '*' symbol.
use std::ops::Mul;

// 2. DEFINE THE STRUCTS (No magic #[derive] used here!)
pub struct GajiHarian {
    pub upah: i32
}

pub struct HariKerja {
    pub hari: i32
}

pub struct GajiTotal{
    pub total: i32
}

// 3. IMPLEMENT THE OPERATOR OVERLOADING
// Meaning: "Allow GajiHarian to be multiplied by HariKerja"
impl Mul<HariKerja> for GajiHarian {
    // Associated Type: We tell Rust that the final result of this multiplication 
    // will be a brand new struct called 'GajiTotal'.
    type Output = GajiTotal;

    // 'self' is the Left-Hand Side (GajiHarian).
    // 'rhs' (Right-Hand Side) is the parameter on the right of the '*' symbol (HariKerja).
    fn mul(self, rhs: HariKerja) -> GajiTotal {
        // We multiply the raw integers inside the structs, 
        // and wrap the result inside the new GajiTotal struct.
        GajiTotal {
            total: self.upah * rhs.hari
        }
    }
}

// Assume this is imported properly in your module structure
// use crate::penggajian_karyawan::*;

#[test]
fn test_hitung_gaji() {
    let gaji_karyawan = GajiHarian { upah: 150_000 };
    let absen_bulan_ini = HariKerja { hari: 20 };

    // 4. THE MAGIC HAPPENS HERE! 
    // We multiply two completely different structs using the standard '*' symbol.
    // Behind the scenes, Rust calls the 'mul(self, rhs)' function we defined above.
    let slip_gaji = gaji_karyawan * absen_bulan_ini;

    // Verify the result is exactly 3.000.000 (150.000 x 20)
    assert_eq!(slip_gaji.total, 3_000_000);

    // Because we didn't use #[derive(Debug)], we print the inner field manually
    println!("Total gaji yang harus dibayar: Rp {}", slip_gaji.total);
}
```

---
## Rust Optional Values (`Option<T>`) 📦

### What is `Option<T>`?
In many programming languages (like Java, C++, or PHP), trying to access missing data returns `null`. If your program forgets to check for `null`, it crashes instantly (this is known as the "Billion Dollar Mistake").

Rust **does not have `null`**. Instead, it uses an Enum called `Option<T>` to represent a value that might be there, or might be empty.

Under the hood, it looks like this:
```rust
enum Option<T> {
    None,      // The box is empty (No data)
    Some(T),   // The box has something inside (Contains data of type T)
}
```

### What You CAN Do
1. Prevent Crashes: By using Option, the Rust compiler forces you to handle the empty (None) scenario before you can use the data. This makes NullPointerExceptions impossible.

2. Provide Default Values: You can easily say, "If the data is missing, just use this default number instead."

3. Chain Operations: You can manipulate the data inside the box safely without opening it using methods like .map() or .and_then().


### What You CANNOT Do
* **Use the Value Directly: You cannot do math or operations directly on an Option. For example, Some(5) + 10 will result in an error. You MUST open the box first.**

* **Ignore the Empty Case in a match: If you use a match block to open an Option, you cannot only write the Some case. The compiler will force you to also write the None case (Exhaustive Checking).**

* **Ways to Unpack an Option  There are several ways to extract the data from inside the Option box:**

* **match: The safest and most explicit way. Handles both Some and None comprehensively.**

* **if let: A shortcut when you only care about the Some case and want to ignore the None case silently.**

* **unwrap_or(default): Extracts the value, but if it is None, it replaces it with a default value you provide.**

* **unwrap() / expect("msg"): The dangerous way. Forces the box open. If it's None, the program instantly crashes (Panics).**

### Code Example: Warehouse Inventory System

```rust
// 1. FUNCTION RETURNING OPTION
// Returns Option<i32> because the item might exist (Some), or it might not (None).
fn cari_stok_barang(nama_barang: &str) -> Option<i32> {
if nama_barang == "Laptop" {
Some(50) // Box contains 50
} else if nama_barang == "Keyboard" {
Some(90) // Box contains 90
} else {
None     // Box is empty
}
}

#[test]
fn test_stok_gudang() {
// SCENARIO 1: The item EXISTS
let barang_1 = "Laptop";
let pencarian_1 = cari_stok_barang(barang_1);

    // Unpacking with 'match' (Safest method)
    match pencarian_1 {
        Some(barang) => {
            // We successfully extract the number into 'barang'
            println!("Stok barang laptop adalah {}", barang);
        },
        None => {
            print!("Tidak ada barang yang dicari");
        }
    }

    // SCENARIO 2: The item EXISTS (Different branch)
    let barang_2 = "Keyboard";
    let pencarian_2 = cari_stok_barang(barang_2);

    match pencarian_2 {
        Some(barang) => {
            // Fixed typo: changed 'laptop' to 'keyboard' for clarity
            println!("Stok barang keyboard adalah {}", barang);
        },
        None => {
            print!("Tidak ada barang yang dicari");
        }
    }

    // SCENARIO 3: The item DOES NOT EXIST
    let barang_3 = "Sempak"; // Not in our database!
    let pencarian_3 = cari_stok_barang(barang_3);

    match pencarian_3 {
        Some(barang) => {
            // Fixed typo: changed 'laptop' to 'sempak' for clarity
            println!("Stok barang sempak adalah {}", barang);
        },
        None => {
            // Program safely executes this block instead of crashing!
            print!("Tidak ada barang yang dicari");
        }
    }
}
```

---

## Rust Comparison (Equality & Ordering) ⚖️

### What is Comparison?
Comparison in Rust is the process of evaluating two values to see if they are identical, or if one is greater or lesser than the other.

In Rust, comparison is not built into the core language syntax by magic. Instead, mathematical operators like `==`, `!=`, `>`, and `<` are powered by **Traits** from the `std::cmp` (Compare) module.

###How it Works
When you type `a == b` or `a > b`, the Rust compiler looks for specific traits:
1.  **`PartialEq` (Partial Equality):** Unlocks the `==` and `!=` operators.
2.  **`PartialOrd` (Partial Ordering):** Unlocks the `<`, `>`, `<=`, and `>=` operators.

For custom `structs` or `enums`, you have two ways to make these operators work:
1.  **The Automatic Way:** Add `#[derive(PartialEq, PartialOrd)]` above your struct. Rust will generate the comparison logic for you in the background.
2.  **The Manual Way:** Write `impl PartialEq for YourStruct` (Operator Overloading) if you want custom logic.

*Note: If you use the `derive` macro, Rust compares fields **top-to-bottom**. It checks the first field; if they are a tie, it moves to the second field, and so on.*

### When to Use It
*   **Filtering & Logic:** Checking if a user's input matches a specific value or password.
*   **Ranking & Sorting:** Finding the highest score in a leaderboard or determining which product is better.
*   **Data Validation:** Ensuring a numerical value is within a safe range (e.g., `CPU_temp < 90`).

---

### What You CAN Do
*   **Compare Primitive Types Directly:** You can compare numbers (`i32`), booleans (`bool`), and even text (`String`) right out of the box without any setup. (Alphabetically, `"Z" > "A"` is true).
*   **Write Clean Logic for Structs:** Instead of writing long `if a.rating > b.rating` statements, you can directly write `if a > b` to compare entire objects at once.
*   **Unlock High-Level Features:** Implementing comparison traits allows you to use powerful built-in methods like `.sort()` on arrays and vectors.

### What You CANNOT Do
*   **Compare Structs Without Traits:** You cannot use `==` or `>` on a custom `struct` if you haven't implemented `PartialEq` or `PartialOrd` (either via `derive` or manually). The compiler will throw a hard error.
*   **Compare Different Data Types:** You cannot compare an `i32` with a `String`, or an `i32` with an `f64`. Rust is strictly typed; `5 == "5"` is illegal. You must cast or convert them to the same type first.
*   **Use `=` for Comparison:** A single `=` is strictly for assigning values to variables. You must use `==` to check for equality.

---

### Code Example: E-Commerce Product Recommendation

```rust
// 1. THE MAGIC MACRO (Automatic Trait Implementation)
// - PartialEq: Teaches Rust how to use '==' and '!=' for this struct.
// - PartialOrd: Teaches Rust how to use '<' and '>' for this struct.
// - Debug: Allows us to print the struct using {:?}
#[derive(PartialEq, PartialOrd, Debug)]
struct Produk {
    // 2. THE ORDER MATTERS!
    // Because 'rating' is at the top, Rust prioritizes it. 
    // It will only look at 'terjual' if the ratings are exactly identical.
    rating: i32,
    terjual: i32
}

#[test]
fn test_rekomendasi_produk() {
    // Instantiating three different products
    let produk_a = Produk { rating: 50, terjual: 90 };
    let produk_b = Produk { rating: 3, terjual: 30 };
    let produk_c = Produk { rating: 4, terjual: 190 };

    // 3. EQUALITY CHECK (Powered by PartialEq)
    // Rust compares produk_a.rating with produk_b.rating.
    // If they were equal, it would then compare the 'terjual' field.
    if produk_a == produk_b {
        println!("Produk identik");
    }
    
    // 4. ORDERING CHECK (Powered by PartialOrd)
    // Rust compares produk_b (rating: 3) with produk_c (rating: 4).
    // Because 3 is NOT greater than 4, it falls into the 'else' block.
    // (Note: The printed text logic is a bit funny, but the Rust syntax execution is 100% flawless!)
    if produk_b > produk_c {
        println!("Produk b masih kalah"); 
    } else {
        println!("produk c masih kalah");
    }
}
```