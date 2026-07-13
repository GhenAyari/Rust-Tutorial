pub trait KirimPesan<T> {
    fn kirim(&self,tujuan: T, pesan: String );
}

pub struct SistemKeamanan;

pub struct Email {
    pub email: String
}

pub struct Sms {
    pub nomor: String
}

impl KirimPesan<Email> for SistemKeamanan {
    fn kirim(&self, tujuan: Email, pesan: String ){
        println!("Mengirim email ke {} dengan pesan {}", tujuan.email, pesan);
    }
}
impl KirimPesan<Sms> for SistemKeamanan {
     fn kirim(&self, tujuan: Sms, pesan: String) {
        print!("Mengirim sms darurat ke nomor {} dengan pesan {}", tujuan.nomor, pesan);
    }
}
