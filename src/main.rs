use std::io;
use std::io::Write;

fn main(){

    print!("Inpur your name = ");
    io::stdout().flush().unwrap();

    let mut nama = String::new();
    io::stdin().read_line(&mut nama).expect("Failed to read line");

    print!("input your age = ");
    io::stdout().flush().unwrap();

    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read age");

    let age: f32 = age.trim().parse().unwrap();
    println!();
    print!("Hallo {}", nama);
    print!("your age is {}", age);
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

#[test]
fn boolean_operators(){

    let age = 20;
    let height = 170;

    let category = 18 <= age;
    let height = 165 <= height;

    let result = category && height;

    println!("is he an adult man? {}", result);

}

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


#[test]
fn tuple_practice(){
    let mut me: (&str, i8, i16, &str) = ("Ghen", 19, 2006, "Mulawarman University");

    println!("my name's {}, my age is {}, i was born in {}, and currently i'm student in {} ", me.0, me.1, me.2, me.3 );

    me.0 = "Ghendida";
    me.1 = 20;

    println!("my name's is {}, my age is {}, i was born in {}, and currently i'm student in {} ", me.0, me.1, me.2, me.3 );
}

#[test]
fn array(){

    let array_list: [i8; 3] = [10, 20, 30];
    println!("here are some array = {:?}", array_list);

    let a = array_list[0];
    let b = array_list[1];
    let c = array_list[2];

    println!("{}, {}, {}", a, b, c);

}

#[test]
fn mutable_array(){

    let mut array_can_change: [&str; 3] = ["Ramli", "Ruger", "Razi"];

    println!("{:?}", array_can_change);

    array_can_change[0] = "Rizal";
    array_can_change[1] = "Raditya";
    array_can_change[2] = "Roslan";

    println!("{}, {}, {}" , array_can_change[0], array_can_change[1], array_can_change[2]);

}

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

const MAXIMUM: i16 = 37;
#[test]
fn const_variable() {
    const MINIMUM: i16 = 33;
    println!("Use constant variable {}", MINIMUM);

    println!("We can use variable out of scope {}", MAXIMUM);


}

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
    let name: &str = "  Ghendida  ";

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

#[test]
fn string_not_fixed_size() {

    let name: String = String::from("ghendida ayari");
    println!("{}\n",name.to_lowercase());

    let mut list_name: (String, String, String) = (String::new(), String::from("satrio"), String::from("Rusman"));
    println!("{:?}", list_name);

    list_name.0.push_str("Akmal");

    println!("\n{}, {}, {} ", list_name.0.to_uppercase() , list_name.1.to_uppercase(), list_name.2.replace("Rusman", "Ramli").to_uppercase());


}

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

#[test]
fn data_copy() {
    let a = 16;
    let b = a;

    println!("{}, {}", a, b);

}

#[test]
fn ownership_movement() {

    let name: String = String::from("Ghendida");

    // ownership name has move to name_2
    let name_2 = name;
    // name variable has can't access because ownership move to name_2

    println!("{}", name_2);

}

#[test]
fn clone() {

    let name: String = String::from("Ghendida");

    let name2 = name.clone();

    println!("{}, {}", name, name2); // This is known as a clone.
    // If the string data is 10 MB, Rust will perform a clone of the same size, which is 10 MB

}

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

#[test]
fn let_statement_simple() {
    let value = 75;

    let result = if value >= 80 {
        "Good"
    } else if value >= 70 {
        "Not Bad"
    } else if value >= 60 {
        "Bad"
    } else {
        "Shit"
    };

    println!("{}", result);
}

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


#[test]
fn loop_label() {
    let mut number = 1; // number sama dengan 1

    'luar: loop { // loop paling luar
        let mut i = 1; // variable baru i sama dengan 1

        loop {
            if number > 10 { // jika number lebih dari 10
                break 'luar; // hentikan
            }
            println!("{} X {} = {}", number, i, number * i); // memanggil number dan i = number x i
            i += 1; // i akan ditambahkan 1 terus
            if i > 10 { // jika i lebih dari 10
                break; // hentikan
            }
        }

        number += 1; // number akan terus ditambahkan 1
    }
}

#[test]
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

