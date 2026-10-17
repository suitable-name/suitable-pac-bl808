#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi_config: SpiConfig,
    spi_int_sts: SpiIntSts,
    spi_bus_busy: SpiBusBusy,
    _reserved3: [u8; 0x04],
    spi_prd_0: SpiPrd0,
    spi_prd_1: SpiPrd1,
    spi_rxd_ignr: SpiRxdIgnr,
    spi_sto_value: SpiStoValue,
    _reserved7: [u8; 0x60],
    spi_fifo_config_0: SpiFifoConfig0,
    spi_fifo_config_1: SpiFifoConfig1,
    spi_fifo_wdata: SpiFifoWdata,
    spi_fifo_rdata: SpiFifoRdata,
    _reserved11: [u8; 0x6c],
    spi_backup_io_en: SpiBackupIoEn,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Configuration Register"]
    #[inline(always)]
    pub const fn spi_config(&self) -> &SpiConfig {
        &self.spi_config
    }
    #[doc = "0x04 - SPI Interrupt Status and Control"]
    #[inline(always)]
    pub const fn spi_int_sts(&self) -> &SpiIntSts {
        &self.spi_int_sts
    }
    #[doc = "0x08 - SPI Bus Busy Status"]
    #[inline(always)]
    pub const fn spi_bus_busy(&self) -> &SpiBusBusy {
        &self.spi_bus_busy
    }
    #[doc = "0x10 - SPI Phase Duration Configuration 0"]
    #[inline(always)]
    pub const fn spi_prd_0(&self) -> &SpiPrd0 {
        &self.spi_prd_0
    }
    #[doc = "0x14 - SPI Phase Duration Configuration 1"]
    #[inline(always)]
    pub const fn spi_prd_1(&self) -> &SpiPrd1 {
        &self.spi_prd_1
    }
    #[doc = "0x18 - SPI RX Data Ignore Configuration"]
    #[inline(always)]
    pub const fn spi_rxd_ignr(&self) -> &SpiRxdIgnr {
        &self.spi_rxd_ignr
    }
    #[doc = "0x1c - SPI Slave Timeout Value"]
    #[inline(always)]
    pub const fn spi_sto_value(&self) -> &SpiStoValue {
        &self.spi_sto_value
    }
    #[doc = "0x80 - SPI FIFO Configuration 0"]
    #[inline(always)]
    pub const fn spi_fifo_config_0(&self) -> &SpiFifoConfig0 {
        &self.spi_fifo_config_0
    }
    #[doc = "0x84 - SPI FIFO Status and Thresholds"]
    #[inline(always)]
    pub const fn spi_fifo_config_1(&self) -> &SpiFifoConfig1 {
        &self.spi_fifo_config_1
    }
    #[doc = "0x88 - SPI TX FIFO Data Register"]
    #[inline(always)]
    pub const fn spi_fifo_wdata(&self) -> &SpiFifoWdata {
        &self.spi_fifo_wdata
    }
    #[doc = "0x8c - SPI RX FIFO Data Register"]
    #[inline(always)]
    pub const fn spi_fifo_rdata(&self) -> &SpiFifoRdata {
        &self.spi_fifo_rdata
    }
    #[doc = "0xfc - SPI Backup IO Enable"]
    #[inline(always)]
    pub const fn spi_backup_io_en(&self) -> &SpiBackupIoEn {
        &self.spi_backup_io_en
    }
}
#[doc = "SPI_CONFIG (rw) register accessor: SPI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_config`] module"]
#[doc(alias = "SPI_CONFIG")]
pub type SpiConfig = crate::Reg<spi_config::SpiConfigSpec>;
#[doc = "SPI Configuration Register"]
pub mod spi_config;
#[doc = "SPI_INT_STS (rw) register accessor: SPI Interrupt Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_int_sts`] module"]
#[doc(alias = "SPI_INT_STS")]
pub type SpiIntSts = crate::Reg<spi_int_sts::SpiIntStsSpec>;
#[doc = "SPI Interrupt Status and Control"]
pub mod spi_int_sts;
#[doc = "SPI_BUS_BUSY (rw) register accessor: SPI Bus Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_bus_busy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_bus_busy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_bus_busy`] module"]
#[doc(alias = "SPI_BUS_BUSY")]
pub type SpiBusBusy = crate::Reg<spi_bus_busy::SpiBusBusySpec>;
#[doc = "SPI Bus Busy Status"]
pub mod spi_bus_busy;
#[doc = "SPI_PRD_0 (rw) register accessor: SPI Phase Duration Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_prd_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_prd_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_prd_0`] module"]
#[doc(alias = "SPI_PRD_0")]
pub type SpiPrd0 = crate::Reg<spi_prd_0::SpiPrd0Spec>;
#[doc = "SPI Phase Duration Configuration 0"]
pub mod spi_prd_0;
#[doc = "SPI_PRD_1 (rw) register accessor: SPI Phase Duration Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_prd_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_prd_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_prd_1`] module"]
#[doc(alias = "SPI_PRD_1")]
pub type SpiPrd1 = crate::Reg<spi_prd_1::SpiPrd1Spec>;
#[doc = "SPI Phase Duration Configuration 1"]
pub mod spi_prd_1;
#[doc = "SPI_RXD_IGNR (rw) register accessor: SPI RX Data Ignore Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_rxd_ignr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_rxd_ignr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_rxd_ignr`] module"]
#[doc(alias = "SPI_RXD_IGNR")]
pub type SpiRxdIgnr = crate::Reg<spi_rxd_ignr::SpiRxdIgnrSpec>;
#[doc = "SPI RX Data Ignore Configuration"]
pub mod spi_rxd_ignr;
#[doc = "SPI_STO_VALUE (rw) register accessor: SPI Slave Timeout Value\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_sto_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_sto_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_sto_value`] module"]
#[doc(alias = "SPI_STO_VALUE")]
pub type SpiStoValue = crate::Reg<spi_sto_value::SpiStoValueSpec>;
#[doc = "SPI Slave Timeout Value"]
pub mod spi_sto_value;
#[doc = "SPI_FIFO_CONFIG_0 (rw) register accessor: SPI FIFO Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fifo_config_0`] module"]
#[doc(alias = "SPI_FIFO_CONFIG_0")]
pub type SpiFifoConfig0 = crate::Reg<spi_fifo_config_0::SpiFifoConfig0Spec>;
#[doc = "SPI FIFO Configuration 0"]
pub mod spi_fifo_config_0;
#[doc = "SPI_FIFO_CONFIG_1 (rw) register accessor: SPI FIFO Status and Thresholds\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fifo_config_1`] module"]
#[doc(alias = "SPI_FIFO_CONFIG_1")]
pub type SpiFifoConfig1 = crate::Reg<spi_fifo_config_1::SpiFifoConfig1Spec>;
#[doc = "SPI FIFO Status and Thresholds"]
pub mod spi_fifo_config_1;
#[doc = "SPI_FIFO_WDATA (rw) register accessor: SPI TX FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fifo_wdata`] module"]
#[doc(alias = "SPI_FIFO_WDATA")]
pub type SpiFifoWdata = crate::Reg<spi_fifo_wdata::SpiFifoWdataSpec>;
#[doc = "SPI TX FIFO Data Register"]
pub mod spi_fifo_wdata;
#[doc = "SPI_FIFO_RDATA (rw) register accessor: SPI RX FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_fifo_rdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_fifo_rdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_fifo_rdata`] module"]
#[doc(alias = "SPI_FIFO_RDATA")]
pub type SpiFifoRdata = crate::Reg<spi_fifo_rdata::SpiFifoRdataSpec>;
#[doc = "SPI RX FIFO Data Register"]
pub mod spi_fifo_rdata;
#[doc = "SPI_BACKUP_IO_EN (rw) register accessor: SPI Backup IO Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_backup_io_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_backup_io_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_backup_io_en`] module"]
#[doc(alias = "SPI_BACKUP_IO_EN")]
pub type SpiBackupIoEn = crate::Reg<spi_backup_io_en::SpiBackupIoEnSpec>;
#[doc = "SPI Backup IO Enable"]
pub mod spi_backup_io_en;
