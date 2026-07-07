> 🌐 **Bahasa Indonesia** | [English](README.md)

---

# Ini adalah repositori saya untuk belajar pemrograman Rust, ditulis pada 29 Juli 2026, oleh GhenAyari.

--- 
## Cara menulis "Hello world di Rust"
<br>

```rust
fn main(){
    println!("Hello, world!");
}
```

--- 

## Pengantar singkat tentang cargo di rust

Cargo adalah package manager bawaan dan build system di Rust. contoh penggunaan cargo di bawah ini:

1. untuk membuat proyek baru di rust, kita bisa menulis

```
cargo akan membuat struktur proyek seperti di bawah ini

belajar_rust/


├── Cargo.toml


└── src/


└── main.rs
```

2. bisa menjalankan program seperti yang ditunjukkan di bawah ini

```
cargo test
```

3. Menjalankan pengujian (test)

```
cargo test
```

4. jika ingin mengukur performa atau merilis sebuah aplikasi

```
cargo build --release
```

atau

```
cargo run --release
```

Menurut saya, kebanyakan programmer rust menghabiskan 90% waktu mereka menggunakan cargo run, lalu beralih ke
cargo build --release setelah aplikasi siap untuk pengembangan
atau pengujian performa

---

## Unit test
Di Rust, satu proyek hanya bisa menggunakan satu fungsi main. saya akan menggunakan metode alternatif yaitu "unit test"
unit test adalah kode yang secara khusus didedikasikan untuk pengujian.

```
#[test]
fn testing(){
    println!("nama saya ghen dan saat ini saya sedang belajar rust");
}
```

ini adalah outputnya, 
![img_10.png](img_10.png)

kita bisa menjalankannya dengan "cargo test nama_fungsi_test -- --exact" atau bisa juga "cargo test nama_fungsi_test -- --nocapture" <br>
tapi, langkah pertama hanya akan menjalankan semua unit test dan tidak akan menampilkan outputnya. jadi saya sering menggunakan langkah kedua

---

## Variabel
Variabel digunakan untuk menyimpan nilai data, untuk membuat atau mendeklarasikan variabel di rust, kita bisa menggunakan kata kunci "let".
contoh penggunaannya ditunjukkan di bawah ini:

```
#[test]
fn variable(){
    let my_name = "Ghendida";
    println!("Hallo {} ", my_name);
}
```

dan ouputnya

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

Di Rust, kita tidak bisa mengubah variabel yang sudah diberi nilai, yang biasanya disebut immutable (tidak dapat diubah). Namun,
Rust memungkinkan kita untuk membuat variabel yang bisa diubah, yang dikenal sebagai mutable (dapat diubah), dan kata kuncinya adalah let mut.

```
#[test]
fn variable_mutable(){
    let mut age_in_2025: i8 = 18;
    println!("umur saya di tahun 2025 adalah {} ", age_in_2025);

    age_in_2025 = 19;
    println!("umur saya di tahun 2026 adalah {} ", age_in_2025);
}
```

output

```
PS D:\Rust\basic_rust> cargo test variable_mutable -- --nocapture
Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.49s                                                                                                           
Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
umur saya di tahun 2025 adalah 18
umur saya di tahun 2026 adalah 19
test variable_mutable ... ok
```

---

Rust adalah bahasa dengan tipe data statis (statically typed), artinya setiap kali Anda membuat variabel dengan tipe data tertentu,
tipenya tidak bisa diubah ke tipe lain.
Berbeda dengan JavaScript dan PHP, hal ini tidak dimungkinkan, contohnya, mengubah dari string menjadi integer tidak akan berhasil di Rust

```
#[test]
fn static_type(){
    let mut my_github = "GhenAyari";
    println!("Github saya adalah {}", my_github);

    my_github = 1;
    println!("Github saya adalah {}", my_github);
}
```

dan outputnya akan menjadi

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

Di Rust, kita bisa membuat variabel dengan nama yang sama, tetapi saat kita melakukannya, variabel sebelumnya akan tertutupi,
atau yang disebut sebagai shadowing. praktik ini tidak ideal, tetapi masih diperbolehkan di Rust

```
#[test]
fn shadowing(){
    let name = "Ghendida";
    println!("Hallo {} ", name);

    let name = 10;
    println!("sekarang tanggal {} ", name);

    let name = 2026;
    println!("ini adalah tahun {} ", name);
}
```

dan outputnya akan menjadi

```
PS D:\Rust\basic_rust> cargo test shadowing -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.52s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Hallo Ghendida 
sekarang tanggal 10 
ini adalah tahun 2026 
test shadowing ... ok
```

Seperti yang terlihat di atas, jika kita membuat variabel dengan nama yang sama tetapi memiliki nilai dan tipe data yang berbeda,
variabel sebelumnya akan terkena shadowing dan menjadi tidak dapat diakses lagi