#[test]
fn loop_tuple_tapi_tidak_bisa() {

    // untuk di tuple itu tidak bisa menggunakan perulangan, yang bisa hanya di array

    let tuple: (i8, i8, &str) = (16, 10, "Ambatukam");

    let (index1, index2, index3) = tuple;

    println!("Nilai 1 = {}", index1);
    println!("Nilai 2 = {}", index2);
    println!("Nilai 3 = {}", index3);

}

#[test]
fn loop_array() {

    let arrayy: [&str; 5] = ["Ambatukam", "Rusman", "Rusdiyansah", "Marlan", "Zuki"]; // membuat array

    for value in arrayy  { // value adalah variable baru dan di dalam array
        print!(" {} ", value); // memanggil value
    }
}

#[test]
fn range_exclude() {
    // let arrayy: [&str; 5] = ["Ambatukam", "Rusman", "Rusdiyansah", "Marlan", "Zuki"];

    // contoh range exclude
    let range = 0..6; // rangenya 0 sampai 5, 6 ga keikut
    println!("Range start {} ", range.start);
    println!("Range end {} ", range.end);


    for i in range {
        println!("Nama = {} ", i);
    }
}

#[test]
fn range_array_exclude() {

    // range exclude
    let range = 0..5;

    let arrayy: [&str; 5] = ["Ambatukam", "Rusman", "Rusdiyansah", "Marlan", "Zuki"];

    for i in range {
        println!("Nama = {} ", arrayy[i]);
    }
}

#[test]
fn range_array_inclusive() {

    let mut angka = 0;
    // range exclude
    let range = 0..=4; // karena ini range inclusive maka 4nya akan muncul beda sama exclude, kalau pakai exclude angka 4 ga masuk

    let arrayy: [&str; 5] = ["Ambatukam", "Rusman", "Rusdiyansah", "Marlan", "Zuki"];

    for i in range {
        println!("Nama = {} range ke {} ", arrayy[i], angka);
        angka += 1;
    }
}

fn test_funcion(){
    let absen: (&str, i32, i8) = ("Ghen", 2006, 16);

    let (tuple1, tuple2, tuple3) = absen;
    println!("Nama {}", tuple1);
    println!("Tahun lahir {}", tuple2);
    println!("Tanggal lahir {}", tuple3);
}

#[test]
fn call_test_function() {
    test_funcion();
}

fn say_hello(first_name: &str, last_name: &str) { // memberikan parameter di functionnya agar nanti ketika memanggil function tinggal memberikan parameter

    println!("Hallo {} dan {}", first_name, last_name);

}

#[test]
fn call_say_hello() {
    say_hello("Ghendida", "Aditya"); // mengisi parameter dari functionnya
    say_hello("Nolan", "Alerick");
}

fn factorial_loop(a: i32) -> i32 { // membuat parameter dengan nilai i32 dan membuat janji akan menghasilkan nilai i32 juga

    if a < 1 {
        return 0;  // jika a kurang dari 1, user memasukkan kurang dari 1 atau negatif
    } // maka hasilnya 0 dan tidak menghasilkann apa ap

    let mut result = 1; // membuat variable hasil yaitu 1
    for i in 1..=a{ // variable i dari 1 sampai a,misal memasukkan angka 5 di parameter artinya 1 sampai 5
       result *= i; // akan menghasilkan angka result dikali dengan i
   }
    result // Implicit Return atau Expression misal memasukkan factorial_loop(3) maka akan menghasilkan atau mengeluarkan 6 dari 1x2x3 (berdasarkan pemrograman)
}

#[test]
fn call_factorial_loop() {
    // factorial_loop(6); // ini tidak mencetak apa apa

    let hasil = factorial_loop(3); // cara memanggilnya
    println!("hasil {}", hasil); // membuat println! untuk memanggil function dan mencetak hasil dari function
}

fn factorial_recursive(a: i32) -> i32 { // contoh faktorial menggunakan recursive function
    if a <= 1{ // jika a atau isi parameternya kurang dari sama dengan 1
        return 1; // hasilnya 1
    }
    a * factorial_recursive(a-1)
    // Memanggil dirinya sendiri dengan parameter yang dikurangi 1 (a - 1).
    // Komputer akan menumpuk perkaliannya secara bertahap:
    // Misal parameter 3, alurnya: 3 * (2 * (1)) sehingga menghasilkan 6.
}

