#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hbn_ctl: HbnCtl,
    hbn_time_l: HbnTimeL,
    hbn_time_h: HbnTimeH,
    rtc_time_l: RtcTimeL,
    rtc_time_h: RtcTimeH,
    hbn_irq_mode: HbnIrqMode,
    hbn_irq_stat: HbnIrqStat,
    hbn_irq_clr: HbnIrqClr,
    hbn_pir_cfg: HbnPirCfg,
    hbn_pir_vth: HbnPirVth,
    hbn_pir_interval: HbnPirInterval,
    hbn_bor_cfg: HbnBorCfg,
    hbn_glb: HbnGlb,
    hbn_sram: HbnSram,
    hbn_pad_ctrl_0: HbnPadCtrl0,
    hbn_pad_ctrl_1: HbnPadCtrl1,
    _reserved16: [u8; 0xc0],
    hbn_rsv0: HbnRsv0,
    hbn_rsv1: HbnRsv1,
    hbn_rsv2: HbnRsv2,
    hbn_rsv3: HbnRsv3,
    _reserved20: [u8; 0xf0],
    rc32k_ctrl0: Rc32kCtrl0,
    xtal32k: Xtal32k,
    rtc_rst_ctrl: RtcRstCtrl,
    rtc_rst_ctrl2: RtcRstCtrl2,
}
impl RegisterBlock {
    #[doc = "0x00 - HBN Main Control Register"]
    #[inline(always)]
    pub const fn hbn_ctl(&self) -> &HbnCtl {
        &self.hbn_ctl
    }
    #[doc = "0x04 - Hibernate Timer Low"]
    #[inline(always)]
    pub const fn hbn_time_l(&self) -> &HbnTimeL {
        &self.hbn_time_l
    }
    #[doc = "0x08 - Hibernate Timer High"]
    #[inline(always)]
    pub const fn hbn_time_h(&self) -> &HbnTimeH {
        &self.hbn_time_h
    }
    #[doc = "0x0c - RTC Latch Low"]
    #[inline(always)]
    pub const fn rtc_time_l(&self) -> &RtcTimeL {
        &self.rtc_time_l
    }
    #[doc = "0x10 - RTC Latch High and Trigger"]
    #[inline(always)]
    pub const fn rtc_time_h(&self) -> &RtcTimeH {
        &self.rtc_time_h
    }
    #[doc = "0x14 - HBN Interrupt Configuration"]
    #[inline(always)]
    pub const fn hbn_irq_mode(&self) -> &HbnIrqMode {
        &self.hbn_irq_mode
    }
    #[doc = "0x18 - HBN Interrupt Status"]
    #[inline(always)]
    pub const fn hbn_irq_stat(&self) -> &HbnIrqStat {
        &self.hbn_irq_stat
    }
    #[doc = "0x1c - HBN Interrupt Clear"]
    #[inline(always)]
    pub const fn hbn_irq_clr(&self) -> &HbnIrqClr {
        &self.hbn_irq_clr
    }
    #[doc = "0x20 - PIR Sensor Configuration"]
    #[inline(always)]
    pub const fn hbn_pir_cfg(&self) -> &HbnPirCfg {
        &self.hbn_pir_cfg
    }
    #[doc = "0x24 - PIR Sensor Voltage Threshold"]
    #[inline(always)]
    pub const fn hbn_pir_vth(&self) -> &HbnPirVth {
        &self.hbn_pir_vth
    }
    #[doc = "0x28 - PIR Check Interval"]
    #[inline(always)]
    pub const fn hbn_pir_interval(&self) -> &HbnPirInterval {
        &self.hbn_pir_interval
    }
    #[doc = "0x2c - Brown-Out Reset Configuration"]
    #[inline(always)]
    pub const fn hbn_bor_cfg(&self) -> &HbnBorCfg {
        &self.hbn_bor_cfg
    }
    #[doc = "0x30 - Clock and LDO Configuration"]
    #[inline(always)]
    pub const fn hbn_glb(&self) -> &HbnGlb {
        &self.hbn_glb
    }
    #[doc = "0x34 - SRAM Retention Control"]
    #[inline(always)]
    pub const fn hbn_sram(&self) -> &HbnSram {
        &self.hbn_sram
    }
    #[doc = "0x38 - AON GPIO Pad Configuration 0"]
    #[inline(always)]
    pub const fn hbn_pad_ctrl_0(&self) -> &HbnPadCtrl0 {
        &self.hbn_pad_ctrl_0
    }
    #[doc = "0x3c - AON GPIO Pad Configuration 1"]
    #[inline(always)]
    pub const fn hbn_pad_ctrl_1(&self) -> &HbnPadCtrl1 {
        &self.hbn_pad_ctrl_1
    }
    #[doc = "0x100 - HBN Reserved Register 0"]
    #[inline(always)]
    pub const fn hbn_rsv0(&self) -> &HbnRsv0 {
        &self.hbn_rsv0
    }
    #[doc = "0x104 - HBN Reserved Register 1"]
    #[inline(always)]
    pub const fn hbn_rsv1(&self) -> &HbnRsv1 {
        &self.hbn_rsv1
    }
    #[doc = "0x108 - HBN Reserved Register 2"]
    #[inline(always)]
    pub const fn hbn_rsv2(&self) -> &HbnRsv2 {
        &self.hbn_rsv2
    }
    #[doc = "0x10c - HBN Reserved Register 3"]
    #[inline(always)]
    pub const fn hbn_rsv3(&self) -> &HbnRsv3 {
        &self.hbn_rsv3
    }
    #[doc = "0x200 - RC32K Internal Oscillator Control"]
    #[inline(always)]
    pub const fn rc32k_ctrl0(&self) -> &Rc32kCtrl0 {
        &self.rc32k_ctrl0
    }
    #[doc = "0x204 - XTAL32K External Oscillator Control"]
    #[inline(always)]
    pub const fn xtal32k(&self) -> &Xtal32k {
        &self.xtal32k
    }
    #[doc = "0x208 - RTC Reset Configuration"]
    #[inline(always)]
    pub const fn rtc_rst_ctrl(&self) -> &RtcRstCtrl {
        &self.rtc_rst_ctrl
    }
    #[doc = "0x20c - RTC Reset Configuration 2"]
    #[inline(always)]
    pub const fn rtc_rst_ctrl2(&self) -> &RtcRstCtrl2 {
        &self.rtc_rst_ctrl2
    }
}
#[doc = "HBN_CTL (rw) register accessor: HBN Main Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_ctl`] module"]
#[doc(alias = "HBN_CTL")]
pub type HbnCtl = crate::Reg<hbn_ctl::HbnCtlSpec>;
#[doc = "HBN Main Control Register"]
pub mod hbn_ctl;
#[doc = "HBN_TIME_L (rw) register accessor: Hibernate Timer Low\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_time_l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_time_l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_time_l`] module"]
#[doc(alias = "HBN_TIME_L")]
pub type HbnTimeL = crate::Reg<hbn_time_l::HbnTimeLSpec>;
#[doc = "Hibernate Timer Low"]
pub mod hbn_time_l;
#[doc = "HBN_TIME_H (rw) register accessor: Hibernate Timer High\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_time_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_time_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_time_h`] module"]
#[doc(alias = "HBN_TIME_H")]
pub type HbnTimeH = crate::Reg<hbn_time_h::HbnTimeHSpec>;
#[doc = "Hibernate Timer High"]
pub mod hbn_time_h;
#[doc = "RTC_TIME_L (r) register accessor: RTC Latch Low\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_time_l::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_time_l`] module"]
#[doc(alias = "RTC_TIME_L")]
pub type RtcTimeL = crate::Reg<rtc_time_l::RtcTimeLSpec>;
#[doc = "RTC Latch Low"]
pub mod rtc_time_l;
#[doc = "RTC_TIME_H (rw) register accessor: RTC Latch High and Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_time_h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_time_h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_time_h`] module"]
#[doc(alias = "RTC_TIME_H")]
pub type RtcTimeH = crate::Reg<rtc_time_h::RtcTimeHSpec>;
#[doc = "RTC Latch High and Trigger"]
pub mod rtc_time_h;
#[doc = "HBN_IRQ_MODE (rw) register accessor: HBN Interrupt Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_irq_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_irq_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_irq_mode`] module"]
#[doc(alias = "HBN_IRQ_MODE")]
pub type HbnIrqMode = crate::Reg<hbn_irq_mode::HbnIrqModeSpec>;
#[doc = "HBN Interrupt Configuration"]
pub mod hbn_irq_mode;
#[doc = "HBN_IRQ_STAT (r) register accessor: HBN Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_irq_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_irq_stat`] module"]
#[doc(alias = "HBN_IRQ_STAT")]
pub type HbnIrqStat = crate::Reg<hbn_irq_stat::HbnIrqStatSpec>;
#[doc = "HBN Interrupt Status"]
pub mod hbn_irq_stat;
#[doc = "HBN_IRQ_CLR (w) register accessor: HBN Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_irq_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_irq_clr`] module"]
#[doc(alias = "HBN_IRQ_CLR")]
pub type HbnIrqClr = crate::Reg<hbn_irq_clr::HbnIrqClrSpec>;
#[doc = "HBN Interrupt Clear"]
pub mod hbn_irq_clr;
#[doc = "HBN_PIR_CFG (rw) register accessor: PIR Sensor Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_pir_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_pir_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_pir_cfg`] module"]
#[doc(alias = "HBN_PIR_CFG")]
pub type HbnPirCfg = crate::Reg<hbn_pir_cfg::HbnPirCfgSpec>;
#[doc = "PIR Sensor Configuration"]
pub mod hbn_pir_cfg;
#[doc = "HBN_PIR_VTH (rw) register accessor: PIR Sensor Voltage Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_pir_vth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_pir_vth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_pir_vth`] module"]
#[doc(alias = "HBN_PIR_VTH")]
pub type HbnPirVth = crate::Reg<hbn_pir_vth::HbnPirVthSpec>;
#[doc = "PIR Sensor Voltage Threshold"]
pub mod hbn_pir_vth;
#[doc = "HBN_PIR_INTERVAL (rw) register accessor: PIR Check Interval\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_pir_interval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_pir_interval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_pir_interval`] module"]
#[doc(alias = "HBN_PIR_INTERVAL")]
pub type HbnPirInterval = crate::Reg<hbn_pir_interval::HbnPirIntervalSpec>;
#[doc = "PIR Check Interval"]
pub mod hbn_pir_interval;
#[doc = "HBN_BOR_CFG (rw) register accessor: Brown-Out Reset Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_bor_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_bor_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_bor_cfg`] module"]
#[doc(alias = "HBN_BOR_CFG")]
pub type HbnBorCfg = crate::Reg<hbn_bor_cfg::HbnBorCfgSpec>;
#[doc = "Brown-Out Reset Configuration"]
pub mod hbn_bor_cfg;
#[doc = "HBN_GLB (rw) register accessor: Clock and LDO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_glb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_glb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_glb`] module"]
#[doc(alias = "HBN_GLB")]
pub type HbnGlb = crate::Reg<hbn_glb::HbnGlbSpec>;
#[doc = "Clock and LDO Configuration"]
pub mod hbn_glb;
#[doc = "HBN_SRAM (rw) register accessor: SRAM Retention Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_sram::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_sram::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_sram`] module"]
#[doc(alias = "HBN_SRAM")]
pub type HbnSram = crate::Reg<hbn_sram::HbnSramSpec>;
#[doc = "SRAM Retention Control"]
pub mod hbn_sram;
#[doc = "HBN_PAD_CTRL_0 (rw) register accessor: AON GPIO Pad Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_pad_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_pad_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_pad_ctrl_0`] module"]
#[doc(alias = "HBN_PAD_CTRL_0")]
pub type HbnPadCtrl0 = crate::Reg<hbn_pad_ctrl_0::HbnPadCtrl0Spec>;
#[doc = "AON GPIO Pad Configuration 0"]
pub mod hbn_pad_ctrl_0;
#[doc = "HBN_PAD_CTRL_1 (rw) register accessor: AON GPIO Pad Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_pad_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_pad_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_pad_ctrl_1`] module"]
#[doc(alias = "HBN_PAD_CTRL_1")]
pub type HbnPadCtrl1 = crate::Reg<hbn_pad_ctrl_1::HbnPadCtrl1Spec>;
#[doc = "AON GPIO Pad Configuration 1"]
pub mod hbn_pad_ctrl_1;
#[doc = "HBN_RSV0 (rw) register accessor: HBN Reserved Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_rsv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_rsv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_rsv0`] module"]
#[doc(alias = "HBN_RSV0")]
pub type HbnRsv0 = crate::Reg<hbn_rsv0::HbnRsv0Spec>;
#[doc = "HBN Reserved Register 0"]
pub mod hbn_rsv0;
#[doc = "HBN_RSV1 (rw) register accessor: HBN Reserved Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_rsv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_rsv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_rsv1`] module"]
#[doc(alias = "HBN_RSV1")]
pub type HbnRsv1 = crate::Reg<hbn_rsv1::HbnRsv1Spec>;
#[doc = "HBN Reserved Register 1"]
pub mod hbn_rsv1;
#[doc = "HBN_RSV2 (rw) register accessor: HBN Reserved Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_rsv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_rsv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_rsv2`] module"]
#[doc(alias = "HBN_RSV2")]
pub type HbnRsv2 = crate::Reg<hbn_rsv2::HbnRsv2Spec>;
#[doc = "HBN Reserved Register 2"]
pub mod hbn_rsv2;
#[doc = "HBN_RSV3 (rw) register accessor: HBN Reserved Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_rsv3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_rsv3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_rsv3`] module"]
#[doc(alias = "HBN_RSV3")]
pub type HbnRsv3 = crate::Reg<hbn_rsv3::HbnRsv3Spec>;
#[doc = "HBN Reserved Register 3"]
pub mod hbn_rsv3;
#[doc = "RC32K_CTRL0 (rw) register accessor: RC32K Internal Oscillator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32k_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32k_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32k_ctrl0`] module"]
#[doc(alias = "RC32K_CTRL0")]
pub type Rc32kCtrl0 = crate::Reg<rc32k_ctrl0::Rc32kCtrl0Spec>;
#[doc = "RC32K Internal Oscillator Control"]
pub mod rc32k_ctrl0;
#[doc = "XTAL32K (rw) register accessor: XTAL32K External Oscillator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32k::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32k::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32k`] module"]
#[doc(alias = "XTAL32K")]
pub type Xtal32k = crate::Reg<xtal32k::Xtal32kSpec>;
#[doc = "XTAL32K External Oscillator Control"]
pub mod xtal32k;
#[doc = "RTC_RST_CTRL (rw) register accessor: RTC Reset Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_rst_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_rst_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_rst_ctrl`] module"]
#[doc(alias = "RTC_RST_CTRL")]
pub type RtcRstCtrl = crate::Reg<rtc_rst_ctrl::RtcRstCtrlSpec>;
#[doc = "RTC Reset Configuration"]
pub mod rtc_rst_ctrl;
#[doc = "RTC_RST_CTRL2 (rw) register accessor: RTC Reset Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_rst_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_rst_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_rst_ctrl2`] module"]
#[doc(alias = "RTC_RST_CTRL2")]
pub type RtcRstCtrl2 = crate::Reg<rtc_rst_ctrl2::RtcRstCtrl2Spec>;
#[doc = "RTC Reset Configuration 2"]
pub mod rtc_rst_ctrl2;
