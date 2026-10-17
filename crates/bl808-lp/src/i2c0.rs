#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_config: I2cConfig,
    i2c_int_sts: I2cIntSts,
    i2c_sub_addr: I2cSubAddr,
    i2c_bus_busy: I2cBusBusy,
    i2c_prd_start: I2cPrdStart,
    i2c_prd_stop: I2cPrdStop,
    i2c_prd_data: I2cPrdData,
    _reserved7: [u8; 0x64],
    i2c_fifo_config_0: I2cFifoConfig0,
    i2c_fifo_config_1: I2cFifoConfig1,
    i2c_fifo_wdata: I2cFifoWdata,
    i2c_fifo_rdata: I2cFifoRdata,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C Configuration Register"]
    #[inline(always)]
    pub const fn i2c_config(&self) -> &I2cConfig {
        &self.i2c_config
    }
    #[doc = "0x04 - Interrupt Status, Mask, and Control"]
    #[inline(always)]
    pub const fn i2c_int_sts(&self) -> &I2cIntSts {
        &self.i2c_int_sts
    }
    #[doc = "0x08 - I2C Sub-address Configuration"]
    #[inline(always)]
    pub const fn i2c_sub_addr(&self) -> &I2cSubAddr {
        &self.i2c_sub_addr
    }
    #[doc = "0x0c - I2C Bus Status"]
    #[inline(always)]
    pub const fn i2c_bus_busy(&self) -> &I2cBusBusy {
        &self.i2c_bus_busy
    }
    #[doc = "0x10 - Start Condition Timing Phases"]
    #[inline(always)]
    pub const fn i2c_prd_start(&self) -> &I2cPrdStart {
        &self.i2c_prd_start
    }
    #[doc = "0x14 - Stop Condition Timing Phases"]
    #[inline(always)]
    pub const fn i2c_prd_stop(&self) -> &I2cPrdStop {
        &self.i2c_prd_stop
    }
    #[doc = "0x18 - Data Phase Timing"]
    #[inline(always)]
    pub const fn i2c_prd_data(&self) -> &I2cPrdData {
        &self.i2c_prd_data
    }
    #[doc = "0x80 - FIFO Configuration 0"]
    #[inline(always)]
    pub const fn i2c_fifo_config_0(&self) -> &I2cFifoConfig0 {
        &self.i2c_fifo_config_0
    }
    #[doc = "0x84 - FIFO Configuration 1"]
    #[inline(always)]
    pub const fn i2c_fifo_config_1(&self) -> &I2cFifoConfig1 {
        &self.i2c_fifo_config_1
    }
    #[doc = "0x88 - FIFO Write Data Port"]
    #[inline(always)]
    pub const fn i2c_fifo_wdata(&self) -> &I2cFifoWdata {
        &self.i2c_fifo_wdata
    }
    #[doc = "0x8c - FIFO Read Data Port"]
    #[inline(always)]
    pub const fn i2c_fifo_rdata(&self) -> &I2cFifoRdata {
        &self.i2c_fifo_rdata
    }
}
#[doc = "I2C_CONFIG (rw) register accessor: I2C Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_config`] module"]
#[doc(alias = "I2C_CONFIG")]
pub type I2cConfig = crate::Reg<i2c_config::I2cConfigSpec>;
#[doc = "I2C Configuration Register"]
pub mod i2c_config;
#[doc = "I2C_INT_STS (rw) register accessor: Interrupt Status, Mask, and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_int_sts`] module"]
#[doc(alias = "I2C_INT_STS")]
pub type I2cIntSts = crate::Reg<i2c_int_sts::I2cIntStsSpec>;
#[doc = "Interrupt Status, Mask, and Control"]
pub mod i2c_int_sts;
#[doc = "I2C_SUB_ADDR (rw) register accessor: I2C Sub-address Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_sub_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_sub_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_sub_addr`] module"]
#[doc(alias = "I2C_SUB_ADDR")]
pub type I2cSubAddr = crate::Reg<i2c_sub_addr::I2cSubAddrSpec>;
#[doc = "I2C Sub-address Configuration"]
pub mod i2c_sub_addr;
#[doc = "I2C_BUS_BUSY (rw) register accessor: I2C Bus Status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_bus_busy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_bus_busy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_bus_busy`] module"]
#[doc(alias = "I2C_BUS_BUSY")]
pub type I2cBusBusy = crate::Reg<i2c_bus_busy::I2cBusBusySpec>;
#[doc = "I2C Bus Status"]
pub mod i2c_bus_busy;
#[doc = "I2C_PRD_START (rw) register accessor: Start Condition Timing Phases\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_prd_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_prd_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_prd_start`] module"]
#[doc(alias = "I2C_PRD_START")]
pub type I2cPrdStart = crate::Reg<i2c_prd_start::I2cPrdStartSpec>;
#[doc = "Start Condition Timing Phases"]
pub mod i2c_prd_start;
#[doc = "I2C_PRD_STOP (rw) register accessor: Stop Condition Timing Phases\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_prd_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_prd_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_prd_stop`] module"]
#[doc(alias = "I2C_PRD_STOP")]
pub type I2cPrdStop = crate::Reg<i2c_prd_stop::I2cPrdStopSpec>;
#[doc = "Stop Condition Timing Phases"]
pub mod i2c_prd_stop;
#[doc = "I2C_PRD_DATA (rw) register accessor: Data Phase Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_prd_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_prd_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_prd_data`] module"]
#[doc(alias = "I2C_PRD_DATA")]
pub type I2cPrdData = crate::Reg<i2c_prd_data::I2cPrdDataSpec>;
#[doc = "Data Phase Timing"]
pub mod i2c_prd_data;
#[doc = "I2C_FIFO_CONFIG_0 (rw) register accessor: FIFO Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_fifo_config_0`] module"]
#[doc(alias = "I2C_FIFO_CONFIG_0")]
pub type I2cFifoConfig0 = crate::Reg<i2c_fifo_config_0::I2cFifoConfig0Spec>;
#[doc = "FIFO Configuration 0"]
pub mod i2c_fifo_config_0;
#[doc = "I2C_FIFO_CONFIG_1 (rw) register accessor: FIFO Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_fifo_config_1`] module"]
#[doc(alias = "I2C_FIFO_CONFIG_1")]
pub type I2cFifoConfig1 = crate::Reg<i2c_fifo_config_1::I2cFifoConfig1Spec>;
#[doc = "FIFO Configuration 1"]
pub mod i2c_fifo_config_1;
#[doc = "I2C_FIFO_WDATA (rw) register accessor: FIFO Write Data Port\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_fifo_wdata`] module"]
#[doc(alias = "I2C_FIFO_WDATA")]
pub type I2cFifoWdata = crate::Reg<i2c_fifo_wdata::I2cFifoWdataSpec>;
#[doc = "FIFO Write Data Port"]
pub mod i2c_fifo_wdata;
#[doc = "I2C_FIFO_RDATA (rw) register accessor: FIFO Read Data Port\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_fifo_rdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_fifo_rdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_fifo_rdata`] module"]
#[doc(alias = "I2C_FIFO_RDATA")]
pub type I2cFifoRdata = crate::Reg<i2c_fifo_rdata::I2cFifoRdataSpec>;
#[doc = "FIFO Read Data Port"]
pub mod i2c_fifo_rdata;
