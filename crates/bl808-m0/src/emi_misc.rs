#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    emi_bus_ctrl: EmiBusCtrl,
    emi_bus_ctrl2: EmiBusCtrl2,
    emi_bus_thre: EmiBusThre,
    _reserved3: [u8; 0xf0],
    emi_misc_dummy: EmiMiscDummy,
}
impl RegisterBlock {
    #[doc = "0x00 - EMI Bus Control"]
    #[inline(always)]
    pub const fn emi_bus_ctrl(&self) -> &EmiBusCtrl {
        &self.emi_bus_ctrl
    }
    #[doc = "0x04 - EMI Bus Control 2"]
    #[inline(always)]
    pub const fn emi_bus_ctrl2(&self) -> &EmiBusCtrl2 {
        &self.emi_bus_ctrl2
    }
    #[doc = "0x08 - EMI Bus Threshold"]
    #[inline(always)]
    pub const fn emi_bus_thre(&self) -> &EmiBusThre {
        &self.emi_bus_thre
    }
    #[doc = "0xfc - EMI Miscellaneous Dummy Register"]
    #[inline(always)]
    pub const fn emi_misc_dummy(&self) -> &EmiMiscDummy {
        &self.emi_misc_dummy
    }
}
#[doc = "EMI_BUS_CTRL (rw) register accessor: EMI Bus Control\n\nYou can [`read`](crate::Reg::read) this register and get [`emi_bus_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emi_bus_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emi_bus_ctrl`] module"]
#[doc(alias = "EMI_BUS_CTRL")]
pub type EmiBusCtrl = crate::Reg<emi_bus_ctrl::EmiBusCtrlSpec>;
#[doc = "EMI Bus Control"]
pub mod emi_bus_ctrl;
#[doc = "EMI_BUS_CTRL2 (rw) register accessor: EMI Bus Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`emi_bus_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emi_bus_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emi_bus_ctrl2`] module"]
#[doc(alias = "EMI_BUS_CTRL2")]
pub type EmiBusCtrl2 = crate::Reg<emi_bus_ctrl2::EmiBusCtrl2Spec>;
#[doc = "EMI Bus Control 2"]
pub mod emi_bus_ctrl2;
#[doc = "EMI_BUS_THRE (rw) register accessor: EMI Bus Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`emi_bus_thre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emi_bus_thre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emi_bus_thre`] module"]
#[doc(alias = "EMI_BUS_THRE")]
pub type EmiBusThre = crate::Reg<emi_bus_thre::EmiBusThreSpec>;
#[doc = "EMI Bus Threshold"]
pub mod emi_bus_thre;
#[doc = "EMI_MISC_DUMMY (rw) register accessor: EMI Miscellaneous Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`emi_misc_dummy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emi_misc_dummy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emi_misc_dummy`] module"]
#[doc(alias = "EMI_MISC_DUMMY")]
pub type EmiMiscDummy = crate::Reg<emi_misc_dummy::EmiMiscDummySpec>;
#[doc = "EMI Miscellaneous Dummy Register"]
pub mod emi_misc_dummy;
