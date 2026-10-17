#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tccr: Tccr,
    _reserved1: [u8; 0x0c],
    tmr2_: [Tmr2_; 3],
    tmr3_: [Tmr3_; 3],
    _reserved3: [u8; 0x04],
    tcr2: Tcr2,
    tcr3: Tcr3,
    _reserved5: [u8; 0x04],
    tsr2: Tsr2,
    tsr3: Tsr3,
    _reserved7: [u8; 0x04],
    tier2: Tier2,
    tier3: Tier3,
    _reserved9: [u8; 0x04],
    tplvr2: Tplvr2,
    tplvr3: Tplvr3,
    _reserved11: [u8; 0x04],
    tplcr2: Tplcr2,
    tplcr3: Tplcr3,
    wmer: Wmer,
    wmr: Wmr,
    wvr: Wvr,
    wsr: Wsr,
    _reserved17: [u8; 0x04],
    ticr2: Ticr2,
    ticr3: Ticr3,
    wicr: Wicr,
    tcer: Tcer,
    tcmr: Tcmr,
    _reserved22: [u8; 0x04],
    tilr2: Tilr2,
    tilr3: Tilr3,
    wcr: Wcr,
    wfar: Wfar,
    wsar: Wsar,
    _reserved27: [u8; 0x04],
    tcvwr2: Tcvwr2,
    tcvwr3: Tcvwr3,
    _reserved29: [u8; 0x04],
    tcvsyn2: Tcvsyn2,
    tcvsyn3: Tcvsyn3,
    tcdr: Tcdr,
    gpio: Gpio,
    gpio_lat1: GpioLat1,
    gpio_lat2: GpioLat2,
    tcdr_force: TcdrForce,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer Clock Source Configuration"]
    #[inline(always)]
    pub const fn tccr(&self) -> &Tccr {
        &self.tccr
    }
    #[doc = "0x10..0x1c - Timer 2 Match Value %s"]
    #[inline(always)]
    pub const fn tmr2_(&self, n: usize) -> &Tmr2_ {
        &self.tmr2_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x1c - Timer 2 Match Value %s"]
    #[inline(always)]
    pub fn tmr2__iter(&self) -> impl Iterator<Item = &Tmr2_> {
        self.tmr2_.iter()
    }
    #[doc = "0x1c..0x28 - Timer 3 Match Value %s"]
    #[inline(always)]
    pub const fn tmr3_(&self, n: usize) -> &Tmr3_ {
        &self.tmr3_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x28 - Timer 3 Match Value %s"]
    #[inline(always)]
    pub fn tmr3__iter(&self) -> impl Iterator<Item = &Tmr3_> {
        self.tmr3_.iter()
    }
    #[doc = "0x2c - Timer 2 Counter Value"]
    #[inline(always)]
    pub const fn tcr2(&self) -> &Tcr2 {
        &self.tcr2
    }
    #[doc = "0x30 - Timer 3 Counter Value"]
    #[inline(always)]
    pub const fn tcr3(&self) -> &Tcr3 {
        &self.tcr3
    }
    #[doc = "0x38 - Timer 2 Match Status"]
    #[inline(always)]
    pub const fn tsr2(&self) -> &Tsr2 {
        &self.tsr2
    }
    #[doc = "0x3c - Timer 3 Match Status"]
    #[inline(always)]
    pub const fn tsr3(&self) -> &Tsr3 {
        &self.tsr3
    }
    #[doc = "0x44 - Timer 2 Match Interrupt Enable"]
    #[inline(always)]
    pub const fn tier2(&self) -> &Tier2 {
        &self.tier2
    }
    #[doc = "0x48 - Timer 3 Match Interrupt Enable"]
    #[inline(always)]
    pub const fn tier3(&self) -> &Tier3 {
        &self.tier3
    }
    #[doc = "0x50 - Timer 2 Pre-Load Value"]
    #[inline(always)]
    pub const fn tplvr2(&self) -> &Tplvr2 {
        &self.tplvr2
    }
    #[doc = "0x54 - Timer 3 Pre-Load Value"]
    #[inline(always)]
    pub const fn tplvr3(&self) -> &Tplvr3 {
        &self.tplvr3
    }
    #[doc = "0x5c - Timer 2 Pre-Load Control"]
    #[inline(always)]
    pub const fn tplcr2(&self) -> &Tplcr2 {
        &self.tplcr2
    }
    #[doc = "0x60 - Timer 3 Pre-Load Control"]
    #[inline(always)]
    pub const fn tplcr3(&self) -> &Tplcr3 {
        &self.tplcr3
    }
    #[doc = "0x64 - Watchdog reset/interrupt Mode"]
    #[inline(always)]
    pub const fn wmer(&self) -> &Wmer {
        &self.wmer
    }
    #[doc = "0x68 - Watchdog Match Value"]
    #[inline(always)]
    pub const fn wmr(&self) -> &Wmr {
        &self.wmr
    }
    #[doc = "0x6c - Watchdog Counter Value"]
    #[inline(always)]
    pub const fn wvr(&self) -> &Wvr {
        &self.wvr
    }
    #[doc = "0x70 - Watchdog Reset Status"]
    #[inline(always)]
    pub const fn wsr(&self) -> &Wsr {
        &self.wsr
    }
    #[doc = "0x78 - Timer 2 Interrupt Clear"]
    #[inline(always)]
    pub const fn ticr2(&self) -> &Ticr2 {
        &self.ticr2
    }
    #[doc = "0x7c - Timer 3 Interrupt Clear"]
    #[inline(always)]
    pub const fn ticr3(&self) -> &Ticr3 {
        &self.ticr3
    }
    #[doc = "0x80 - WDT Interrupt Clear"]
    #[inline(always)]
    pub const fn wicr(&self) -> &Wicr {
        &self.wicr
    }
    #[doc = "0x84 - Timer Counter Enable/Clear"]
    #[inline(always)]
    pub const fn tcer(&self) -> &Tcer {
        &self.tcer
    }
    #[doc = "0x88 - Timer Counter Mode"]
    #[inline(always)]
    pub const fn tcmr(&self) -> &Tcmr {
        &self.tcmr
    }
    #[doc = "0x90 - Timer 2 Match Interrupt Mode"]
    #[inline(always)]
    pub const fn tilr2(&self) -> &Tilr2 {
        &self.tilr2
    }
    #[doc = "0x94 - Timer 3 Match Interrupt Mode"]
    #[inline(always)]
    pub const fn tilr3(&self) -> &Tilr3 {
        &self.tilr3
    }
    #[doc = "0x98 - WDT Counter Reset"]
    #[inline(always)]
    pub const fn wcr(&self) -> &Wcr {
        &self.wcr
    }
    #[doc = "0x9c - WDT Access Key1 (0xBABA)"]
    #[inline(always)]
    pub const fn wfar(&self) -> &Wfar {
        &self.wfar
    }
    #[doc = "0xa0 - WDT Access Key2 (0xEB10)"]
    #[inline(always)]
    pub const fn wsar(&self) -> &Wsar {
        &self.wsar
    }
    #[doc = "0xa8 - Timer 2 Counter Latch Value"]
    #[inline(always)]
    pub const fn tcvwr2(&self) -> &Tcvwr2 {
        &self.tcvwr2
    }
    #[doc = "0xac - Timer 3 Counter Latch Value"]
    #[inline(always)]
    pub const fn tcvwr3(&self) -> &Tcvwr3 {
        &self.tcvwr3
    }
    #[doc = "0xb4 - Timer 2 Counter Sync Value"]
    #[inline(always)]
    pub const fn tcvsyn2(&self) -> &Tcvsyn2 {
        &self.tcvsyn2
    }
    #[doc = "0xb8 - Timer 3 Counter Sync Value"]
    #[inline(always)]
    pub const fn tcvsyn3(&self) -> &Tcvsyn3 {
        &self.tcvsyn3
    }
    #[doc = "0xbc - Timer and WDT Division"]
    #[inline(always)]
    pub const fn tcdr(&self) -> &Tcdr {
        &self.tcdr
    }
    #[doc = "0xc0 - GPIO Integration and Latch Mode"]
    #[inline(always)]
    pub const fn gpio(&self) -> &Gpio {
        &self.gpio
    }
    #[doc = "0xc4 - GPIO Positive Edge Latch Value"]
    #[inline(always)]
    pub const fn gpio_lat1(&self) -> &GpioLat1 {
        &self.gpio_lat1
    }
    #[doc = "0xc8 - GPIO Negative Edge Latch Value"]
    #[inline(always)]
    pub const fn gpio_lat2(&self) -> &GpioLat2 {
        &self.gpio_lat2
    }
    #[doc = "0xcc - Timer Division Force Configuration"]
    #[inline(always)]
    pub const fn tcdr_force(&self) -> &TcdrForce {
        &self.tcdr_force
    }
}
#[doc = "TCCR (rw) register accessor: Timer Clock Source Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tccr`] module"]
#[doc(alias = "TCCR")]
pub type Tccr = crate::Reg<tccr::TccrSpec>;
#[doc = "Timer Clock Source Configuration"]
pub mod tccr;
#[doc = "TMR2_ (rw) register accessor: Timer 2 Match Value %s\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr2_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr2_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr2_`] module"]
#[doc(alias = "TMR2_")]
pub type Tmr2_ = crate::Reg<tmr2_::Tmr2_Spec>;
#[doc = "Timer 2 Match Value %s"]
pub mod tmr2_;
#[doc = "TMR3_ (rw) register accessor: Timer 3 Match Value %s\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr3_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr3_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr3_`] module"]
#[doc(alias = "TMR3_")]
pub type Tmr3_ = crate::Reg<tmr3_::Tmr3_Spec>;
#[doc = "Timer 3 Match Value %s"]
pub mod tmr3_;
#[doc = "TCR2 (r) register accessor: Timer 2 Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr2`] module"]
#[doc(alias = "TCR2")]
pub type Tcr2 = crate::Reg<tcr2::Tcr2Spec>;
#[doc = "Timer 2 Counter Value"]
pub mod tcr2;
#[doc = "TCR3 (r) register accessor: Timer 3 Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr3`] module"]
#[doc(alias = "TCR3")]
pub type Tcr3 = crate::Reg<tcr3::Tcr3Spec>;
#[doc = "Timer 3 Counter Value"]
pub mod tcr3;
#[doc = "TSR2 (r) register accessor: Timer 2 Match Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr2`] module"]
#[doc(alias = "TSR2")]
pub type Tsr2 = crate::Reg<tsr2::Tsr2Spec>;
#[doc = "Timer 2 Match Status"]
pub mod tsr2;
#[doc = "TSR3 (r) register accessor: Timer 3 Match Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsr3`] module"]
#[doc(alias = "TSR3")]
pub type Tsr3 = crate::Reg<tsr3::Tsr3Spec>;
#[doc = "Timer 3 Match Status"]
pub mod tsr3;
#[doc = "TIER2 (rw) register accessor: Timer 2 Match Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`tier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tier2`] module"]
#[doc(alias = "TIER2")]
pub type Tier2 = crate::Reg<tier2::Tier2Spec>;
#[doc = "Timer 2 Match Interrupt Enable"]
pub mod tier2;
#[doc = "TIER3 (rw) register accessor: Timer 3 Match Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`tier3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tier3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tier3`] module"]
#[doc(alias = "TIER3")]
pub type Tier3 = crate::Reg<tier3::Tier3Spec>;
#[doc = "Timer 3 Match Interrupt Enable"]
pub mod tier3;
#[doc = "TPLVR2 (rw) register accessor: Timer 2 Pre-Load Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tplvr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplvr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tplvr2`] module"]
#[doc(alias = "TPLVR2")]
pub type Tplvr2 = crate::Reg<tplvr2::Tplvr2Spec>;
#[doc = "Timer 2 Pre-Load Value"]
pub mod tplvr2;
#[doc = "TPLVR3 (rw) register accessor: Timer 3 Pre-Load Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tplvr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplvr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tplvr3`] module"]
#[doc(alias = "TPLVR3")]
pub type Tplvr3 = crate::Reg<tplvr3::Tplvr3Spec>;
#[doc = "Timer 3 Pre-Load Value"]
pub mod tplvr3;
#[doc = "TPLCR2 (rw) register accessor: Timer 2 Pre-Load Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tplcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tplcr2`] module"]
#[doc(alias = "TPLCR2")]
pub type Tplcr2 = crate::Reg<tplcr2::Tplcr2Spec>;
#[doc = "Timer 2 Pre-Load Control"]
pub mod tplcr2;
#[doc = "TPLCR3 (rw) register accessor: Timer 3 Pre-Load Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tplcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tplcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tplcr3`] module"]
#[doc(alias = "TPLCR3")]
pub type Tplcr3 = crate::Reg<tplcr3::Tplcr3Spec>;
#[doc = "Timer 3 Pre-Load Control"]
pub mod tplcr3;
#[doc = "WMER (rw) register accessor: Watchdog reset/interrupt Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wmer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wmer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wmer`] module"]
#[doc(alias = "WMER")]
pub type Wmer = crate::Reg<wmer::WmerSpec>;
#[doc = "Watchdog reset/interrupt Mode"]
pub mod wmer;
#[doc = "WMR (rw) register accessor: Watchdog Match Value\n\nYou can [`read`](crate::Reg::read) this register and get [`wmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wmr`] module"]
#[doc(alias = "WMR")]
pub type Wmr = crate::Reg<wmr::WmrSpec>;
#[doc = "Watchdog Match Value"]
pub mod wmr;
#[doc = "WVR (r) register accessor: Watchdog Counter Value\n\nYou can [`read`](crate::Reg::read) this register and get [`wvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wvr`] module"]
#[doc(alias = "WVR")]
pub type Wvr = crate::Reg<wvr::WvrSpec>;
#[doc = "Watchdog Counter Value"]
pub mod wvr;
#[doc = "WSR (rw) register accessor: Watchdog Reset Status\n\nYou can [`read`](crate::Reg::read) this register and get [`wsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wsr`] module"]
#[doc(alias = "WSR")]
pub type Wsr = crate::Reg<wsr::WsrSpec>;
#[doc = "Watchdog Reset Status"]
pub mod wsr;
#[doc = "TICR2 (w) register accessor: Timer 2 Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ticr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ticr2`] module"]
#[doc(alias = "TICR2")]
pub type Ticr2 = crate::Reg<ticr2::Ticr2Spec>;
#[doc = "Timer 2 Interrupt Clear"]
pub mod ticr2;
#[doc = "TICR3 (w) register accessor: Timer 3 Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ticr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ticr3`] module"]
#[doc(alias = "TICR3")]
pub type Ticr3 = crate::Reg<ticr3::Ticr3Spec>;
#[doc = "Timer 3 Interrupt Clear"]
pub mod ticr3;
#[doc = "WICR (w) register accessor: WDT Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wicr`] module"]
#[doc(alias = "WICR")]
pub type Wicr = crate::Reg<wicr::WicrSpec>;
#[doc = "WDT Interrupt Clear"]
pub mod wicr;
#[doc = "TCER (rw) register accessor: Timer Counter Enable/Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`tcer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcer`] module"]
#[doc(alias = "TCER")]
pub type Tcer = crate::Reg<tcer::TcerSpec>;
#[doc = "Timer Counter Enable/Clear"]
pub mod tcer;
#[doc = "TCMR (rw) register accessor: Timer Counter Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcmr`] module"]
#[doc(alias = "TCMR")]
pub type Tcmr = crate::Reg<tcmr::TcmrSpec>;
#[doc = "Timer Counter Mode"]
pub mod tcmr;
#[doc = "TILR2 (rw) register accessor: Timer 2 Match Interrupt Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`tilr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tilr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tilr2`] module"]
#[doc(alias = "TILR2")]
pub type Tilr2 = crate::Reg<tilr2::Tilr2Spec>;
#[doc = "Timer 2 Match Interrupt Mode"]
pub mod tilr2;
#[doc = "TILR3 (rw) register accessor: Timer 3 Match Interrupt Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`tilr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tilr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tilr3`] module"]
#[doc(alias = "TILR3")]
pub type Tilr3 = crate::Reg<tilr3::Tilr3Spec>;
#[doc = "Timer 3 Match Interrupt Mode"]
pub mod tilr3;
#[doc = "WCR (w) register accessor: WDT Counter Reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcr`] module"]
#[doc(alias = "WCR")]
pub type Wcr = crate::Reg<wcr::WcrSpec>;
#[doc = "WDT Counter Reset"]
pub mod wcr;
#[doc = "WFAR (w) register accessor: WDT Access Key1 (0xBABA)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wfar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfar`] module"]
#[doc(alias = "WFAR")]
pub type Wfar = crate::Reg<wfar::WfarSpec>;
#[doc = "WDT Access Key1 (0xBABA)"]
pub mod wfar;
#[doc = "WSAR (w) register accessor: WDT Access Key2 (0xEB10)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wsar::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wsar`] module"]
#[doc(alias = "WSAR")]
pub type Wsar = crate::Reg<wsar::WsarSpec>;
#[doc = "WDT Access Key2 (0xEB10)"]
pub mod wsar;
#[doc = "TCVWR2 (r) register accessor: Timer 2 Counter Latch Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvwr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcvwr2`] module"]
#[doc(alias = "TCVWR2")]
pub type Tcvwr2 = crate::Reg<tcvwr2::Tcvwr2Spec>;
#[doc = "Timer 2 Counter Latch Value"]
pub mod tcvwr2;
#[doc = "TCVWR3 (r) register accessor: Timer 3 Counter Latch Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvwr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcvwr3`] module"]
#[doc(alias = "TCVWR3")]
pub type Tcvwr3 = crate::Reg<tcvwr3::Tcvwr3Spec>;
#[doc = "Timer 3 Counter Latch Value"]
pub mod tcvwr3;
#[doc = "TCVSYN2 (r) register accessor: Timer 2 Counter Sync Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvsyn2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcvsyn2`] module"]
#[doc(alias = "TCVSYN2")]
pub type Tcvsyn2 = crate::Reg<tcvsyn2::Tcvsyn2Spec>;
#[doc = "Timer 2 Counter Sync Value"]
pub mod tcvsyn2;
#[doc = "TCVSYN3 (r) register accessor: Timer 3 Counter Sync Value\n\nYou can [`read`](crate::Reg::read) this register and get [`tcvsyn3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcvsyn3`] module"]
#[doc(alias = "TCVSYN3")]
pub type Tcvsyn3 = crate::Reg<tcvsyn3::Tcvsyn3Spec>;
#[doc = "Timer 3 Counter Sync Value"]
pub mod tcvsyn3;
#[doc = "TCDR (rw) register accessor: Timer and WDT Division\n\nYou can [`read`](crate::Reg::read) this register and get [`tcdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcdr`] module"]
#[doc(alias = "TCDR")]
pub type Tcdr = crate::Reg<tcdr::TcdrSpec>;
#[doc = "Timer and WDT Division"]
pub mod tcdr;
#[doc = "GPIO (rw) register accessor: GPIO Integration and Latch Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio`] module"]
#[doc(alias = "GPIO")]
pub type Gpio = crate::Reg<gpio::GpioSpec>;
#[doc = "GPIO Integration and Latch Mode"]
pub mod gpio;
#[doc = "GPIO_LAT1 (r) register accessor: GPIO Positive Edge Latch Value\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_lat1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_lat1`] module"]
#[doc(alias = "GPIO_LAT1")]
pub type GpioLat1 = crate::Reg<gpio_lat1::GpioLat1Spec>;
#[doc = "GPIO Positive Edge Latch Value"]
pub mod gpio_lat1;
#[doc = "GPIO_LAT2 (r) register accessor: GPIO Negative Edge Latch Value\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_lat2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_lat2`] module"]
#[doc(alias = "GPIO_LAT2")]
pub type GpioLat2 = crate::Reg<gpio_lat2::GpioLat2Spec>;
#[doc = "GPIO Negative Edge Latch Value"]
pub mod gpio_lat2;
#[doc = "TCDR_FORCE (rw) register accessor: Timer Division Force Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tcdr_force::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcdr_force::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcdr_force`] module"]
#[doc(alias = "TCDR_FORCE")]
pub type TcdrForce = crate::Reg<tcdr_force::TcdrForceSpec>;
#[doc = "Timer Division Force Configuration"]
pub mod tcdr_force;
