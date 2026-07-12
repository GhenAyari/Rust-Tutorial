pub trait PekerjaanSambilan {
    fn kerjakan_tugas(&self);

}

pub struct TukangSapu {
    pub nama: String,
    pub area: String
}

pub struct PenjagaMalam {
    pub nama: String,
    pub durasi_jam: i32
}

impl PekerjaanSambilan for TukangSapu {
    fn kerjakan_tugas(&self){
        println!("Pekerja {} telah menyelesaikan tugasnya di area {}", self.nama, self.area)
    }
}

impl PekerjaanSambilan for PenjagaMalam {
    fn kerjakan_tugas(&self) {
        println!("Pekerja {} telah menyelesaikan tugasnya berpatroli di area selama {} jam", self.nama, self.durasi_jam)
    }
}

pub fn lapor_ke_guild(lapor: &impl PekerjaanSambilan) {
    println!("Ketua Guild menerima laporan pekerjaan");
    lapor.kerjakan_tugas();
}