#[test]
fn call_factorial_recursive() {
    let hasil = factorial_recursive(3);
    println!("hasil {}", hasil);
}

fn function_recursive(name: String, times: u32){ //
    if times == 0{
        return;
    } else {
        println!("{}", name);
    }
    function_recursive(name, times - 1);

}

#[test]
fn call_function_recursive() {
    function_recursive(String::from("Baskoro"), 3);
}

fn number_function(number: i16){
    println!("umur = {}", number);

}

fn name(name: String) -> String{ // funcntion ini berjanji menghasilkan String

    format!("nama {}", name) // format! mirip println! bedanya kalau format tidak langsung memunculkannya ia menyimpan terlebih dahulu menjadi variable
}

#[test]
fn show_name_number() {
    let number = 16; // 16 dicopy karena fix size
    number_function(number); // ini sama aja dengan number_function(10) karena number disimpan di stack, di stack tidak ada pindah ownership yang ada hanya copy datanya

    let nama = String::from("Rusdi"); // Rusdi disimpan di heap karena dia String dan Rusdi milik nama
    name(nama); // nama sekarang berpindah kepikimilkan menjadi name jadi nama sudah tidak bisa dipanggil lagi karena sudah milik name
    // println!("nama {}", nama);  ini gabisa karena nama sudah menjadi milik name
    println!("{}", name(String::from("Amba"))); // ini bisa dan tidak perlu menggunakan {:?} karena tadi di function name pakai format!
}

fn full_name(first_name: String, last_name: String)-> String {

    format!("{} {}", first_name, last_name)
}

#[test]
fn show_full_name() {
    let nama_depan = String::from("Ambasat");
    let nama_belakang = String::from("Zaki");

    // 1. nama_depan dan nama_belakang pindah kepemilikan ke dalam fungsi full_name (lalu hangus).
    // 2. Fungsi full_name mengembalikan data String BARU hasil rakitan, lalu ditangkap oleh variabel 'name'.
    let name = full_name(nama_depan, nama_belakang);
    println!("{} ", name);

    // println!("{}", nama_depan); ini sudah tidak bisa digunakan karena ownership/ kepemilikannya sudah berpindah ke variable name
    // println!("{}", nama_belakang); // ini juga sama kayak nama_depan
}

fn full_name_return_function(first_name: String, last_name: String) -> (String, String, String) {
    // 1. Variabel 'first_name' dan 'last_name' menerima kepemilikan data dari luar.
    // 2. Teks baru dirakit di Heap dan disimpan di variabel 'full_name'.
    let full_name = format!("{} {}", first_name, last_name);

    // 3. MENGBALIKKAN OWNERSHIP VIA TUPLE:
    // Daripada membiarkan 'first_name' dan 'last_name' mati (drop) di dalam fungsi,
    // kita bungkus mereka kembali ke dalam Tuple bersama dengan 'full_name',
    // lalu kita lemparkan (return) keluar untuk mengembalikan hak miliknya ke fungsi utama.
    (first_name, last_name, full_name)
}

#[test]
fn show_full_name_return_function() {
    // 1. Kita membuat data String "Ezra" dan "Arden" langsung saat memanggil fungsi.
    // 2. Data tersebut dikirim (Move) ke dalam fungsi 'full_name_return_function'.
    // 3. Setelah fungsi selesai memproses, fungsi mengembalikan paket Tuple berisi 3 String.
    // 4. Kita gunakan teknik Destructuring 'let (a, b, c)' untuk menangkap kembali
    //    kepemilikan atas ketiga data String tersebut dari memori.
    let (a, b, c) = full_name_return_function(String::from("Ezra"), String::from("Arden"));

    // Sekarang variabel a, b, dan c sah memegang hak milik masing-masing data String,
    // sehingga ketiganya bisa dicetak dengan aman tanpa error!
    println!("{} ", a); // Mencetak: Ezra
    println!("{} ", b); // Mencetak: Arden
    println!("{} ", c); // Mencetak: Ezra Arden
}

fn full_name_references(first_name: &String, last_name: &String)-> String{
    format!("{} {}", first_name, last_name)
}

