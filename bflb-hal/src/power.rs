//! Internal LDO (Low Drop Out) power output management.


use crate::bl808::GLB;


/// Power domain controller.
pub struct Power(pub(crate) ());

impl Power {

    /// Set the voltage level of VDD15_CIS 
    #[inline]
    pub fn set_vdd15_cis(&mut self, level: Vdd15cis) {
        GLB.ldo15cis().modify(|reg| reg.ldo15cis_vout_sel().set(level as _))
    }
        
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vdd15cis {
    Volt1p00 = 0,
    Volt1p05 = 1,
    Volt1p10 = 2,
    Volt1p15 = 3,
    Volt1p20 = 4,
    Volt1p30 = 5,
    Volt1p40 = 6,
    Volt1p45 = 7,
    Volt1p50 = 8,
    Volt1p55 = 9,
    Volt1p60 = 10,
    Volt1p65 = 11,
    Volt1p70 = 12,
    Volt1p75 = 13,
    Volt1p80 = 14,
    Volt1p85 = 15
}
