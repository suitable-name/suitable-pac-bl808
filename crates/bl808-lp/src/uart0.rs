#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    utx_config: UtxConfig,
    urx_config: UrxConfig,
    uart_bit_prd: UartBitPrd,
    data_config: DataConfig,
    utx_ir_position: UtxIrPosition,
    urx_ir_position: UrxIrPosition,
    urx_rto_timer: UrxRtoTimer,
    uart_sw_mode: UartSwMode,
    uart_int_sts: UartIntSts,
    uart_int_mask: UartIntMask,
    uart_int_clear: UartIntClear,
    uart_int_en: UartIntEn,
    uart_status: UartStatus,
    sts_urx_abr_prd: StsUrxAbrPrd,
    urx_abr_prd_b01: UrxAbrPrdB01,
    urx_abr_prd_b23: UrxAbrPrdB23,
    urx_abr_prd_b45: UrxAbrPrdB45,
    urx_abr_prd_b67: UrxAbrPrdB67,
    urx_abr_pw_tol: UrxAbrPwTol,
    _reserved19: [u8; 0x04],
    urx_bcr_int_cfg: UrxBcrIntCfg,
    utx_rs485_cfg: UtxRs485Cfg,
    _reserved21: [u8; 0x28],
    uart_fifo_config_0: UartFifoConfig0,
    uart_fifo_config_1: UartFifoConfig1,
    uart_fifo_wdata: UartFifoWdata,
    uart_fifo_rdata: UartFifoRdata,
}
impl RegisterBlock {
    #[doc = "0x00 - UART TX Configuration"]
    #[inline(always)]
    pub const fn utx_config(&self) -> &UtxConfig {
        &self.utx_config
    }
    #[doc = "0x04 - UART RX Configuration"]
    #[inline(always)]
    pub const fn urx_config(&self) -> &UrxConfig {
        &self.urx_config
    }
    #[doc = "0x08 - UART Baud Rate Bit Period"]
    #[inline(always)]
    pub const fn uart_bit_prd(&self) -> &UartBitPrd {
        &self.uart_bit_prd
    }
    #[doc = "0x0c - UART Data Format Config"]
    #[inline(always)]
    pub const fn data_config(&self) -> &DataConfig {
        &self.data_config
    }
    #[doc = "0x10 - UART TX IR Pulse Position"]
    #[inline(always)]
    pub const fn utx_ir_position(&self) -> &UtxIrPosition {
        &self.utx_ir_position
    }
    #[doc = "0x14 - UART RX IR Pulse Position"]
    #[inline(always)]
    pub const fn urx_ir_position(&self) -> &UrxIrPosition {
        &self.urx_ir_position
    }
    #[doc = "0x18 - UART RX Timeout Timer"]
    #[inline(always)]
    pub const fn urx_rto_timer(&self) -> &UrxRtoTimer {
        &self.urx_rto_timer
    }
    #[doc = "0x1c - UART Software Control Mode"]
    #[inline(always)]
    pub const fn uart_sw_mode(&self) -> &UartSwMode {
        &self.uart_sw_mode
    }
    #[doc = "0x20 - UART Interrupt Status Register"]
    #[inline(always)]
    pub const fn uart_int_sts(&self) -> &UartIntSts {
        &self.uart_int_sts
    }
    #[doc = "0x24 - UART Interrupt Mask Register"]
    #[inline(always)]
    pub const fn uart_int_mask(&self) -> &UartIntMask {
        &self.uart_int_mask
    }
    #[doc = "0x28 - UART Interrupt Clear Register"]
    #[inline(always)]
    pub const fn uart_int_clear(&self) -> &UartIntClear {
        &self.uart_int_clear
    }
    #[doc = "0x2c - UART Interrupt Enable Register"]
    #[inline(always)]
    pub const fn uart_int_en(&self) -> &UartIntEn {
        &self.uart_int_en
    }
    #[doc = "0x30 - UART Busy Status"]
    #[inline(always)]
    pub const fn uart_status(&self) -> &UartStatus {
        &self.uart_status
    }
    #[doc = "0x34 - Auto Baud Rate Detection Results"]
    #[inline(always)]
    pub const fn sts_urx_abr_prd(&self) -> &StsUrxAbrPrd {
        &self.sts_urx_abr_prd
    }
    #[doc = "0x38 - ABR Bit 0 and 1 Period"]
    #[inline(always)]
    pub const fn urx_abr_prd_b01(&self) -> &UrxAbrPrdB01 {
        &self.urx_abr_prd_b01
    }
    #[doc = "0x3c - ABR Bit 2 and 3 Period"]
    #[inline(always)]
    pub const fn urx_abr_prd_b23(&self) -> &UrxAbrPrdB23 {
        &self.urx_abr_prd_b23
    }
    #[doc = "0x40 - ABR Bit 4 and 5 Period"]
    #[inline(always)]
    pub const fn urx_abr_prd_b45(&self) -> &UrxAbrPrdB45 {
        &self.urx_abr_prd_b45
    }
    #[doc = "0x44 - ABR Bit 6 and 7 Period"]
    #[inline(always)]
    pub const fn urx_abr_prd_b67(&self) -> &UrxAbrPrdB67 {
        &self.urx_abr_prd_b67
    }
    #[doc = "0x48 - ABR Pulse Width Tolerance"]
    #[inline(always)]
    pub const fn urx_abr_pw_tol(&self) -> &UrxAbrPwTol {
        &self.urx_abr_pw_tol
    }
    #[doc = "0x50 - RX Byte Count Interrupt Config"]
    #[inline(always)]
    pub const fn urx_bcr_int_cfg(&self) -> &UrxBcrIntCfg {
        &self.urx_bcr_int_cfg
    }
    #[doc = "0x54 - UART RS-485 Configuration"]
    #[inline(always)]
    pub const fn utx_rs485_cfg(&self) -> &UtxRs485Cfg {
        &self.utx_rs485_cfg
    }
    #[doc = "0x80 - UART FIFO Configuration 0"]
    #[inline(always)]
    pub const fn uart_fifo_config_0(&self) -> &UartFifoConfig0 {
        &self.uart_fifo_config_0
    }
    #[doc = "0x84 - UART FIFO Threshold and Count"]
    #[inline(always)]
    pub const fn uart_fifo_config_1(&self) -> &UartFifoConfig1 {
        &self.uart_fifo_config_1
    }
    #[doc = "0x88 - UART TX FIFO Data Register"]
    #[inline(always)]
    pub const fn uart_fifo_wdata(&self) -> &UartFifoWdata {
        &self.uart_fifo_wdata
    }
    #[doc = "0x8c - UART RX FIFO Data Register"]
    #[inline(always)]
    pub const fn uart_fifo_rdata(&self) -> &UartFifoRdata {
        &self.uart_fifo_rdata
    }
}
#[doc = "UTX_CONFIG (rw) register accessor: UART TX Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`utx_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utx_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utx_config`] module"]
#[doc(alias = "UTX_CONFIG")]
pub type UtxConfig = crate::Reg<utx_config::UtxConfigSpec>;
#[doc = "UART TX Configuration"]
pub mod utx_config;
#[doc = "URX_CONFIG (rw) register accessor: UART RX Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_config`] module"]
#[doc(alias = "URX_CONFIG")]
pub type UrxConfig = crate::Reg<urx_config::UrxConfigSpec>;
#[doc = "UART RX Configuration"]
pub mod urx_config;
#[doc = "UART_BIT_PRD (rw) register accessor: UART Baud Rate Bit Period\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_bit_prd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_bit_prd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_bit_prd`] module"]
#[doc(alias = "UART_BIT_PRD")]
pub type UartBitPrd = crate::Reg<uart_bit_prd::UartBitPrdSpec>;
#[doc = "UART Baud Rate Bit Period"]
pub mod uart_bit_prd;
#[doc = "DATA_CONFIG (rw) register accessor: UART Data Format Config\n\nYou can [`read`](crate::Reg::read) this register and get [`data_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_config`] module"]
#[doc(alias = "DATA_CONFIG")]
pub type DataConfig = crate::Reg<data_config::DataConfigSpec>;
#[doc = "UART Data Format Config"]
pub mod data_config;
#[doc = "UTX_IR_POSITION (rw) register accessor: UART TX IR Pulse Position\n\nYou can [`read`](crate::Reg::read) this register and get [`utx_ir_position::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utx_ir_position::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utx_ir_position`] module"]
#[doc(alias = "UTX_IR_POSITION")]
pub type UtxIrPosition = crate::Reg<utx_ir_position::UtxIrPositionSpec>;
#[doc = "UART TX IR Pulse Position"]
pub mod utx_ir_position;
#[doc = "URX_IR_POSITION (rw) register accessor: UART RX IR Pulse Position\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_ir_position::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_ir_position::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_ir_position`] module"]
#[doc(alias = "URX_IR_POSITION")]
pub type UrxIrPosition = crate::Reg<urx_ir_position::UrxIrPositionSpec>;
#[doc = "UART RX IR Pulse Position"]
pub mod urx_ir_position;
#[doc = "URX_RTO_TIMER (rw) register accessor: UART RX Timeout Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_rto_timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_rto_timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_rto_timer`] module"]
#[doc(alias = "URX_RTO_TIMER")]
pub type UrxRtoTimer = crate::Reg<urx_rto_timer::UrxRtoTimerSpec>;
#[doc = "UART RX Timeout Timer"]
pub mod urx_rto_timer;
#[doc = "UART_SW_MODE (rw) register accessor: UART Software Control Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_sw_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_sw_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_sw_mode`] module"]
#[doc(alias = "UART_SW_MODE")]
pub type UartSwMode = crate::Reg<uart_sw_mode::UartSwModeSpec>;
#[doc = "UART Software Control Mode"]
pub mod uart_sw_mode;
#[doc = "UART_INT_STS (r) register accessor: UART Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_sts`] module"]
#[doc(alias = "UART_INT_STS")]
pub type UartIntSts = crate::Reg<uart_int_sts::UartIntStsSpec>;
#[doc = "UART Interrupt Status Register"]
pub mod uart_int_sts;
#[doc = "UART_INT_MASK (rw) register accessor: UART Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_mask`] module"]
#[doc(alias = "UART_INT_MASK")]
pub type UartIntMask = crate::Reg<uart_int_mask::UartIntMaskSpec>;
#[doc = "UART Interrupt Mask Register"]
pub mod uart_int_mask;
#[doc = "UART_INT_CLEAR (w) register accessor: UART Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_clear`] module"]
#[doc(alias = "UART_INT_CLEAR")]
pub type UartIntClear = crate::Reg<uart_int_clear::UartIntClearSpec>;
#[doc = "UART Interrupt Clear Register"]
pub mod uart_int_clear;
#[doc = "UART_INT_EN (rw) register accessor: UART Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_int_en`] module"]
#[doc(alias = "UART_INT_EN")]
pub type UartIntEn = crate::Reg<uart_int_en::UartIntEnSpec>;
#[doc = "UART Interrupt Enable Register"]
pub mod uart_int_en;
#[doc = "UART_STATUS (r) register accessor: UART Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_status`] module"]
#[doc(alias = "UART_STATUS")]
pub type UartStatus = crate::Reg<uart_status::UartStatusSpec>;
#[doc = "UART Busy Status"]
pub mod uart_status;
#[doc = "STS_URX_ABR_PRD (r) register accessor: Auto Baud Rate Detection Results\n\nYou can [`read`](crate::Reg::read) this register and get [`sts_urx_abr_prd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts_urx_abr_prd`] module"]
#[doc(alias = "STS_URX_ABR_PRD")]
pub type StsUrxAbrPrd = crate::Reg<sts_urx_abr_prd::StsUrxAbrPrdSpec>;
#[doc = "Auto Baud Rate Detection Results"]
pub mod sts_urx_abr_prd;
#[doc = "URX_ABR_PRD_B01 (r) register accessor: ABR Bit 0 and 1 Period\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_abr_prd_b01::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_abr_prd_b01`] module"]
#[doc(alias = "URX_ABR_PRD_B01")]
pub type UrxAbrPrdB01 = crate::Reg<urx_abr_prd_b01::UrxAbrPrdB01Spec>;
#[doc = "ABR Bit 0 and 1 Period"]
pub mod urx_abr_prd_b01;
#[doc = "URX_ABR_PRD_B23 (r) register accessor: ABR Bit 2 and 3 Period\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_abr_prd_b23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_abr_prd_b23`] module"]
#[doc(alias = "URX_ABR_PRD_B23")]
pub type UrxAbrPrdB23 = crate::Reg<urx_abr_prd_b23::UrxAbrPrdB23Spec>;
#[doc = "ABR Bit 2 and 3 Period"]
pub mod urx_abr_prd_b23;
#[doc = "URX_ABR_PRD_B45 (r) register accessor: ABR Bit 4 and 5 Period\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_abr_prd_b45::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_abr_prd_b45`] module"]
#[doc(alias = "URX_ABR_PRD_B45")]
pub type UrxAbrPrdB45 = crate::Reg<urx_abr_prd_b45::UrxAbrPrdB45Spec>;
#[doc = "ABR Bit 4 and 5 Period"]
pub mod urx_abr_prd_b45;
#[doc = "URX_ABR_PRD_B67 (r) register accessor: ABR Bit 6 and 7 Period\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_abr_prd_b67::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_abr_prd_b67`] module"]
#[doc(alias = "URX_ABR_PRD_B67")]
pub type UrxAbrPrdB67 = crate::Reg<urx_abr_prd_b67::UrxAbrPrdB67Spec>;
#[doc = "ABR Bit 6 and 7 Period"]
pub mod urx_abr_prd_b67;
#[doc = "URX_ABR_PW_TOL (rw) register accessor: ABR Pulse Width Tolerance\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_abr_pw_tol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_abr_pw_tol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_abr_pw_tol`] module"]
#[doc(alias = "URX_ABR_PW_TOL")]
pub type UrxAbrPwTol = crate::Reg<urx_abr_pw_tol::UrxAbrPwTolSpec>;
#[doc = "ABR Pulse Width Tolerance"]
pub mod urx_abr_pw_tol;
#[doc = "URX_BCR_INT_CFG (rw) register accessor: RX Byte Count Interrupt Config\n\nYou can [`read`](crate::Reg::read) this register and get [`urx_bcr_int_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urx_bcr_int_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urx_bcr_int_cfg`] module"]
#[doc(alias = "URX_BCR_INT_CFG")]
pub type UrxBcrIntCfg = crate::Reg<urx_bcr_int_cfg::UrxBcrIntCfgSpec>;
#[doc = "RX Byte Count Interrupt Config"]
pub mod urx_bcr_int_cfg;
#[doc = "UTX_RS485_CFG (rw) register accessor: UART RS-485 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`utx_rs485_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utx_rs485_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@utx_rs485_cfg`] module"]
#[doc(alias = "UTX_RS485_CFG")]
pub type UtxRs485Cfg = crate::Reg<utx_rs485_cfg::UtxRs485CfgSpec>;
#[doc = "UART RS-485 Configuration"]
pub mod utx_rs485_cfg;
#[doc = "UART_FIFO_CONFIG_0 (rw) register accessor: UART FIFO Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_fifo_config_0`] module"]
#[doc(alias = "UART_FIFO_CONFIG_0")]
pub type UartFifoConfig0 = crate::Reg<uart_fifo_config_0::UartFifoConfig0Spec>;
#[doc = "UART FIFO Configuration 0"]
pub mod uart_fifo_config_0;
#[doc = "UART_FIFO_CONFIG_1 (rw) register accessor: UART FIFO Threshold and Count\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_config_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_config_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_fifo_config_1`] module"]
#[doc(alias = "UART_FIFO_CONFIG_1")]
pub type UartFifoConfig1 = crate::Reg<uart_fifo_config_1::UartFifoConfig1Spec>;
#[doc = "UART FIFO Threshold and Count"]
pub mod uart_fifo_config_1;
#[doc = "UART_FIFO_WDATA (w) register accessor: UART TX FIFO Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_fifo_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_fifo_wdata`] module"]
#[doc(alias = "UART_FIFO_WDATA")]
pub type UartFifoWdata = crate::Reg<uart_fifo_wdata::UartFifoWdataSpec>;
#[doc = "UART TX FIFO Data Register"]
pub mod uart_fifo_wdata;
#[doc = "UART_FIFO_RDATA (r) register accessor: UART RX FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_fifo_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_fifo_rdata`] module"]
#[doc(alias = "UART_FIFO_RDATA")]
pub type UartFifoRdata = crate::Reg<uart_fifo_rdata::UartFifoRdataSpec>;
#[doc = "UART RX FIFO Data Register"]
pub mod uart_fifo_rdata;