#[test]
fn show_full_name_references() {

    let first_name = String::from("Caleum");
    let last_name = String::from("Lucien");

    let name = full_name_references(&first_name, &last_name);

    println!("{}", first_name);
    println!("{} ", last_name);
    println!("{} ", name);

}

fn change_value(value: &mut String){
    value.push_str(" Testing");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Rusdiyansah");
    change_value(&mut value);
    println!("{}", value);
    
}

// 1. REFERENCE (Tipe Data)
// Parameter 'teks' meminta sebuah REFERENCE bertipe data '&String'
fn baca_buku(teks: &String) {
    println!("Membaca: {}", teks);
}

#[test]
fn test_perbedaan() {
    let buku = String::from("Pemrograman Rust");

    // 2. BORROWING (Aksi Peminjaman)
    // Dengan menambahkan '&' di depan variabel 'buku',
    // kita sedang MELAKUKAN BORROWING (tindakan meminjamkan data).
    baca_buku(&buku);
}

fn ubah_buku(teks: &mut String, jumlah_tetap: &mut i32){
    teks.push_str("Edisi terbaru");
    *jumlah_tetap += 10; // untuk tipe data number yang mutable references, wajib menggunakan *nama_variable
}

#[test]
fn test_ubah_buku() {

    let mut buku = String::from("Pemrograman Rust ");
    let mut jumlah_sementara = 15;

    ubah_buku(&mut buku, &mut jumlah_sementara);
    println!("{} dan jumlahnya digabung jumlah sementara dan jumlah tetap adalah {}", buku, jumlah_sementara);

}
#[test]
fn slice_references() {
    let angka: [i16; 6] = [1, 2, 3, 4, 5, 6];

    let slice1: &[i16] = &angka[..];
    println!("slice1: {:?}", slice1);

    let slice2: &[i16] = &angka[0..6];
    println!("slice2: {:?}", slice2);

    let slice3: &[i16] = &angka[2..];
    println!("slice3: {:?}", slice3);

    let slice4: &[i16] = &angka[..5];
    println!("slice4: {:?}", slice4);

}

#[test]
fn string_slice_references() {

    let name: String = String::from("Ghen Ayari");
    let first_name: &str = &name[..4];
    println!("{}", first_name);

    let last_name: &str = &name[5..];
    println!("{}", last_name);
}


struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8
}

#[test]
fn struct_person() {

    let person: Person = Person {
        first_name: String::from("Ghendida"),
        last_name: String::from("Ayari"),
        middle_name: String::from("Gantari"),
        age: 20
    };

    println!("{}", person.first_name);
    println!("{}", person.middle_name);
    println!("{}", person.last_name);
    println!("{}", person.age);

}

fn print_person(person: &Person) { // struct bisa  juga ditaruh di parameter sebuah function

    println!("Nama depan = {}", person.first_name);
    println!("Nama Tengah = {}", person.middle_name);
    println!("Nama Belakang = {}", person.last_name);
    println!("Usia = {}", person.age);


}

#[test]
fn show_print_person() {

    let person: Person = Person {
        first_name: String::from("Ghendida"),
        last_name: String::from("Ayari"),
        middle_name: String::from("Gantari"),
        age: 16
    };

    print_person(&person);
}

#[test]
fn struct_init_shorthand() {

    let first_name: String = String::from("Ghendida");
    let last_name: String = String::from("Ayari");
    let age: u8 = 21;

    let person: Person = Person {
        first_name, // bisa menyingkat menjadi seperti ini tapi ownership first_name sekarang berubah kepemilikan menjadi milik person
        middle_name: String::from("Gantari"),
        last_name, // ini juga sama
        age // sama seperti tadi
    };

    // println!("{}", first_name); ini sudah tidak bisa digunakan lagi karena ownership sudah berpindah

    print_person(&person);

    let person2: Person = Person {

        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person};

    print_person(&person2);
    print_person(&person);
}

struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-656.73, 314.431);
    println!("lat = {} ", geo_point.0);
    println!("let = {} ", geo_point.1);

}

struct Nothing;

#[test]
fn test_nothing() {
    let _notihing1 = Nothing;
    let _nothing2 = Nothing{};
}

struct Motor{
    nama: String,
    warna: String,
    no_rangka: u32,
    bensin: u8
}

impl Motor { // membuat method dari motor menggunakan kata kuncni "Impl" dan namanya harus sama dengan nama struct

