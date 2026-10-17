#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aon: Aon,
    aon_common: AonCommon,
    aon_misc: AonMisc,
    _reserved3: [u8; 0x04],
    bg_sys_top: BgSysTop,
    dcdc_top_0: DcdcTop0,
    dcdc_top_1: DcdcTop1,
    ldo11soc_and_dctest: Ldo11socAndDctest,
    dcdc18_top_0: Dcdc18Top0,
    dcdc18_top_1: Dcdc18Top1,
    dcdc18_top_2: Dcdc18Top2,
    psw_irrcv: PswIrrcv,
    _reserved11: [u8; 0x50],
    rf_top_aon: RfTopAon,
    xtal_cfg: XtalCfg,
    xtal_cfg2: XtalCfg2,
    xtal_cfg3: XtalCfg3,
    tsen: Tsen,
    _reserved16: [u8; 0x30],
    ldo18io: Ldo18io,
    _reserved17: [u8; 0x38],
    acomp0_ctrl: Acomp0Ctrl,
    acomp1_ctrl: Acomp1Ctrl,
    acomp_ctrl: AcompCtrl,
    gpadc_reg_cmd: GpadcRegCmd,
    gpadc_reg_config1: GpadcRegConfig1,
    gpadc_reg_config2: GpadcRegConfig2,
    gpadc_reg_scn_pos1: GpadcRegScnPos1,
    gpadc_reg_scn_pos2: GpadcRegScnPos2,
    gpadc_reg_scn_neg1: GpadcRegScnNeg1,
    gpadc_reg_scn_neg2: GpadcRegScnNeg2,
    gpadc_reg_status: GpadcRegStatus,
    gpadc_reg_isr: GpadcRegIsr,
    gpadc_reg_result: GpadcRegResult,
    gpadc_reg_raw_result: GpadcRegRawResult,
    gpadc_reg_define: GpadcRegDefine,
    hbncore_resv0: HbncoreResv0,
    hbncore_resv1: HbncoreResv1,
}
impl RegisterBlock {
    #[doc = "0x00 - AON Register"]
    #[inline(always)]
    pub const fn aon(&self) -> &Aon {
        &self.aon
    }
    #[doc = "0x04 - AON Common Controls"]
    #[inline(always)]
    pub const fn aon_common(&self) -> &AonCommon {
        &self.aon_common
    }
    #[doc = "0x08 - AON Miscellaneous Controls"]
    #[inline(always)]
    pub const fn aon_misc(&self) -> &AonMisc {
        &self.aon_misc
    }
    #[doc = "0x10 - Bandgap System Top Control"]
    #[inline(always)]
    pub const fn bg_sys_top(&self) -> &BgSysTop {
        &self.bg_sys_top
    }
    #[doc = "0x14 - DCDC11 (1.1V) Configuration 0"]
    #[inline(always)]
    pub const fn dcdc_top_0(&self) -> &DcdcTop0 {
        &self.dcdc_top_0
    }
    #[doc = "0x18 - DCDC11 (1.1V) Configuration 1"]
    #[inline(always)]
    pub const fn dcdc_top_1(&self) -> &DcdcTop1 {
        &self.dcdc_top_1
    }
    #[doc = "0x1c - LDO11SOC and DCDC Test Configuration"]
    #[inline(always)]
    pub const fn ldo11soc_and_dctest(&self) -> &Ldo11socAndDctest {
        &self.ldo11soc_and_dctest
    }
    #[doc = "0x20 - DCDC18 (1.8V) Configuration 0"]
    #[inline(always)]
    pub const fn dcdc18_top_0(&self) -> &Dcdc18Top0 {
        &self.dcdc18_top_0
    }
    #[doc = "0x24 - DCDC18 (1.8V) Configuration 1"]
    #[inline(always)]
    pub const fn dcdc18_top_1(&self) -> &Dcdc18Top1 {
        &self.dcdc18_top_1
    }
    #[doc = "0x28 - DCDC18 (1.8V) Configuration 2"]
    #[inline(always)]
    pub const fn dcdc18_top_2(&self) -> &Dcdc18Top2 {
        &self.dcdc18_top_2
    }
    #[doc = "0x2c - PSW and USB Reference"]
    #[inline(always)]
    pub const fn psw_irrcv(&self) -> &PswIrrcv {
        &self.psw_irrcv
    }
    #[doc = "0x80 - RF LDO15 and XTAL Top Control"]
    #[inline(always)]
    pub const fn rf_top_aon(&self) -> &RfTopAon {
        &self.rf_top_aon
    }
    #[doc = "0x84 - Crystal Oscillator Configuration"]
    #[inline(always)]
    pub const fn xtal_cfg(&self) -> &XtalCfg {
        &self.xtal_cfg
    }
    #[doc = "0x88 - Crystal Oscillator Configuration 2"]
    #[inline(always)]
    pub const fn xtal_cfg2(&self) -> &XtalCfg2 {
        &self.xtal_cfg2
    }
    #[doc = "0x8c - Crystal Oscillator Configuration 3"]
    #[inline(always)]
    pub const fn xtal_cfg3(&self) -> &XtalCfg3 {
        &self.xtal_cfg3
    }
    #[doc = "0x90 - Temperature Sensor and XTAL Ready"]
    #[inline(always)]
    pub const fn tsen(&self) -> &Tsen {
        &self.tsen
    }
    #[doc = "0xc4 - LDO18 IO Control"]
    #[inline(always)]
    pub const fn ldo18io(&self) -> &Ldo18io {
        &self.ldo18io
    }
    #[doc = "0x100 - Analog Comparator 0 Control"]
    #[inline(always)]
    pub const fn acomp0_ctrl(&self) -> &Acomp0Ctrl {
        &self.acomp0_ctrl
    }
    #[doc = "0x104 - Analog Comparator 1 Control"]
    #[inline(always)]
    pub const fn acomp1_ctrl(&self) -> &Acomp1Ctrl {
        &self.acomp1_ctrl
    }
    #[doc = "0x108 - Analog Comparator Shared Control"]
    #[inline(always)]
    pub const fn acomp_ctrl(&self) -> &AcompCtrl {
        &self.acomp_ctrl
    }
    #[doc = "0x10c - GPADC Command Register"]
    #[inline(always)]
    pub const fn gpadc_reg_cmd(&self) -> &GpadcRegCmd {
        &self.gpadc_reg_cmd
    }
    #[doc = "0x110 - GPADC Configuration Register 1"]
    #[inline(always)]
    pub const fn gpadc_reg_config1(&self) -> &GpadcRegConfig1 {
        &self.gpadc_reg_config1
    }
    #[doc = "0x114 - GPADC Configuration Register 2"]
    #[inline(always)]
    pub const fn gpadc_reg_config2(&self) -> &GpadcRegConfig2 {
        &self.gpadc_reg_config2
    }
    #[doc = "0x118 - GPADC Conversion Sequence 1 (Positive Channels 0-5)"]
    #[inline(always)]
    pub const fn gpadc_reg_scn_pos1(&self) -> &GpadcRegScnPos1 {
        &self.gpadc_reg_scn_pos1
    }
    #[doc = "0x11c - GPADC Conversion Sequence 2 (Positive Channels 6-11)"]
    #[inline(always)]
    pub const fn gpadc_reg_scn_pos2(&self) -> &GpadcRegScnPos2 {
        &self.gpadc_reg_scn_pos2
    }
    #[doc = "0x120 - GPADC Conversion Sequence 3 (Negative Channels 0-5)"]
    #[inline(always)]
    pub const fn gpadc_reg_scn_neg1(&self) -> &GpadcRegScnNeg1 {
        &self.gpadc_reg_scn_neg1
    }
    #[doc = "0x124 - GPADC Conversion Sequence 4 (Negative Channels 6-11)"]
    #[inline(always)]
    pub const fn gpadc_reg_scn_neg2(&self) -> &GpadcRegScnNeg2 {
        &self.gpadc_reg_scn_neg2
    }
    #[doc = "0x128 - GPADC Status Register"]
    #[inline(always)]
    pub const fn gpadc_reg_status(&self) -> &GpadcRegStatus {
        &self.gpadc_reg_status
    }
    #[doc = "0x12c - GPADC Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpadc_reg_isr(&self) -> &GpadcRegIsr {
        &self.gpadc_reg_isr
    }
    #[doc = "0x130 - GPADC Conversion Result"]
    #[inline(always)]
    pub const fn gpadc_reg_result(&self) -> &GpadcRegResult {
        &self.gpadc_reg_result
    }
    #[doc = "0x134 - GPADC Raw Conversion Result"]
    #[inline(always)]
    pub const fn gpadc_reg_raw_result(&self) -> &GpadcRegRawResult {
        &self.gpadc_reg_raw_result
    }
    #[doc = "0x138 - GPADC Defined Value (e.g., Offset Calibration)"]
    #[inline(always)]
    pub const fn gpadc_reg_define(&self) -> &GpadcRegDefine {
        &self.gpadc_reg_define
    }
    #[doc = "0x13c - Hibernate Core Reserved Register 0"]
    #[inline(always)]
    pub const fn hbncore_resv0(&self) -> &HbncoreResv0 {
        &self.hbncore_resv0
    }
    #[doc = "0x140 - Hibernate Core Reserved Register 1"]
    #[inline(always)]
    pub const fn hbncore_resv1(&self) -> &HbncoreResv1 {
        &self.hbncore_resv1
    }
}
#[doc = "AON (rw) register accessor: AON Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon`] module"]
#[doc(alias = "AON")]
pub type Aon = crate::Reg<aon::AonSpec>;
#[doc = "AON Register"]
pub mod aon;
#[doc = "AON_COMMON (rw) register accessor: AON Common Controls\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_common::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_common::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_common`] module"]
#[doc(alias = "AON_COMMON")]
pub type AonCommon = crate::Reg<aon_common::AonCommonSpec>;
#[doc = "AON Common Controls"]
pub mod aon_common;
#[doc = "AON_MISC (rw) register accessor: AON Miscellaneous Controls\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_misc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_misc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_misc`] module"]
#[doc(alias = "AON_MISC")]
pub type AonMisc = crate::Reg<aon_misc::AonMiscSpec>;
#[doc = "AON Miscellaneous Controls"]
pub mod aon_misc;
#[doc = "BG_SYS_TOP (rw) register accessor: Bandgap System Top Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bg_sys_top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bg_sys_top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bg_sys_top`] module"]
#[doc(alias = "BG_SYS_TOP")]
pub type BgSysTop = crate::Reg<bg_sys_top::BgSysTopSpec>;
#[doc = "Bandgap System Top Control"]
pub mod bg_sys_top;
#[doc = "DCDC_TOP_0 (rw) register accessor: DCDC11 (1.1V) Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_top_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_top_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_top_0`] module"]
#[doc(alias = "DCDC_TOP_0")]
pub type DcdcTop0 = crate::Reg<dcdc_top_0::DcdcTop0Spec>;
#[doc = "DCDC11 (1.1V) Configuration 0"]
pub mod dcdc_top_0;
#[doc = "DCDC_TOP_1 (rw) register accessor: DCDC11 (1.1V) Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_top_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_top_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_top_1`] module"]
#[doc(alias = "DCDC_TOP_1")]
pub type DcdcTop1 = crate::Reg<dcdc_top_1::DcdcTop1Spec>;
#[doc = "DCDC11 (1.1V) Configuration 1"]
pub mod dcdc_top_1;
#[doc = "LDO11SOC_AND_DCTEST (rw) register accessor: LDO11SOC and DCDC Test Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo11soc_and_dctest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo11soc_and_dctest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo11soc_and_dctest`] module"]
#[doc(alias = "LDO11SOC_AND_DCTEST")]
pub type Ldo11socAndDctest = crate::Reg<ldo11soc_and_dctest::Ldo11socAndDctestSpec>;
#[doc = "LDO11SOC and DCDC Test Configuration"]
pub mod ldo11soc_and_dctest;
#[doc = "DCDC18_TOP_0 (rw) register accessor: DCDC18 (1.8V) Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc18_top_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc18_top_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc18_top_0`] module"]
#[doc(alias = "DCDC18_TOP_0")]
pub type Dcdc18Top0 = crate::Reg<dcdc18_top_0::Dcdc18Top0Spec>;
#[doc = "DCDC18 (1.8V) Configuration 0"]
pub mod dcdc18_top_0;
#[doc = "DCDC18_TOP_1 (rw) register accessor: DCDC18 (1.8V) Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc18_top_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc18_top_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc18_top_1`] module"]
#[doc(alias = "DCDC18_TOP_1")]
pub type Dcdc18Top1 = crate::Reg<dcdc18_top_1::Dcdc18Top1Spec>;
#[doc = "DCDC18 (1.8V) Configuration 1"]
pub mod dcdc18_top_1;
#[doc = "DCDC18_TOP_2 (rw) register accessor: DCDC18 (1.8V) Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc18_top_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc18_top_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc18_top_2`] module"]
#[doc(alias = "DCDC18_TOP_2")]
pub type Dcdc18Top2 = crate::Reg<dcdc18_top_2::Dcdc18Top2Spec>;
#[doc = "DCDC18 (1.8V) Configuration 2"]
pub mod dcdc18_top_2;
#[doc = "PSW_IRRCV (rw) register accessor: PSW and USB Reference\n\nYou can [`read`](crate::Reg::read) this register and get [`psw_irrcv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psw_irrcv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psw_irrcv`] module"]
#[doc(alias = "PSW_IRRCV")]
pub type PswIrrcv = crate::Reg<psw_irrcv::PswIrrcvSpec>;
#[doc = "PSW and USB Reference"]
pub mod psw_irrcv;
#[doc = "RF_TOP_AON (rw) register accessor: RF LDO15 and XTAL Top Control\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_top_aon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_top_aon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_top_aon`] module"]
#[doc(alias = "RF_TOP_AON")]
pub type RfTopAon = crate::Reg<rf_top_aon::RfTopAonSpec>;
#[doc = "RF LDO15 and XTAL Top Control"]
pub mod rf_top_aon;
#[doc = "XTAL_CFG (rw) register accessor: Crystal Oscillator Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_cfg`] module"]
#[doc(alias = "XTAL_CFG")]
pub type XtalCfg = crate::Reg<xtal_cfg::XtalCfgSpec>;
#[doc = "Crystal Oscillator Configuration"]
pub mod xtal_cfg;
#[doc = "XTAL_CFG2 (rw) register accessor: Crystal Oscillator Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_cfg2`] module"]
#[doc(alias = "XTAL_CFG2")]
pub type XtalCfg2 = crate::Reg<xtal_cfg2::XtalCfg2Spec>;
#[doc = "Crystal Oscillator Configuration 2"]
pub mod xtal_cfg2;
#[doc = "XTAL_CFG3 (rw) register accessor: Crystal Oscillator Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_cfg3`] module"]
#[doc(alias = "XTAL_CFG3")]
pub type XtalCfg3 = crate::Reg<xtal_cfg3::XtalCfg3Spec>;
#[doc = "Crystal Oscillator Configuration 3"]
pub mod xtal_cfg3;
#[doc = "TSEN (rw) register accessor: Temperature Sensor and XTAL Ready\n\nYou can [`read`](crate::Reg::read) this register and get [`tsen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsen`] module"]
#[doc(alias = "TSEN")]
pub type Tsen = crate::Reg<tsen::TsenSpec>;
#[doc = "Temperature Sensor and XTAL Ready"]
pub mod tsen;
#[doc = "LDO18IO (rw) register accessor: LDO18 IO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo18io::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo18io::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo18io`] module"]
#[doc(alias = "LDO18IO")]
pub type Ldo18io = crate::Reg<ldo18io::Ldo18ioSpec>;
#[doc = "LDO18 IO Control"]
pub mod ldo18io;
#[doc = "ACOMP0_CTRL (rw) register accessor: Analog Comparator 0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`acomp0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acomp0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acomp0_ctrl`] module"]
#[doc(alias = "ACOMP0_CTRL")]
pub type Acomp0Ctrl = crate::Reg<acomp0_ctrl::Acomp0CtrlSpec>;
#[doc = "Analog Comparator 0 Control"]
pub mod acomp0_ctrl;
#[doc = "ACOMP1_CTRL (rw) register accessor: Analog Comparator 1 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`acomp1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acomp1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acomp1_ctrl`] module"]
#[doc(alias = "ACOMP1_CTRL")]
pub type Acomp1Ctrl = crate::Reg<acomp1_ctrl::Acomp1CtrlSpec>;
#[doc = "Analog Comparator 1 Control"]
pub mod acomp1_ctrl;
#[doc = "ACOMP_CTRL (rw) register accessor: Analog Comparator Shared Control\n\nYou can [`read`](crate::Reg::read) this register and get [`acomp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acomp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acomp_ctrl`] module"]
#[doc(alias = "ACOMP_CTRL")]
pub type AcompCtrl = crate::Reg<acomp_ctrl::AcompCtrlSpec>;
#[doc = "Analog Comparator Shared Control"]
pub mod acomp_ctrl;
#[doc = "GPADC_REG_CMD (rw) register accessor: GPADC Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_cmd`] module"]
#[doc(alias = "GPADC_REG_CMD")]
pub type GpadcRegCmd = crate::Reg<gpadc_reg_cmd::GpadcRegCmdSpec>;
#[doc = "GPADC Command Register"]
pub mod gpadc_reg_cmd;
#[doc = "GPADC_REG_CONFIG1 (rw) register accessor: GPADC Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_config1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_config1`] module"]
#[doc(alias = "GPADC_REG_CONFIG1")]
pub type GpadcRegConfig1 = crate::Reg<gpadc_reg_config1::GpadcRegConfig1Spec>;
#[doc = "GPADC Configuration Register 1"]
pub mod gpadc_reg_config1;
#[doc = "GPADC_REG_CONFIG2 (rw) register accessor: GPADC Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_config2`] module"]
#[doc(alias = "GPADC_REG_CONFIG2")]
pub type GpadcRegConfig2 = crate::Reg<gpadc_reg_config2::GpadcRegConfig2Spec>;
#[doc = "GPADC Configuration Register 2"]
pub mod gpadc_reg_config2;
#[doc = "GPADC_REG_SCN_POS1 (rw) register accessor: GPADC Conversion Sequence 1 (Positive Channels 0-5)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_pos1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_pos1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_scn_pos1`] module"]
#[doc(alias = "GPADC_REG_SCN_POS1")]
pub type GpadcRegScnPos1 = crate::Reg<gpadc_reg_scn_pos1::GpadcRegScnPos1Spec>;
#[doc = "GPADC Conversion Sequence 1 (Positive Channels 0-5)"]
pub mod gpadc_reg_scn_pos1;
#[doc = "GPADC_REG_SCN_POS2 (rw) register accessor: GPADC Conversion Sequence 2 (Positive Channels 6-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_pos2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_pos2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_scn_pos2`] module"]
#[doc(alias = "GPADC_REG_SCN_POS2")]
pub type GpadcRegScnPos2 = crate::Reg<gpadc_reg_scn_pos2::GpadcRegScnPos2Spec>;
#[doc = "GPADC Conversion Sequence 2 (Positive Channels 6-11)"]
pub mod gpadc_reg_scn_pos2;
#[doc = "GPADC_REG_SCN_NEG1 (rw) register accessor: GPADC Conversion Sequence 3 (Negative Channels 0-5)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_neg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_neg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_scn_neg1`] module"]
#[doc(alias = "GPADC_REG_SCN_NEG1")]
pub type GpadcRegScnNeg1 = crate::Reg<gpadc_reg_scn_neg1::GpadcRegScnNeg1Spec>;
#[doc = "GPADC Conversion Sequence 3 (Negative Channels 0-5)"]
pub mod gpadc_reg_scn_neg1;
#[doc = "GPADC_REG_SCN_NEG2 (rw) register accessor: GPADC Conversion Sequence 4 (Negative Channels 6-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_scn_neg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_scn_neg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_scn_neg2`] module"]
#[doc(alias = "GPADC_REG_SCN_NEG2")]
pub type GpadcRegScnNeg2 = crate::Reg<gpadc_reg_scn_neg2::GpadcRegScnNeg2Spec>;
#[doc = "GPADC Conversion Sequence 4 (Negative Channels 6-11)"]
pub mod gpadc_reg_scn_neg2;
#[doc = "GPADC_REG_STATUS (rw) register accessor: GPADC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_status`] module"]
#[doc(alias = "GPADC_REG_STATUS")]
pub type GpadcRegStatus = crate::Reg<gpadc_reg_status::GpadcRegStatusSpec>;
#[doc = "GPADC Status Register"]
pub mod gpadc_reg_status;
#[doc = "GPADC_REG_ISR (rw) register accessor: GPADC Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_isr`] module"]
#[doc(alias = "GPADC_REG_ISR")]
pub type GpadcRegIsr = crate::Reg<gpadc_reg_isr::GpadcRegIsrSpec>;
#[doc = "GPADC Interrupt Status Register"]
pub mod gpadc_reg_isr;
#[doc = "GPADC_REG_RESULT (r) register accessor: GPADC Conversion Result\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_result`] module"]
#[doc(alias = "GPADC_REG_RESULT")]
pub type GpadcRegResult = crate::Reg<gpadc_reg_result::GpadcRegResultSpec>;
#[doc = "GPADC Conversion Result"]
pub mod gpadc_reg_result;
#[doc = "GPADC_REG_RAW_RESULT (r) register accessor: GPADC Raw Conversion Result\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_raw_result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_raw_result`] module"]
#[doc(alias = "GPADC_REG_RAW_RESULT")]
pub type GpadcRegRawResult = crate::Reg<gpadc_reg_raw_result::GpadcRegRawResultSpec>;
#[doc = "GPADC Raw Conversion Result"]
pub mod gpadc_reg_raw_result;
#[doc = "GPADC_REG_DEFINE (rw) register accessor: GPADC Defined Value (e.g., Offset Calibration)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_reg_define::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_reg_define::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_define`] module"]
#[doc(alias = "GPADC_REG_DEFINE")]
pub type GpadcRegDefine = crate::Reg<gpadc_reg_define::GpadcRegDefineSpec>;
#[doc = "GPADC Defined Value (e.g., Offset Calibration)"]
pub mod gpadc_reg_define;
#[doc = "HBNCORE_RESV0 (rw) register accessor: Hibernate Core Reserved Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hbncore_resv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbncore_resv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbncore_resv0`] module"]
#[doc(alias = "HBNCORE_RESV0")]
pub type HbncoreResv0 = crate::Reg<hbncore_resv0::HbncoreResv0Spec>;
#[doc = "Hibernate Core Reserved Register 0"]
pub mod hbncore_resv0;
#[doc = "HBNCORE_RESV1 (rw) register accessor: Hibernate Core Reserved Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hbncore_resv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbncore_resv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbncore_resv1`] module"]
#[doc(alias = "HBNCORE_RESV1")]
pub type HbncoreResv1 = crate::Reg<hbncore_resv1::HbncoreResv1Spec>;
#[doc = "Hibernate Core Reserved Register 1"]
pub mod hbncore_resv1;
