pub enum KatergoriMalware {
    Ransomware,
    Spyware,
    Aman
}


pub struct LogJaringan {
    pub nama_file: String,
    pub ip_sumber: String,
    status: KatergoriMalware
}

impl LogJaringan {
    pub fn analisis_file(nama: String, ip: String, status: KatergoriMalware) -> LogJaringan {
        LogJaringan {
            nama_file: nama,
            ip_sumber: ip,
            status: status
        }
    }
    pub fn cetak_peringatan(&self){

            match &self.status {
                KatergoriMalware::Ransomware => {
                    println!("Bahaya! {} dari {} terdeteksi sebagai ransomware. Segera blokir jaringan ",self.nama_file, self.ip_sumber);
                }
                KatergoriMalware::Spyware => {
                    println!("awas aktivitas mencurigakan dari file {} dengan ip {} terindikasi spyware ",self.nama_file, self.ip_sumber);
                }
                KatergoriMalware::Aman => {
                    println!("Jaringan aman!, file {} bersih ",self.nama_file);
                }
            }
        }

    }