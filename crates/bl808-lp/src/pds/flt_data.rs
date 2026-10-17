#[repr(C)]
#[doc = "Touch Filtered Data Registers"]
#[doc(alias = "FLT_DATA")]
pub struct FltData {
    reg: Reg,
}
impl FltData {
    #[doc = "0x00 - Touch Filtered Data for a single channel"]
    #[inline(always)]
    pub const fn reg(&self) -> &Reg {
        &self.reg
    }
}
#[doc = "REG (r) register accessor: Touch Filtered Data for a single channel\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg`] module"]
#[doc(alias = "REG")]
pub type Reg = crate::Reg<reg::RegSpec>;
#[doc = "Touch Filtered Data for a single channel"]
pub mod reg;
