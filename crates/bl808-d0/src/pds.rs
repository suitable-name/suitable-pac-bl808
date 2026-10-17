#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pds_ctl: PdsCtl,
    pds_time1: PdsTime1,
    _reserved2: [u8; 0x04],
    pds_int: PdsInt,
    pds_ctl2: PdsCtl2,
    pds_ctl3: PdsCtl3,
    pds_ctl4: PdsCtl4,
    pds_stat: PdsStat,
    pds_ram1: PdsRam1,
    pds_ctl5: PdsCtl5,
    pds_ram2: PdsRam2,
    _reserved10: [u8; 0x04],
    pds_gpio_i_set: PdsGpioISet,
    pds_gpio_pd_set: PdsGpioPdSet,
    _reserved12: [u8; 0x08],
    pds_gpio_int: PdsGpioInt,
    pds_gpio_stat: PdsGpioStat,
    _reserved14: [u8; 0xc8],
    cpu_core_cfg0: CpuCoreCfg0,
    cpu_core_cfg1: CpuCoreCfg1,
    _reserved16: [u8; 0x14],
    cpu_core_cfg7: CpuCoreCfg7,
    cpu_core_cfg8: CpuCoreCfg8,
    cpu_core_cfg9: CpuCoreCfg9,
    cpu_core_cfg10: CpuCoreCfg10,
    _reserved20: [u8; 0x04],
    cpu_core_cfg12: CpuCoreCfg12,
    cpu_core_cfg13: CpuCoreCfg13,
    cpu_core_cfg14: CpuCoreCfg14,
    tzc_pds: TzcPds,
    _reserved24: [u8; 0x01b0],
    rc32m_ctrl0: Rc32mCtrl0,
    rc32m_ctrl1: Rc32mCtrl1,
    _reserved26: [u8; 0xf8],
    pu_rst_clkpll: PuRstClkpll,
    _reserved27: [u8; 0xfc],
    usb_ctl: UsbCtl,
    usb_phy_ctrl: UsbPhyCtrl,
    _reserved29: [u8; 0x04f8],
    touch1: Touch1,
    touch2: Touch2,
    touch3: Touch3,
    touch_sleep_time: TouchSleepTime,
    touch_data_hystersis: TouchDataHystersis,
    force_data: [ForceData; 6],
    channel_vth_data_0: ChannelVthData0,
    channel_vth_data_1: ChannelVthData1,
    channel_vth_data_2: ChannelVthData2,
    raw_data: [RawData; 12],
    lta_data: [LtaData; 12],
    flt_data: [FltData; 12],
    touch_rsvd: TouchRsvd,
    touch_int_setting: TouchIntSetting,
    touch_int_status: TouchIntStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - PDS Global Control Register"]
    #[inline(always)]
    pub const fn pds_ctl(&self) -> &PdsCtl {
        &self.pds_ctl
    }
    #[doc = "0x04 - PDS Sleep Duration Register"]
    #[inline(always)]
    pub const fn pds_time1(&self) -> &PdsTime1 {
        &self.pds_time1
    }
    #[doc = "0x0c - PDS Interrupt Status and Mask Register"]
    #[inline(always)]
    pub const fn pds_int(&self) -> &PdsInt {
        &self.pds_int
    }
    #[doc = "0x10 - PDS Force Sleep Control Register 2"]
    #[inline(always)]
    pub const fn pds_ctl2(&self) -> &PdsCtl2 {
        &self.pds_ctl2
    }
    #[doc = "0x14 - PDS Force Sleep Control Register 3"]
    #[inline(always)]
    pub const fn pds_ctl3(&self) -> &PdsCtl3 {
        &self.pds_ctl3
    }
    #[doc = "0x18 - PDS Individual Domain Control Register 4"]
    #[inline(always)]
    pub const fn pds_ctl4(&self) -> &PdsCtl4 {
        &self.pds_ctl4
    }
    #[doc = "0x1c - PDS Status Register"]
    #[inline(always)]
    pub const fn pds_stat(&self) -> &PdsStat {
        &self.pds_stat
    }
    #[doc = "0x20 - PDS RAM Control Register 1"]
    #[inline(always)]
    pub const fn pds_ram1(&self) -> &PdsRam1 {
        &self.pds_ram1
    }
    #[doc = "0x24 - PDS Control Register 5"]
    #[inline(always)]
    pub const fn pds_ctl5(&self) -> &PdsCtl5 {
        &self.pds_ctl5
    }
    #[doc = "0x28 - PDS RAM Control Register 2"]
    #[inline(always)]
    pub const fn pds_ram2(&self) -> &PdsRam2 {
        &self.pds_ram2
    }
    #[doc = "0x30 - PDS GPIO Configuration Register"]
    #[inline(always)]
    pub const fn pds_gpio_i_set(&self) -> &PdsGpioISet {
        &self.pds_gpio_i_set
    }
    #[doc = "0x34 - PDS GPIO Interrupt Mask"]
    #[inline(always)]
    pub const fn pds_gpio_pd_set(&self) -> &PdsGpioPdSet {
        &self.pds_gpio_pd_set
    }
    #[doc = "0x40 - PDS GPIO Interrupt Mode and Clear"]
    #[inline(always)]
    pub const fn pds_gpio_int(&self) -> &PdsGpioInt {
        &self.pds_gpio_int
    }
    #[doc = "0x44 - PDS GPIO Interrupt Status"]
    #[inline(always)]
    pub const fn pds_gpio_stat(&self) -> &PdsGpioStat {
        &self.pds_gpio_stat
    }
    #[doc = "0x110 - E902 (PICO) core clock and DFS control"]
    #[inline(always)]
    pub const fn cpu_core_cfg0(&self) -> &CpuCoreCfg0 {
        &self.cpu_core_cfg0
    }
    #[doc = "0x114 - C906 core clock enable and PLL selection"]
    #[inline(always)]
    pub const fn cpu_core_cfg1(&self) -> &CpuCoreCfg1 {
        &self.cpu_core_cfg1
    }
    #[doc = "0x12c - E902 (PICO) core clock divider and low power state"]
    #[inline(always)]
    pub const fn cpu_core_cfg7(&self) -> &CpuCoreCfg7 {
        &self.cpu_core_cfg7
    }
    #[doc = "0x130 - E902 (PICO) RTC configuration"]
    #[inline(always)]
    pub const fn cpu_core_cfg8(&self) -> &CpuCoreCfg8 {
        &self.cpu_core_cfg8
    }
    #[doc = "0x134 - PICO RTC Counter Low"]
    #[inline(always)]
    pub const fn cpu_core_cfg9(&self) -> &CpuCoreCfg9 {
        &self.cpu_core_cfg9
    }
    #[doc = "0x138 - PICO RTC Counter High"]
    #[inline(always)]
    pub const fn cpu_core_cfg10(&self) -> &CpuCoreCfg10 {
        &self.cpu_core_cfg10
    }
    #[doc = "0x140 - E902 (PICO) I-AHB-Lite bus remapping/masking"]
    #[inline(always)]
    pub const fn cpu_core_cfg12(&self) -> &CpuCoreCfg12 {
        &self.cpu_core_cfg12
    }
    #[doc = "0x144 - E902 Reset Address"]
    #[inline(always)]
    pub const fn cpu_core_cfg13(&self) -> &CpuCoreCfg13 {
        &self.cpu_core_cfg13
    }
    #[doc = "0x148 - E906/C906 Reset Address"]
    #[inline(always)]
    pub const fn cpu_core_cfg14(&self) -> &CpuCoreCfg14 {
        &self.cpu_core_cfg14
    }
    #[doc = "0x14c - TrustZone write lock for CPU config registers"]
    #[inline(always)]
    pub const fn tzc_pds(&self) -> &TzcPds {
        &self.tzc_pds
    }
    #[doc = "0x300 - RC 32MHz oscillator control and calibration"]
    #[inline(always)]
    pub const fn rc32m_ctrl0(&self) -> &Rc32mCtrl0 {
        &self.rc32m_ctrl0
    }
    #[doc = "0x304 - RC 32MHz oscillator software reset and clock control"]
    #[inline(always)]
    pub const fn rc32m_ctrl1(&self) -> &Rc32mCtrl1 {
        &self.rc32m_ctrl1
    }
    #[doc = "0x400 - Clock PLL power up and reset control"]
    #[inline(always)]
    pub const fn pu_rst_clkpll(&self) -> &PuRstClkpll {
        &self.pu_rst_clkpll
    }
    #[doc = "0x500 - USB top-level software control"]
    #[inline(always)]
    pub const fn usb_ctl(&self) -> &UsbCtl {
        &self.usb_ctl
    }
    #[doc = "0x504 - USB PHY analog control"]
    #[inline(always)]
    pub const fn usb_phy_ctrl(&self) -> &UsbPhyCtrl {
        &self.usb_phy_ctrl
    }
    #[doc = "0xa00 - Touch Configuration 1"]
    #[inline(always)]
    pub const fn touch1(&self) -> &Touch1 {
        &self.touch1
    }
    #[doc = "0xa04 - Touch Configuration 2 (Channel High-Z)"]
    #[inline(always)]
    pub const fn touch2(&self) -> &Touch2 {
        &self.touch2
    }
    #[doc = "0xa08 - Touch Configuration 3 (Data Process)"]
    #[inline(always)]
    pub const fn touch3(&self) -> &Touch3 {
        &self.touch3
    }
    #[doc = "0xa0c - Touch Sleep Time Configuration"]
    #[inline(always)]
    pub const fn touch_sleep_time(&self) -> &TouchSleepTime {
        &self.touch_sleep_time
    }
    #[doc = "0xa10 - Touch Data Hysteresis Configuration"]
    #[inline(always)]
    pub const fn touch_data_hystersis(&self) -> &TouchDataHystersis {
        &self.touch_data_hystersis
    }
    #[doc = "0xa14..0xa2c - Touch Force Data Registers"]
    #[inline(always)]
    pub const fn force_data(&self, n: usize) -> &ForceData {
        &self.force_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa14..0xa2c - Touch Force Data Registers"]
    #[inline(always)]
    pub fn force_data_iter(&self) -> impl Iterator<Item = &ForceData> {
        self.force_data.iter()
    }
    #[doc = "0xa2c - Touch Channel 0-3 Threshold Configuration"]
    #[inline(always)]
    pub const fn channel_vth_data_0(&self) -> &ChannelVthData0 {
        &self.channel_vth_data_0
    }
    #[doc = "0xa30 - Touch Channel 4-7 Threshold Configuration"]
    #[inline(always)]
    pub const fn channel_vth_data_1(&self) -> &ChannelVthData1 {
        &self.channel_vth_data_1
    }
    #[doc = "0xa34 - Touch Channel 8-11 Threshold Configuration"]
    #[inline(always)]
    pub const fn channel_vth_data_2(&self) -> &ChannelVthData2 {
        &self.channel_vth_data_2
    }
    #[doc = "0xa38..0xa68 - Touch Raw Data Status Registers"]
    #[inline(always)]
    pub const fn raw_data(&self, n: usize) -> &RawData {
        &self.raw_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa38..0xa68 - Touch Raw Data Status Registers"]
    #[inline(always)]
    pub fn raw_data_iter(&self) -> impl Iterator<Item = &RawData> {
        self.raw_data.iter()
    }
    #[doc = "0xa68..0xa98 - Touch Long-Term Average Data Registers"]
    #[inline(always)]
    pub const fn lta_data(&self, n: usize) -> &LtaData {
        &self.lta_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa68..0xa98 - Touch Long-Term Average Data Registers"]
    #[inline(always)]
    pub fn lta_data_iter(&self) -> impl Iterator<Item = &LtaData> {
        self.lta_data.iter()
    }
    #[doc = "0xa98..0xac8 - Touch Filtered Data Registers"]
    #[inline(always)]
    pub const fn flt_data(&self, n: usize) -> &FltData {
        &self.flt_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa98..0xac8 - Touch Filtered Data Registers"]
    #[inline(always)]
    pub fn flt_data_iter(&self) -> impl Iterator<Item = &FltData> {
        self.flt_data.iter()
    }
    #[doc = "0xac8 - Touch Reserved Register"]
    #[inline(always)]
    pub const fn touch_rsvd(&self) -> &TouchRsvd {
        &self.touch_rsvd
    }
    #[doc = "0xacc - Touch sensor interrupt configuration"]
    #[inline(always)]
    pub const fn touch_int_setting(&self) -> &TouchIntSetting {
        &self.touch_int_setting
    }
    #[doc = "0xad0 - Touch sensor interrupt status"]
    #[inline(always)]
    pub const fn touch_int_status(&self) -> &TouchIntStatus {
        &self.touch_int_status
    }
}
#[doc = "PDS_CTL (rw) register accessor: PDS Global Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl`] module"]
#[doc(alias = "PDS_CTL")]
pub type PdsCtl = crate::Reg<pds_ctl::PdsCtlSpec>;
#[doc = "PDS Global Control Register"]
pub mod pds_ctl;
#[doc = "PDS_TIME1 (rw) register accessor: PDS Sleep Duration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_time1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_time1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_time1`] module"]
#[doc(alias = "PDS_TIME1")]
pub type PdsTime1 = crate::Reg<pds_time1::PdsTime1Spec>;
#[doc = "PDS Sleep Duration Register"]
pub mod pds_time1;
#[doc = "PDS_INT (rw) register accessor: PDS Interrupt Status and Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_int`] module"]
#[doc(alias = "PDS_INT")]
pub type PdsInt = crate::Reg<pds_int::PdsIntSpec>;
#[doc = "PDS Interrupt Status and Mask Register"]
pub mod pds_int;
#[doc = "PDS_CTL2 (rw) register accessor: PDS Force Sleep Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl2`] module"]
#[doc(alias = "PDS_CTL2")]
pub type PdsCtl2 = crate::Reg<pds_ctl2::PdsCtl2Spec>;
#[doc = "PDS Force Sleep Control Register 2"]
pub mod pds_ctl2;
#[doc = "PDS_CTL3 (rw) register accessor: PDS Force Sleep Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl3`] module"]
#[doc(alias = "PDS_CTL3")]
pub type PdsCtl3 = crate::Reg<pds_ctl3::PdsCtl3Spec>;
#[doc = "PDS Force Sleep Control Register 3"]
pub mod pds_ctl3;
#[doc = "PDS_CTL4 (rw) register accessor: PDS Individual Domain Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl4`] module"]
#[doc(alias = "PDS_CTL4")]
pub type PdsCtl4 = crate::Reg<pds_ctl4::PdsCtl4Spec>;
#[doc = "PDS Individual Domain Control Register 4"]
pub mod pds_ctl4;
#[doc = "PDS_STAT (rw) register accessor: PDS Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_stat`] module"]
#[doc(alias = "PDS_STAT")]
pub type PdsStat = crate::Reg<pds_stat::PdsStatSpec>;
#[doc = "PDS Status Register"]
pub mod pds_stat;
#[doc = "PDS_RAM1 (rw) register accessor: PDS RAM Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ram1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ram1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ram1`] module"]
#[doc(alias = "PDS_RAM1")]
pub type PdsRam1 = crate::Reg<pds_ram1::PdsRam1Spec>;
#[doc = "PDS RAM Control Register 1"]
pub mod pds_ram1;
#[doc = "PDS_CTL5 (rw) register accessor: PDS Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl5`] module"]
#[doc(alias = "PDS_CTL5")]
pub type PdsCtl5 = crate::Reg<pds_ctl5::PdsCtl5Spec>;
#[doc = "PDS Control Register 5"]
pub mod pds_ctl5;
#[doc = "PDS_RAM2 (rw) register accessor: PDS RAM Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_ram2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_ram2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ram2`] module"]
#[doc(alias = "PDS_RAM2")]
pub type PdsRam2 = crate::Reg<pds_ram2::PdsRam2Spec>;
#[doc = "PDS RAM Control Register 2"]
pub mod pds_ram2;
#[doc = "PDS_GPIO_I_SET (rw) register accessor: PDS GPIO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_gpio_i_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_gpio_i_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_gpio_i_set`] module"]
#[doc(alias = "PDS_GPIO_I_SET")]
pub type PdsGpioISet = crate::Reg<pds_gpio_i_set::PdsGpioISetSpec>;
#[doc = "PDS GPIO Configuration Register"]
pub mod pds_gpio_i_set;
#[doc = "PDS_GPIO_PD_SET (rw) register accessor: PDS GPIO Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_gpio_pd_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_gpio_pd_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_gpio_pd_set`] module"]
#[doc(alias = "PDS_GPIO_PD_SET")]
pub type PdsGpioPdSet = crate::Reg<pds_gpio_pd_set::PdsGpioPdSetSpec>;
#[doc = "PDS GPIO Interrupt Mask"]
pub mod pds_gpio_pd_set;
#[doc = "PDS_GPIO_INT (rw) register accessor: PDS GPIO Interrupt Mode and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_gpio_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_gpio_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_gpio_int`] module"]
#[doc(alias = "PDS_GPIO_INT")]
pub type PdsGpioInt = crate::Reg<pds_gpio_int::PdsGpioIntSpec>;
#[doc = "PDS GPIO Interrupt Mode and Clear"]
pub mod pds_gpio_int;
#[doc = "PDS_GPIO_STAT (rw) register accessor: PDS GPIO Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_gpio_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_gpio_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_gpio_stat`] module"]
#[doc(alias = "PDS_GPIO_STAT")]
pub type PdsGpioStat = crate::Reg<pds_gpio_stat::PdsGpioStatSpec>;
#[doc = "PDS GPIO Interrupt Status"]
pub mod pds_gpio_stat;
#[doc = "CPU_CORE_CFG0 (rw) register accessor: E902 (PICO) core clock and DFS control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg0`] module"]
#[doc(alias = "CPU_CORE_CFG0")]
pub type CpuCoreCfg0 = crate::Reg<cpu_core_cfg0::CpuCoreCfg0Spec>;
#[doc = "E902 (PICO) core clock and DFS control"]
pub mod cpu_core_cfg0;
#[doc = "CPU_CORE_CFG1 (rw) register accessor: C906 core clock enable and PLL selection\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg1`] module"]
#[doc(alias = "CPU_CORE_CFG1")]
pub type CpuCoreCfg1 = crate::Reg<cpu_core_cfg1::CpuCoreCfg1Spec>;
#[doc = "C906 core clock enable and PLL selection"]
pub mod cpu_core_cfg1;
#[doc = "CPU_CORE_CFG7 (rw) register accessor: E902 (PICO) core clock divider and low power state\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg7`] module"]
#[doc(alias = "CPU_CORE_CFG7")]
pub type CpuCoreCfg7 = crate::Reg<cpu_core_cfg7::CpuCoreCfg7Spec>;
#[doc = "E902 (PICO) core clock divider and low power state"]
pub mod cpu_core_cfg7;
#[doc = "CPU_CORE_CFG8 (rw) register accessor: E902 (PICO) RTC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg8`] module"]
#[doc(alias = "CPU_CORE_CFG8")]
pub type CpuCoreCfg8 = crate::Reg<cpu_core_cfg8::CpuCoreCfg8Spec>;
#[doc = "E902 (PICO) RTC configuration"]
pub mod cpu_core_cfg8;
#[doc = "CPU_CORE_CFG9 (r) register accessor: PICO RTC Counter Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg9`] module"]
#[doc(alias = "CPU_CORE_CFG9")]
pub type CpuCoreCfg9 = crate::Reg<cpu_core_cfg9::CpuCoreCfg9Spec>;
#[doc = "PICO RTC Counter Low"]
pub mod cpu_core_cfg9;
#[doc = "CPU_CORE_CFG10 (r) register accessor: PICO RTC Counter High\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg10`] module"]
#[doc(alias = "CPU_CORE_CFG10")]
pub type CpuCoreCfg10 = crate::Reg<cpu_core_cfg10::CpuCoreCfg10Spec>;
#[doc = "PICO RTC Counter High"]
pub mod cpu_core_cfg10;
#[doc = "CPU_CORE_CFG12 (rw) register accessor: E902 (PICO) I-AHB-Lite bus remapping/masking\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg12`] module"]
#[doc(alias = "CPU_CORE_CFG12")]
pub type CpuCoreCfg12 = crate::Reg<cpu_core_cfg12::CpuCoreCfg12Spec>;
#[doc = "E902 (PICO) I-AHB-Lite bus remapping/masking"]
pub mod cpu_core_cfg12;
#[doc = "CPU_CORE_CFG13 (rw) register accessor: E902 Reset Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg13`] module"]
#[doc(alias = "CPU_CORE_CFG13")]
pub type CpuCoreCfg13 = crate::Reg<cpu_core_cfg13::CpuCoreCfg13Spec>;
#[doc = "E902 Reset Address"]
pub mod cpu_core_cfg13;
#[doc = "CPU_CORE_CFG14 (rw) register accessor: E906/C906 Reset Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg14`] module"]
#[doc(alias = "CPU_CORE_CFG14")]
pub type CpuCoreCfg14 = crate::Reg<cpu_core_cfg14::CpuCoreCfg14Spec>;
#[doc = "E906/C906 Reset Address"]
pub mod cpu_core_cfg14;
#[doc = "TZC_PDS (rw) register accessor: TrustZone write lock for CPU config registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_pds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_pds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_pds`] module"]
#[doc(alias = "TZC_PDS")]
pub type TzcPds = crate::Reg<tzc_pds::TzcPdsSpec>;
#[doc = "TrustZone write lock for CPU config registers"]
pub mod tzc_pds;
#[doc = "RC32M_CTRL0 (rw) register accessor: RC 32MHz oscillator control and calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32m_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32m_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32m_ctrl0`] module"]
#[doc(alias = "RC32M_CTRL0")]
pub type Rc32mCtrl0 = crate::Reg<rc32m_ctrl0::Rc32mCtrl0Spec>;
#[doc = "RC 32MHz oscillator control and calibration"]
pub mod rc32m_ctrl0;
#[doc = "RC32M_CTRL1 (rw) register accessor: RC 32MHz oscillator software reset and clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32m_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32m_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32m_ctrl1`] module"]
#[doc(alias = "RC32M_CTRL1")]
pub type Rc32mCtrl1 = crate::Reg<rc32m_ctrl1::Rc32mCtrl1Spec>;
#[doc = "RC 32MHz oscillator software reset and clock control"]
pub mod rc32m_ctrl1;
#[doc = "PU_RST_CLKPLL (rw) register accessor: Clock PLL power up and reset control\n\nYou can [`read`](crate::Reg::read) this register and get [`pu_rst_clkpll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pu_rst_clkpll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pu_rst_clkpll`] module"]
#[doc(alias = "PU_RST_CLKPLL")]
pub type PuRstClkpll = crate::Reg<pu_rst_clkpll::PuRstClkpllSpec>;
#[doc = "Clock PLL power up and reset control"]
pub mod pu_rst_clkpll;
#[doc = "USB_CTL (rw) register accessor: USB top-level software control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ctl`] module"]
#[doc(alias = "USB_CTL")]
pub type UsbCtl = crate::Reg<usb_ctl::UsbCtlSpec>;
#[doc = "USB top-level software control"]
pub mod usb_ctl;
#[doc = "USB_PHY_CTRL (rw) register accessor: USB PHY analog control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_phy_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_phy_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_phy_ctrl`] module"]
#[doc(alias = "USB_PHY_CTRL")]
pub type UsbPhyCtrl = crate::Reg<usb_phy_ctrl::UsbPhyCtrlSpec>;
#[doc = "USB PHY analog control"]
pub mod usb_phy_ctrl;
#[doc = "TOUCH1 (rw) register accessor: Touch Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`touch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch1`] module"]
#[doc(alias = "TOUCH1")]
pub type Touch1 = crate::Reg<touch1::Touch1Spec>;
#[doc = "Touch Configuration 1"]
pub mod touch1;
#[doc = "TOUCH2 (rw) register accessor: Touch Configuration 2 (Channel High-Z)\n\nYou can [`read`](crate::Reg::read) this register and get [`touch2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch2`] module"]
#[doc(alias = "TOUCH2")]
pub type Touch2 = crate::Reg<touch2::Touch2Spec>;
#[doc = "Touch Configuration 2 (Channel High-Z)"]
pub mod touch2;
#[doc = "TOUCH3 (rw) register accessor: Touch Configuration 3 (Data Process)\n\nYou can [`read`](crate::Reg::read) this register and get [`touch3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch3`] module"]
#[doc(alias = "TOUCH3")]
pub type Touch3 = crate::Reg<touch3::Touch3Spec>;
#[doc = "Touch Configuration 3 (Data Process)"]
pub mod touch3;
#[doc = "TOUCH_SLEEP_TIME (rw) register accessor: Touch Sleep Time Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_sleep_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_sleep_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_sleep_time`] module"]
#[doc(alias = "TOUCH_SLEEP_TIME")]
pub type TouchSleepTime = crate::Reg<touch_sleep_time::TouchSleepTimeSpec>;
#[doc = "Touch Sleep Time Configuration"]
pub mod touch_sleep_time;
#[doc = "TOUCH_DATA_HYSTERSIS (rw) register accessor: Touch Data Hysteresis Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_data_hystersis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_data_hystersis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_data_hystersis`] module"]
#[doc(alias = "TOUCH_DATA_HYSTERSIS")]
pub type TouchDataHystersis = crate::Reg<touch_data_hystersis::TouchDataHystersisSpec>;
#[doc = "Touch Data Hysteresis Configuration"]
pub mod touch_data_hystersis;
#[doc = "Touch Force Data Registers"]
pub use self::force_data::ForceData;
#[doc = r"Cluster"]
#[doc = "Touch Force Data Registers"]
pub mod force_data;
#[doc = "CHANNEL_VTH_DATA_0 (rw) register accessor: Touch Channel 0-3 Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_vth_data_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel_vth_data_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_vth_data_0`] module"]
#[doc(alias = "CHANNEL_VTH_DATA_0")]
pub type ChannelVthData0 = crate::Reg<channel_vth_data_0::ChannelVthData0Spec>;
#[doc = "Touch Channel 0-3 Threshold Configuration"]
pub mod channel_vth_data_0;
#[doc = "CHANNEL_VTH_DATA_1 (rw) register accessor: Touch Channel 4-7 Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_vth_data_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel_vth_data_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_vth_data_1`] module"]
#[doc(alias = "CHANNEL_VTH_DATA_1")]
pub type ChannelVthData1 = crate::Reg<channel_vth_data_1::ChannelVthData1Spec>;
#[doc = "Touch Channel 4-7 Threshold Configuration"]
pub mod channel_vth_data_1;
#[doc = "CHANNEL_VTH_DATA_2 (rw) register accessor: Touch Channel 8-11 Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_vth_data_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel_vth_data_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_vth_data_2`] module"]
#[doc(alias = "CHANNEL_VTH_DATA_2")]
pub type ChannelVthData2 = crate::Reg<channel_vth_data_2::ChannelVthData2Spec>;
#[doc = "Touch Channel 8-11 Threshold Configuration"]
pub mod channel_vth_data_2;
#[doc = "Touch Raw Data Status Registers"]
pub use self::raw_data::RawData;
#[doc = r"Cluster"]
#[doc = "Touch Raw Data Status Registers"]
pub mod raw_data;
#[doc = "Touch Long-Term Average Data Registers"]
pub use self::lta_data::LtaData;
#[doc = r"Cluster"]
#[doc = "Touch Long-Term Average Data Registers"]
pub mod lta_data;
#[doc = "Touch Filtered Data Registers"]
pub use self::flt_data::FltData;
#[doc = r"Cluster"]
#[doc = "Touch Filtered Data Registers"]
pub mod flt_data;
#[doc = "TOUCH_RSVD (rw) register accessor: Touch Reserved Register\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_rsvd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_rsvd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_rsvd`] module"]
#[doc(alias = "TOUCH_RSVD")]
pub type TouchRsvd = crate::Reg<touch_rsvd::TouchRsvdSpec>;
#[doc = "Touch Reserved Register"]
pub mod touch_rsvd;
#[doc = "TOUCH_INT_SETTING (rw) register accessor: Touch sensor interrupt configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_int_setting::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_int_setting::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_int_setting`] module"]
#[doc(alias = "TOUCH_INT_SETTING")]
pub type TouchIntSetting = crate::Reg<touch_int_setting::TouchIntSettingSpec>;
#[doc = "Touch sensor interrupt configuration"]
pub mod touch_int_setting;
#[doc = "TOUCH_INT_STATUS (r) register accessor: Touch sensor interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_int_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_int_status`] module"]
#[doc(alias = "TOUCH_INT_STATUS")]
pub type TouchIntStatus = crate::Reg<touch_int_status::TouchIntStatusSpec>;
#[doc = "Touch sensor interrupt status"]
pub mod touch_int_status;
