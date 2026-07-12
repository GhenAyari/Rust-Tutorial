pub trait KendaraanTrait {
    fn gas_pol(&self);
}

pub struct MotorSupra {
    pub pemilik: String,
    pub kecepatan: i32
}

impl KendaraanTrait for MotorSupra {
    fn gas_pol(&self) {
        println!("gaspolll motor supra milik {} melaju dengan kecepatan {} km/jam", self.pemilik, self.kecepatan);
    }
}

pub fn dapat_hadiah_giveaway() -> impl KendaraanTrait {
    let hadiah = MotorSupra {
        pemilik: String::from("Ambarukmo "),
        kecepatan: 30
    };
    hadiah
}


