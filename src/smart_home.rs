pub trait BisaMenyala {
    fn hidupkan(&self);
}

pub struct Lampu;
pub struct KipasAngin;

#[allow(dead_code)]
pub struct Meja;

impl BisaMenyala for Lampu {
    fn hidupkan(&self) {
        println!("Lampu menyala terang")
    }
}
impl BisaMenyala for KipasAngin {
    fn hidupkan(&self) {
        println!("Kipas Angin berputar mantap")
    }
}

pub fn tombol_pintar<T: BisaMenyala>(alat: T) {
    alat.hidupkan();
    
}