    /*
    1. Yang BISA dilakukan: bisa membuat lebih dari satu blok impl untuk Struct yang sama (Rust akan menggabungkannya otomatis saat kompilasi).
    2. Yang TIDAK BISA dilakukan:  tidak bisa menaruh variabel/field data baru di dalam blok impl. Ruangan ini khusus untuk fungsi dan konstanta saja.
    */

    fn motor_baru(nama_merek: String, warna_motor: String) -> Motor{ // ini namanya associated function, fungsi ini ibarat pabrik tugasnya hanya mencetak wujud dari struct
        // seperti di sini tugasnya mencetak motor, function ini juga tidak ada &self (membaca), &mut self (membaca mengubah), self (mengambil ownership)
        Motor{
            nama: nama_merek,
            warna: warna_motor,
            bensin: 0,
            no_rangka: 18765,
        }
    }


    fn keluar_pabrik_cek_status(&self) { // function ini menggunakan &self untuk membaca saja
        println!("Motor dengan nomor rangka {} keluar pabrik dan sekarang bensinnya {} ", self.no_rangka, self.bensin);
    }

    fn isi_bensin(&mut self, tambah_bensin: u8){ // fungsi ini mengguanakn &mut self untuk membaca dan mengubah/mengedit
        self.bensin += tambah_bensin; // seperti di sini menambah bensin dengan bensin ditambah variable baru yang akan diisi nantinya
        println!("Motor dengan nomor rangka {} dan warna {} sudah isi bensin menjadi {} liter ", self.no_rangka, self.warna, self.bensin);
    }

    fn hancurkan_motor(self){ // mengambil ownership, kepemilikan sekarang berpindah,semua tentang Motor telah menjad milik hancurkan_motor
        println!("Hancurkan motor menjadi rongsokan {} ", self.nama);
    }

}

#[test]
fn show_motor() {

    let mut motor_saya = Motor::motor_baru(String::from("Yamaha"), String::from("Hitam")); // cara memanggil associated function

    motor_saya.keluar_pabrik_cek_status(); // cara memanggil fucntion lain dengan associatedfuncton.namafunctionain()

    motor_saya.isi_bensin(3);

    motor_saya.hancurkan_motor(); // pemilik dari Motor sekarang sudah dipangil dan sudah berakhir, function lain sudah tidak bisa dipanggill lagi

    // motor_saya.keluar_pabrik_cek_status();  sudah tidak bisa lagi karena hancurkan_motor telah dipanggil/berakhir artinya yang lain sudah tidak bisa dipanggil
}

struct Novel{
    judul: String,
    genre: String,
    halaman: u32
}

impl Novel {
    fn mulai_menulis(judul_naskah: String, genre_naskah: String) -> Novel{
        Novel{
            judul: judul_naskah,
            genre: genre_naskah,
            halaman: 6
        }
    }

    fn cek_progress(&self){
        println!("Naskah {} bergenre {} saat ini memiliki {} halaman awal ", self.judul, self.genre, self.halaman);
    }

    fn tambah_halaman(&mut self, menambah_halaman: u32){
        self.halaman += menambah_halaman;
        println!("Sekarang halaman {} memiliki {} halaman setelah ditambah ", self.judul, self.halaman);
    }

    fn kirim_ke_penerbit(self){
        println!("Naskah dengan judul {} telah dikirim ke penerbit, sudah tidak dimiliki lagi ", self.judul);
    }
}

#[test]
fn show_novel() {
    let mut novel_baru = Novel::mulai_menulis(String::from("Obsession"), String::from("Thriller, Romance"));

    novel_baru.cek_progress();

    novel_baru.tambah_halaman(31);

    novel_baru.kirim_ke_penerbit();
}

#[allow(dead_code)] // agar compiler rust tidak marah marah karena ada isi dari enum yang tidak digunakan
enum Arah{ // enum adalah singkatan dari Enumeration, kalau Struct konsepnya dan kalau enum konsepnya atau
    Atas, // isi dari enum harus diawal dengan capital contoh Atas, tidak bisa jika atas
    Bawah,
    Kiri,
    Kanan
}

