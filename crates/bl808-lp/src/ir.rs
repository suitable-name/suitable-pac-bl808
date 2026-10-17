#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    irtx_config: IrtxConfig,
    irtx_int_sts: IrtxIntSts,
    _reserved2: [u8; 0x08],
    irtx_pulse_width: IrtxPulseWidth,
    irtx_pw_0: IrtxPw0,
    irtx_pw_1: IrtxPw1,
    _reserved5: [u8; 0x24],
    irrx_config: IrrxConfig,
    irrx_int_sts: IrrxIntSts,
    irrx_pw_config: IrrxPwConfig,
    _reserved8: [u8; 0x04],
    irrx_data_count: IrrxDataCount,
    irrx_data_word0: IrrxDataWord0,
    irrx_data_word1: IrrxDataWord1,
    _reserved11: [u8; 0x24],
    irtx_fifo_config_0: IrtxFifoConfig0,
    irtx_fifo_config_1: IrtxFifoConfig1,
    ir_fifo_wdata: IrFifoWdata,
    ir_fifo_rdata: IrFifoRdata,
}
impl RegisterBlock {
    #[doc = "0x00 - IRTX Configuration Register"]
    #[inline(always)]
    pub const fn irtx_config(&self) -> &IrtxConfig {
        &self.irtx_config
    }
    #[doc = "0x04 - IRTX Interrupt Status and Control"]
    #[inline(always)]
    pub const fn irtx_int_sts(&self) -> &IrtxIntSts {
        &self.irtx_int_sts
    }
    #[doc = "0x10 - IRTX Basic Pulse Width and Modulation Configuration"]
    #[inline(always)]
    pub const fn irtx_pulse_width(&self) -> &IrtxPulseWidth {
        &self.irtx_pulse_width
    }
    #[doc = "0x14 - IRTX Logic 0/1 Pulse Width Configuration"]
    #[inline(always)]
    pub const fn irtx_pw_0(&self) -> &IrtxPw0 {
        &self.irtx_pw_0
    }
    #[doc = "0x18 - IRTX Head/Tail Pulse Width Configuration"]
    #[inline(always)]
    pub const fn irtx_pw_1(&self) -> &IrtxPw1 {
        &self.irtx_pw_1
    }
    #[doc = "0x40 - IRRX Configuration Register"]
    #[inline(always)]
    pub const fn irrx_config(&self) -> &IrrxConfig {
        &self.irrx_config
    }
    #[doc = "0x44 - IRRX Interrupt Status and Control"]
    #[inline(always)]
    pub const fn irrx_int_sts(&self) -> &IrrxIntSts {
        &self.irrx_int_sts
    }
    #[doc = "0x48 - IRRX Pulse Width Threshold Configuration"]
    #[inline(always)]
    pub const fn irrx_pw_config(&self) -> &IrrxPwConfig {
        &self.irrx_pw_config
    }
    #[doc = "0x50 - IRRX Received Data Bit Count"]
    #[inline(always)]
    pub const fn irrx_data_count(&self) -> &IrrxDataCount {
        &self.irrx_data_count
    }
    #[doc = "0x54 - IRRX Received Data Word 0"]
    #[inline(always)]
    pub const fn irrx_data_word0(&self) -> &IrrxDataWord0 {
        &self.irrx_data_word0
    }
    #[doc = "0x58 - IRRX Received Data Word 1"]
    #[inline(always)]
    pub const fn irrx_data_word1(&self) -> &IrrxDataWord1 {
        &self.irrx_data_word1
    }
    #[doc = "0x80 - IRTX FIFO Configuration and Status 0"]
    #[inline(always)]
    pub const fn irtx_fifo_config_0(&self) -> &IrtxFifoConfig0 {
        &self.irtx_fifo_config_0
    }
    #[doc = "0x84 - IRTX FIFO Status and Thresholds 1"]
    #[inline(always)]
    pub const fn irtx_fifo_config_1(&self) -> &IrtxFifoConfig1 {
        &self.irtx_fifo_config_1
    }
    #[doc = "0x88 - IR TX FIFO Data Register"]
    #[inline(always)]
    pub const fn ir_fifo_wdata(&self) -> &IrFifoWdata {
        &self.ir_fifo_wdata
    }
    #[doc = "0x8c - IR RX FIFO Data Register"]
    #[inline(always)]
    pub const fn ir_fifo_rdata(&self) -> &IrFifoRdata {
        &self.ir_fifo_rdata
    }
}
#[doc = "IRTX_CONFIG (rw) register accessor: IRTX Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_config`] module"]
#[doc(alias = "IRTX_CONFIG")]
pub type IrtxConfig = crate::Reg<irtx_config::IrtxConfigSpec>;
#[doc = "IRTX Configuration Register"]
pub mod irtx_config;
#[doc = "IRTX_INT_STS (rw) register accessor: IRTX Interrupt Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_int_sts`] module"]
#[doc(alias = "IRTX_INT_STS")]
pub type IrtxIntSts = crate::Reg<irtx_int_sts::IrtxIntStsSpec>;
#[doc = "IRTX Interrupt Status and Control"]
pub mod irtx_int_sts;
#[doc = "IRTX_PULSE_WIDTH (rw) register accessor: IRTX Basic Pulse Width and Modulation Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_pulse_width::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_pulse_width::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_pulse_width`] module"]
#[doc(alias = "IRTX_PULSE_WIDTH")]
pub type IrtxPulseWidth = crate::Reg<irtx_pulse_width::IrtxPulseWidthSpec>;
#[doc = "IRTX Basic Pulse Width and Modulation Configuration"]
pub mod irtx_pulse_width;
#[doc = "IRTX_PW_0 (rw) register accessor: IRTX Logic 0/1 Pulse Width Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_pw_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_pw_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_pw_0`] module"]
#[doc(alias = "IRTX_PW_0")]
pub type IrtxPw0 = crate::Reg<irtx_pw_0::IrtxPw0Spec>;
#[doc = "IRTX Logic 0/1 Pulse Width Configuration"]
pub mod irtx_pw_0;
#[doc = "IRTX_PW_1 (rw) register accessor: IRTX Head/Tail Pulse Width Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_pw_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_pw_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_pw_1`] module"]
#[doc(alias = "IRTX_PW_1")]
pub type IrtxPw1 = crate::Reg<irtx_pw_1::IrtxPw1Spec>;
#[doc = "IRTX Head/Tail Pulse Width Configuration"]
pub mod irtx_pw_1;
#[doc = "IRRX_CONFIG (rw) register accessor: IRRX Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_config`] module"]
#[doc(alias = "IRRX_CONFIG")]
pub type IrrxConfig = crate::Reg<irrx_config::IrrxConfigSpec>;
#[doc = "IRRX Configuration Register"]
pub mod irrx_config;
#[doc = "IRRX_INT_STS (rw) register accessor: IRRX Interrupt Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_int_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_int_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_int_sts`] module"]
#[doc(alias = "IRRX_INT_STS")]
pub type IrrxIntSts = crate::Reg<irrx_int_sts::IrrxIntStsSpec>;
#[doc = "IRRX Interrupt Status and Control"]
pub mod irrx_int_sts;
#[doc = "IRRX_PW_CONFIG (rw) register accessor: IRRX Pulse Width Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_pw_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_pw_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_pw_config`] module"]
#[doc(alias = "IRRX_PW_CONFIG")]
pub type IrrxPwConfig = crate::Reg<irrx_pw_config::IrrxPwConfigSpec>;
#[doc = "IRRX Pulse Width Threshold Configuration"]
pub mod irrx_pw_config;
#[doc = "IRRX_DATA_COUNT (r) register accessor: IRRX Received Data Bit Count\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_data_count`] module"]
#[doc(alias = "IRRX_DATA_COUNT")]
pub type IrrxDataCount = crate::Reg<irrx_data_count::IrrxDataCountSpec>;
#[doc = "IRRX Received Data Bit Count"]
pub mod irrx_data_count;
#[doc = "IRRX_DATA_WORD0 (r) register accessor: IRRX Received Data Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_word0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_data_word0`] module"]
#[doc(alias = "IRRX_DATA_WORD0")]
pub type IrrxDataWord0 = crate::Reg<irrx_data_word0::IrrxDataWord0Spec>;
#[doc = "IRRX Received Data Word 0"]
pub mod irrx_data_word0;
#[doc = "IRRX_DATA_WORD1 (r) register accessor: IRRX Received Data Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_data_word1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irrx_data_word1`] module"]
#[doc(alias = "IRRX_DATA_WORD1")]
pub type IrrxDataWord1 = crate::Reg<irrx_data_word1::IrrxDataWord1Spec>;
#[doc = "IRRX Received Data Word 1"]
pub mod irrx_data_word1;
#[doc = "IRTX_FIFO_CONFIG_0 (rw) register accessor: IRTX FIFO Configuration and Status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_fifo_config_0`] module"]
#[doc(alias = "IRTX_FIFO_CONFIG_0")]
pub type IrtxFifoConfig0 = crate::Reg<irtx_fifo_config_0::IrtxFifoConfig0Spec>;
#[doc = "IRTX FIFO Configuration and Status 0"]
pub mod irtx_fifo_config_0;
#[doc = "IRTX_FIFO_CONFIG_1 (rw) register accessor: IRTX FIFO Status and Thresholds 1\n\nYou can [`read`](crate::Reg::read) this register and get [`irtx_fifo_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irtx_fifo_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irtx_fifo_config_1`] module"]
#[doc(alias = "IRTX_FIFO_CONFIG_1")]
pub type IrtxFifoConfig1 = crate::Reg<irtx_fifo_config_1::IrtxFifoConfig1Spec>;
#[doc = "IRTX FIFO Status and Thresholds 1"]
pub mod irtx_fifo_config_1;
#[doc = "IR_FIFO_WDATA (w) register accessor: IR TX FIFO Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir_fifo_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir_fifo_wdata`] module"]
#[doc(alias = "IR_FIFO_WDATA")]
pub type IrFifoWdata = crate::Reg<ir_fifo_wdata::IrFifoWdataSpec>;
#[doc = "IR TX FIFO Data Register"]
pub mod ir_fifo_wdata;
#[doc = "IR_FIFO_RDATA (r) register accessor: IR RX FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir_fifo_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir_fifo_rdata`] module"]
#[doc(alias = "IR_FIFO_RDATA")]
pub type IrFifoRdata = crate::Reg<ir_fifo_rdata::IrFifoRdataSpec>;
#[doc = "IR RX FIFO Data Register"]
pub mod ir_fifo_rdata;
