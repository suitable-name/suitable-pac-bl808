#[repr(C)]
#[doc = "PWM Group (Multi-Channel Block)"]
#[doc(alias = "GRP")]
pub struct Grp {
    config0: Config0,
    config1: Config1,
    period: Period,
    dead_time: DeadTime,
    thre: [Thre; 4],
    int_sts: IntSts,
    int_mask: IntMask,
    int_clear: IntClear,
    int_en: IntEn,
}
impl Grp {
    #[doc = "0x00 - PWM Configuration Register 0"]
    #[inline(always)]
    pub const fn config0(&self) -> &Config0 {
        &self.config0
    }
    #[doc = "0x04 - PWM Configuration Register 1"]
    #[inline(always)]
    pub const fn config1(&self) -> &Config1 {
        &self.config1
    }
    #[doc = "0x08 - PWM Period and Interrupt Configuration"]
    #[inline(always)]
    pub const fn period(&self) -> &Period {
        &self.period
    }
    #[doc = "0x0c - PWM Dead-Time Configuration"]
    #[inline(always)]
    pub const fn dead_time(&self) -> &DeadTime {
        &self.dead_time
    }
    #[doc = "0x10..0x20 - PWM Channel %s Threshold"]
    #[inline(always)]
    pub const fn thre(&self, n: usize) -> &Thre {
        &self.thre[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - PWM Channel %s Threshold"]
    #[inline(always)]
    pub fn thre_iter(&self) -> impl Iterator<Item = &Thre> {
        self.thre.iter()
    }
    #[doc = "0x20 - PWM Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_sts(&self) -> &IntSts {
        &self.int_sts
    }
    #[doc = "0x24 - PWM Interrupt Mask Register"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0x28 - PWM Interrupt Clear Register"]
    #[inline(always)]
    pub const fn int_clear(&self) -> &IntClear {
        &self.int_clear
    }
    #[doc = "0x2c - PWM Interrupt Enable Register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
}
#[doc = "CONFIG0 (rw) register accessor: PWM Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`config0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config0`] module"]
#[doc(alias = "CONFIG0")]
pub type Config0 = crate::Reg<config0::Config0Spec>;
#[doc = "PWM Configuration Register 0"]
pub mod config0;
#[doc = "CONFIG1 (rw) register accessor: PWM Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config1`] module"]
#[doc(alias = "CONFIG1")]
pub type Config1 = crate::Reg<config1::Config1Spec>;
#[doc = "PWM Configuration Register 1"]
pub mod config1;
#[doc = "PERIOD (rw) register accessor: PWM Period and Interrupt Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period`] module"]
#[doc(alias = "PERIOD")]
pub type Period = crate::Reg<period::PeriodSpec>;
#[doc = "PWM Period and Interrupt Configuration"]
pub mod period;
#[doc = "DEAD_TIME (rw) register accessor: PWM Dead-Time Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dead_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dead_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dead_time`] module"]
#[doc(alias = "DEAD_TIME")]
pub type DeadTime = crate::Reg<dead_time::DeadTimeSpec>;
#[doc = "PWM Dead-Time Configuration"]
pub mod dead_time;
#[doc = "THRE (rw) register accessor: PWM Channel %s Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`thre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thre`] module"]
#[doc(alias = "THRE")]
pub type Thre = crate::Reg<thre::ThreSpec>;
#[doc = "PWM Channel %s Threshold"]
pub mod thre;
#[doc = "INT_STS (r) register accessor: PWM Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_sts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_sts`] module"]
#[doc(alias = "INT_STS")]
pub type IntSts = crate::Reg<int_sts::IntStsSpec>;
#[doc = "PWM Interrupt Status Register"]
pub mod int_sts;
#[doc = "INT_MASK (rw) register accessor: PWM Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`] module"]
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "PWM Interrupt Mask Register"]
pub mod int_mask;
#[doc = "INT_CLEAR (rw) register accessor: PWM Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`] module"]
#[doc(alias = "INT_CLEAR")]
pub type IntClear = crate::Reg<int_clear::IntClearSpec>;
#[doc = "PWM Interrupt Clear Register"]
pub mod int_clear;
#[doc = "INT_EN (rw) register accessor: PWM Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`] module"]
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
#[doc = "PWM Interrupt Enable Register"]
pub mod int_en;