#[test]
fn show_enum_arah() {
    // memilih salah satu varian
    let _langkah_pertama = Arah::Atas; // tanda _ agar mengabaikan itu dipakai atau engga
    let _langkah_kedua = Arah::Bawah;
}

enum Aktivitas{
    Logout,

    // untuk pesan obrolan
    KirimPesan(String),

    // untuk semisal String pindah kemana dan i32 untuk kordinatnya
    PindahPosisi(String, i32),

}

// impl untuk enum Aktivitas
impl Aktivitas{
    // Asssociated Function
    fn aktivitas_sekarang(isi_pesan: String) -> Aktivitas{
        Aktivitas::KirimPesan(isi_pesan)
    }

    fn jalankan_aktivitas(&self) {
        // melakukan match pada dirinya sendiri
        match self {
            Aktivitas::Logout => {
                println!("User menekan tombol logout");
            }
            Aktivitas::KirimPesan(teks) => {
                println!("Mengirim pesan rahasia berbunyi {} ", teks);
            }
            Aktivitas::PindahPosisi(arah, kordinat) => {
                println!("Berjalan ke arah {} sejauh {} meter ", arah, kordinat);
            }
        }
    }
}

#[test]
fn show_aktivitas() {

    let status_1 = Aktivitas::Logout;

    let status_2 = Aktivitas::PindahPosisi(String::from("Ke arah timur"), 87567);

    let status_3 = Aktivitas::aktivitas_sekarang(String::from("Aku sayang kamu hehehehe"));

    status_1.jalankan_aktivitas();
    status_2.jalankan_aktivitas();
    status_3.jalankan_aktivitas();

}

enum Pembayaran {
    Tunai,
    Ewalllet(String),
    TransferBank(String, u32)
}

impl Pembayaran {
    fn pilih_transfer(nama_bank: String, jumlah: u32) -> Pembayaran {
        Pembayaran::TransferBank(nama_bank, jumlah)
    }
    fn proses_bayar(&self) {
        match self {
            Pembayaran::Tunai => {
                println!("Harap siapkan uang pas");
            }
            Pembayaran::Ewalllet(nama_ewallet) => {
                println!("Masukkan nama Ewallet {} ", nama_ewallet);
            }
            Pembayaran::TransferBank(nama_bank, jumlah) => {
                println!("nama Bank adalah {} dan nomor rekening {} ", nama_bank, jumlah);
            }
        }
    }
}

#[test]
fn test_pembayaran() {

    let pembayaran_1 = Pembayaran::Tunai;
    let pembayaran_2 = Pembayaran::Ewalllet(String::from("Gopay"));
    let pembayaran_3 = Pembayaran::pilih_transfer(String::from("Mandiri"), 3000000);

    pembayaran_1.proses_bayar();
    pembayaran_2.proses_bayar();
    pembayaran_3.proses_bayar();

}

enum MesinKopi {
    Mati,
    Menyala(u32)
}

impl MesinKopi {

    fn mesin_baru() -> MesinKopi {
        MesinKopi::Mati
    }

    fn cek_status(&self) {
        match self {
            MesinKopi::Mati => {
                println!("Mesin kopi mati")
            }
            MesinKopi::Menyala(stok) => {
                println!("Mesin kopi siap, sisa stok {} gelas ", stok);
            }
        }
    }

    fn isi_kopi(&mut self, tambahan: u32) {
        match self {
            MesinKopi::Mati => {
                *self = MesinKopi::Menyala(tambahan);
                println!("Mesin otomatis dihidupkan dan diisi dengan tambahan {} gekas kopi ", tambahan);
            }
             MesinKopi::Menyala(stok) => {
                 *stok += tambahan;
                 println!("stok ditambah sekarang mesin memiliki stok {} gelas kopi ", stok)
            }
        }
    }
    fn hancurkan_mesin(self) {
        match self {
            MesinKopi::Mati => {
                println!("Mesin kopi mati dan dibuang ke rongsokan")
            }
            MesinKopi::Menyala(stok) => {
                println!("Sayang sekali mesin dihancurkan padahal masih menyala dan memiliki stok {} gelas kopi di dalamnya", stok)
            }
        }

    }
}

#[test]
fn test_mesin_kopi() {

    let mut mesin_kantor = MesinKopi::mesin_baru();

    mesin_kantor.cek_status();
    mesin_kantor.isi_kopi(15);
    mesin_kantor.cek_status();
    mesin_kantor.isi_kopi(13);
    mesin_kantor.cek_status();
    mesin_kantor.hancurkan_mesin();

}

