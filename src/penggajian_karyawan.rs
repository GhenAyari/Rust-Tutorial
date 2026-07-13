use std::ops::Mul;
pub struct GajiHarian {
    pub upah: i32
}

pub struct HariKerja {
    pub hari: i32
}

pub struct GajiTotal{
    pub total: i32
}

impl Mul<HariKerja> for GajiHarian {
    type Output = GajiTotal;

    fn mul(self, rhs: HariKerja) -> GajiTotal {
        GajiTotal {
            total: self.upah * rhs.hari
        }
    }
}