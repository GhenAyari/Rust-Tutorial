
fn main(){
    println!("Hello, world!");
}

#[test]
fn testing(){
    println!("my name's ghen and i currently learn rust");
}

#[test]
fn variable(){
    let my_name = "Ghendida";
    println!("Hallo {} ", my_name);
}

#[test]
fn variable_mutable(){
    let mut age_in_2025: i8 = 18;
    println!("my age in 2025 is {} ", age_in_2025);

    age_in_2025 = 19;
    println!("my age in 2026 is {} ", age_in_2025);
}

// #[test]
// fn static_type(){
//     let mut my_github = "GhenAyari";
//     println!("My github is {}", my_github);
//
//     my_github = 1;
//     println!("My github is {}", my_github);
// }

#[test]
fn shadowing(){
    let name = "Ghendida";
    println!("Hallo {} ", name);

    let name = 10;
    println!("it's the {}th now ", name);

    let name = 2026;
    println!("this is {} year ", name);
}

#[test]
fn explicit_variable(){
    let age: i8 = 19;
    println!("My age is {} ", age);

    let weight: f32 = 51.5;
    println!("my body weight is {} ", weight);
}

#[test]
fn conversion(){
    let a: i8 = 19;
    println!("my number {} ", a);

    let b: i16 = a as i16;
    println!("his number is {} ", b);

    let c : i32 = a as i32;
    println!("my number {} ", c);
}

#[test]
fn conversion_to_large(){
    let a: i64 = 1000000;
    println!("number {} ", a);

    let b: i8 = a as i8;
    println!("number {} ", b);

}

#[test]
fn operators_numeric(){

    let height = 3.0;

    let a = 5.0;

    let b = 8.0;

    let l = 0.5;

    let result = l * (a + b) * height;

    println!("result = {}, ({} + {}), X {}, = {} ", l, a, b, height, result);


}

