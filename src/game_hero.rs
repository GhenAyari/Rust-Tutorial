pub trait SkillBertarung {
    fn serang(&self){
    }
    fn lari(&self){
        println!("Berlari menjauh dari medan pertempuran")
    }
}

pub struct Ksatria {
    pub nama: String,
    pub pedang: String,
}

pub struct Magic {
    pub nama: String,
    pub jumlah_mana: i32
}

impl SkillBertarung for Ksatria {
    fn serang(&self){
        println!("Seorang ksatria bernama {} Menebas penyihir dengan {} ", self.nama, self.pedang);
    }
}

impl SkillBertarung for Magic {
    fn serang(&self){
        println!("Penyihir {} telah kehabisan mana setelah bertarung dengan ksatria, mananya sekarang {} ",self.nama, self.jumlah_mana);
    }
    fn lari(&self){
        println!("Lari ada bala bantuan dari pasukan Ambacong")
    }
}