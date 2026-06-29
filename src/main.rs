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

#[test]
fn shadowing(){
    let name = "Ghendida";
    println!("Hallo {} ", name);

    let name = 10;
    println!("it's the {}th now ", name);

    let name = 2026;
    println!("this is {} year ", name);
}