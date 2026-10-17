#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpu1_ipc_iswr: Cpu1IpcIswr,
    cpu1_ipc_irsrr: Cpu1IpcIrsrr,
    cpu1_ipc_icr: Cpu1IpcIcr,
    cpu1_ipc_iusr: Cpu1IpcIusr,
    cpu1_ipc_iucr: Cpu1IpcIucr,
    cpu1_ipc_ilslr: Cpu1IpcIlslr,
    cpu1_ipc_ilshr: Cpu1IpcIlshr,
    cpu1_ipc_isr: Cpu1IpcIsr,
    cpu0_ipc_iswr: Cpu0IpcIswr,
    cpu0_ipc_irsrr: Cpu0IpcIrsrr,
    cpu0_ipc_icr: Cpu0IpcIcr,
    cpu0_ipc_iusr: Cpu0IpcIusr,
    cpu0_ipc_iucr: Cpu0IpcIucr,
    cpu0_ipc_ilslr: Cpu0IpcIlslr,
    cpu0_ipc_ilshr: Cpu0IpcIlshr,
    cpu0_ipc_isr: Cpu0IpcIsr,
}
impl RegisterBlock {
    #[doc = "0x00 - CPU1 Interrupt Set Write Register"]
    #[inline(always)]
    pub const fn cpu1_ipc_iswr(&self) -> &Cpu1IpcIswr {
        &self.cpu1_ipc_iswr
    }
    #[doc = "0x04 - CPU1 Interrupt Raw Status Register"]
    #[inline(always)]
    pub const fn cpu1_ipc_irsrr(&self) -> &Cpu1IpcIrsrr {
        &self.cpu1_ipc_irsrr
    }
    #[doc = "0x08 - CPU1 Interrupt Clear Register"]
    #[inline(always)]
    pub const fn cpu1_ipc_icr(&self) -> &Cpu1IpcIcr {
        &self.cpu1_ipc_icr
    }
    #[doc = "0x0c - CPU1 Interrupt Unmask Set Register"]
    #[inline(always)]
    pub const fn cpu1_ipc_iusr(&self) -> &Cpu1IpcIusr {
        &self.cpu1_ipc_iusr
    }
    #[doc = "0x10 - CPU1 Interrupt Unmask Clear Register"]
    #[inline(always)]
    pub const fn cpu1_ipc_iucr(&self) -> &Cpu1IpcIucr {
        &self.cpu1_ipc_iucr
    }
    #[doc = "0x14 - CPU1 Interrupt Line Sel Low Register"]
    #[inline(always)]
    pub const fn cpu1_ipc_ilslr(&self) -> &Cpu1IpcIlslr {
        &self.cpu1_ipc_ilslr
    }
    #[doc = "0x18 - CPU1 Interrupt Line Sel High Register"]
    #[inline(always)]
    pub const fn cpu1_ipc_ilshr(&self) -> &Cpu1IpcIlshr {
        &self.cpu1_ipc_ilshr
    }
    #[doc = "0x1c - CPU1 Interrupt Status Register"]
    #[inline(always)]
    pub const fn cpu1_ipc_isr(&self) -> &Cpu1IpcIsr {
        &self.cpu1_ipc_isr
    }
    #[doc = "0x20 - CPU0 Interrupt Set Write Register"]
    #[inline(always)]
    pub const fn cpu0_ipc_iswr(&self) -> &Cpu0IpcIswr {
        &self.cpu0_ipc_iswr
    }
    #[doc = "0x24 - CPU0 Interrupt Raw Status Register"]
    #[inline(always)]
    pub const fn cpu0_ipc_irsrr(&self) -> &Cpu0IpcIrsrr {
        &self.cpu0_ipc_irsrr
    }
    #[doc = "0x28 - CPU0 Interrupt Clear Register"]
    #[inline(always)]
    pub const fn cpu0_ipc_icr(&self) -> &Cpu0IpcIcr {
        &self.cpu0_ipc_icr
    }
    #[doc = "0x2c - CPU0 Interrupt Unmask Set Register"]
    #[inline(always)]
    pub const fn cpu0_ipc_iusr(&self) -> &Cpu0IpcIusr {
        &self.cpu0_ipc_iusr
    }
    #[doc = "0x30 - CPU0 Interrupt Unmask Clear Register"]
    #[inline(always)]
    pub const fn cpu0_ipc_iucr(&self) -> &Cpu0IpcIucr {
        &self.cpu0_ipc_iucr
    }
    #[doc = "0x34 - CPU0 Interrupt Line Sel Low Register"]
    #[inline(always)]
    pub const fn cpu0_ipc_ilslr(&self) -> &Cpu0IpcIlslr {
        &self.cpu0_ipc_ilslr
    }
    #[doc = "0x38 - CPU0 Interrupt Line Sel High Register"]
    #[inline(always)]
    pub const fn cpu0_ipc_ilshr(&self) -> &Cpu0IpcIlshr {
        &self.cpu0_ipc_ilshr
    }
    #[doc = "0x3c - CPU0 Interrupt Status Register"]
    #[inline(always)]
    pub const fn cpu0_ipc_isr(&self) -> &Cpu0IpcIsr {
        &self.cpu0_ipc_isr
    }
}
#[doc = "CPU1_IPC_ISWR (rw) register accessor: CPU1 Interrupt Set Write Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_iswr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_iswr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1_ipc_iswr`] module"]
#[doc(alias = "CPU1_IPC_ISWR")]
pub type Cpu1IpcIswr = crate::Reg<cpu1_ipc_iswr::Cpu1IpcIswrSpec>;
#[doc = "CPU1 Interrupt Set Write Register"]
pub mod cpu1_ipc_iswr;
#[doc = "CPU1_IPC_IRSRR (rw) register accessor: CPU1 Interrupt Raw Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_irsrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_irsrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1_ipc_irsrr`] module"]
#[doc(alias = "CPU1_IPC_IRSRR")]
pub type Cpu1IpcIrsrr = crate::Reg<cpu1_ipc_irsrr::Cpu1IpcIrsrrSpec>;
#[doc = "CPU1 Interrupt Raw Status Register"]
pub mod cpu1_ipc_irsrr;
#[doc = "CPU1_IPC_ICR (rw) register accessor: CPU1 Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1_ipc_icr`] module"]
#[doc(alias = "CPU1_IPC_ICR")]
pub type Cpu1IpcIcr = crate::Reg<cpu1_ipc_icr::Cpu1IpcIcrSpec>;
#[doc = "CPU1 Interrupt Clear Register"]
pub mod cpu1_ipc_icr;
#[doc = "CPU1_IPC_IUSR (rw) register accessor: CPU1 Interrupt Unmask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_iusr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_iusr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1_ipc_iusr`] module"]
#[doc(alias = "CPU1_IPC_IUSR")]
pub type Cpu1IpcIusr = crate::Reg<cpu1_ipc_iusr::Cpu1IpcIusrSpec>;
#[doc = "CPU1 Interrupt Unmask Set Register"]
pub mod cpu1_ipc_iusr;
#[doc = "CPU1_IPC_IUCR (rw) register accessor: CPU1 Interrupt Unmask Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_iucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_iucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1_ipc_iucr`] module"]
#[doc(alias = "CPU1_IPC_IUCR")]
pub type Cpu1IpcIucr = crate::Reg<cpu1_ipc_iucr::Cpu1IpcIucrSpec>;
#[doc = "CPU1 Interrupt Unmask Clear Register"]
pub mod cpu1_ipc_iucr;
#[doc = "CPU1_IPC_ILSLR (rw) register accessor: CPU1 Interrupt Line Sel Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_ilslr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_ilslr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1_ipc_ilslr`] module"]
#[doc(alias = "CPU1_IPC_ILSLR")]
pub type Cpu1IpcIlslr = crate::Reg<cpu1_ipc_ilslr::Cpu1IpcIlslrSpec>;
#[doc = "CPU1 Interrupt Line Sel Low Register"]
pub mod cpu1_ipc_ilslr;
#[doc = "CPU1_IPC_ILSHR (rw) register accessor: CPU1 Interrupt Line Sel High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_ilshr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_ilshr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1_ipc_ilshr`] module"]
#[doc(alias = "CPU1_IPC_ILSHR")]
pub type Cpu1IpcIlshr = crate::Reg<cpu1_ipc_ilshr::Cpu1IpcIlshrSpec>;
#[doc = "CPU1 Interrupt Line Sel High Register"]
pub mod cpu1_ipc_ilshr;
#[doc = "CPU1_IPC_ISR (rw) register accessor: CPU1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_ipc_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_ipc_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1_ipc_isr`] module"]
#[doc(alias = "CPU1_IPC_ISR")]
pub type Cpu1IpcIsr = crate::Reg<cpu1_ipc_isr::Cpu1IpcIsrSpec>;
#[doc = "CPU1 Interrupt Status Register"]
pub mod cpu1_ipc_isr;
#[doc = "CPU0_IPC_ISWR (rw) register accessor: CPU0 Interrupt Set Write Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_iswr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_iswr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_ipc_iswr`] module"]
#[doc(alias = "CPU0_IPC_ISWR")]
pub type Cpu0IpcIswr = crate::Reg<cpu0_ipc_iswr::Cpu0IpcIswrSpec>;
#[doc = "CPU0 Interrupt Set Write Register"]
pub mod cpu0_ipc_iswr;
#[doc = "CPU0_IPC_IRSRR (rw) register accessor: CPU0 Interrupt Raw Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_irsrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_irsrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_ipc_irsrr`] module"]
#[doc(alias = "CPU0_IPC_IRSRR")]
pub type Cpu0IpcIrsrr = crate::Reg<cpu0_ipc_irsrr::Cpu0IpcIrsrrSpec>;
#[doc = "CPU0 Interrupt Raw Status Register"]
pub mod cpu0_ipc_irsrr;
#[doc = "CPU0_IPC_ICR (rw) register accessor: CPU0 Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_ipc_icr`] module"]
#[doc(alias = "CPU0_IPC_ICR")]
pub type Cpu0IpcIcr = crate::Reg<cpu0_ipc_icr::Cpu0IpcIcrSpec>;
#[doc = "CPU0 Interrupt Clear Register"]
pub mod cpu0_ipc_icr;
#[doc = "CPU0_IPC_IUSR (rw) register accessor: CPU0 Interrupt Unmask Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_iusr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_iusr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_ipc_iusr`] module"]
#[doc(alias = "CPU0_IPC_IUSR")]
pub type Cpu0IpcIusr = crate::Reg<cpu0_ipc_iusr::Cpu0IpcIusrSpec>;
#[doc = "CPU0 Interrupt Unmask Set Register"]
pub mod cpu0_ipc_iusr;
#[doc = "CPU0_IPC_IUCR (rw) register accessor: CPU0 Interrupt Unmask Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_iucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_iucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_ipc_iucr`] module"]
#[doc(alias = "CPU0_IPC_IUCR")]
pub type Cpu0IpcIucr = crate::Reg<cpu0_ipc_iucr::Cpu0IpcIucrSpec>;
#[doc = "CPU0 Interrupt Unmask Clear Register"]
pub mod cpu0_ipc_iucr;
#[doc = "CPU0_IPC_ILSLR (rw) register accessor: CPU0 Interrupt Line Sel Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_ilslr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_ilslr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_ipc_ilslr`] module"]
#[doc(alias = "CPU0_IPC_ILSLR")]
pub type Cpu0IpcIlslr = crate::Reg<cpu0_ipc_ilslr::Cpu0IpcIlslrSpec>;
#[doc = "CPU0 Interrupt Line Sel Low Register"]
pub mod cpu0_ipc_ilslr;
#[doc = "CPU0_IPC_ILSHR (rw) register accessor: CPU0 Interrupt Line Sel High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_ilshr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_ilshr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_ipc_ilshr`] module"]
#[doc(alias = "CPU0_IPC_ILSHR")]
pub type Cpu0IpcIlshr = crate::Reg<cpu0_ipc_ilshr::Cpu0IpcIlshrSpec>;
#[doc = "CPU0 Interrupt Line Sel High Register"]
pub mod cpu0_ipc_ilshr;
#[doc = "CPU0_IPC_ISR (rw) register accessor: CPU0 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0_ipc_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0_ipc_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0_ipc_isr`] module"]
#[doc(alias = "CPU0_IPC_ISR")]
pub type Cpu0IpcIsr = crate::Reg<cpu0_ipc_isr::Cpu0IpcIsrSpec>;
#[doc = "CPU0 Interrupt Status Register"]
pub mod cpu0_ipc_isr;
