#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sd_chip_id_low: SdChipIdLow,
    sd_chip_id_high: SdChipIdHigh,
    sd_dbg_pwd_low: SdDbgPwdLow,
    sd_dbg_pwd_high: SdDbgPwdHigh,
    sd_dbg_pwd2_low: SdDbgPwd2Low,
    sd_dbg_pwd2_high: SdDbgPwd2High,
    sd_status: SdStatus,
    sd_dbg_reserved: SdDbgReserved,
}
impl RegisterBlock {
    #[doc = "0x00 - Chip ID Register Low Word"]
    #[inline(always)]
    pub const fn sd_chip_id_low(&self) -> &SdChipIdLow {
        &self.sd_chip_id_low
    }
    #[doc = "0x04 - Chip ID Register High Word"]
    #[inline(always)]
    pub const fn sd_chip_id_high(&self) -> &SdChipIdHigh {
        &self.sd_chip_id_high
    }
    #[doc = "0x08 - Debug Access Password 1 Low Word"]
    #[inline(always)]
    pub const fn sd_dbg_pwd_low(&self) -> &SdDbgPwdLow {
        &self.sd_dbg_pwd_low
    }
    #[doc = "0x0c - Debug Access Password 1 High Word"]
    #[inline(always)]
    pub const fn sd_dbg_pwd_high(&self) -> &SdDbgPwdHigh {
        &self.sd_dbg_pwd_high
    }
    #[doc = "0x10 - Debug Access Password 2 Low Word (Write to this triggers authentication)"]
    #[inline(always)]
    pub const fn sd_dbg_pwd2_low(&self) -> &SdDbgPwd2Low {
        &self.sd_dbg_pwd2_low
    }
    #[doc = "0x14 - Debug Access Password 2 High Word"]
    #[inline(always)]
    pub const fn sd_dbg_pwd2_high(&self) -> &SdDbgPwd2High {
        &self.sd_dbg_pwd2_high
    }
    #[doc = "0x18 - Debug Status and Control Register"]
    #[inline(always)]
    pub const fn sd_status(&self) -> &SdStatus {
        &self.sd_status
    }
    #[doc = "0x1c - Debug Reserved Register"]
    #[inline(always)]
    pub const fn sd_dbg_reserved(&self) -> &SdDbgReserved {
        &self.sd_dbg_reserved
    }
}
#[doc = "SD_CHIP_ID_LOW (r) register accessor: Chip ID Register Low Word\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_chip_id_low::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_chip_id_low`] module"]
#[doc(alias = "SD_CHIP_ID_LOW")]
pub type SdChipIdLow = crate::Reg<sd_chip_id_low::SdChipIdLowSpec>;
#[doc = "Chip ID Register Low Word"]
pub mod sd_chip_id_low;
#[doc = "SD_CHIP_ID_HIGH (r) register accessor: Chip ID Register High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_chip_id_high::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_chip_id_high`] module"]
#[doc(alias = "SD_CHIP_ID_HIGH")]
pub type SdChipIdHigh = crate::Reg<sd_chip_id_high::SdChipIdHighSpec>;
#[doc = "Chip ID Register High Word"]
pub mod sd_chip_id_high;
#[doc = "SD_DBG_PWD_LOW (rw) register accessor: Debug Access Password 1 Low Word\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_pwd_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_pwd_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_dbg_pwd_low`] module"]
#[doc(alias = "SD_DBG_PWD_LOW")]
pub type SdDbgPwdLow = crate::Reg<sd_dbg_pwd_low::SdDbgPwdLowSpec>;
#[doc = "Debug Access Password 1 Low Word"]
pub mod sd_dbg_pwd_low;
#[doc = "SD_DBG_PWD_HIGH (rw) register accessor: Debug Access Password 1 High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_pwd_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_pwd_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_dbg_pwd_high`] module"]
#[doc(alias = "SD_DBG_PWD_HIGH")]
pub type SdDbgPwdHigh = crate::Reg<sd_dbg_pwd_high::SdDbgPwdHighSpec>;
#[doc = "Debug Access Password 1 High Word"]
pub mod sd_dbg_pwd_high;
#[doc = "SD_DBG_PWD2_LOW (rw) register accessor: Debug Access Password 2 Low Word (Write to this triggers authentication)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_pwd2_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_pwd2_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_dbg_pwd2_low`] module"]
#[doc(alias = "SD_DBG_PWD2_LOW")]
pub type SdDbgPwd2Low = crate::Reg<sd_dbg_pwd2_low::SdDbgPwd2LowSpec>;
#[doc = "Debug Access Password 2 Low Word (Write to this triggers authentication)"]
pub mod sd_dbg_pwd2_low;
#[doc = "SD_DBG_PWD2_HIGH (rw) register accessor: Debug Access Password 2 High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_pwd2_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_pwd2_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_dbg_pwd2_high`] module"]
#[doc(alias = "SD_DBG_PWD2_HIGH")]
pub type SdDbgPwd2High = crate::Reg<sd_dbg_pwd2_high::SdDbgPwd2HighSpec>;
#[doc = "Debug Access Password 2 High Word"]
pub mod sd_dbg_pwd2_high;
#[doc = "SD_STATUS (rw) register accessor: Debug Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_status`] module"]
#[doc(alias = "SD_STATUS")]
pub type SdStatus = crate::Reg<sd_status::SdStatusSpec>;
#[doc = "Debug Status and Control Register"]
pub mod sd_status;
#[doc = "SD_DBG_RESERVED (rw) register accessor: Debug Reserved Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sd_dbg_reserved`] module"]
#[doc(alias = "SD_DBG_RESERVED")]
pub type SdDbgReserved = crate::Reg<sd_dbg_reserved::SdDbgReservedSpec>;
#[doc = "Debug Reserved Register"]
pub mod sd_dbg_reserved;