Setiap variabel di Rust memiliki tipe data, yang dikelompokkan menjadi dua jenis: skalar (scalar) dan gabungan (compound). Tipe skalar mewakili sebuah nilai tunggal, contohnya: string, integer, float,
boolean, dan char. Sementara itu, tipe gabungan mewakili lebih dari satu nilai, yaitu tuple dan array

---

Di Rust, saat membuat variabel, tidak perlu menyebutkan tipe datanya secara eksplisit karena Rust akan
secara otomatis mengenali tipe data yang digunakan. Namun, tetap memungkinkan jika Anda ingin menyebutkan tipe data secara eksplisit saat membuat variabel dengan kata kunci titik dua (:)

contoh variabel eksplisit

```
#[test]
fn explicit_variable(){
    let age: i8 = 19;
    println!("Umur saya {} ", age);

    let weight: f32 = 51.5;
    println!("berat badan saya {} ", weight);
}
```

output

```
PS D:\Rust\basic_rust> cargo test explicit_variable -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.48s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Umur saya 19 
berat badan saya 51.5 
test explicit_variable ... ok
```

---

Berikut ini adalah tipe data integer dan float

![img_11.png](img_11.png)

![img_12.png](img_12.png)

Jika Anda membuat variabel secara implisit atau tidak menyebutkan tipe datanya,
Rust akan otomatis memberikan tipe i32 untuk integer dan f64 untuk bilangan desimal

---

Konversi tipe data

Rust bisa melakukan konversi tipe data dari tipe yang lebih kecil ke tipe yang lebih besar, dan sebaliknya. Namun, ada hal yang perlu diingat: mengonversi tipe data yang lebih besar ke yang lebih kecil dapat menyebabkan integer overflow.
Contohnya, mencoba mengubah nilai 100.000 dari tipe i32 ke i8 akan memicu integer overflow


pertama, contoh dari tipe yang lebih kecil ke tipe yang lebih besar

```
#[test]
fn conversion(){
    let a: i8 = 19;
    println!("angka saya {} ", a);

    let b: i16 = a as i16;
    println!("angkanya adalah {} ", b);

    let c : i32 = a as i32;
    println!("angka saya {} ", c); 
}
```

outputnya akan menjadi

```
PS D:\Rust\basic_rust> cargo test conversion -- --nocapture       
Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.51s                                                                                                           
Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
angka saya 19
angkanya adalah 19
angka saya 19
test conversion ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s
```

dan contoh untuk tipe data besar ke kecil

```
#[test]
fn conversion_to_large(){
    let a: i64 = 1000000;
    println!("angka {} ", a);

    let b: i8 = a as i8;
    println!("angka {} ", b);

}
```

outputnya menjadi

```
PS D:\Rust\basic_rust> cargo test conversion_to_large -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.42s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
angka 1000000 
angka 64 
test conversion_to_large ... ok
```

---

## Operator

Operator numerik
di bawah ini adalah contoh penggunaan operator numerik untuk studi kasus rumus luas trapesium

```
#[test]
fn operators_numeric(){

    let height = 3.0;

    let a = 5.0;

    let b = 8.0;

    let l = 0.5;

    let result = l * (a + b) * height;

    println!("hasil = {}, ({} + {}), X {}, = {} ", l, a, b, height, result);

}
```

dan hasilnya adalah

```
PS D:\Rust\basic_rust> cargo test operators_numeric -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.50s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
hasil = 0.5, (5 + 8), X 3, = 19.5 
test operators_numeric ... ok
```

---

### operator perbandingan


Operator perbandingan adalah simbol khusus dalam pemrograman yang digunakan untuk membandingkan dua nilai atau ekspresi guna menentukan hubungan di antara keduanya. Hasil dari operasi perbandingan
selalu berupa nilai boolean—baik itu True atau False—yang biasanya digunakan dalam struktur pengambilan keputusan seperti if statement atau perulangan (loop)

contoh untuk operator perbandingan
```
#[test]
fn comparison_operators(){

    let a = 15 > 10;
    let b = 10 >= 10;
    let c = 15 < 10;
    let d = 10 == 10;

    println!("apakah angka 15 lebih besar dari 10? = {}", a);
    println!("apakah angka 10 lebih besar atau sama dengan 10? = {}", b);
    println!("apakah angka 15 kurang dari 10? = {}", c);
    println!("apakah angka 10 sama dengan 10? = {}", d);

}
```

dan outputnya 

```
PS D:\Rust\basic_rust> cargo test comparison_operators -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.57s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
apakah angka 15 lebih besar dari 10? = true
apakah angka 10 lebih besar atau sama dengan 10? = true
apakah angka 15 kurang dari 10? = false
apakah angka 10 sama dengan 10? = true
test comparison_operators ... ok
```