// contoh match sebagai expression
#[test]
fn match_sebagai_expression() {

    let umur = 7; // variable umur bertipe i32

    let kategori = match umur {
        17 => "Baru dapat ktp",
        18 => "Agak gedean dikit",
        19 => "Bolehlah",
        _  => "Umur lainnya" // membuang angka yang lainnya karena i32 bisa mencetak angka sampai ribuan jadi menggunakan _
    };

    println!("umur dia adalah {} dan kategorinya {}", umur, kategori);
    
}

// menggunakan range untuk match
#[test]
fn match_dengan_range() {

    let nilai_ujian = 19;

    match nilai_ujian {
        0..=10 => println!("goblok ini"),

        11..=30 => println!("coba lagi bos"),

        31..50 => println!("ya oke lah"),

        50..=79 => println!("Baik"),

        80 | 90  => println!("Baik"), // bisa juga seperti ini dengan menggunakan | (atau)

        81..=89 => println!("Baik"),

        91..=99 => println!("dikit lagi bos"),

        100 => println!("Luar biasa"),

        _ => println!("Angka tidak tersedia")
    };

    println!("nilai ujian {} ", nilai_ujian);

}

// match untuk membongkar tuple

#[test]
fn match_security() {

    let pengunjung = (true, true); // (bawa id?, bawa tas?)

    match pengunjung {
        (true, false) => {
            println!("Silahkan masuk");
        }
        (true, true) => {
            println!("Silahkan masuk tapi tasnya kami lihat dulu");
        }
        (false, _) => { // tanda _ untuk mengabaikan atau membuang atau tidak peduli kemungkinan lain
            println!("maaf tidak boleh masuk karena tidak membawa id card");
        }
    }
}

// contoh lain match tuple dengan kondisi player game

#[test]
fn match_player() {

    let player = (50, 70); // (nyawa?, peluru)

    match player {
        (0, _) => {
            println!("Nyawa anda telah tidak ada")
        }
        (100, 100) => {
            println!("Kondisi prime siap bertempur")
        }
        (n, 0) => {
            println!("nyawa masih {} tapi pelurumu ga ada bjir ", n)
        }
        (n, p) => {
            println!("Terus berjuang dengan sisa nyawa {} dan sisa peluru {}", n, p)
        }

    }
}

// contoh match untuk membongkar struct

struct Karyawan{
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
        Karyawan {nama, divisi} // menggunakan kurung {} untuk menyimpan variabel
        if divisi == "IT" => {
            println!("Tolong codingkan le {} ", nama)
        }
        Karyawan {nama, ..} => {
            println!("Selamat bekerja {} ", nama)
        }

    }
}

// match guard (menggunakan if di dalam match)

enum Kendaraan {
    Motor(u32),
    Mobil(u32),
    Truk(u32)
}

fn cek_tilang(target: Kendaraan) {
    match target {
        Kendaraan::Mobil(kecepatan)
        if kecepatan > 100 => {
            println!("Kilat!, Mobil melaju dengan kecepatan {}/jam!. Tilang! ", kecepatan)
        }
        Kendaraan::Motor(kecepatan)
        if kecepatan > 70 => {
            println!("Anak Amor!, Tilang, karena membahayakan. berjalan dengan kecepatan {}/jam ", kecepatan)
        }
        Kendaraan::Truk(kecepatan)
        if kecepatan > 60 => {
            println!("Anak Amor!, Tilang, karena membahayakan. berjalan dengan kecepatan {}/jam ", kecepatan)
        }
        _ => {
            println!("Kecepatan aman silahkan jalan")
        }
    }
}

#[test]
fn test_kamera_tol() {

    let kendaraan_1 = Kendaraan::Mobil(110);
    let kendaraan_2 = Kendaraan::Mobil(60);
    let kendaraan_3 = Kendaraan::Motor(75);
    let kendaraan_4 = Kendaraan::Truk(50);

    cek_tilang(kendaraan_1);
    cek_tilang(kendaraan_2);
    cek_tilang(kendaraan_3);
    cek_tilang(kendaraan_4);
    
}