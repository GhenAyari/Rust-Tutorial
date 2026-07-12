pub trait SihirDasar {
    fn keluarkan_cahaya(&self);
}

pub trait SihirTingkatLanjut: SihirDasar {
    fn panggil_meteor(&self);
}

pub struct PenyihirSakti {
    pub nama: String,
    pub level: i32
}

impl SihirDasar for PenyihirSakti {
    fn keluarkan_cahaya(&self) {
        println!("penyihir {} mengeluarkan bola cahaya dari tongkatnya dan levelnya sekarang {}", self.nama, self.level);
    }
}

impl SihirTingkatLanjut for PenyihirSakti {
    fn panggil_meteor(&self) {
        println!("Penyihir {} berhasil memanggil meteor dan levelnya sekarang {}", self.nama, self.level);
    }
}

pub fn jalankan_ujian_elite(lulus: &impl SihirTingkatLanjut){
    println!("=========Ujian dimulai===========");
    lulus.keluarkan_cahaya();
    lulus.panggil_meteor();
    println!("Penyihir lulus dan kini menjadi penyihir sakti");
}