### operator boolean

Operator boolean adalah operator logika yang digunakan untuk membandingkan nilai atau mengevaluasi ekspresi, menghasilkan nilai akhir berupa benar (true) atau salah (false).
Operator ini berfungsi sebagai dasar pengendalian alur program dan penyaringan informasi dalam berbagai sistem digital

contoh untuk operator boolean
```
#[test]
fn boolean_operators(){

    let age = 20;
    let height = 170;

    let category = 18 <= age;
    let height_check = 165 <= height; // diubah namanya agar tidak tumpang tindih (shadowing) secara membingungkan

    let result = category && height_check;

    println!("apakah dia pria dewasa? {}", result);

}
```

dan outputnya:
```
PS D:\Rust\basic_rust> cargo test boolean_operators -- --nocapture
Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.55s                                                                                                           
Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
apakah dia pria dewasa? true
test boolean_operators ... ok
```

## Tipe data gabungan (Compound data type)

Tuple, sebuah Tuple adalah tipe data yang mengelompokkan sekumpulan tipe data.
Jumlah elemen dalam sebuah tuple bersifat final dan tidak bisa diubah, dikurangi, atau ditambah. untuk membuat tuple, kita bisa menggunakan tanda kurung biasa ()

contoh untuk tuple
```
#[test]
fn tuple(){
    let a: (i32, f64, &str) = (500, 6.4, "Hello");

    println!("Ini adalah tuple = {:?} ", a);

    let tuple1 = a.0;
    let tuple2 = a.1;
    let tuple3 = a.2;

    println!("{}, {}, {} ", tuple1, tuple2, tuple3);

    // atau kita juga bisa melakukan Destructuring tuple
    let (x, y, _) = a; // gunakan _ jika tidak ingin menggunakan salah satunya
    println!("Menggunakan destructuring tuple = {}, {}", x, y);
}
```

dan outputnya

```
dan outputnya:

S D:\Rust\basic_rust> cargo test tuple -- --nocapture
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.52s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Ini adalah tuple = (500, 6.4, "Hello") 
500, 6.4, Hello 
500, 6.4
test tuple ... ok
```

### Tuple yang bisa diubah (Mutable Tuple)

Secara teknis, kita masih bisa memodifikasi isi dari tuple dengan menjadikannya mutable tuple. Anda hanya perlu menambahkan kata kunci mut

contoh untuk mutable tuple
```
#[test]
fn mutable_tuple(){
    let mut about_me: (&str, i8, &str) = ("Ghen", 19, "Universitas Mulawarman");

    let (a, b, c) = about_me;

    println!("{}, {}, {}", a, b, c);

    about_me.0 = "Ghendida";
    about_me.1 = 20;
    about_me.2 = "Dari universitas mulawarman";

    println!("{:?}", about_me);

}
```

dan outputnya

```
PS D:\Rust\basic_rust> cargo test mutable_tuple -- --nocapture 
   Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.48s                                                                                                           
     Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
Ghen, 19, Universitas Mulawarman
("Ghendida", 20, "Dari universitas mulawarman")
test mutable_tuple ... ok
```

### Array

Array adalah tipe data yang menampung sekumpulan data seperti halnya tuple. Perbedaannya adalah dalam
array Anda hanya bisa menggunakan satu tipe data, berbeda dengan tuple yang bisa menggunakan banyak tipe data. Untuk membuat array, gunakan kurung siku []

contoh kodenya di bawah ini:

```
#[test]
fn array(){

    let array_list: [i8; 3] = [10, 20, 30];
    println!("berikut adalah beberapa array = {:?}", array_list);

    let a = array_list[0];
    let b = array_list[1];
    let c = array_list[2];

    println!("{}, {}, {}", a, b, c);


}
```

hasil output di bawah ini:

```
PS D:\Rust\basic_rust> cargo test array -- --nocapture
Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.58s                                                                                                           
Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
berikut adalah beberapa array = [10, 20, 30]
10, 20, 30
test array ... ok
```

### Array yang bisa diubah (Mutable Array)

kita bisa mengubah isi array dengan menggunakan kata kunci "mut".
contoh kodenya di bawah ini

```
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

hasil output di bawah ini

```
PS D:\Rust\basic_rust> cargo test mutable_array -- --nocapture
Compiling basic_rust v0.1.0 (D:\Rust\basic_rust)
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.59s                                                                                                           
Running unittests src\main.rs (target\debug\deps\basic_rust-4923d86b01c67cd4.exe)

running 1 test
["Ramli", "Ruger", "Razi"]
Rizal, Raditya, Roslan
test mutable_array ... ok
```

### array dua dimensi


kita bisa membuat array di dalam array, yang biasa disebut sebagai array dua dimensi

contoh kodenya di bawah ini

```
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