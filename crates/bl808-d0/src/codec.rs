#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    codec_bus_ctrl: CodecBusCtrl,
    codec_qos_ctrl: CodecQosCtrl,
    codec_bus_thre: CodecBusThre,
    _reserved3: [u8; 0x04],
    codec_bus_dec_err: CodecBusDecErr,
    codec_bus_dec_err_addr: CodecBusDecErrAddr,
    _reserved5: [u8; 0x08],
    blai_lmtr_rd: BlaiLmtrRd,
    blai_lmtr_wr: BlaiLmtrWr,
    id_selection: IdSelection,
    _reserved8: [u8; 0xd0],
    codec_misc_dummy: CodecMiscDummy,
}
impl RegisterBlock {
    #[doc = "0x00 - Codec Bus Control Register"]
    #[inline(always)]
    pub const fn codec_bus_ctrl(&self) -> &CodecBusCtrl {
        &self.codec_bus_ctrl
    }
    #[doc = "0x04 - Codec Quality of Service Control Register"]
    #[inline(always)]
    pub const fn codec_qos_ctrl(&self) -> &CodecQosCtrl {
        &self.codec_qos_ctrl
    }
    #[doc = "0x08 - Codec Bus Thresholds"]
    #[inline(always)]
    pub const fn codec_bus_thre(&self) -> &CodecBusThre {
        &self.codec_bus_thre
    }
    #[doc = "0x10 - Codec Bus Decode Error Status"]
    #[inline(always)]
    pub const fn codec_bus_dec_err(&self) -> &CodecBusDecErr {
        &self.codec_bus_dec_err
    }
    #[doc = "0x14 - Codec Bus Decode Error Address"]
    #[inline(always)]
    pub const fn codec_bus_dec_err_addr(&self) -> &CodecBusDecErrAddr {
        &self.codec_bus_dec_err_addr
    }
    #[doc = "0x20 - NPU (BLAI) Read Command Limiter"]
    #[inline(always)]
    pub const fn blai_lmtr_rd(&self) -> &BlaiLmtrRd {
        &self.blai_lmtr_rd
    }
    #[doc = "0x24 - NPU (BLAI) Write Command Limiter"]
    #[inline(always)]
    pub const fn blai_lmtr_wr(&self) -> &BlaiLmtrWr {
        &self.blai_lmtr_wr
    }
    #[doc = "0x28 - ID Selection Register"]
    #[inline(always)]
    pub const fn id_selection(&self) -> &IdSelection {
        &self.id_selection
    }
    #[doc = "0xfc - Dummy Register"]
    #[inline(always)]
    pub const fn codec_misc_dummy(&self) -> &CodecMiscDummy {
        &self.codec_misc_dummy
    }
}
#[doc = "CODEC_BUS_CTRL (rw) register accessor: Codec Bus Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_bus_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_bus_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_bus_ctrl`] module"]
#[doc(alias = "CODEC_BUS_CTRL")]
pub type CodecBusCtrl = crate::Reg<codec_bus_ctrl::CodecBusCtrlSpec>;
#[doc = "Codec Bus Control Register"]
pub mod codec_bus_ctrl;
#[doc = "CODEC_QOS_CTRL (rw) register accessor: Codec Quality of Service Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_qos_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_qos_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_qos_ctrl`] module"]
#[doc(alias = "CODEC_QOS_CTRL")]
pub type CodecQosCtrl = crate::Reg<codec_qos_ctrl::CodecQosCtrlSpec>;
#[doc = "Codec Quality of Service Control Register"]
pub mod codec_qos_ctrl;
#[doc = "CODEC_BUS_THRE (rw) register accessor: Codec Bus Thresholds\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_bus_thre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_bus_thre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_bus_thre`] module"]
#[doc(alias = "CODEC_BUS_THRE")]
pub type CodecBusThre = crate::Reg<codec_bus_thre::CodecBusThreSpec>;
#[doc = "Codec Bus Thresholds"]
pub mod codec_bus_thre;
#[doc = "CODEC_BUS_DEC_ERR (rw) register accessor: Codec Bus Decode Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_bus_dec_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_bus_dec_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_bus_dec_err`] module"]
#[doc(alias = "CODEC_BUS_DEC_ERR")]
pub type CodecBusDecErr = crate::Reg<codec_bus_dec_err::CodecBusDecErrSpec>;
#[doc = "Codec Bus Decode Error Status"]
pub mod codec_bus_dec_err;
#[doc = "CODEC_BUS_DEC_ERR_ADDR (r) register accessor: Codec Bus Decode Error Address\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_bus_dec_err_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_bus_dec_err_addr`] module"]
#[doc(alias = "CODEC_BUS_DEC_ERR_ADDR")]
pub type CodecBusDecErrAddr = crate::Reg<codec_bus_dec_err_addr::CodecBusDecErrAddrSpec>;
#[doc = "Codec Bus Decode Error Address"]
pub mod codec_bus_dec_err_addr;
#[doc = "BLAI_LMTR_RD (rw) register accessor: NPU (BLAI) Read Command Limiter\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_lmtr_rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_lmtr_rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_lmtr_rd`] module"]
#[doc(alias = "BLAI_LMTR_RD")]
pub type BlaiLmtrRd = crate::Reg<blai_lmtr_rd::BlaiLmtrRdSpec>;
#[doc = "NPU (BLAI) Read Command Limiter"]
pub mod blai_lmtr_rd;
#[doc = "BLAI_LMTR_WR (rw) register accessor: NPU (BLAI) Write Command Limiter\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_lmtr_wr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_lmtr_wr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_lmtr_wr`] module"]
#[doc(alias = "BLAI_LMTR_WR")]
pub type BlaiLmtrWr = crate::Reg<blai_lmtr_wr::BlaiLmtrWrSpec>;
#[doc = "NPU (BLAI) Write Command Limiter"]
pub mod blai_lmtr_wr;
#[doc = "ID_SELECTION (rw) register accessor: ID Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id_selection::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_selection::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id_selection`] module"]
#[doc(alias = "ID_SELECTION")]
pub type IdSelection = crate::Reg<id_selection::IdSelectionSpec>;
#[doc = "ID Selection Register"]
pub mod id_selection;
#[doc = "CODEC_MISC_DUMMY (rw) register accessor: Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_misc_dummy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_misc_dummy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_misc_dummy`] module"]
#[doc(alias = "CODEC_MISC_DUMMY")]
pub type CodecMiscDummy = crate::Reg<codec_misc_dummy::CodecMiscDummySpec>;
#[doc = "Dummy Register"]
pub mod codec_misc_dummy;
