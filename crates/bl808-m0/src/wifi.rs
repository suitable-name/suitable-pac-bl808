#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    coex_control_0: CoexControl0,
    coex_pti: CoexPti,
    coex_state: CoexState,
    coex_interrupt: CoexInterrupt,
    coex_control_2: CoexControl2,
}
impl RegisterBlock {
    #[doc = "0x400 - Coexistence Control Register 0 (Arbitration)"]
    #[inline(always)]
    pub const fn coex_control_0(&self) -> &CoexControl0 {
        &self.coex_control_0
    }
    #[doc = "0x404 - Coexistence Packet Traffic Interface Priority Configuration"]
    #[inline(always)]
    pub const fn coex_pti(&self) -> &CoexPti {
        &self.coex_pti
    }
    #[doc = "0x408 - Coexistence State Register (Read-Only)"]
    #[inline(always)]
    pub const fn coex_state(&self) -> &CoexState {
        &self.coex_state
    }
    #[doc = "0x40c - Coexistence Interrupt Status/Control"]
    #[inline(always)]
    pub const fn coex_interrupt(&self) -> &CoexInterrupt {
        &self.coex_interrupt
    }
    #[doc = "0x410 - Coexistence Control Register 2 (Traffic Class Force Grants)"]
    #[inline(always)]
    pub const fn coex_control_2(&self) -> &CoexControl2 {
        &self.coex_control_2
    }
}
#[doc = "coex_control_0 (rw) register accessor: Coexistence Control Register 0 (Arbitration)\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_control_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_control_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coex_control_0`] module"]
#[doc(alias = "coex_control_0")]
pub type CoexControl0 = crate::Reg<coex_control_0::CoexControl0Spec>;
#[doc = "Coexistence Control Register 0 (Arbitration)"]
pub mod coex_control_0;
#[doc = "coex_pti (rw) register accessor: Coexistence Packet Traffic Interface Priority Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_pti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_pti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coex_pti`] module"]
#[doc(alias = "coex_pti")]
pub type CoexPti = crate::Reg<coex_pti::CoexPtiSpec>;
#[doc = "Coexistence Packet Traffic Interface Priority Configuration"]
pub mod coex_pti;
#[doc = "coex_state (r) register accessor: Coexistence State Register (Read-Only)\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coex_state`] module"]
#[doc(alias = "coex_state")]
pub type CoexState = crate::Reg<coex_state::CoexStateSpec>;
#[doc = "Coexistence State Register (Read-Only)"]
pub mod coex_state;
#[doc = "coex_interrupt (rw) register accessor: Coexistence Interrupt Status/Control\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coex_interrupt`] module"]
#[doc(alias = "coex_interrupt")]
pub type CoexInterrupt = crate::Reg<coex_interrupt::CoexInterruptSpec>;
#[doc = "Coexistence Interrupt Status/Control"]
pub mod coex_interrupt;
#[doc = "coex_control_2 (rw) register accessor: Coexistence Control Register 2 (Traffic Class Force Grants)\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_control_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_control_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coex_control_2`] module"]
#[doc(alias = "coex_control_2")]
pub type CoexControl2 = crate::Reg<coex_control_2::CoexControl2Spec>;
#[doc = "Coexistence Control Register 2 (Traffic Class Force Grants)"]
pub mod coex_control_2;
