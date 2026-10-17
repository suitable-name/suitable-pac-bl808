#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dbi_config: DbiConfig,
    dbi_int_sts: DbiIntSts,
    dbi_bus_busy: DbiBusBusy,
    dbi_pix_cnt: DbiPixCnt,
    dbi_prd: DbiPrd,
    _reserved5: [u8; 0x04],
    dbi_wdata: DbiWdata,
    dbi_rdata: DbiRdata,
    _reserved7: [u8; 0x60],
    dbi_fifo_config_0: DbiFifoConfig0,
    dbi_fifo_config_1: DbiFifoConfig1,
    dbi_fifo_wdata: DbiFifoWdata,
}
impl RegisterBlock {
    #[doc = "0x00 - DBI Configuration Register"]
    #[inline(always)]
    pub const fn dbi_config(&self) -> &DbiConfig {
        &self.dbi_config
    }
    #[doc = "0x04 - DBI Interrupt Status and Control"]
    #[inline(always)]
    pub const fn dbi_int_sts(&self) -> &DbiIntSts {
        &self.dbi_int_sts
    }
    #[doc = "0x08 - DBI Bus Busy Status"]
    #[inline(always)]
    pub const fn dbi_bus_busy(&self) -> &DbiBusBusy {
        &self.dbi_bus_busy
    }
    #[doc = "0x0c - DBI Pixel Count and Format"]
    #[inline(always)]
    pub const fn dbi_pix_cnt(&self) -> &DbiPixCnt {
        &self.dbi_pix_cnt
    }
    #[doc = "0x10 - DBI Timing Period Configuration"]
    #[inline(always)]
    pub const fn dbi_prd(&self) -> &DbiPrd {
        &self.dbi_prd
    }
    #[doc = "0x18 - DBI Write Data Register (Manual/FIFO Bypass)"]
    #[inline(always)]
    pub const fn dbi_wdata(&self) -> &DbiWdata {
        &self.dbi_wdata
    }
    #[doc = "0x1c - DBI Read Data Register"]
    #[inline(always)]
    pub const fn dbi_rdata(&self) -> &DbiRdata {
        &self.dbi_rdata
    }
    #[doc = "0x80 - DBI FIFO Configuration 0"]
    #[inline(always)]
    pub const fn dbi_fifo_config_0(&self) -> &DbiFifoConfig0 {
        &self.dbi_fifo_config_0
    }
    #[doc = "0x84 - DBI FIFO Status and Thresholds"]
    #[inline(always)]
    pub const fn dbi_fifo_config_1(&self) -> &DbiFifoConfig1 {
        &self.dbi_fifo_config_1
    }
    #[doc = "0x88 - DBI FIFO Write Data Register"]
    #[inline(always)]
    pub const fn dbi_fifo_wdata(&self) -> &DbiFifoWdata {
        &self.dbi_fifo_wdata
    }
}
#[doc = "DBI_CONFIG (rw) register accessor: DBI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_config`] module"]
#[doc(alias = "DBI_CONFIG")]
pub type DbiConfig = crate::Reg<dbi_config::DbiConfigSpec>;
#[doc = "DBI Configuration Register"]
pub mod dbi_config;
#[doc = "DBI_INT_STS (rw) register accessor: DBI Interrupt Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_int_sts`] module"]
#[doc(alias = "DBI_INT_STS")]
pub type DbiIntSts = crate::Reg<dbi_int_sts::DbiIntStsSpec>;
#[doc = "DBI Interrupt Status and Control"]
pub mod dbi_int_sts;
#[doc = "DBI_BUS_BUSY (r) register accessor: DBI Bus Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_bus_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_bus_busy`] module"]
#[doc(alias = "DBI_BUS_BUSY")]
pub type DbiBusBusy = crate::Reg<dbi_bus_busy::DbiBusBusySpec>;
#[doc = "DBI Bus Busy Status"]
pub mod dbi_bus_busy;
#[doc = "DBI_PIX_CNT (rw) register accessor: DBI Pixel Count and Format\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_pix_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_pix_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_pix_cnt`] module"]
#[doc(alias = "DBI_PIX_CNT")]
pub type DbiPixCnt = crate::Reg<dbi_pix_cnt::DbiPixCntSpec>;
#[doc = "DBI Pixel Count and Format"]
pub mod dbi_pix_cnt;
#[doc = "DBI_PRD (rw) register accessor: DBI Timing Period Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_prd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_prd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_prd`] module"]
#[doc(alias = "DBI_PRD")]
pub type DbiPrd = crate::Reg<dbi_prd::DbiPrdSpec>;
#[doc = "DBI Timing Period Configuration"]
pub mod dbi_prd;
#[doc = "DBI_WDATA (rw) register accessor: DBI Write Data Register (Manual/FIFO Bypass)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_wdata`] module"]
#[doc(alias = "DBI_WDATA")]
pub type DbiWdata = crate::Reg<dbi_wdata::DbiWdataSpec>;
#[doc = "DBI Write Data Register (Manual/FIFO Bypass)"]
pub mod dbi_wdata;
#[doc = "DBI_RDATA (r) register accessor: DBI Read Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_rdata`] module"]
#[doc(alias = "DBI_RDATA")]
pub type DbiRdata = crate::Reg<dbi_rdata::DbiRdataSpec>;
#[doc = "DBI Read Data Register"]
pub mod dbi_rdata;
#[doc = "DBI_FIFO_CONFIG_0 (rw) register accessor: DBI FIFO Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_fifo_config_0`] module"]
#[doc(alias = "DBI_FIFO_CONFIG_0")]
pub type DbiFifoConfig0 = crate::Reg<dbi_fifo_config_0::DbiFifoConfig0Spec>;
#[doc = "DBI FIFO Configuration 0"]
pub mod dbi_fifo_config_0;
#[doc = "DBI_FIFO_CONFIG_1 (rw) register accessor: DBI FIFO Status and Thresholds\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_fifo_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_fifo_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_fifo_config_1`] module"]
#[doc(alias = "DBI_FIFO_CONFIG_1")]
pub type DbiFifoConfig1 = crate::Reg<dbi_fifo_config_1::DbiFifoConfig1Spec>;
#[doc = "DBI FIFO Status and Thresholds"]
pub mod dbi_fifo_config_1;
#[doc = "DBI_FIFO_WDATA (w) register accessor: DBI FIFO Write Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_fifo_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbi_fifo_wdata`] module"]
#[doc(alias = "DBI_FIFO_WDATA")]
pub type DbiFifoWdata = crate::Reg<dbi_fifo_wdata::DbiFifoWdataSpec>;
#[doc = "DBI FIFO Write Data Register"]
pub mod dbi_fifo_wdata;
