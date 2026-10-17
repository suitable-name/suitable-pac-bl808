#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mipi_config: MipiConfig,
    _reserved1: [u8; 0x0c],
    mipi_int_status: MipiIntStatus,
    mipi_int_mask: MipiIntMask,
    mipi_int_clear: MipiIntClear,
    mipi_int_enable: MipiIntEnable,
    gnr_buf_status: GnrBufStatus,
    gnr_buf_rdata: GnrBufRdata,
    _reserved7: [u8; 0x58],
    dphy_config_0: DphyConfig0,
    dphy_config_1: DphyConfig1,
    dphy_config_2: DphyConfig2,
    dphy_config_3: DphyConfig3,
    dphy_config_4: DphyConfig4,
    dphy_config_5: DphyConfig5,
    _reserved13: [u8; 0x64],
    dummy_reg: DummyReg,
}
impl RegisterBlock {
    #[doc = "0x00 - MIPI CSI Configuration Register"]
    #[inline(always)]
    pub const fn mipi_config(&self) -> &MipiConfig {
        &self.mipi_config
    }
    #[doc = "0x10 - MIPI CSI Interrupt Status"]
    #[inline(always)]
    pub const fn mipi_int_status(&self) -> &MipiIntStatus {
        &self.mipi_int_status
    }
    #[doc = "0x14 - MIPI CSI Interrupt Mask"]
    #[inline(always)]
    pub const fn mipi_int_mask(&self) -> &MipiIntMask {
        &self.mipi_int_mask
    }
    #[doc = "0x18 - MIPI CSI Interrupt Clear"]
    #[inline(always)]
    pub const fn mipi_int_clear(&self) -> &MipiIntClear {
        &self.mipi_int_clear
    }
    #[doc = "0x1c - MIPI CSI Interrupt Enable"]
    #[inline(always)]
    pub const fn mipi_int_enable(&self) -> &MipiIntEnable {
        &self.mipi_int_enable
    }
    #[doc = "0x20 - Generic Buffer Status"]
    #[inline(always)]
    pub const fn gnr_buf_status(&self) -> &GnrBufStatus {
        &self.gnr_buf_status
    }
    #[doc = "0x24 - Generic Buffer Read Data"]
    #[inline(always)]
    pub const fn gnr_buf_rdata(&self) -> &GnrBufRdata {
        &self.gnr_buf_rdata
    }
    #[doc = "0x80 - D-PHY Configuration 0 (Lane/Mode Status)"]
    #[inline(always)]
    pub const fn dphy_config_0(&self) -> &DphyConfig0 {
        &self.dphy_config_0
    }
    #[doc = "0x84 - D-PHY Timing Configuration"]
    #[inline(always)]
    pub const fn dphy_config_1(&self) -> &DphyConfig1 {
        &self.dphy_config_1
    }
    #[doc = "0x88 - D-PHY Analog Configuration and Test"]
    #[inline(always)]
    pub const fn dphy_config_2(&self) -> &DphyConfig2 {
        &self.dphy_config_2
    }
    #[doc = "0x8c - D-PHY Analog Configuration 3"]
    #[inline(always)]
    pub const fn dphy_config_3(&self) -> &DphyConfig3 {
        &self.dphy_config_3
    }
    #[doc = "0x90 - D-PHY Analog Configuration 4"]
    #[inline(always)]
    pub const fn dphy_config_4(&self) -> &DphyConfig4 {
        &self.dphy_config_4
    }
    #[doc = "0x94 - D-PHY Clock Inversion Configuration"]
    #[inline(always)]
    pub const fn dphy_config_5(&self) -> &DphyConfig5 {
        &self.dphy_config_5
    }
    #[doc = "0xfc - Dummy Register (0xFC)"]
    #[inline(always)]
    pub const fn dummy_reg(&self) -> &DummyReg {
        &self.dummy_reg
    }
}
#[doc = "MIPI_CONFIG (rw) register accessor: MIPI CSI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_config`] module"]
#[doc(alias = "MIPI_CONFIG")]
pub type MipiConfig = crate::Reg<mipi_config::MipiConfigSpec>;
#[doc = "MIPI CSI Configuration Register"]
pub mod mipi_config;
#[doc = "MIPI_INT_STATUS (r) register accessor: MIPI CSI Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_int_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_int_status`] module"]
#[doc(alias = "MIPI_INT_STATUS")]
pub type MipiIntStatus = crate::Reg<mipi_int_status::MipiIntStatusSpec>;
#[doc = "MIPI CSI Interrupt Status"]
pub mod mipi_int_status;
#[doc = "MIPI_INT_MASK (rw) register accessor: MIPI CSI Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_int_mask`] module"]
#[doc(alias = "MIPI_INT_MASK")]
pub type MipiIntMask = crate::Reg<mipi_int_mask::MipiIntMaskSpec>;
#[doc = "MIPI CSI Interrupt Mask"]
pub mod mipi_int_mask;
#[doc = "MIPI_INT_CLEAR (w) register accessor: MIPI CSI Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_int_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_int_clear`] module"]
#[doc(alias = "MIPI_INT_CLEAR")]
pub type MipiIntClear = crate::Reg<mipi_int_clear::MipiIntClearSpec>;
#[doc = "MIPI CSI Interrupt Clear"]
pub mod mipi_int_clear;
#[doc = "MIPI_INT_ENABLE (rw) register accessor: MIPI CSI Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_int_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_int_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_int_enable`] module"]
#[doc(alias = "MIPI_INT_ENABLE")]
pub type MipiIntEnable = crate::Reg<mipi_int_enable::MipiIntEnableSpec>;
#[doc = "MIPI CSI Interrupt Enable"]
pub mod mipi_int_enable;
#[doc = "GNR_BUF_STATUS (r) register accessor: Generic Buffer Status\n\nYou can [`read`](crate::Reg::read) this register and get [`gnr_buf_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnr_buf_status`] module"]
#[doc(alias = "GNR_BUF_STATUS")]
pub type GnrBufStatus = crate::Reg<gnr_buf_status::GnrBufStatusSpec>;
#[doc = "Generic Buffer Status"]
pub mod gnr_buf_status;
#[doc = "GNR_BUF_RDATA (r) register accessor: Generic Buffer Read Data\n\nYou can [`read`](crate::Reg::read) this register and get [`gnr_buf_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnr_buf_rdata`] module"]
#[doc(alias = "GNR_BUF_RDATA")]
pub type GnrBufRdata = crate::Reg<gnr_buf_rdata::GnrBufRdataSpec>;
#[doc = "Generic Buffer Read Data"]
pub mod gnr_buf_rdata;
#[doc = "DPHY_CONFIG_0 (rw) register accessor: D-PHY Configuration 0 (Lane/Mode Status)\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_0`] module"]
#[doc(alias = "DPHY_CONFIG_0")]
pub type DphyConfig0 = crate::Reg<dphy_config_0::DphyConfig0Spec>;
#[doc = "D-PHY Configuration 0 (Lane/Mode Status)"]
pub mod dphy_config_0;
#[doc = "DPHY_CONFIG_1 (rw) register accessor: D-PHY Timing Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_1`] module"]
#[doc(alias = "DPHY_CONFIG_1")]
pub type DphyConfig1 = crate::Reg<dphy_config_1::DphyConfig1Spec>;
#[doc = "D-PHY Timing Configuration"]
pub mod dphy_config_1;
#[doc = "DPHY_CONFIG_2 (rw) register accessor: D-PHY Analog Configuration and Test\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_2`] module"]
#[doc(alias = "DPHY_CONFIG_2")]
pub type DphyConfig2 = crate::Reg<dphy_config_2::DphyConfig2Spec>;
#[doc = "D-PHY Analog Configuration and Test"]
pub mod dphy_config_2;
#[doc = "DPHY_CONFIG_3 (rw) register accessor: D-PHY Analog Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_3`] module"]
#[doc(alias = "DPHY_CONFIG_3")]
pub type DphyConfig3 = crate::Reg<dphy_config_3::DphyConfig3Spec>;
#[doc = "D-PHY Analog Configuration 3"]
pub mod dphy_config_3;
#[doc = "DPHY_CONFIG_4 (rw) register accessor: D-PHY Analog Configuration 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_4`] module"]
#[doc(alias = "DPHY_CONFIG_4")]
pub type DphyConfig4 = crate::Reg<dphy_config_4::DphyConfig4Spec>;
#[doc = "D-PHY Analog Configuration 4"]
pub mod dphy_config_4;
#[doc = "DPHY_CONFIG_5 (rw) register accessor: D-PHY Clock Inversion Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_config_5`] module"]
#[doc(alias = "DPHY_CONFIG_5")]
pub type DphyConfig5 = crate::Reg<dphy_config_5::DphyConfig5Spec>;
#[doc = "D-PHY Clock Inversion Configuration"]
pub mod dphy_config_5;
#[doc = "DUMMY_REG (rw) register accessor: Dummy Register (0xFC)\n\nYou can [`read`](crate::Reg::read) this register and get [`dummy_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dummy_reg`] module"]
#[doc(alias = "DUMMY_REG")]
pub type DummyReg = crate::Reg<dummy_reg::DummyRegSpec>;
#[doc = "Dummy Register (0xFC)"]
pub mod dummy_reg;
