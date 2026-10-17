#[repr(C)]
#[doc = "Touch Long-Term Average Data Registers"]
#[doc(alias = "LTA_DATA")]
pub struct LtaData {
    reg: Reg,
}
impl LtaData {
    #[doc = "0x00 - Touch LTA Data for a single channel"]
    #[inline(always)]
    pub const fn reg(&self) -> &Reg {
        &self.reg
    }
}
#[doc = "REG (r) register accessor: Touch LTA Data for a single channel\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg`] module"]
#[doc(alias = "REG")]
pub type Reg = crate::Reg<reg::RegSpec>;
#[doc = "Touch LTA Data for a single channel"]
pub mod reg;
