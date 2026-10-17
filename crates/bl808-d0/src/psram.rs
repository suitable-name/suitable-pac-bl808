#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    psram_configure: PsramConfigure,
    psram_manual_control: PsramManualControl,
    fifo_thres_control: FifoThresControl,
    psram_manual_control2: PsramManualControl2,
    winbond_psram_configure: WinbondPsramConfigure,
    winbond_psram_status: WinbondPsramStatus,
    winbond_psram_configure2: WinbondPsramConfigure2,
    _reserved7: [u8; 0x04],
    apmemory_psram_configure: ApmemoryPsramConfigure,
    apmemory_psram_status: ApmemoryPsramStatus,
    _reserved9: [u8; 0x08],
    psram_manual_control3: PsramManualControl3,
    _reserved10: [u8; 0x4c],
    psram_intf_delay_ctrl0: PsramIntfDelayCtrl0,
    psram_intf_delay_ctrl1: PsramIntfDelayCtrl1,
    psram_intf_delay_ctrl2: PsramIntfDelayCtrl2,
    psram_intf_delay_ctrl3: PsramIntfDelayCtrl3,
    psram_intf_delay_ctrl4: PsramIntfDelayCtrl4,
    psram_intf_delay_ctrl5: PsramIntfDelayCtrl5,
    psram_intf_delay_ctrl6: PsramIntfDelayCtrl6,
    psram_intf_delay_ctrl7: PsramIntfDelayCtrl7,
    psram_intf_delay_ctrl8: PsramIntfDelayCtrl8,
    psram_intf_delay_ctrl9: PsramIntfDelayCtrl9,
    psram_intf_delay_ctrla: PsramIntfDelayCtrla,
    psram_intf_delay_ctrlb: PsramIntfDelayCtrlb,
    _reserved22: [u8; 0x10],
    psram_dbg_sel: PsramDbgSel,
    _reserved23: [u8; 0x2c],
    psram_dummy_reg: PsramDummyReg,
    psram_timeout_reg: PsramTimeoutReg,
    _reserved25: [u8; 0x08],
    psram_rough_delay_ctrl0: PsramRoughDelayCtrl0,
    psram_rough_delay_ctrl1: PsramRoughDelayCtrl1,
    psram_rough_delay_ctrl2: PsramRoughDelayCtrl2,
    psram_rough_delay_ctrl3: PsramRoughDelayCtrl3,
    psram_rough_delay_ctrl4: PsramRoughDelayCtrl4,
    psram_rough_delay_ctrl5: PsramRoughDelayCtrl5,
    psram_rough_delay_ctrl6: PsramRoughDelayCtrl6,
    psram_rough_delay_ctrl7: PsramRoughDelayCtrl7,
    psram_rough_delay_ctrl8: PsramRoughDelayCtrl8,
    psram_rough_delay_ctrl9: PsramRoughDelayCtrl9,
    psram_rough_delay_ctrla: PsramRoughDelayCtrla,
    psram_rough_delay_ctrlb: PsramRoughDelayCtrlb,
}
impl RegisterBlock {
    #[doc = "0x00 - PSRAM Main Configuration Register"]
    #[inline(always)]
    pub const fn psram_configure(&self) -> &PsramConfigure {
        &self.psram_configure
    }
    #[doc = "0x04 - PSRAM Manual Signal Control"]
    #[inline(always)]
    pub const fn psram_manual_control(&self) -> &PsramManualControl {
        &self.psram_manual_control
    }
    #[doc = "0x08 - FIFO Threshold Control"]
    #[inline(always)]
    pub const fn fifo_thres_control(&self) -> &FifoThresControl {
        &self.fifo_thres_control
    }
    #[doc = "0x0c - PSRAM Manual Control 2"]
    #[inline(always)]
    pub const fn psram_manual_control2(&self) -> &PsramManualControl2 {
        &self.psram_manual_control2
    }
    #[doc = "0x10 - Winbond Specific Configuration"]
    #[inline(always)]
    pub const fn winbond_psram_configure(&self) -> &WinbondPsramConfigure {
        &self.winbond_psram_configure
    }
    #[doc = "0x14 - Winbond Specific Status"]
    #[inline(always)]
    pub const fn winbond_psram_status(&self) -> &WinbondPsramStatus {
        &self.winbond_psram_status
    }
    #[doc = "0x18 - Winbond Specific Configuration 2"]
    #[inline(always)]
    pub const fn winbond_psram_configure2(&self) -> &WinbondPsramConfigure2 {
        &self.winbond_psram_configure2
    }
    #[doc = "0x20 - APMemory Specific Configuration"]
    #[inline(always)]
    pub const fn apmemory_psram_configure(&self) -> &ApmemoryPsramConfigure {
        &self.apmemory_psram_configure
    }
    #[doc = "0x24 - APMemory Specific Status"]
    #[inline(always)]
    pub const fn apmemory_psram_status(&self) -> &ApmemoryPsramStatus {
        &self.apmemory_psram_status
    }
    #[doc = "0x30 - PSRAM Manual Control 3"]
    #[inline(always)]
    pub const fn psram_manual_control3(&self) -> &PsramManualControl3 {
        &self.psram_manual_control3
    }
    #[doc = "0x80 - PSRAM Interface Fine Delay Control Register 0 (DQS_OEN0, CEB, CLK_N, CLK)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrl0(&self) -> &PsramIntfDelayCtrl0 {
        &self.psram_intf_delay_ctrl0
    }
    #[doc = "0x84 - PSRAM Interface Fine Delay Control Register 1 (ADQ1, ADQ0, ADQ_OEN0, DQS0)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrl1(&self) -> &PsramIntfDelayCtrl1 {
        &self.psram_intf_delay_ctrl1
    }
    #[doc = "0x88 - PSRAM Interface Fine Delay Control Register 2 (ADQ5-2)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrl2(&self) -> &PsramIntfDelayCtrl2 {
        &self.psram_intf_delay_ctrl2
    }
    #[doc = "0x8c - PSRAM Interface Fine Delay Control Register 3 (ADQ1-0 In, ADQ7-6 Out)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrl3(&self) -> &PsramIntfDelayCtrl3 {
        &self.psram_intf_delay_ctrl3
    }
    #[doc = "0x90 - PSRAM Interface Fine Delay Control Register 4 (ADQ5-2 In)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrl4(&self) -> &PsramIntfDelayCtrl4 {
        &self.psram_intf_delay_ctrl4
    }
    #[doc = "0x94 - PSRAM Interface Fine Delay Control Register 5 (DQS0 In, ADQ7-6 In)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrl5(&self) -> &PsramIntfDelayCtrl5 {
        &self.psram_intf_delay_ctrl5
    }
    #[doc = "0x98 - PSRAM Interface Fine Delay Control Register 6 (ADQ9-8 Out, ADQ_OEN1, DQS1 Out)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrl6(&self) -> &PsramIntfDelayCtrl6 {
        &self.psram_intf_delay_ctrl6
    }
    #[doc = "0x9c - PSRAM Interface Fine Delay Control Register 7 (ADQ13-10 Out)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrl7(&self) -> &PsramIntfDelayCtrl7 {
        &self.psram_intf_delay_ctrl7
    }
    #[doc = "0xa0 - PSRAM Interface Fine Delay Control Register 8 (ADQ9-8 In, ADQ15-14 Out)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrl8(&self) -> &PsramIntfDelayCtrl8 {
        &self.psram_intf_delay_ctrl8
    }
    #[doc = "0xa4 - PSRAM Interface Fine Delay Control Register 9 (ADQ13-10 In)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrl9(&self) -> &PsramIntfDelayCtrl9 {
        &self.psram_intf_delay_ctrl9
    }
    #[doc = "0xa8 - PSRAM Interface Fine Delay Control Register A (DQS1 In, ADQ15-14 In)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrla(&self) -> &PsramIntfDelayCtrla {
        &self.psram_intf_delay_ctrla
    }
    #[doc = "0xac - PSRAM Interface Fine Delay Control Register B (DQS Mask, DQS_OEN1)"]
    #[inline(always)]
    pub const fn psram_intf_delay_ctrlb(&self) -> &PsramIntfDelayCtrlb {
        &self.psram_intf_delay_ctrlb
    }
    #[doc = "0xc0 - PSRAM Debug Select Register"]
    #[inline(always)]
    pub const fn psram_dbg_sel(&self) -> &PsramDbgSel {
        &self.psram_dbg_sel
    }
    #[doc = "0xf0 - PSRAM Dummy Register"]
    #[inline(always)]
    pub const fn psram_dummy_reg(&self) -> &PsramDummyReg {
        &self.psram_dummy_reg
    }
    #[doc = "0xf4 - Timeout Configuration and Status"]
    #[inline(always)]
    pub const fn psram_timeout_reg(&self) -> &PsramTimeoutReg {
        &self.psram_timeout_reg
    }
    #[doc = "0x100 - PSRAM Rough Delay Control Register 0 (DQS_OEN0, CEB, CLK_N, CLK)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl0(&self) -> &PsramRoughDelayCtrl0 {
        &self.psram_rough_delay_ctrl0
    }
    #[doc = "0x104 - PSRAM Rough Delay Control Register 1 (ADQ1, ADQ0, ADQ_OEN0, DQS0)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl1(&self) -> &PsramRoughDelayCtrl1 {
        &self.psram_rough_delay_ctrl1
    }
    #[doc = "0x108 - PSRAM Rough Delay Control Register 2 (ADQ5-2)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl2(&self) -> &PsramRoughDelayCtrl2 {
        &self.psram_rough_delay_ctrl2
    }
    #[doc = "0x10c - PSRAM Rough Delay Control Register 3 (ADQ1-0 In, ADQ7-6 Out)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl3(&self) -> &PsramRoughDelayCtrl3 {
        &self.psram_rough_delay_ctrl3
    }
    #[doc = "0x110 - PSRAM Rough Delay Control Register 4 (ADQ5-2 In)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl4(&self) -> &PsramRoughDelayCtrl4 {
        &self.psram_rough_delay_ctrl4
    }
    #[doc = "0x114 - PSRAM Rough Delay Control Register 5 (DQS0 In, ADQ7-6 In)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl5(&self) -> &PsramRoughDelayCtrl5 {
        &self.psram_rough_delay_ctrl5
    }
    #[doc = "0x118 - PSRAM Rough Delay Control Register 6 (ADQ9-8 Out, ADQ_OEN1, DQS1 Out)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl6(&self) -> &PsramRoughDelayCtrl6 {
        &self.psram_rough_delay_ctrl6
    }
    #[doc = "0x11c - PSRAM Rough Delay Control Register 7 (ADQ13-10 Out)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl7(&self) -> &PsramRoughDelayCtrl7 {
        &self.psram_rough_delay_ctrl7
    }
    #[doc = "0x120 - PSRAM Rough Delay Control Register 8 (ADQ9-8 In, ADQ15-14 Out)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl8(&self) -> &PsramRoughDelayCtrl8 {
        &self.psram_rough_delay_ctrl8
    }
    #[doc = "0x124 - PSRAM Rough Delay Control Register 9 (ADQ13-10 In)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrl9(&self) -> &PsramRoughDelayCtrl9 {
        &self.psram_rough_delay_ctrl9
    }
    #[doc = "0x128 - PSRAM Rough Delay Control Register A (DQS1 In, ADQ15-14 In)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrla(&self) -> &PsramRoughDelayCtrla {
        &self.psram_rough_delay_ctrla
    }
    #[doc = "0x12c - PSRAM Rough Delay Control Register B (DQS Mask, DQS_OEN1)"]
    #[inline(always)]
    pub const fn psram_rough_delay_ctrlb(&self) -> &PsramRoughDelayCtrlb {
        &self.psram_rough_delay_ctrlb
    }
}
#[doc = "PSRAM_CONFIGURE (rw) register accessor: PSRAM Main Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_configure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_configure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_configure`] module"]
#[doc(alias = "PSRAM_CONFIGURE")]
pub type PsramConfigure = crate::Reg<psram_configure::PsramConfigureSpec>;
#[doc = "PSRAM Main Configuration Register"]
pub mod psram_configure;
#[doc = "PSRAM_MANUAL_CONTROL (rw) register accessor: PSRAM Manual Signal Control\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_manual_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_manual_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_manual_control`] module"]
#[doc(alias = "PSRAM_MANUAL_CONTROL")]
pub type PsramManualControl = crate::Reg<psram_manual_control::PsramManualControlSpec>;
#[doc = "PSRAM Manual Signal Control"]
pub mod psram_manual_control;
#[doc = "FIFO_THRES_CONTROL (rw) register accessor: FIFO Threshold Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_thres_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_thres_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_thres_control`] module"]
#[doc(alias = "FIFO_THRES_CONTROL")]
pub type FifoThresControl = crate::Reg<fifo_thres_control::FifoThresControlSpec>;
#[doc = "FIFO Threshold Control"]
pub mod fifo_thres_control;
#[doc = "PSRAM_MANUAL_CONTROL2 (rw) register accessor: PSRAM Manual Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_manual_control2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_manual_control2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_manual_control2`] module"]
#[doc(alias = "PSRAM_MANUAL_CONTROL2")]
pub type PsramManualControl2 = crate::Reg<psram_manual_control2::PsramManualControl2Spec>;
#[doc = "PSRAM Manual Control 2"]
pub mod psram_manual_control2;
#[doc = "WINBOND_PSRAM_CONFIGURE (rw) register accessor: Winbond Specific Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`winbond_psram_configure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winbond_psram_configure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winbond_psram_configure`] module"]
#[doc(alias = "WINBOND_PSRAM_CONFIGURE")]
pub type WinbondPsramConfigure = crate::Reg<winbond_psram_configure::WinbondPsramConfigureSpec>;
#[doc = "Winbond Specific Configuration"]
pub mod winbond_psram_configure;
#[doc = "WINBOND_PSRAM_STATUS (r) register accessor: Winbond Specific Status\n\nYou can [`read`](crate::Reg::read) this register and get [`winbond_psram_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winbond_psram_status`] module"]
#[doc(alias = "WINBOND_PSRAM_STATUS")]
pub type WinbondPsramStatus = crate::Reg<winbond_psram_status::WinbondPsramStatusSpec>;
#[doc = "Winbond Specific Status"]
pub mod winbond_psram_status;
#[doc = "WINBOND_PSRAM_CONFIGURE2 (rw) register accessor: Winbond Specific Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`winbond_psram_configure2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winbond_psram_configure2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winbond_psram_configure2`] module"]
#[doc(alias = "WINBOND_PSRAM_CONFIGURE2")]
pub type WinbondPsramConfigure2 = crate::Reg<winbond_psram_configure2::WinbondPsramConfigure2Spec>;
#[doc = "Winbond Specific Configuration 2"]
pub mod winbond_psram_configure2;
#[doc = "APMEMORY_PSRAM_CONFIGURE (rw) register accessor: APMemory Specific Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`apmemory_psram_configure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apmemory_psram_configure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apmemory_psram_configure`] module"]
#[doc(alias = "APMEMORY_PSRAM_CONFIGURE")]
pub type ApmemoryPsramConfigure = crate::Reg<apmemory_psram_configure::ApmemoryPsramConfigureSpec>;
#[doc = "APMemory Specific Configuration"]
pub mod apmemory_psram_configure;
#[doc = "APMEMORY_PSRAM_STATUS (r) register accessor: APMemory Specific Status\n\nYou can [`read`](crate::Reg::read) this register and get [`apmemory_psram_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apmemory_psram_status`] module"]
#[doc(alias = "APMEMORY_PSRAM_STATUS")]
pub type ApmemoryPsramStatus = crate::Reg<apmemory_psram_status::ApmemoryPsramStatusSpec>;
#[doc = "APMemory Specific Status"]
pub mod apmemory_psram_status;
#[doc = "PSRAM_MANUAL_CONTROL3 (rw) register accessor: PSRAM Manual Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_manual_control3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_manual_control3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_manual_control3`] module"]
#[doc(alias = "PSRAM_MANUAL_CONTROL3")]
pub type PsramManualControl3 = crate::Reg<psram_manual_control3::PsramManualControl3Spec>;
#[doc = "PSRAM Manual Control 3"]
pub mod psram_manual_control3;
#[doc = "PSRAM_INTF_DELAY_CTRL0 (rw) register accessor: PSRAM Interface Fine Delay Control Register 0 (DQS_OEN0, CEB, CLK_N, CLK)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrl0`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRL0")]
pub type PsramIntfDelayCtrl0 = crate::Reg<psram_intf_delay_ctrl0::PsramIntfDelayCtrl0Spec>;
#[doc = "PSRAM Interface Fine Delay Control Register 0 (DQS_OEN0, CEB, CLK_N, CLK)"]
pub mod psram_intf_delay_ctrl0;
#[doc = "PSRAM_INTF_DELAY_CTRL1 (rw) register accessor: PSRAM Interface Fine Delay Control Register 1 (ADQ1, ADQ0, ADQ_OEN0, DQS0)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrl1`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRL1")]
pub type PsramIntfDelayCtrl1 = crate::Reg<psram_intf_delay_ctrl1::PsramIntfDelayCtrl1Spec>;
#[doc = "PSRAM Interface Fine Delay Control Register 1 (ADQ1, ADQ0, ADQ_OEN0, DQS0)"]
pub mod psram_intf_delay_ctrl1;
#[doc = "PSRAM_INTF_DELAY_CTRL2 (rw) register accessor: PSRAM Interface Fine Delay Control Register 2 (ADQ5-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrl2`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRL2")]
pub type PsramIntfDelayCtrl2 = crate::Reg<psram_intf_delay_ctrl2::PsramIntfDelayCtrl2Spec>;
#[doc = "PSRAM Interface Fine Delay Control Register 2 (ADQ5-2)"]
pub mod psram_intf_delay_ctrl2;
#[doc = "PSRAM_INTF_DELAY_CTRL3 (rw) register accessor: PSRAM Interface Fine Delay Control Register 3 (ADQ1-0 In, ADQ7-6 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrl3`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRL3")]
pub type PsramIntfDelayCtrl3 = crate::Reg<psram_intf_delay_ctrl3::PsramIntfDelayCtrl3Spec>;
#[doc = "PSRAM Interface Fine Delay Control Register 3 (ADQ1-0 In, ADQ7-6 Out)"]
pub mod psram_intf_delay_ctrl3;
#[doc = "PSRAM_INTF_DELAY_CTRL4 (rw) register accessor: PSRAM Interface Fine Delay Control Register 4 (ADQ5-2 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrl4`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRL4")]
pub type PsramIntfDelayCtrl4 = crate::Reg<psram_intf_delay_ctrl4::PsramIntfDelayCtrl4Spec>;
#[doc = "PSRAM Interface Fine Delay Control Register 4 (ADQ5-2 In)"]
pub mod psram_intf_delay_ctrl4;
#[doc = "PSRAM_INTF_DELAY_CTRL5 (rw) register accessor: PSRAM Interface Fine Delay Control Register 5 (DQS0 In, ADQ7-6 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrl5`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRL5")]
pub type PsramIntfDelayCtrl5 = crate::Reg<psram_intf_delay_ctrl5::PsramIntfDelayCtrl5Spec>;
#[doc = "PSRAM Interface Fine Delay Control Register 5 (DQS0 In, ADQ7-6 In)"]
pub mod psram_intf_delay_ctrl5;
#[doc = "PSRAM_INTF_DELAY_CTRL6 (rw) register accessor: PSRAM Interface Fine Delay Control Register 6 (ADQ9-8 Out, ADQ_OEN1, DQS1 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrl6`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRL6")]
pub type PsramIntfDelayCtrl6 = crate::Reg<psram_intf_delay_ctrl6::PsramIntfDelayCtrl6Spec>;
#[doc = "PSRAM Interface Fine Delay Control Register 6 (ADQ9-8 Out, ADQ_OEN1, DQS1 Out)"]
pub mod psram_intf_delay_ctrl6;
#[doc = "PSRAM_INTF_DELAY_CTRL7 (rw) register accessor: PSRAM Interface Fine Delay Control Register 7 (ADQ13-10 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrl7`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRL7")]
pub type PsramIntfDelayCtrl7 = crate::Reg<psram_intf_delay_ctrl7::PsramIntfDelayCtrl7Spec>;
#[doc = "PSRAM Interface Fine Delay Control Register 7 (ADQ13-10 Out)"]
pub mod psram_intf_delay_ctrl7;
#[doc = "PSRAM_INTF_DELAY_CTRL8 (rw) register accessor: PSRAM Interface Fine Delay Control Register 8 (ADQ9-8 In, ADQ15-14 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrl8`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRL8")]
pub type PsramIntfDelayCtrl8 = crate::Reg<psram_intf_delay_ctrl8::PsramIntfDelayCtrl8Spec>;
#[doc = "PSRAM Interface Fine Delay Control Register 8 (ADQ9-8 In, ADQ15-14 Out)"]
pub mod psram_intf_delay_ctrl8;
#[doc = "PSRAM_INTF_DELAY_CTRL9 (rw) register accessor: PSRAM Interface Fine Delay Control Register 9 (ADQ13-10 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrl9`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRL9")]
pub type PsramIntfDelayCtrl9 = crate::Reg<psram_intf_delay_ctrl9::PsramIntfDelayCtrl9Spec>;
#[doc = "PSRAM Interface Fine Delay Control Register 9 (ADQ13-10 In)"]
pub mod psram_intf_delay_ctrl9;
#[doc = "PSRAM_INTF_DELAY_CTRLA (rw) register accessor: PSRAM Interface Fine Delay Control Register A (DQS1 In, ADQ15-14 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrla`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRLA")]
pub type PsramIntfDelayCtrla = crate::Reg<psram_intf_delay_ctrla::PsramIntfDelayCtrlaSpec>;
#[doc = "PSRAM Interface Fine Delay Control Register A (DQS1 In, ADQ15-14 In)"]
pub mod psram_intf_delay_ctrla;
#[doc = "PSRAM_INTF_DELAY_CTRLB (rw) register accessor: PSRAM Interface Fine Delay Control Register B (DQS Mask, DQS_OEN1)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_intf_delay_ctrlb`] module"]
#[doc(alias = "PSRAM_INTF_DELAY_CTRLB")]
pub type PsramIntfDelayCtrlb = crate::Reg<psram_intf_delay_ctrlb::PsramIntfDelayCtrlbSpec>;
#[doc = "PSRAM Interface Fine Delay Control Register B (DQS Mask, DQS_OEN1)"]
pub mod psram_intf_delay_ctrlb;
#[doc = "PSRAM_DBG_SEL (rw) register accessor: PSRAM Debug Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dbg_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dbg_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_dbg_sel`] module"]
#[doc(alias = "PSRAM_DBG_SEL")]
pub type PsramDbgSel = crate::Reg<psram_dbg_sel::PsramDbgSelSpec>;
#[doc = "PSRAM Debug Select Register"]
pub mod psram_dbg_sel;
#[doc = "PSRAM_DUMMY_REG (rw) register accessor: PSRAM Dummy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dummy_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dummy_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_dummy_reg`] module"]
#[doc(alias = "PSRAM_DUMMY_REG")]
pub type PsramDummyReg = crate::Reg<psram_dummy_reg::PsramDummyRegSpec>;
#[doc = "PSRAM Dummy Register"]
pub mod psram_dummy_reg;
#[doc = "PSRAM_TIMEOUT_REG (rw) register accessor: Timeout Configuration and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_timeout_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_timeout_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_timeout_reg`] module"]
#[doc(alias = "PSRAM_TIMEOUT_REG")]
pub type PsramTimeoutReg = crate::Reg<psram_timeout_reg::PsramTimeoutRegSpec>;
#[doc = "Timeout Configuration and Status"]
pub mod psram_timeout_reg;
#[doc = "PSRAM_ROUGH_DELAY_CTRL0 (rw) register accessor: PSRAM Rough Delay Control Register 0 (DQS_OEN0, CEB, CLK_N, CLK)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrl0`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRL0")]
pub type PsramRoughDelayCtrl0 = crate::Reg<psram_rough_delay_ctrl0::PsramRoughDelayCtrl0Spec>;
#[doc = "PSRAM Rough Delay Control Register 0 (DQS_OEN0, CEB, CLK_N, CLK)"]
pub mod psram_rough_delay_ctrl0;
#[doc = "PSRAM_ROUGH_DELAY_CTRL1 (rw) register accessor: PSRAM Rough Delay Control Register 1 (ADQ1, ADQ0, ADQ_OEN0, DQS0)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrl1`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRL1")]
pub type PsramRoughDelayCtrl1 = crate::Reg<psram_rough_delay_ctrl1::PsramRoughDelayCtrl1Spec>;
#[doc = "PSRAM Rough Delay Control Register 1 (ADQ1, ADQ0, ADQ_OEN0, DQS0)"]
pub mod psram_rough_delay_ctrl1;
#[doc = "PSRAM_ROUGH_DELAY_CTRL2 (rw) register accessor: PSRAM Rough Delay Control Register 2 (ADQ5-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrl2`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRL2")]
pub type PsramRoughDelayCtrl2 = crate::Reg<psram_rough_delay_ctrl2::PsramRoughDelayCtrl2Spec>;
#[doc = "PSRAM Rough Delay Control Register 2 (ADQ5-2)"]
pub mod psram_rough_delay_ctrl2;
#[doc = "PSRAM_ROUGH_DELAY_CTRL3 (rw) register accessor: PSRAM Rough Delay Control Register 3 (ADQ1-0 In, ADQ7-6 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrl3`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRL3")]
pub type PsramRoughDelayCtrl3 = crate::Reg<psram_rough_delay_ctrl3::PsramRoughDelayCtrl3Spec>;
#[doc = "PSRAM Rough Delay Control Register 3 (ADQ1-0 In, ADQ7-6 Out)"]
pub mod psram_rough_delay_ctrl3;
#[doc = "PSRAM_ROUGH_DELAY_CTRL4 (rw) register accessor: PSRAM Rough Delay Control Register 4 (ADQ5-2 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrl4`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRL4")]
pub type PsramRoughDelayCtrl4 = crate::Reg<psram_rough_delay_ctrl4::PsramRoughDelayCtrl4Spec>;
#[doc = "PSRAM Rough Delay Control Register 4 (ADQ5-2 In)"]
pub mod psram_rough_delay_ctrl4;
#[doc = "PSRAM_ROUGH_DELAY_CTRL5 (rw) register accessor: PSRAM Rough Delay Control Register 5 (DQS0 In, ADQ7-6 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrl5`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRL5")]
pub type PsramRoughDelayCtrl5 = crate::Reg<psram_rough_delay_ctrl5::PsramRoughDelayCtrl5Spec>;
#[doc = "PSRAM Rough Delay Control Register 5 (DQS0 In, ADQ7-6 In)"]
pub mod psram_rough_delay_ctrl5;
#[doc = "PSRAM_ROUGH_DELAY_CTRL6 (rw) register accessor: PSRAM Rough Delay Control Register 6 (ADQ9-8 Out, ADQ_OEN1, DQS1 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrl6`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRL6")]
pub type PsramRoughDelayCtrl6 = crate::Reg<psram_rough_delay_ctrl6::PsramRoughDelayCtrl6Spec>;
#[doc = "PSRAM Rough Delay Control Register 6 (ADQ9-8 Out, ADQ_OEN1, DQS1 Out)"]
pub mod psram_rough_delay_ctrl6;
#[doc = "PSRAM_ROUGH_DELAY_CTRL7 (rw) register accessor: PSRAM Rough Delay Control Register 7 (ADQ13-10 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrl7`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRL7")]
pub type PsramRoughDelayCtrl7 = crate::Reg<psram_rough_delay_ctrl7::PsramRoughDelayCtrl7Spec>;
#[doc = "PSRAM Rough Delay Control Register 7 (ADQ13-10 Out)"]
pub mod psram_rough_delay_ctrl7;
#[doc = "PSRAM_ROUGH_DELAY_CTRL8 (rw) register accessor: PSRAM Rough Delay Control Register 8 (ADQ9-8 In, ADQ15-14 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrl8`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRL8")]
pub type PsramRoughDelayCtrl8 = crate::Reg<psram_rough_delay_ctrl8::PsramRoughDelayCtrl8Spec>;
#[doc = "PSRAM Rough Delay Control Register 8 (ADQ9-8 In, ADQ15-14 Out)"]
pub mod psram_rough_delay_ctrl8;
#[doc = "PSRAM_ROUGH_DELAY_CTRL9 (rw) register accessor: PSRAM Rough Delay Control Register 9 (ADQ13-10 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrl9`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRL9")]
pub type PsramRoughDelayCtrl9 = crate::Reg<psram_rough_delay_ctrl9::PsramRoughDelayCtrl9Spec>;
#[doc = "PSRAM Rough Delay Control Register 9 (ADQ13-10 In)"]
pub mod psram_rough_delay_ctrl9;
#[doc = "PSRAM_ROUGH_DELAY_CTRLA (rw) register accessor: PSRAM Rough Delay Control Register A (DQS1 In, ADQ15-14 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrla`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRLA")]
pub type PsramRoughDelayCtrla = crate::Reg<psram_rough_delay_ctrla::PsramRoughDelayCtrlaSpec>;
#[doc = "PSRAM Rough Delay Control Register A (DQS1 In, ADQ15-14 In)"]
pub mod psram_rough_delay_ctrla;
#[doc = "PSRAM_ROUGH_DELAY_CTRLB (rw) register accessor: PSRAM Rough Delay Control Register B (DQS Mask, DQS_OEN1)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psram_rough_delay_ctrlb`] module"]
#[doc(alias = "PSRAM_ROUGH_DELAY_CTRLB")]
pub type PsramRoughDelayCtrlb = crate::Reg<psram_rough_delay_ctrlb::PsramRoughDelayCtrlbSpec>;
#[doc = "PSRAM Rough Delay Control Register B (DQS Mask, DQS_OEN1)"]
pub mod psram_rough_delay_ctrlb;
