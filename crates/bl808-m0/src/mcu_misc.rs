#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcu_bus_cfg0: McuBusCfg0,
    mcu_bus_cfg1: McuBusCfg1,
    _reserved2: [u8; 0x0c],
    mcu_e907_rtc: McuE907Rtc,
    _reserved3: [u8; 0xe8],
    mcu_cfg1: McuCfg1,
    _reserved4: [u8; 0x0c],
    mcu1_log1: Mcu1Log1,
    mcu1_log2: Mcu1Log2,
    mcu1_log3: Mcu1Log3,
    mcu1_log4: Mcu1Log4,
    mcu1_log5: Mcu1Log5,
    _reserved9: [u8; 0xe4],
    irom1_misr_dataout_0: Irom1MisrDataout0,
    irom1_misr_dataout_1: Irom1MisrDataout1,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU Bus Configuration 0 (Timeout)"]
    #[inline(always)]
    pub const fn mcu_bus_cfg0(&self) -> &McuBusCfg0 {
        &self.mcu_bus_cfg0
    }
    #[doc = "0x04 - MCU Bus Configuration 1 (QoS and Arbitration)"]
    #[inline(always)]
    pub const fn mcu_bus_cfg1(&self) -> &McuBusCfg1 {
        &self.mcu_bus_cfg1
    }
    #[doc = "0x14 - E907 (MCU1) Real-Time Clock Configuration"]
    #[inline(always)]
    pub const fn mcu_e907_rtc(&self) -> &McuE907Rtc {
        &self.mcu_e907_rtc
    }
    #[doc = "0x100 - MCU Core Configuration 1 (E907 Reset/DFS)"]
    #[inline(always)]
    pub const fn mcu_cfg1(&self) -> &McuCfg1 {
        &self.mcu_cfg1
    }
    #[doc = "0x110 - E907 Debug Log 1 (MCAUSE)"]
    #[inline(always)]
    pub const fn mcu1_log1(&self) -> &Mcu1Log1 {
        &self.mcu1_log1
    }
    #[doc = "0x114 - E907 Debug Log 2 (MINTSTATUS)"]
    #[inline(always)]
    pub const fn mcu1_log2(&self) -> &Mcu1Log2 {
        &self.mcu1_log2
    }
    #[doc = "0x118 - E907 Debug Log 3 (MSTATUS)"]
    #[inline(always)]
    pub const fn mcu1_log3(&self) -> &Mcu1Log3 {
        &self.mcu1_log3
    }
    #[doc = "0x11c - E907 Debug Log 4 (Stack Pointer / Program Counter)"]
    #[inline(always)]
    pub const fn mcu1_log4(&self) -> &Mcu1Log4 {
        &self.mcu1_log4
    }
    #[doc = "0x120 - E907 Debug Log 5 (Lockup/Halted Status)"]
    #[inline(always)]
    pub const fn mcu1_log5(&self) -> &Mcu1Log5 {
        &self.mcu1_log5
    }
    #[doc = "0x208 - IROM1 MISR Data Output 0 (Test/Debug)"]
    #[inline(always)]
    pub const fn irom1_misr_dataout_0(&self) -> &Irom1MisrDataout0 {
        &self.irom1_misr_dataout_0
    }
    #[doc = "0x20c - IROM1 MISR Data Output 1 (Test/Debug)"]
    #[inline(always)]
    pub const fn irom1_misr_dataout_1(&self) -> &Irom1MisrDataout1 {
        &self.irom1_misr_dataout_1
    }
}
#[doc = "MCU_BUS_CFG0 (rw) register accessor: MCU Bus Configuration 0 (Timeout)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu_bus_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcu_bus_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu_bus_cfg0`] module"]
#[doc(alias = "MCU_BUS_CFG0")]
pub type McuBusCfg0 = crate::Reg<mcu_bus_cfg0::McuBusCfg0Spec>;
#[doc = "MCU Bus Configuration 0 (Timeout)"]
pub mod mcu_bus_cfg0;
#[doc = "MCU_BUS_CFG1 (rw) register accessor: MCU Bus Configuration 1 (QoS and Arbitration)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu_bus_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcu_bus_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu_bus_cfg1`] module"]
#[doc(alias = "MCU_BUS_CFG1")]
pub type McuBusCfg1 = crate::Reg<mcu_bus_cfg1::McuBusCfg1Spec>;
#[doc = "MCU Bus Configuration 1 (QoS and Arbitration)"]
pub mod mcu_bus_cfg1;
#[doc = "MCU_E907_RTC (rw) register accessor: E907 (MCU1) Real-Time Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu_e907_rtc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcu_e907_rtc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu_e907_rtc`] module"]
#[doc(alias = "MCU_E907_RTC")]
pub type McuE907Rtc = crate::Reg<mcu_e907_rtc::McuE907RtcSpec>;
#[doc = "E907 (MCU1) Real-Time Clock Configuration"]
pub mod mcu_e907_rtc;
#[doc = "MCU_CFG1 (rw) register accessor: MCU Core Configuration 1 (E907 Reset/DFS)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcu_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu_cfg1`] module"]
#[doc(alias = "MCU_CFG1")]
pub type McuCfg1 = crate::Reg<mcu_cfg1::McuCfg1Spec>;
#[doc = "MCU Core Configuration 1 (E907 Reset/DFS)"]
pub mod mcu_cfg1;
#[doc = "MCU1_LOG1 (r) register accessor: E907 Debug Log 1 (MCAUSE)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu1_log1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu1_log1`] module"]
#[doc(alias = "MCU1_LOG1")]
pub type Mcu1Log1 = crate::Reg<mcu1_log1::Mcu1Log1Spec>;
#[doc = "E907 Debug Log 1 (MCAUSE)"]
pub mod mcu1_log1;
#[doc = "MCU1_LOG2 (r) register accessor: E907 Debug Log 2 (MINTSTATUS)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu1_log2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu1_log2`] module"]
#[doc(alias = "MCU1_LOG2")]
pub type Mcu1Log2 = crate::Reg<mcu1_log2::Mcu1Log2Spec>;
#[doc = "E907 Debug Log 2 (MINTSTATUS)"]
pub mod mcu1_log2;
#[doc = "MCU1_LOG3 (r) register accessor: E907 Debug Log 3 (MSTATUS)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu1_log3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu1_log3`] module"]
#[doc(alias = "MCU1_LOG3")]
pub type Mcu1Log3 = crate::Reg<mcu1_log3::Mcu1Log3Spec>;
#[doc = "E907 Debug Log 3 (MSTATUS)"]
pub mod mcu1_log3;
#[doc = "MCU1_LOG4 (r) register accessor: E907 Debug Log 4 (Stack Pointer / Program Counter)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu1_log4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu1_log4`] module"]
#[doc(alias = "MCU1_LOG4")]
pub type Mcu1Log4 = crate::Reg<mcu1_log4::Mcu1Log4Spec>;
#[doc = "E907 Debug Log 4 (Stack Pointer / Program Counter)"]
pub mod mcu1_log4;
#[doc = "MCU1_LOG5 (r) register accessor: E907 Debug Log 5 (Lockup/Halted Status)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu1_log5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu1_log5`] module"]
#[doc(alias = "MCU1_LOG5")]
pub type Mcu1Log5 = crate::Reg<mcu1_log5::Mcu1Log5Spec>;
#[doc = "E907 Debug Log 5 (Lockup/Halted Status)"]
pub mod mcu1_log5;
#[doc = "IROM1_MISR_DATAOUT_0 (r) register accessor: IROM1 MISR Data Output 0 (Test/Debug)\n\nYou can [`read`](crate::Reg::read) this register and get [`irom1_misr_dataout_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irom1_misr_dataout_0`] module"]
#[doc(alias = "IROM1_MISR_DATAOUT_0")]
pub type Irom1MisrDataout0 = crate::Reg<irom1_misr_dataout_0::Irom1MisrDataout0Spec>;
#[doc = "IROM1 MISR Data Output 0 (Test/Debug)"]
pub mod irom1_misr_dataout_0;
#[doc = "IROM1_MISR_DATAOUT_1 (r) register accessor: IROM1 MISR Data Output 1 (Test/Debug)\n\nYou can [`read`](crate::Reg::read) this register and get [`irom1_misr_dataout_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irom1_misr_dataout_1`] module"]
#[doc(alias = "IROM1_MISR_DATAOUT_1")]
pub type Irom1MisrDataout1 = crate::Reg<irom1_misr_dataout_1::Irom1MisrDataout1Spec>;
#[doc = "IROM1 MISR Data Output 1 (Test/Debug)"]
pub mod irom1_misr_dataout_1;
