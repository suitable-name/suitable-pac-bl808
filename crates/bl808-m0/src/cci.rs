#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cci_cfg: CciCfg,
    cci_addr: CciAddr,
    cci_wdata: CciWdata,
    cci_rdata: CciRdata,
    cci_ctl: CciCtl,
    _reserved5: [u8; 0x073c],
    audio_pll_cfg0: AudioPllCfg0,
    audio_pll_cfg1: AudioPllCfg1,
    audio_pll_cfg2: AudioPllCfg2,
    audio_pll_cfg3: AudioPllCfg3,
    audio_pll_cfg4: AudioPllCfg4,
    audio_pll_cfg5: AudioPllCfg5,
    audio_pll_cfg6: AudioPllCfg6,
    audio_pll_cfg7: AudioPllCfg7,
    audio_pll_cfg8: AudioPllCfg8,
    audio_pll_cfg9: AudioPllCfg9,
    audio_pll_cfg10: AudioPllCfg10,
    audio_pll_cfg11: AudioPllCfg11,
    _reserved17: [u8; 0x50],
    cpu_pll_cfg0: CpuPllCfg0,
    cpu_pll_cfg1: CpuPllCfg1,
    cpu_pll_cfg2: CpuPllCfg2,
    cpu_pll_cfg3: CpuPllCfg3,
    cpu_pll_cfg4: CpuPllCfg4,
    cpu_pll_cfg5: CpuPllCfg5,
    cpu_pll_cfg6: CpuPllCfg6,
    cpu_pll_cfg7: CpuPllCfg7,
    cpu_pll_cfg8: CpuPllCfg8,
    cpu_pll_cfg9: CpuPllCfg9,
    cpu_pll_cfg10: CpuPllCfg10,
    cpu_pll_cfg11: CpuPllCfg11,
}
impl RegisterBlock {
    #[doc = "0x00 - CCI Configuration Register"]
    #[inline(always)]
    pub const fn cci_cfg(&self) -> &CciCfg {
        &self.cci_cfg
    }
    #[doc = "0x04 - CCI Access Address Register"]
    #[inline(always)]
    pub const fn cci_addr(&self) -> &CciAddr {
        &self.cci_addr
    }
    #[doc = "0x08 - CCI Write Data Register"]
    #[inline(always)]
    pub const fn cci_wdata(&self) -> &CciWdata {
        &self.cci_wdata
    }
    #[doc = "0x0c - CCI Read Data Register"]
    #[inline(always)]
    pub const fn cci_rdata(&self) -> &CciRdata {
        &self.cci_rdata
    }
    #[doc = "0x10 - CCI Control and Status"]
    #[inline(always)]
    pub const fn cci_ctl(&self) -> &CciCtl {
        &self.cci_ctl
    }
    #[doc = "0x750 - Audio PLL Configuration 0 - Resets and Power Up"]
    #[inline(always)]
    pub const fn audio_pll_cfg0(&self) -> &AudioPllCfg0 {
        &self.audio_pll_cfg0
    }
    #[doc = "0x754 - Audio PLL Configuration 1 - Dividers"]
    #[inline(always)]
    pub const fn audio_pll_cfg1(&self) -> &AudioPllCfg1 {
        &self.audio_pll_cfg1
    }
    #[doc = "0x758 - Audio PLL Configuration 2 - Charge Pump and Control"]
    #[inline(always)]
    pub const fn audio_pll_cfg2(&self) -> &AudioPllCfg2 {
        &self.audio_pll_cfg2
    }
    #[doc = "0x75c - Audio PLL Configuration 3 - Loop Filter"]
    #[inline(always)]
    pub const fn audio_pll_cfg3(&self) -> &AudioPllCfg3 {
        &self.audio_pll_cfg3
    }
    #[doc = "0x760 - Audio PLL Configuration 4 - Clock Select"]
    #[inline(always)]
    pub const fn audio_pll_cfg4(&self) -> &AudioPllCfg4 {
        &self.audio_pll_cfg4
    }
    #[doc = "0x764 - Audio PLL Configuration 5 - VCO Speed"]
    #[inline(always)]
    pub const fn audio_pll_cfg5(&self) -> &AudioPllCfg5 {
        &self.audio_pll_cfg5
    }
    #[doc = "0x768 - Audio PLL Configuration 6 - SDM Input"]
    #[inline(always)]
    pub const fn audio_pll_cfg6(&self) -> &AudioPllCfg6 {
        &self.audio_pll_cfg6
    }
    #[doc = "0x76c - Audio PLL Configuration 7 - SDM Control"]
    #[inline(always)]
    pub const fn audio_pll_cfg7(&self) -> &AudioPllCfg7 {
        &self.audio_pll_cfg7
    }
    #[doc = "0x770 - Audio PLL Configuration 8 - Output Dividers"]
    #[inline(always)]
    pub const fn audio_pll_cfg8(&self) -> &AudioPllCfg8 {
        &self.audio_pll_cfg8
    }
    #[doc = "0x774 - Audio PLL Configuration 9 - Test and Digital Enable"]
    #[inline(always)]
    pub const fn audio_pll_cfg9(&self) -> &AudioPllCfg9 {
        &self.audio_pll_cfg9
    }
    #[doc = "0x778 - Audio PLL Configuration 10 - SSC Control"]
    #[inline(always)]
    pub const fn audio_pll_cfg10(&self) -> &AudioPllCfg10 {
        &self.audio_pll_cfg10
    }
    #[doc = "0x77c - Audio PLL Configuration 11 - Reserved and DL Control"]
    #[inline(always)]
    pub const fn audio_pll_cfg11(&self) -> &AudioPllCfg11 {
        &self.audio_pll_cfg11
    }
    #[doc = "0x7d0 - CPU PLL Configuration 0 - Resets and Power Up"]
    #[inline(always)]
    pub const fn cpu_pll_cfg0(&self) -> &CpuPllCfg0 {
        &self.cpu_pll_cfg0
    }
    #[doc = "0x7d4 - CPU PLL Configuration 1 - Dividers"]
    #[inline(always)]
    pub const fn cpu_pll_cfg1(&self) -> &CpuPllCfg1 {
        &self.cpu_pll_cfg1
    }
    #[doc = "0x7d8 - CPU PLL Configuration 2 - Charge Pump"]
    #[inline(always)]
    pub const fn cpu_pll_cfg2(&self) -> &CpuPllCfg2 {
        &self.cpu_pll_cfg2
    }
    #[doc = "0x7dc - CPU PLL Configuration 3 - Loop Filter"]
    #[inline(always)]
    pub const fn cpu_pll_cfg3(&self) -> &CpuPllCfg3 {
        &self.cpu_pll_cfg3
    }
    #[doc = "0x7e0 - CPU PLL Configuration 4 - Clock Select"]
    #[inline(always)]
    pub const fn cpu_pll_cfg4(&self) -> &CpuPllCfg4 {
        &self.cpu_pll_cfg4
    }
    #[doc = "0x7e4 - CPU PLL Configuration 5 - VCO Speed"]
    #[inline(always)]
    pub const fn cpu_pll_cfg5(&self) -> &CpuPllCfg5 {
        &self.cpu_pll_cfg5
    }
    #[doc = "0x7e8 - CPU PLL Configuration 6 - SDM Input"]
    #[inline(always)]
    pub const fn cpu_pll_cfg6(&self) -> &CpuPllCfg6 {
        &self.cpu_pll_cfg6
    }
    #[doc = "0x7ec - CPU PLL Configuration 7 - SDM Control"]
    #[inline(always)]
    pub const fn cpu_pll_cfg7(&self) -> &CpuPllCfg7 {
        &self.cpu_pll_cfg7
    }
    #[doc = "0x7f0 - CPU PLL Configuration 8 - Output Dividers"]
    #[inline(always)]
    pub const fn cpu_pll_cfg8(&self) -> &CpuPllCfg8 {
        &self.cpu_pll_cfg8
    }
    #[doc = "0x7f4 - CPU PLL Configuration 9 - Test and Digital Enable"]
    #[inline(always)]
    pub const fn cpu_pll_cfg9(&self) -> &CpuPllCfg9 {
        &self.cpu_pll_cfg9
    }
    #[doc = "0x7f8 - CPU PLL Configuration 10 - SSC Control"]
    #[inline(always)]
    pub const fn cpu_pll_cfg10(&self) -> &CpuPllCfg10 {
        &self.cpu_pll_cfg10
    }
    #[doc = "0x7fc - CPU PLL Configuration 11 - Reserved and DL Control"]
    #[inline(always)]
    pub const fn cpu_pll_cfg11(&self) -> &CpuPllCfg11 {
        &self.cpu_pll_cfg11
    }
}
#[doc = "CCI_CFG (rw) register accessor: CCI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_cfg`] module"]
#[doc(alias = "CCI_CFG")]
pub type CciCfg = crate::Reg<cci_cfg::CciCfgSpec>;
#[doc = "CCI Configuration Register"]
pub mod cci_cfg;
#[doc = "CCI_ADDR (rw) register accessor: CCI Access Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_addr`] module"]
#[doc(alias = "CCI_ADDR")]
pub type CciAddr = crate::Reg<cci_addr::CciAddrSpec>;
#[doc = "CCI Access Address Register"]
pub mod cci_addr;
#[doc = "CCI_WDATA (rw) register accessor: CCI Write Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_wdata`] module"]
#[doc(alias = "CCI_WDATA")]
pub type CciWdata = crate::Reg<cci_wdata::CciWdataSpec>;
#[doc = "CCI Write Data Register"]
pub mod cci_wdata;
#[doc = "CCI_RDATA (r) register accessor: CCI Read Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_rdata`] module"]
#[doc(alias = "CCI_RDATA")]
pub type CciRdata = crate::Reg<cci_rdata::CciRdataSpec>;
#[doc = "CCI Read Data Register"]
pub mod cci_rdata;
#[doc = "CCI_CTL (r) register accessor: CCI Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_ctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_ctl`] module"]
#[doc(alias = "CCI_CTL")]
pub type CciCtl = crate::Reg<cci_ctl::CciCtlSpec>;
#[doc = "CCI Control and Status"]
pub mod cci_ctl;
#[doc = "AUDIO_PLL_CFG0 (rw) register accessor: Audio PLL Configuration 0 - Resets and Power Up\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg0`] module"]
#[doc(alias = "AUDIO_PLL_CFG0")]
pub type AudioPllCfg0 = crate::Reg<audio_pll_cfg0::AudioPllCfg0Spec>;
#[doc = "Audio PLL Configuration 0 - Resets and Power Up"]
pub mod audio_pll_cfg0;
#[doc = "AUDIO_PLL_CFG1 (rw) register accessor: Audio PLL Configuration 1 - Dividers\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg1`] module"]
#[doc(alias = "AUDIO_PLL_CFG1")]
pub type AudioPllCfg1 = crate::Reg<audio_pll_cfg1::AudioPllCfg1Spec>;
#[doc = "Audio PLL Configuration 1 - Dividers"]
pub mod audio_pll_cfg1;
#[doc = "AUDIO_PLL_CFG2 (rw) register accessor: Audio PLL Configuration 2 - Charge Pump and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg2`] module"]
#[doc(alias = "AUDIO_PLL_CFG2")]
pub type AudioPllCfg2 = crate::Reg<audio_pll_cfg2::AudioPllCfg2Spec>;
#[doc = "Audio PLL Configuration 2 - Charge Pump and Control"]
pub mod audio_pll_cfg2;
#[doc = "AUDIO_PLL_CFG3 (rw) register accessor: Audio PLL Configuration 3 - Loop Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg3`] module"]
#[doc(alias = "AUDIO_PLL_CFG3")]
pub type AudioPllCfg3 = crate::Reg<audio_pll_cfg3::AudioPllCfg3Spec>;
#[doc = "Audio PLL Configuration 3 - Loop Filter"]
pub mod audio_pll_cfg3;
#[doc = "AUDIO_PLL_CFG4 (rw) register accessor: Audio PLL Configuration 4 - Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg4`] module"]
#[doc(alias = "AUDIO_PLL_CFG4")]
pub type AudioPllCfg4 = crate::Reg<audio_pll_cfg4::AudioPllCfg4Spec>;
#[doc = "Audio PLL Configuration 4 - Clock Select"]
pub mod audio_pll_cfg4;
#[doc = "AUDIO_PLL_CFG5 (rw) register accessor: Audio PLL Configuration 5 - VCO Speed\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg5`] module"]
#[doc(alias = "AUDIO_PLL_CFG5")]
pub type AudioPllCfg5 = crate::Reg<audio_pll_cfg5::AudioPllCfg5Spec>;
#[doc = "Audio PLL Configuration 5 - VCO Speed"]
pub mod audio_pll_cfg5;
#[doc = "AUDIO_PLL_CFG6 (rw) register accessor: Audio PLL Configuration 6 - SDM Input\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg6`] module"]
#[doc(alias = "AUDIO_PLL_CFG6")]
pub type AudioPllCfg6 = crate::Reg<audio_pll_cfg6::AudioPllCfg6Spec>;
#[doc = "Audio PLL Configuration 6 - SDM Input"]
pub mod audio_pll_cfg6;
#[doc = "AUDIO_PLL_CFG7 (rw) register accessor: Audio PLL Configuration 7 - SDM Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg7`] module"]
#[doc(alias = "AUDIO_PLL_CFG7")]
pub type AudioPllCfg7 = crate::Reg<audio_pll_cfg7::AudioPllCfg7Spec>;
#[doc = "Audio PLL Configuration 7 - SDM Control"]
pub mod audio_pll_cfg7;
#[doc = "AUDIO_PLL_CFG8 (rw) register accessor: Audio PLL Configuration 8 - Output Dividers\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg8`] module"]
#[doc(alias = "AUDIO_PLL_CFG8")]
pub type AudioPllCfg8 = crate::Reg<audio_pll_cfg8::AudioPllCfg8Spec>;
#[doc = "Audio PLL Configuration 8 - Output Dividers"]
pub mod audio_pll_cfg8;
#[doc = "AUDIO_PLL_CFG9 (rw) register accessor: Audio PLL Configuration 9 - Test and Digital Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg9`] module"]
#[doc(alias = "AUDIO_PLL_CFG9")]
pub type AudioPllCfg9 = crate::Reg<audio_pll_cfg9::AudioPllCfg9Spec>;
#[doc = "Audio PLL Configuration 9 - Test and Digital Enable"]
pub mod audio_pll_cfg9;
#[doc = "AUDIO_PLL_CFG10 (rw) register accessor: Audio PLL Configuration 10 - SSC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg10`] module"]
#[doc(alias = "AUDIO_PLL_CFG10")]
pub type AudioPllCfg10 = crate::Reg<audio_pll_cfg10::AudioPllCfg10Spec>;
#[doc = "Audio PLL Configuration 10 - SSC Control"]
pub mod audio_pll_cfg10;
#[doc = "AUDIO_PLL_CFG11 (rw) register accessor: Audio PLL Configuration 11 - Reserved and DL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg11`] module"]
#[doc(alias = "AUDIO_PLL_CFG11")]
pub type AudioPllCfg11 = crate::Reg<audio_pll_cfg11::AudioPllCfg11Spec>;
#[doc = "Audio PLL Configuration 11 - Reserved and DL Control"]
pub mod audio_pll_cfg11;
#[doc = "CPU_PLL_CFG0 (rw) register accessor: CPU PLL Configuration 0 - Resets and Power Up\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg0`] module"]
#[doc(alias = "CPU_PLL_CFG0")]
pub type CpuPllCfg0 = crate::Reg<cpu_pll_cfg0::CpuPllCfg0Spec>;
#[doc = "CPU PLL Configuration 0 - Resets and Power Up"]
pub mod cpu_pll_cfg0;
#[doc = "CPU_PLL_CFG1 (rw) register accessor: CPU PLL Configuration 1 - Dividers\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg1`] module"]
#[doc(alias = "CPU_PLL_CFG1")]
pub type CpuPllCfg1 = crate::Reg<cpu_pll_cfg1::CpuPllCfg1Spec>;
#[doc = "CPU PLL Configuration 1 - Dividers"]
pub mod cpu_pll_cfg1;
#[doc = "CPU_PLL_CFG2 (rw) register accessor: CPU PLL Configuration 2 - Charge Pump\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg2`] module"]
#[doc(alias = "CPU_PLL_CFG2")]
pub type CpuPllCfg2 = crate::Reg<cpu_pll_cfg2::CpuPllCfg2Spec>;
#[doc = "CPU PLL Configuration 2 - Charge Pump"]
pub mod cpu_pll_cfg2;
#[doc = "CPU_PLL_CFG3 (rw) register accessor: CPU PLL Configuration 3 - Loop Filter\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg3`] module"]
#[doc(alias = "CPU_PLL_CFG3")]
pub type CpuPllCfg3 = crate::Reg<cpu_pll_cfg3::CpuPllCfg3Spec>;
#[doc = "CPU PLL Configuration 3 - Loop Filter"]
pub mod cpu_pll_cfg3;
#[doc = "CPU_PLL_CFG4 (rw) register accessor: CPU PLL Configuration 4 - Clock Select\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg4`] module"]
#[doc(alias = "CPU_PLL_CFG4")]
pub type CpuPllCfg4 = crate::Reg<cpu_pll_cfg4::CpuPllCfg4Spec>;
#[doc = "CPU PLL Configuration 4 - Clock Select"]
pub mod cpu_pll_cfg4;
#[doc = "CPU_PLL_CFG5 (rw) register accessor: CPU PLL Configuration 5 - VCO Speed\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg5`] module"]
#[doc(alias = "CPU_PLL_CFG5")]
pub type CpuPllCfg5 = crate::Reg<cpu_pll_cfg5::CpuPllCfg5Spec>;
#[doc = "CPU PLL Configuration 5 - VCO Speed"]
pub mod cpu_pll_cfg5;
#[doc = "CPU_PLL_CFG6 (rw) register accessor: CPU PLL Configuration 6 - SDM Input\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg6`] module"]
#[doc(alias = "CPU_PLL_CFG6")]
pub type CpuPllCfg6 = crate::Reg<cpu_pll_cfg6::CpuPllCfg6Spec>;
#[doc = "CPU PLL Configuration 6 - SDM Input"]
pub mod cpu_pll_cfg6;
#[doc = "CPU_PLL_CFG7 (rw) register accessor: CPU PLL Configuration 7 - SDM Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg7`] module"]
#[doc(alias = "CPU_PLL_CFG7")]
pub type CpuPllCfg7 = crate::Reg<cpu_pll_cfg7::CpuPllCfg7Spec>;
#[doc = "CPU PLL Configuration 7 - SDM Control"]
pub mod cpu_pll_cfg7;
#[doc = "CPU_PLL_CFG8 (rw) register accessor: CPU PLL Configuration 8 - Output Dividers\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg8`] module"]
#[doc(alias = "CPU_PLL_CFG8")]
pub type CpuPllCfg8 = crate::Reg<cpu_pll_cfg8::CpuPllCfg8Spec>;
#[doc = "CPU PLL Configuration 8 - Output Dividers"]
pub mod cpu_pll_cfg8;
#[doc = "CPU_PLL_CFG9 (rw) register accessor: CPU PLL Configuration 9 - Test and Digital Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg9`] module"]
#[doc(alias = "CPU_PLL_CFG9")]
pub type CpuPllCfg9 = crate::Reg<cpu_pll_cfg9::CpuPllCfg9Spec>;
#[doc = "CPU PLL Configuration 9 - Test and Digital Enable"]
pub mod cpu_pll_cfg9;
#[doc = "CPU_PLL_CFG10 (rw) register accessor: CPU PLL Configuration 10 - SSC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg10`] module"]
#[doc(alias = "CPU_PLL_CFG10")]
pub type CpuPllCfg10 = crate::Reg<cpu_pll_cfg10::CpuPllCfg10Spec>;
#[doc = "CPU PLL Configuration 10 - SSC Control"]
pub mod cpu_pll_cfg10;
#[doc = "CPU_PLL_CFG11 (rw) register accessor: CPU PLL Configuration 11 - Reserved and DL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_pll_cfg11`] module"]
#[doc(alias = "CPU_PLL_CFG11")]
pub type CpuPllCfg11 = crate::Reg<cpu_pll_cfg11::CpuPllCfg11Spec>;
#[doc = "CPU PLL Configuration 11 - Reserved and DL Control"]
pub mod cpu_pll_cfg11;
