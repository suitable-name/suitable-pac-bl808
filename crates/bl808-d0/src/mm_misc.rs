#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpu0_boot: Cpu0Boot,
    _reserved1: [u8; 0x04],
    cpu_cfg: CpuCfg,
    cpu_sts1: CpuSts1,
    cpu_sts2: CpuSts2,
    _reserved4: [u8; 0x04],
    cpu_rtc: CpuRtc,
    tzc_mmsys_misc: TzcMmsysMisc,
    peri_apb_ctrl: PeriApbCtrl,
    _reserved7: [u8; 0x08],
    mm_infra_qos_ctrl: MmInfraQosCtrl,
    _reserved8: [u8; 0x10],
    dma_clk_ctrl: DmaClkCtrl,
    _reserved9: [u8; 0x0c],
    vram_ctrl: VramCtrl,
    _reserved10: [u8; 0x0c],
    sram_parm: SramParm,
    _reserved11: [u8; 0x3c],
    mm_int_sta0: MmIntSta0,
    mm_int_mask0: MmIntMask0,
    mm_int_clr_0: MmIntClr0,
    mm_int_sta1: MmIntSta1,
    mm_int_mask1: MmIntMask1,
    mm_int_clr_1: MmIntClr1,
    _reserved17: [u8; 0x38],
    mmsys_debug_sel: MmsysDebugSel,
    _reserved18: [u8; 0x08],
    mmsys_misc_dummy: MmsysMiscDummy,
    ddr_debug: DdrDebug,
    _reserved20: [u8; 0x3c],
    mm_berr_cfg0: MmBerrCfg0,
    mm_berr_cfg1: MmBerrCfg1,
    mm_berr_cfg2: MmBerrCfg2,
    mm_berr_cfg3: MmBerrCfg3,
    mm_berr_cfg4: MmBerrCfg4,
    mm_berr_cfg5: MmBerrCfg5,
    mm_berr_cfg6: MmBerrCfg6,
    mm_berr_cfg7: MmBerrCfg7,
}
impl RegisterBlock {
    #[doc = "0x00 - D0 Core (C906) Boot Vector Address"]
    #[inline(always)]
    pub const fn cpu0_boot(&self) -> &Cpu0Boot {
        &self.cpu0_boot
    }
    #[doc = "0x08 - D0 Core Configuration and Reset Control"]
    #[inline(always)]
    pub const fn cpu_cfg(&self) -> &CpuCfg {
        &self.cpu_cfg
    }
    #[doc = "0x0c - D0 Core Status Register 1 (Halt and LPMD)"]
    #[inline(always)]
    pub const fn cpu_sts1(&self) -> &CpuSts1 {
        &self.cpu_sts1
    }
    #[doc = "0x10 - D0 Core Status Register 2 (Program Counter Low Word)"]
    #[inline(always)]
    pub const fn cpu_sts2(&self) -> &CpuSts2 {
        &self.cpu_sts2
    }
    #[doc = "0x18 - D0 Core (C906) RTC Clock Configuration"]
    #[inline(always)]
    pub const fn cpu_rtc(&self) -> &CpuRtc {
        &self.cpu_rtc
    }
    #[doc = "0x1c - Multimedia TrustZone and SRAM Protection Status"]
    #[inline(always)]
    pub const fn tzc_mmsys_misc(&self) -> &TzcMmsysMisc {
        &self.tzc_mmsys_misc
    }
    #[doc = "0x20 - Multimedia Peripheral APB and Bus Error Control"]
    #[inline(always)]
    pub const fn peri_apb_ctrl(&self) -> &PeriApbCtrl {
        &self.peri_apb_ctrl
    }
    #[doc = "0x2c - Multimedia Infrastructure Quality-of-Service (QoS) Control"]
    #[inline(always)]
    pub const fn mm_infra_qos_ctrl(&self) -> &MmInfraQosCtrl {
        &self.mm_infra_qos_ctrl
    }
    #[doc = "0x40 - Multimedia DMA Clock Gating Control"]
    #[inline(always)]
    pub const fn dma_clk_ctrl(&self) -> &DmaClkCtrl {
        &self.dma_clk_ctrl
    }
    #[doc = "0x50 - VRAM and Multimedia SRAM Partition Control"]
    #[inline(always)]
    pub const fn vram_ctrl(&self) -> &VramCtrl {
        &self.vram_ctrl
    }
    #[doc = "0x60 - SRAM Power and DVS (Dynamic Voltage Scaling) Parameters"]
    #[inline(always)]
    pub const fn sram_parm(&self) -> &SramParm {
        &self.sram_parm
    }
    #[doc = "0xa0 - Multimedia Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn mm_int_sta0(&self) -> &MmIntSta0 {
        &self.mm_int_sta0
    }
    #[doc = "0xa4 - Multimedia Interrupt Mask Register 0"]
    #[inline(always)]
    pub const fn mm_int_mask0(&self) -> &MmIntMask0 {
        &self.mm_int_mask0
    }
    #[doc = "0xa8 - Multimedia Interrupt Clear Register 0"]
    #[inline(always)]
    pub const fn mm_int_clr_0(&self) -> &MmIntClr0 {
        &self.mm_int_clr_0
    }
    #[doc = "0xac - Multimedia Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn mm_int_sta1(&self) -> &MmIntSta1 {
        &self.mm_int_sta1
    }
    #[doc = "0xb0 - Multimedia Interrupt Mask Register 1"]
    #[inline(always)]
    pub const fn mm_int_mask1(&self) -> &MmIntMask1 {
        &self.mm_int_mask1
    }
    #[doc = "0xb4 - Multimedia Interrupt Clear Register 1"]
    #[inline(always)]
    pub const fn mm_int_clr_1(&self) -> &MmIntClr1 {
        &self.mm_int_clr_1
    }
    #[doc = "0xf0 - Multimedia System Debug Bus Select"]
    #[inline(always)]
    pub const fn mmsys_debug_sel(&self) -> &MmsysDebugSel {
        &self.mmsys_debug_sel
    }
    #[doc = "0xfc - Multimedia System Miscellaneous IO and Debug Dummy Control"]
    #[inline(always)]
    pub const fn mmsys_misc_dummy(&self) -> &MmsysMiscDummy {
        &self.mmsys_misc_dummy
    }
    #[doc = "0x100 - DDR Calibration Status"]
    #[inline(always)]
    pub const fn ddr_debug(&self) -> &DdrDebug {
        &self.ddr_debug
    }
    #[doc = "0x140 - Multimedia Bus Error Enable Register"]
    #[inline(always)]
    pub const fn mm_berr_cfg0(&self) -> &MmBerrCfg0 {
        &self.mm_berr_cfg0
    }
    #[doc = "0x144 - Multimedia Bus Error Clear, Last-Error Select, and Status Register"]
    #[inline(always)]
    pub const fn mm_berr_cfg1(&self) -> &MmBerrCfg1 {
        &self.mm_berr_cfg1
    }
    #[doc = "0x148 - Multimedia Bus Error Source and ID Register (Main and CODEC)"]
    #[inline(always)]
    pub const fn mm_berr_cfg2(&self) -> &MmBerrCfg2 {
        &self.mm_berr_cfg2
    }
    #[doc = "0x14c - Multimedia Bus Error Source and ID Register (MM CPU and MM Infra)"]
    #[inline(always)]
    pub const fn mm_berr_cfg3(&self) -> &MmBerrCfg3 {
        &self.mm_berr_cfg3
    }
    #[doc = "0x150 - Multimedia Main Bus Error Address Capture"]
    #[inline(always)]
    pub const fn mm_berr_cfg4(&self) -> &MmBerrCfg4 {
        &self.mm_berr_cfg4
    }
    #[doc = "0x154 - Multimedia CODEC Bus Error Address Capture"]
    #[inline(always)]
    pub const fn mm_berr_cfg5(&self) -> &MmBerrCfg5 {
        &self.mm_berr_cfg5
    }
    #[doc = "0x158 - Multimedia MM CPU Bus Error Address Capture"]
    #[inline(always)]
    pub const fn mm_berr_cfg6(&self) -> &MmBerrCfg6 {
        &self.mm_berr_cfg6
    }
    #[doc = "0x15c - Multimedia Infrastructure Bus Error Address Capture"]
    #[inline(always)]
    pub const fn mm_berr_cfg7(&self) -> &MmBerrCfg7 {
        &self.mm_berr_cfg7
    }
}
#[doc = "CPU0_BOOT (rw) register accessor: D0 Core (C906) Boot Vector Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_boot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_boot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_boot`] module"]
#[doc(alias = "CPU0_BOOT")]
pub type Cpu0Boot = crate::Reg<cpu0_boot::Cpu0BootSpec>;
#[doc = "D0 Core (C906) Boot Vector Address"]
pub mod cpu0_boot;
#[doc = "CPU_CFG (rw) register accessor: D0 Core Configuration and Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_cfg`] module"]
#[doc(alias = "CPU_CFG")]
pub type CpuCfg = crate::Reg<cpu_cfg::CpuCfgSpec>;
#[doc = "D0 Core Configuration and Reset Control"]
pub mod cpu_cfg;
#[doc = "CPU_STS1 (r) register accessor: D0 Core Status Register 1 (Halt and LPMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_sts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_sts1`] module"]
#[doc(alias = "CPU_STS1")]
pub type CpuSts1 = crate::Reg<cpu_sts1::CpuSts1Spec>;
#[doc = "D0 Core Status Register 1 (Halt and LPMD)"]
pub mod cpu_sts1;
#[doc = "CPU_STS2 (r) register accessor: D0 Core Status Register 2 (Program Counter Low Word)\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_sts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_sts2`] module"]
#[doc(alias = "CPU_STS2")]
pub type CpuSts2 = crate::Reg<cpu_sts2::CpuSts2Spec>;
#[doc = "D0 Core Status Register 2 (Program Counter Low Word)"]
pub mod cpu_sts2;
#[doc = "CPU_RTC (rw) register accessor: D0 Core (C906) RTC Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_rtc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_rtc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_rtc`] module"]
#[doc(alias = "CPU_RTC")]
pub type CpuRtc = crate::Reg<cpu_rtc::CpuRtcSpec>;
#[doc = "D0 Core (C906) RTC Clock Configuration"]
pub mod cpu_rtc;
#[doc = "TZC_MMSYS_MISC (r) register accessor: Multimedia TrustZone and SRAM Protection Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mmsys_misc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mmsys_misc`] module"]
#[doc(alias = "TZC_MMSYS_MISC")]
pub type TzcMmsysMisc = crate::Reg<tzc_mmsys_misc::TzcMmsysMiscSpec>;
#[doc = "Multimedia TrustZone and SRAM Protection Status"]
pub mod tzc_mmsys_misc;
#[doc = "PERI_APB_CTRL (rw) register accessor: Multimedia Peripheral APB and Bus Error Control\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_apb_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_apb_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peri_apb_ctrl`] module"]
#[doc(alias = "PERI_APB_CTRL")]
pub type PeriApbCtrl = crate::Reg<peri_apb_ctrl::PeriApbCtrlSpec>;
#[doc = "Multimedia Peripheral APB and Bus Error Control"]
pub mod peri_apb_ctrl;
#[doc = "MM_INFRA_QOS_CTRL (rw) register accessor: Multimedia Infrastructure Quality-of-Service (QoS) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_infra_qos_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_infra_qos_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_infra_qos_ctrl`] module"]
#[doc(alias = "MM_INFRA_QOS_CTRL")]
pub type MmInfraQosCtrl = crate::Reg<mm_infra_qos_ctrl::MmInfraQosCtrlSpec>;
#[doc = "Multimedia Infrastructure Quality-of-Service (QoS) Control"]
pub mod mm_infra_qos_ctrl;
#[doc = "DMA_CLK_CTRL (rw) register accessor: Multimedia DMA Clock Gating Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_clk_ctrl`] module"]
#[doc(alias = "DMA_CLK_CTRL")]
pub type DmaClkCtrl = crate::Reg<dma_clk_ctrl::DmaClkCtrlSpec>;
#[doc = "Multimedia DMA Clock Gating Control"]
pub mod dma_clk_ctrl;
#[doc = "VRAM_CTRL (rw) register accessor: VRAM and Multimedia SRAM Partition Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vram_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vram_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vram_ctrl`] module"]
#[doc(alias = "VRAM_CTRL")]
pub type VramCtrl = crate::Reg<vram_ctrl::VramCtrlSpec>;
#[doc = "VRAM and Multimedia SRAM Partition Control"]
pub mod vram_ctrl;
#[doc = "SRAM_PARM (rw) register accessor: SRAM Power and DVS (Dynamic Voltage Scaling) Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_parm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_parm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_parm`] module"]
#[doc(alias = "SRAM_PARM")]
pub type SramParm = crate::Reg<sram_parm::SramParmSpec>;
#[doc = "SRAM Power and DVS (Dynamic Voltage Scaling) Parameters"]
pub mod sram_parm;
#[doc = "MM_INT_STA0 (r) register accessor: Multimedia Interrupt Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_int_sta0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_int_sta0`] module"]
#[doc(alias = "MM_INT_STA0")]
pub type MmIntSta0 = crate::Reg<mm_int_sta0::MmIntSta0Spec>;
#[doc = "Multimedia Interrupt Status Register 0"]
pub mod mm_int_sta0;
#[doc = "MM_INT_MASK0 (rw) register accessor: Multimedia Interrupt Mask Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_int_mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_int_mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_int_mask0`] module"]
#[doc(alias = "MM_INT_MASK0")]
pub type MmIntMask0 = crate::Reg<mm_int_mask0::MmIntMask0Spec>;
#[doc = "Multimedia Interrupt Mask Register 0"]
pub mod mm_int_mask0;
#[doc = "MM_INT_CLR_0 (w) register accessor: Multimedia Interrupt Clear Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_int_clr_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_int_clr_0`] module"]
#[doc(alias = "MM_INT_CLR_0")]
pub type MmIntClr0 = crate::Reg<mm_int_clr_0::MmIntClr0Spec>;
#[doc = "Multimedia Interrupt Clear Register 0"]
pub mod mm_int_clr_0;
#[doc = "MM_INT_STA1 (r) register accessor: Multimedia Interrupt Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_int_sta1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_int_sta1`] module"]
#[doc(alias = "MM_INT_STA1")]
pub type MmIntSta1 = crate::Reg<mm_int_sta1::MmIntSta1Spec>;
#[doc = "Multimedia Interrupt Status Register 1"]
pub mod mm_int_sta1;
#[doc = "MM_INT_MASK1 (rw) register accessor: Multimedia Interrupt Mask Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_int_mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_int_mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_int_mask1`] module"]
#[doc(alias = "MM_INT_MASK1")]
pub type MmIntMask1 = crate::Reg<mm_int_mask1::MmIntMask1Spec>;
#[doc = "Multimedia Interrupt Mask Register 1"]
pub mod mm_int_mask1;
#[doc = "MM_INT_CLR_1 (w) register accessor: Multimedia Interrupt Clear Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_int_clr_1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_int_clr_1`] module"]
#[doc(alias = "MM_INT_CLR_1")]
pub type MmIntClr1 = crate::Reg<mm_int_clr_1::MmIntClr1Spec>;
#[doc = "Multimedia Interrupt Clear Register 1"]
pub mod mm_int_clr_1;
#[doc = "MMSYS_DEBUG_SEL (rw) register accessor: Multimedia System Debug Bus Select\n\nYou can [`read`](crate::Reg::read) this register and get [`mmsys_debug_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmsys_debug_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmsys_debug_sel`] module"]
#[doc(alias = "MMSYS_DEBUG_SEL")]
pub type MmsysDebugSel = crate::Reg<mmsys_debug_sel::MmsysDebugSelSpec>;
#[doc = "Multimedia System Debug Bus Select"]
pub mod mmsys_debug_sel;
#[doc = "MMSYS_MISC_DUMMY (rw) register accessor: Multimedia System Miscellaneous IO and Debug Dummy Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mmsys_misc_dummy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmsys_misc_dummy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmsys_misc_dummy`] module"]
#[doc(alias = "MMSYS_MISC_DUMMY")]
pub type MmsysMiscDummy = crate::Reg<mmsys_misc_dummy::MmsysMiscDummySpec>;
#[doc = "Multimedia System Miscellaneous IO and Debug Dummy Control"]
pub mod mmsys_misc_dummy;
#[doc = "DDR_DEBUG (r) register accessor: DDR Calibration Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr_debug::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr_debug`] module"]
#[doc(alias = "DDR_DEBUG")]
pub type DdrDebug = crate::Reg<ddr_debug::DdrDebugSpec>;
#[doc = "DDR Calibration Status"]
pub mod ddr_debug;
#[doc = "MM_BERR_CFG0 (rw) register accessor: Multimedia Bus Error Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_berr_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_berr_cfg0`] module"]
#[doc(alias = "MM_BERR_CFG0")]
pub type MmBerrCfg0 = crate::Reg<mm_berr_cfg0::MmBerrCfg0Spec>;
#[doc = "Multimedia Bus Error Enable Register"]
pub mod mm_berr_cfg0;
#[doc = "MM_BERR_CFG1 (rw) register accessor: Multimedia Bus Error Clear, Last-Error Select, and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_berr_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_berr_cfg1`] module"]
#[doc(alias = "MM_BERR_CFG1")]
pub type MmBerrCfg1 = crate::Reg<mm_berr_cfg1::MmBerrCfg1Spec>;
#[doc = "Multimedia Bus Error Clear, Last-Error Select, and Status Register"]
pub mod mm_berr_cfg1;
#[doc = "MM_BERR_CFG2 (r) register accessor: Multimedia Bus Error Source and ID Register (Main and CODEC)\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_berr_cfg2`] module"]
#[doc(alias = "MM_BERR_CFG2")]
pub type MmBerrCfg2 = crate::Reg<mm_berr_cfg2::MmBerrCfg2Spec>;
#[doc = "Multimedia Bus Error Source and ID Register (Main and CODEC)"]
pub mod mm_berr_cfg2;
#[doc = "MM_BERR_CFG3 (r) register accessor: Multimedia Bus Error Source and ID Register (MM CPU and MM Infra)\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_berr_cfg3`] module"]
#[doc(alias = "MM_BERR_CFG3")]
pub type MmBerrCfg3 = crate::Reg<mm_berr_cfg3::MmBerrCfg3Spec>;
#[doc = "Multimedia Bus Error Source and ID Register (MM CPU and MM Infra)"]
pub mod mm_berr_cfg3;
#[doc = "MM_BERR_CFG4 (r) register accessor: Multimedia Main Bus Error Address Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_berr_cfg4`] module"]
#[doc(alias = "MM_BERR_CFG4")]
pub type MmBerrCfg4 = crate::Reg<mm_berr_cfg4::MmBerrCfg4Spec>;
#[doc = "Multimedia Main Bus Error Address Capture"]
pub mod mm_berr_cfg4;
#[doc = "MM_BERR_CFG5 (r) register accessor: Multimedia CODEC Bus Error Address Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_berr_cfg5`] module"]
#[doc(alias = "MM_BERR_CFG5")]
pub type MmBerrCfg5 = crate::Reg<mm_berr_cfg5::MmBerrCfg5Spec>;
#[doc = "Multimedia CODEC Bus Error Address Capture"]
pub mod mm_berr_cfg5;
#[doc = "MM_BERR_CFG6 (r) register accessor: Multimedia MM CPU Bus Error Address Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_berr_cfg6`] module"]
#[doc(alias = "MM_BERR_CFG6")]
pub type MmBerrCfg6 = crate::Reg<mm_berr_cfg6::MmBerrCfg6Spec>;
#[doc = "Multimedia MM CPU Bus Error Address Capture"]
pub mod mm_berr_cfg6;
#[doc = "MM_BERR_CFG7 (r) register accessor: Multimedia Infrastructure Bus Error Address Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_berr_cfg7`] module"]
#[doc(alias = "MM_BERR_CFG7")]
pub type MmBerrCfg7 = crate::Reg<mm_berr_cfg7::MmBerrCfg7Spec>;
#[doc = "Multimedia Infrastructure Bus Error Address Capture"]
pub mod mm_berr_cfg7;
