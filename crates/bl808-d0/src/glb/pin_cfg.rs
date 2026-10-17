#[repr(C)]
#[doc = "Configuration for GPIO pin %s"]
#[doc(alias = "PIN_CFG")]
pub struct PinCfg {
    reg: Reg,
}
impl PinCfg {
    #[doc = "0x00 - GPIO Configuration Register"]
    #[inline(always)]
    pub const fn reg(&self) -> &Reg {
        &self.reg
    }
}
#[doc = "REG (rw) register accessor: GPIO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg`] module"]
#[doc(alias = "REG")]
pub type Reg = crate::Reg<reg::RegSpec>;
#[doc = "GPIO Configuration Register"]
pub mod reg;
