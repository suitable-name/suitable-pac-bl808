#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mm_clk_ctrl_cpu: MmClkCtrlCpu,
    mm_clk_cpu: MmClkCpu,
    dp_clk: DpClk,
    codec_clk: CodecClk,
    mm_clk_ctrl_peri: MmClkCtrlPeri,
    _reserved5: [u8; 0x04],
    mm_clk_ctrl_peri3: MmClkCtrlPeri3,
    _reserved6: [u8; 0x24],
    mm_sw_sys_reset: MmSwSysReset,
    sw_reset_mm_peri: SwResetMmPeri,
    sw_reset_sub: SwResetSub,
    sw_reset_codec_sub: SwResetCodecSub,
    image_sensor_ctrl: ImageSensorCtrl,
    _reserved11: [u8; 0x0c],
    tz_mm_clkrst: TzMmClkrst,
}
impl RegisterBlock {
    #[doc = "0x00 - Multimedia CPU Clock Control"]
    #[inline(always)]
    pub const fn mm_clk_ctrl_cpu(&self) -> &MmClkCtrlCpu {
        &self.mm_clk_ctrl_cpu
    }
    #[doc = "0x04 - Multimedia CPU and CNN Clock Dividers"]
    #[inline(always)]
    pub const fn mm_clk_cpu(&self) -> &MmClkCpu {
        &self.mm_clk_cpu
    }
    #[doc = "0x08 - Display and Pixel Clock Configuration"]
    #[inline(always)]
    pub const fn dp_clk(&self) -> &DpClk {
        &self.dp_clk
    }
    #[doc = "0x0c - Codec/H264 Clock Configuration"]
    #[inline(always)]
    pub const fn codec_clk(&self) -> &CodecClk {
        &self.codec_clk
    }
    #[doc = "0x10 - MM Peripheral Clock Control 0"]
    #[inline(always)]
    pub const fn mm_clk_ctrl_peri(&self) -> &MmClkCtrlPeri {
        &self.mm_clk_ctrl_peri
    }
    #[doc = "0x18 - MM Peripheral Clock Control 1"]
    #[inline(always)]
    pub const fn mm_clk_ctrl_peri3(&self) -> &MmClkCtrlPeri3 {
        &self.mm_clk_ctrl_peri3
    }
    #[doc = "0x40 - Multimedia System Reset Control"]
    #[inline(always)]
    pub const fn mm_sw_sys_reset(&self) -> &MmSwSysReset {
        &self.mm_sw_sys_reset
    }
    #[doc = "0x44 - MM Peripheral Software Reset"]
    #[inline(always)]
    pub const fn sw_reset_mm_peri(&self) -> &SwResetMmPeri {
        &self.sw_reset_mm_peri
    }
    #[doc = "0x48 - MM Subsystem Software Reset"]
    #[inline(always)]
    pub const fn sw_reset_sub(&self) -> &SwResetSub {
        &self.sw_reset_sub
    }
    #[doc = "0x4c - Codec Subsystem Software Reset"]
    #[inline(always)]
    pub const fn sw_reset_codec_sub(&self) -> &SwResetCodecSub {
        &self.sw_reset_codec_sub
    }
    #[doc = "0x50 - Image Sensor Reset Control"]
    #[inline(always)]
    pub const fn image_sensor_ctrl(&self) -> &ImageSensorCtrl {
        &self.image_sensor_ctrl
    }
    #[doc = "0x60 - TrustZone MM Clock/Reset Lock Status"]
    #[inline(always)]
    pub const fn tz_mm_clkrst(&self) -> &TzMmClkrst {
        &self.tz_mm_clkrst
    }
}
#[doc = "MM_CLK_CTRL_CPU (rw) register accessor: Multimedia CPU Clock Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_clk_ctrl_cpu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_clk_ctrl_cpu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_clk_ctrl_cpu`] module"]
#[doc(alias = "MM_CLK_CTRL_CPU")]
pub type MmClkCtrlCpu = crate::Reg<mm_clk_ctrl_cpu::MmClkCtrlCpuSpec>;
#[doc = "Multimedia CPU Clock Control"]
pub mod mm_clk_ctrl_cpu;
#[doc = "MM_CLK_CPU (rw) register accessor: Multimedia CPU and CNN Clock Dividers\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_clk_cpu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_clk_cpu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_clk_cpu`] module"]
#[doc(alias = "MM_CLK_CPU")]
pub type MmClkCpu = crate::Reg<mm_clk_cpu::MmClkCpuSpec>;
#[doc = "Multimedia CPU and CNN Clock Dividers"]
pub mod mm_clk_cpu;
#[doc = "DP_CLK (rw) register accessor: Display and Pixel Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dp_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dp_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_clk`] module"]
#[doc(alias = "DP_CLK")]
pub type DpClk = crate::Reg<dp_clk::DpClkSpec>;
#[doc = "Display and Pixel Clock Configuration"]
pub mod dp_clk;
#[doc = "CODEC_CLK (rw) register accessor: Codec/H264 Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codec_clk`] module"]
#[doc(alias = "CODEC_CLK")]
pub type CodecClk = crate::Reg<codec_clk::CodecClkSpec>;
#[doc = "Codec/H264 Clock Configuration"]
pub mod codec_clk;
#[doc = "MM_CLK_CTRL_PERI (rw) register accessor: MM Peripheral Clock Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_clk_ctrl_peri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_clk_ctrl_peri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_clk_ctrl_peri`] module"]
#[doc(alias = "MM_CLK_CTRL_PERI")]
pub type MmClkCtrlPeri = crate::Reg<mm_clk_ctrl_peri::MmClkCtrlPeriSpec>;
#[doc = "MM Peripheral Clock Control 0"]
pub mod mm_clk_ctrl_peri;
#[doc = "MM_CLK_CTRL_PERI3 (rw) register accessor: MM Peripheral Clock Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_clk_ctrl_peri3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_clk_ctrl_peri3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_clk_ctrl_peri3`] module"]
#[doc(alias = "MM_CLK_CTRL_PERI3")]
pub type MmClkCtrlPeri3 = crate::Reg<mm_clk_ctrl_peri3::MmClkCtrlPeri3Spec>;
#[doc = "MM Peripheral Clock Control 1"]
pub mod mm_clk_ctrl_peri3;
#[doc = "MM_SW_SYS_RESET (rw) register accessor: Multimedia System Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_sw_sys_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_sw_sys_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mm_sw_sys_reset`] module"]
#[doc(alias = "MM_SW_SYS_RESET")]
pub type MmSwSysReset = crate::Reg<mm_sw_sys_reset::MmSwSysResetSpec>;
#[doc = "Multimedia System Reset Control"]
pub mod mm_sw_sys_reset;
#[doc = "SW_RESET_MM_PERI (rw) register accessor: MM Peripheral Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_reset_mm_peri::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_reset_mm_peri::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_reset_mm_peri`] module"]
#[doc(alias = "SW_RESET_MM_PERI")]
pub type SwResetMmPeri = crate::Reg<sw_reset_mm_peri::SwResetMmPeriSpec>;
#[doc = "MM Peripheral Software Reset"]
pub mod sw_reset_mm_peri;
#[doc = "SW_RESET_SUB (rw) register accessor: MM Subsystem Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_reset_sub::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_reset_sub::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_reset_sub`] module"]
#[doc(alias = "SW_RESET_SUB")]
pub type SwResetSub = crate::Reg<sw_reset_sub::SwResetSubSpec>;
#[doc = "MM Subsystem Software Reset"]
pub mod sw_reset_sub;
#[doc = "SW_RESET_CODEC_SUB (rw) register accessor: Codec Subsystem Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_reset_codec_sub::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_reset_codec_sub::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_reset_codec_sub`] module"]
#[doc(alias = "SW_RESET_CODEC_SUB")]
pub type SwResetCodecSub = crate::Reg<sw_reset_codec_sub::SwResetCodecSubSpec>;
#[doc = "Codec Subsystem Software Reset"]
pub mod sw_reset_codec_sub;
#[doc = "IMAGE_SENSOR_CTRL (rw) register accessor: Image Sensor Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`image_sensor_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`image_sensor_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@image_sensor_ctrl`] module"]
#[doc(alias = "IMAGE_SENSOR_CTRL")]
pub type ImageSensorCtrl = crate::Reg<image_sensor_ctrl::ImageSensorCtrlSpec>;
#[doc = "Image Sensor Reset Control"]
pub mod image_sensor_ctrl;
#[doc = "TZ_MM_CLKRST (r) register accessor: TrustZone MM Clock/Reset Lock Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tz_mm_clkrst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tz_mm_clkrst`] module"]
#[doc(alias = "TZ_MM_CLKRST")]
pub type TzMmClkrst = crate::Reg<tz_mm_clkrst::TzMmClkrstSpec>;
#[doc = "TrustZone MM Clock/Reset Lock Status"]
pub mod tz_mm_clkrst;
