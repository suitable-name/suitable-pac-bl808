#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    uhs_basic: UhsBasic,
    uhs_cmd: UhsCmd,
    uhs_fifo_thre: UhsFifoThre,
    uhs_manual: UhsManual,
    uhs_auto_fresh_1: UhsAutoFresh1,
    uhs_auto_fresh_2: UhsAutoFresh2,
    uhs_auto_fresh_3: UhsAutoFresh3,
    uhs_auto_fresh_4: UhsAutoFresh4,
    uhs_psram_configure: UhsPsramConfigure,
    uhs_psram_status: UhsPsramStatus,
    _reserved10: [u8; 0x08],
    uhs_timing_ctrl: UhsTimingCtrl,
    uhs_rsvd_reg: UhsRsvdReg,
    _reserved12: [u8; 0x88],
    uhs_dbg_sel: UhsDbgSel,
    _reserved13: [u8; 0x2c],
    uhs_dummy_reg: UhsDummyReg,
    _reserved14: [u8; 0x0c],
    phy_cfg_00: PhyCfg00,
    phy_cfg_04: PhyCfg04,
    phy_cfg_08: PhyCfg08,
    phy_cfg_0c: PhyCfg0c,
    phy_cfg_10: PhyCfg10,
    phy_cfg_14: PhyCfg14,
    phy_cfg_18: PhyCfg18,
    phy_cfg_1c: PhyCfg1c,
    phy_cfg_20: PhyCfg20,
    phy_cfg_24: PhyCfg24,
    phy_cfg_28: PhyCfg28,
    phy_cfg_2c: PhyCfg2c,
    phy_cfg_30: PhyCfg30,
    phy_cfg_34: PhyCfg34,
    phy_cfg_38: PhyCfg38,
    phy_cfg_3c: PhyCfg3c,
    phy_cfg_40: PhyCfg40,
    phy_cfg_44: PhyCfg44,
    phy_cfg_48: PhyCfg48,
    phy_cfg_4c: PhyCfg4c,
    phy_cfg_50: PhyCfg50,
}
impl RegisterBlock {
    #[doc = "0x00 - UHS Basic Configuration Register"]
    #[inline(always)]
    pub const fn uhs_basic(&self) -> &UhsBasic {
        &self.uhs_basic
    }
    #[doc = "0x04 - UHS Command and Status Register"]
    #[inline(always)]
    pub const fn uhs_cmd(&self) -> &UhsCmd {
        &self.uhs_cmd
    }
    #[doc = "0x08 - UHS FIFO Threshold Configuration"]
    #[inline(always)]
    pub const fn uhs_fifo_thre(&self) -> &UhsFifoThre {
        &self.uhs_fifo_thre
    }
    #[doc = "0x0c - UHS Manual Control and Clock Divider Register"]
    #[inline(always)]
    pub const fn uhs_manual(&self) -> &UhsManual {
        &self.uhs_manual
    }
    #[doc = "0x10 - UHS Auto Refresh Window Configuration 1"]
    #[inline(always)]
    pub const fn uhs_auto_fresh_1(&self) -> &UhsAutoFresh1 {
        &self.uhs_auto_fresh_1
    }
    #[doc = "0x14 - UHS Auto Refresh Window Configuration 2"]
    #[inline(always)]
    pub const fn uhs_auto_fresh_2(&self) -> &UhsAutoFresh2 {
        &self.uhs_auto_fresh_2
    }
    #[doc = "0x18 - UHS Auto Refresh Threshold and Level"]
    #[inline(always)]
    pub const fn uhs_auto_fresh_3(&self) -> &UhsAutoFresh3 {
        &self.uhs_auto_fresh_3
    }
    #[doc = "0x1c - UHS Auto Refresh Burst Cycle"]
    #[inline(always)]
    pub const fn uhs_auto_fresh_4(&self) -> &UhsAutoFresh4 {
        &self.uhs_auto_fresh_4
    }
    #[doc = "0x20 - UHS PSRAM Device Parameters Configuration"]
    #[inline(always)]
    pub const fn uhs_psram_configure(&self) -> &UhsPsramConfigure {
        &self.uhs_psram_configure
    }
    #[doc = "0x24 - UHS PSRAM Device Parameters Status"]
    #[inline(always)]
    pub const fn uhs_psram_status(&self) -> &UhsPsramStatus {
        &self.uhs_psram_status
    }
    #[doc = "0x30 - UHS Hardware Timing Control"]
    #[inline(always)]
    pub const fn uhs_timing_ctrl(&self) -> &UhsTimingCtrl {
        &self.uhs_timing_ctrl
    }
    #[doc = "0x34 - UHS Mode Register Reserved Bits"]
    #[inline(always)]
    pub const fn uhs_rsvd_reg(&self) -> &UhsRsvdReg {
        &self.uhs_rsvd_reg
    }
    #[doc = "0xc0 - UHS Debug Source Selection"]
    #[inline(always)]
    pub const fn uhs_dbg_sel(&self) -> &UhsDbgSel {
        &self.uhs_dbg_sel
    }
    #[doc = "0xf0 - UHS Dummy Data Register"]
    #[inline(always)]
    pub const fn uhs_dummy_reg(&self) -> &UhsDummyReg {
        &self.uhs_dummy_reg
    }
    #[doc = "0x100 - PHY Pin Configuration 00 (DQS/CK/CEN)"]
    #[inline(always)]
    pub const fn phy_cfg_00(&self) -> &PhyCfg00 {
        &self.phy_cfg_00
    }
    #[doc = "0x104 - PHY Pin Configuration 04 (DM1/DM0)"]
    #[inline(always)]
    pub const fn phy_cfg_04(&self) -> &PhyCfg04 {
        &self.phy_cfg_04
    }
    #[doc = "0x108 - PHY Pin Configuration 08 (DQ1/DQ0)"]
    #[inline(always)]
    pub const fn phy_cfg_08(&self) -> &PhyCfg08 {
        &self.phy_cfg_08
    }
    #[doc = "0x10c - PHY Pin Configuration 0C (DQ3/DQ2)"]
    #[inline(always)]
    pub const fn phy_cfg_0c(&self) -> &PhyCfg0c {
        &self.phy_cfg_0c
    }
    #[doc = "0x110 - PHY Pin Configuration 10 (DQ5/DQ4)"]
    #[inline(always)]
    pub const fn phy_cfg_10(&self) -> &PhyCfg10 {
        &self.phy_cfg_10
    }
    #[doc = "0x114 - PHY Pin Configuration 14 (DQ7/DQ6)"]
    #[inline(always)]
    pub const fn phy_cfg_14(&self) -> &PhyCfg14 {
        &self.phy_cfg_14
    }
    #[doc = "0x118 - PHY Pin Configuration 18 (DQ9/DQ8)"]
    #[inline(always)]
    pub const fn phy_cfg_18(&self) -> &PhyCfg18 {
        &self.phy_cfg_18
    }
    #[doc = "0x11c - PHY Pin Configuration 1C (DQ11/DQ10)"]
    #[inline(always)]
    pub const fn phy_cfg_1c(&self) -> &PhyCfg1c {
        &self.phy_cfg_1c
    }
    #[doc = "0x120 - PHY Pin Configuration 20 (DQ13/DQ12)"]
    #[inline(always)]
    pub const fn phy_cfg_20(&self) -> &PhyCfg20 {
        &self.phy_cfg_20
    }
    #[doc = "0x124 - PHY Pin Configuration 24 (DQ15/DQ14)"]
    #[inline(always)]
    pub const fn phy_cfg_24(&self) -> &PhyCfg24 {
        &self.phy_cfg_24
    }
    #[doc = "0x128 - PHY Pin Configuration 28 (DQS0)"]
    #[inline(always)]
    pub const fn phy_cfg_28(&self) -> &PhyCfg28 {
        &self.phy_cfg_28
    }
    #[doc = "0x12c - PHY Pin Configuration 2C (DQS1)"]
    #[inline(always)]
    pub const fn phy_cfg_2c(&self) -> &PhyCfg2c {
        &self.phy_cfg_2c
    }
    #[doc = "0x130 - PHY Latency and VREF Configuration"]
    #[inline(always)]
    pub const fn phy_cfg_30(&self) -> &PhyCfg30 {
        &self.phy_cfg_30
    }
    #[doc = "0x134 - PHY Timer Configuration 1"]
    #[inline(always)]
    pub const fn phy_cfg_34(&self) -> &PhyCfg34 {
        &self.phy_cfg_34
    }
    #[doc = "0x138 - PHY Timer Configuration 2"]
    #[inline(always)]
    pub const fn phy_cfg_38(&self) -> &PhyCfg38 {
        &self.phy_cfg_38
    }
    #[doc = "0x13c - PHY Timer Configuration 3"]
    #[inline(always)]
    pub const fn phy_cfg_3c(&self) -> &PhyCfg3c {
        &self.phy_cfg_3c
    }
    #[doc = "0x140 - PHY Power and VREF Tuning"]
    #[inline(always)]
    pub const fn phy_cfg_40(&self) -> &PhyCfg40 {
        &self.phy_cfg_40
    }
    #[doc = "0x144 - PHY Timer Busy Status"]
    #[inline(always)]
    pub const fn phy_cfg_44(&self) -> &PhyCfg44 {
        &self.phy_cfg_44
    }
    #[doc = "0x148 - PHY Test and Multiplexer Configuration"]
    #[inline(always)]
    pub const fn phy_cfg_48(&self) -> &PhyCfg48 {
        &self.phy_cfg_48
    }
    #[doc = "0x14c - PHY ODT and Digital Output Configuration"]
    #[inline(always)]
    pub const fn phy_cfg_4c(&self) -> &PhyCfg4c {
        &self.phy_cfg_4c
    }
    #[doc = "0x150 - PHY Output Enable (OE) Tuning"]
    #[inline(always)]
    pub const fn phy_cfg_50(&self) -> &PhyCfg50 {
        &self.phy_cfg_50
    }
}
#[doc = "UHS_BASIC (rw) register accessor: UHS Basic Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_basic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_basic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_basic`] module"]
#[doc(alias = "UHS_BASIC")]
pub type UhsBasic = crate::Reg<uhs_basic::UhsBasicSpec>;
#[doc = "UHS Basic Configuration Register"]
pub mod uhs_basic;
#[doc = "UHS_CMD (rw) register accessor: UHS Command and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_cmd`] module"]
#[doc(alias = "UHS_CMD")]
pub type UhsCmd = crate::Reg<uhs_cmd::UhsCmdSpec>;
#[doc = "UHS Command and Status Register"]
pub mod uhs_cmd;
#[doc = "UHS_FIFO_THRE (rw) register accessor: UHS FIFO Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_fifo_thre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_fifo_thre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_fifo_thre`] module"]
#[doc(alias = "UHS_FIFO_THRE")]
pub type UhsFifoThre = crate::Reg<uhs_fifo_thre::UhsFifoThreSpec>;
#[doc = "UHS FIFO Threshold Configuration"]
pub mod uhs_fifo_thre;
#[doc = "UHS_MANUAL (rw) register accessor: UHS Manual Control and Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_manual::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_manual::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_manual`] module"]
#[doc(alias = "UHS_MANUAL")]
pub type UhsManual = crate::Reg<uhs_manual::UhsManualSpec>;
#[doc = "UHS Manual Control and Clock Divider Register"]
pub mod uhs_manual;
#[doc = "UHS_AUTO_FRESH_1 (rw) register accessor: UHS Auto Refresh Window Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_auto_fresh_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_auto_fresh_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_auto_fresh_1`] module"]
#[doc(alias = "UHS_AUTO_FRESH_1")]
pub type UhsAutoFresh1 = crate::Reg<uhs_auto_fresh_1::UhsAutoFresh1Spec>;
#[doc = "UHS Auto Refresh Window Configuration 1"]
pub mod uhs_auto_fresh_1;
#[doc = "UHS_AUTO_FRESH_2 (rw) register accessor: UHS Auto Refresh Window Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_auto_fresh_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_auto_fresh_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_auto_fresh_2`] module"]
#[doc(alias = "UHS_AUTO_FRESH_2")]
pub type UhsAutoFresh2 = crate::Reg<uhs_auto_fresh_2::UhsAutoFresh2Spec>;
#[doc = "UHS Auto Refresh Window Configuration 2"]
pub mod uhs_auto_fresh_2;
#[doc = "UHS_AUTO_FRESH_3 (rw) register accessor: UHS Auto Refresh Threshold and Level\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_auto_fresh_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_auto_fresh_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_auto_fresh_3`] module"]
#[doc(alias = "UHS_AUTO_FRESH_3")]
pub type UhsAutoFresh3 = crate::Reg<uhs_auto_fresh_3::UhsAutoFresh3Spec>;
#[doc = "UHS Auto Refresh Threshold and Level"]
pub mod uhs_auto_fresh_3;
#[doc = "UHS_AUTO_FRESH_4 (rw) register accessor: UHS Auto Refresh Burst Cycle\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_auto_fresh_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_auto_fresh_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_auto_fresh_4`] module"]
#[doc(alias = "UHS_AUTO_FRESH_4")]
pub type UhsAutoFresh4 = crate::Reg<uhs_auto_fresh_4::UhsAutoFresh4Spec>;
#[doc = "UHS Auto Refresh Burst Cycle"]
pub mod uhs_auto_fresh_4;
#[doc = "UHS_PSRAM_CONFIGURE (rw) register accessor: UHS PSRAM Device Parameters Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_psram_configure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_psram_configure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_psram_configure`] module"]
#[doc(alias = "UHS_PSRAM_CONFIGURE")]
pub type UhsPsramConfigure = crate::Reg<uhs_psram_configure::UhsPsramConfigureSpec>;
#[doc = "UHS PSRAM Device Parameters Configuration"]
pub mod uhs_psram_configure;
#[doc = "UHS_PSRAM_STATUS (r) register accessor: UHS PSRAM Device Parameters Status\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_psram_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_psram_status`] module"]
#[doc(alias = "UHS_PSRAM_STATUS")]
pub type UhsPsramStatus = crate::Reg<uhs_psram_status::UhsPsramStatusSpec>;
#[doc = "UHS PSRAM Device Parameters Status"]
pub mod uhs_psram_status;
#[doc = "UHS_TIMING_CTRL (rw) register accessor: UHS Hardware Timing Control\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_timing_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_timing_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_timing_ctrl`] module"]
#[doc(alias = "UHS_TIMING_CTRL")]
pub type UhsTimingCtrl = crate::Reg<uhs_timing_ctrl::UhsTimingCtrlSpec>;
#[doc = "UHS Hardware Timing Control"]
pub mod uhs_timing_ctrl;
#[doc = "UHS_RSVD_REG (rw) register accessor: UHS Mode Register Reserved Bits\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_rsvd_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_rsvd_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_rsvd_reg`] module"]
#[doc(alias = "UHS_RSVD_REG")]
pub type UhsRsvdReg = crate::Reg<uhs_rsvd_reg::UhsRsvdRegSpec>;
#[doc = "UHS Mode Register Reserved Bits"]
pub mod uhs_rsvd_reg;
#[doc = "UHS_DBG_SEL (rw) register accessor: UHS Debug Source Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_dbg_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_dbg_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_dbg_sel`] module"]
#[doc(alias = "UHS_DBG_SEL")]
pub type UhsDbgSel = crate::Reg<uhs_dbg_sel::UhsDbgSelSpec>;
#[doc = "UHS Debug Source Selection"]
pub mod uhs_dbg_sel;
#[doc = "UHS_DUMMY_REG (rw) register accessor: UHS Dummy Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_dummy_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_dummy_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhs_dummy_reg`] module"]
#[doc(alias = "UHS_DUMMY_REG")]
pub type UhsDummyReg = crate::Reg<uhs_dummy_reg::UhsDummyRegSpec>;
#[doc = "UHS Dummy Data Register"]
pub mod uhs_dummy_reg;
#[doc = "PHY_CFG_00 (rw) register accessor: PHY Pin Configuration 00 (DQS/CK/CEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_00`] module"]
#[doc(alias = "PHY_CFG_00")]
pub type PhyCfg00 = crate::Reg<phy_cfg_00::PhyCfg00Spec>;
#[doc = "PHY Pin Configuration 00 (DQS/CK/CEN)"]
pub mod phy_cfg_00;
#[doc = "PHY_CFG_04 (rw) register accessor: PHY Pin Configuration 04 (DM1/DM0)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_04`] module"]
#[doc(alias = "PHY_CFG_04")]
pub type PhyCfg04 = crate::Reg<phy_cfg_04::PhyCfg04Spec>;
#[doc = "PHY Pin Configuration 04 (DM1/DM0)"]
pub mod phy_cfg_04;
#[doc = "PHY_CFG_08 (rw) register accessor: PHY Pin Configuration 08 (DQ1/DQ0)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_08`] module"]
#[doc(alias = "PHY_CFG_08")]
pub type PhyCfg08 = crate::Reg<phy_cfg_08::PhyCfg08Spec>;
#[doc = "PHY Pin Configuration 08 (DQ1/DQ0)"]
pub mod phy_cfg_08;
#[doc = "PHY_CFG_0C (rw) register accessor: PHY Pin Configuration 0C (DQ3/DQ2)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_0c`] module"]
#[doc(alias = "PHY_CFG_0C")]
pub type PhyCfg0c = crate::Reg<phy_cfg_0c::PhyCfg0cSpec>;
#[doc = "PHY Pin Configuration 0C (DQ3/DQ2)"]
pub mod phy_cfg_0c;
#[doc = "PHY_CFG_10 (rw) register accessor: PHY Pin Configuration 10 (DQ5/DQ4)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_10`] module"]
#[doc(alias = "PHY_CFG_10")]
pub type PhyCfg10 = crate::Reg<phy_cfg_10::PhyCfg10Spec>;
#[doc = "PHY Pin Configuration 10 (DQ5/DQ4)"]
pub mod phy_cfg_10;
#[doc = "PHY_CFG_14 (rw) register accessor: PHY Pin Configuration 14 (DQ7/DQ6)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_14`] module"]
#[doc(alias = "PHY_CFG_14")]
pub type PhyCfg14 = crate::Reg<phy_cfg_14::PhyCfg14Spec>;
#[doc = "PHY Pin Configuration 14 (DQ7/DQ6)"]
pub mod phy_cfg_14;
#[doc = "PHY_CFG_18 (rw) register accessor: PHY Pin Configuration 18 (DQ9/DQ8)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_18`] module"]
#[doc(alias = "PHY_CFG_18")]
pub type PhyCfg18 = crate::Reg<phy_cfg_18::PhyCfg18Spec>;
#[doc = "PHY Pin Configuration 18 (DQ9/DQ8)"]
pub mod phy_cfg_18;
#[doc = "PHY_CFG_1C (rw) register accessor: PHY Pin Configuration 1C (DQ11/DQ10)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_1c`] module"]
#[doc(alias = "PHY_CFG_1C")]
pub type PhyCfg1c = crate::Reg<phy_cfg_1c::PhyCfg1cSpec>;
#[doc = "PHY Pin Configuration 1C (DQ11/DQ10)"]
pub mod phy_cfg_1c;
#[doc = "PHY_CFG_20 (rw) register accessor: PHY Pin Configuration 20 (DQ13/DQ12)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_20`] module"]
#[doc(alias = "PHY_CFG_20")]
pub type PhyCfg20 = crate::Reg<phy_cfg_20::PhyCfg20Spec>;
#[doc = "PHY Pin Configuration 20 (DQ13/DQ12)"]
pub mod phy_cfg_20;
#[doc = "PHY_CFG_24 (rw) register accessor: PHY Pin Configuration 24 (DQ15/DQ14)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_24`] module"]
#[doc(alias = "PHY_CFG_24")]
pub type PhyCfg24 = crate::Reg<phy_cfg_24::PhyCfg24Spec>;
#[doc = "PHY Pin Configuration 24 (DQ15/DQ14)"]
pub mod phy_cfg_24;
#[doc = "PHY_CFG_28 (rw) register accessor: PHY Pin Configuration 28 (DQS0)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_28`] module"]
#[doc(alias = "PHY_CFG_28")]
pub type PhyCfg28 = crate::Reg<phy_cfg_28::PhyCfg28Spec>;
#[doc = "PHY Pin Configuration 28 (DQS0)"]
pub mod phy_cfg_28;
#[doc = "PHY_CFG_2C (rw) register accessor: PHY Pin Configuration 2C (DQS1)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_2c`] module"]
#[doc(alias = "PHY_CFG_2C")]
pub type PhyCfg2c = crate::Reg<phy_cfg_2c::PhyCfg2cSpec>;
#[doc = "PHY Pin Configuration 2C (DQS1)"]
pub mod phy_cfg_2c;
#[doc = "PHY_CFG_30 (rw) register accessor: PHY Latency and VREF Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_30`] module"]
#[doc(alias = "PHY_CFG_30")]
pub type PhyCfg30 = crate::Reg<phy_cfg_30::PhyCfg30Spec>;
#[doc = "PHY Latency and VREF Configuration"]
pub mod phy_cfg_30;
#[doc = "PHY_CFG_34 (rw) register accessor: PHY Timer Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_34`] module"]
#[doc(alias = "PHY_CFG_34")]
pub type PhyCfg34 = crate::Reg<phy_cfg_34::PhyCfg34Spec>;
#[doc = "PHY Timer Configuration 1"]
pub mod phy_cfg_34;
#[doc = "PHY_CFG_38 (rw) register accessor: PHY Timer Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_38`] module"]
#[doc(alias = "PHY_CFG_38")]
pub type PhyCfg38 = crate::Reg<phy_cfg_38::PhyCfg38Spec>;
#[doc = "PHY Timer Configuration 2"]
pub mod phy_cfg_38;
#[doc = "PHY_CFG_3C (rw) register accessor: PHY Timer Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_3c`] module"]
#[doc(alias = "PHY_CFG_3C")]
pub type PhyCfg3c = crate::Reg<phy_cfg_3c::PhyCfg3cSpec>;
#[doc = "PHY Timer Configuration 3"]
pub mod phy_cfg_3c;
#[doc = "PHY_CFG_40 (rw) register accessor: PHY Power and VREF Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_40`] module"]
#[doc(alias = "PHY_CFG_40")]
pub type PhyCfg40 = crate::Reg<phy_cfg_40::PhyCfg40Spec>;
#[doc = "PHY Power and VREF Tuning"]
pub mod phy_cfg_40;
#[doc = "PHY_CFG_44 (rw) register accessor: PHY Timer Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_44`] module"]
#[doc(alias = "PHY_CFG_44")]
pub type PhyCfg44 = crate::Reg<phy_cfg_44::PhyCfg44Spec>;
#[doc = "PHY Timer Busy Status"]
pub mod phy_cfg_44;
#[doc = "PHY_CFG_48 (rw) register accessor: PHY Test and Multiplexer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_48`] module"]
#[doc(alias = "PHY_CFG_48")]
pub type PhyCfg48 = crate::Reg<phy_cfg_48::PhyCfg48Spec>;
#[doc = "PHY Test and Multiplexer Configuration"]
pub mod phy_cfg_48;
#[doc = "PHY_CFG_4C (rw) register accessor: PHY ODT and Digital Output Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_4c`] module"]
#[doc(alias = "PHY_CFG_4C")]
pub type PhyCfg4c = crate::Reg<phy_cfg_4c::PhyCfg4cSpec>;
#[doc = "PHY ODT and Digital Output Configuration"]
pub mod phy_cfg_4c;
#[doc = "PHY_CFG_50 (rw) register accessor: PHY Output Enable (OE) Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_cfg_50`] module"]
#[doc(alias = "PHY_CFG_50")]
pub type PhyCfg50 = crate::Reg<phy_cfg_50::PhyCfg50Spec>;
#[doc = "PHY Output Enable (OE) Tuning"]
pub mod phy_cfg_50;
