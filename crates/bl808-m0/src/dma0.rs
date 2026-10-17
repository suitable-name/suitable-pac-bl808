#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    int_status: IntStatus,
    int_tcstatus: IntTcstatus,
    int_tcclear: IntTcclear,
    int_error_status: IntErrorStatus,
    int_err_clr: IntErrClr,
    raw_int_tcstatus: RawIntTcstatus,
    raw_int_error_status: RawIntErrorStatus,
    enbld_chns: EnbldChns,
    soft_breq: SoftBreq,
    soft_sreq: SoftSreq,
    soft_lbreq: SoftLbreq,
    soft_lsreq: SoftLsreq,
    top_config: TopConfig,
    sync: Sync,
    _reserved14: [u8; 0xc8],
    ch: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Status of the DMA interrupts after masking"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x04 - Interrupt terminal count request status"]
    #[inline(always)]
    pub const fn int_tcstatus(&self) -> &IntTcstatus {
        &self.int_tcstatus
    }
    #[doc = "0x08 - Terminal count request clear"]
    #[inline(always)]
    pub const fn int_tcclear(&self) -> &IntTcclear {
        &self.int_tcclear
    }
    #[doc = "0x0c - Interrupt error status"]
    #[inline(always)]
    pub const fn int_error_status(&self) -> &IntErrorStatus {
        &self.int_error_status
    }
    #[doc = "0x10 - Interrupt error clear"]
    #[inline(always)]
    pub const fn int_err_clr(&self) -> &IntErrClr {
        &self.int_err_clr
    }
    #[doc = "0x14 - Status of the terminal count interrupt prior to masking"]
    #[inline(always)]
    pub const fn raw_int_tcstatus(&self) -> &RawIntTcstatus {
        &self.raw_int_tcstatus
    }
    #[doc = "0x18 - Status of the error interrupt prior to masking"]
    #[inline(always)]
    pub const fn raw_int_error_status(&self) -> &RawIntErrorStatus {
        &self.raw_int_error_status
    }
    #[doc = "0x1c - Channel enable status"]
    #[inline(always)]
    pub const fn enbld_chns(&self) -> &EnbldChns {
        &self.enbld_chns
    }
    #[doc = "0x20 - Software Burst Request"]
    #[inline(always)]
    pub const fn soft_breq(&self) -> &SoftBreq {
        &self.soft_breq
    }
    #[doc = "0x24 - Software Single Request"]
    #[inline(always)]
    pub const fn soft_sreq(&self) -> &SoftSreq {
        &self.soft_sreq
    }
    #[doc = "0x28 - Software Last Burst Request"]
    #[inline(always)]
    pub const fn soft_lbreq(&self) -> &SoftLbreq {
        &self.soft_lbreq
    }
    #[doc = "0x2c - Software Last Single Request"]
    #[inline(always)]
    pub const fn soft_lsreq(&self) -> &SoftLsreq {
        &self.soft_lsreq
    }
    #[doc = "0x30 - DMA Top Configuration Register"]
    #[inline(always)]
    pub const fn top_config(&self) -> &TopConfig {
        &self.top_config
    }
    #[doc = "0x34 - DMA Synchronization Register"]
    #[inline(always)]
    pub const fn sync(&self) -> &Sync {
        &self.sync
    }
    #[doc = "0x100..0x200 - DMA Channel Registers"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x200 - DMA Channel Registers"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        })
    }
}
#[doc = "IntStatus (r) register accessor: Status of the DMA interrupts after masking\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`] module"]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Status of the DMA interrupts after masking"]
pub mod int_status;
#[doc = "IntTCStatus (r) register accessor: Interrupt terminal count request status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_tcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_tcstatus`] module"]
#[doc(alias = "IntTCStatus")]
pub type IntTcstatus = crate::Reg<int_tcstatus::IntTcstatusSpec>;
#[doc = "Interrupt terminal count request status"]
pub mod int_tcstatus;
#[doc = "IntTCClear (w) register accessor: Terminal count request clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_tcclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_tcclear`] module"]
#[doc(alias = "IntTCClear")]
pub type IntTcclear = crate::Reg<int_tcclear::IntTcclearSpec>;
#[doc = "Terminal count request clear"]
pub mod int_tcclear;
#[doc = "IntErrorStatus (r) register accessor: Interrupt error status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_error_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_error_status`] module"]
pub type IntErrorStatus = crate::Reg<int_error_status::IntErrorStatusSpec>;
#[doc = "Interrupt error status"]
pub mod int_error_status;
#[doc = "IntErrClr (w) register accessor: Interrupt error clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_err_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_err_clr`] module"]
pub type IntErrClr = crate::Reg<int_err_clr::IntErrClrSpec>;
#[doc = "Interrupt error clear"]
pub mod int_err_clr;
#[doc = "RawIntTCStatus (r) register accessor: Status of the terminal count interrupt prior to masking\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_int_tcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_int_tcstatus`] module"]
#[doc(alias = "RawIntTCStatus")]
pub type RawIntTcstatus = crate::Reg<raw_int_tcstatus::RawIntTcstatusSpec>;
#[doc = "Status of the terminal count interrupt prior to masking"]
pub mod raw_int_tcstatus;
#[doc = "RawIntErrorStatus (r) register accessor: Status of the error interrupt prior to masking\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_int_error_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_int_error_status`] module"]
pub type RawIntErrorStatus = crate::Reg<raw_int_error_status::RawIntErrorStatusSpec>;
#[doc = "Status of the error interrupt prior to masking"]
pub mod raw_int_error_status;
#[doc = "EnbldChns (r) register accessor: Channel enable status\n\nYou can [`read`](crate::Reg::read) this register and get [`enbld_chns::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enbld_chns`] module"]
pub type EnbldChns = crate::Reg<enbld_chns::EnbldChnsSpec>;
#[doc = "Channel enable status"]
pub mod enbld_chns;
#[doc = "SoftBReq (rw) register accessor: Software Burst Request\n\nYou can [`read`](crate::Reg::read) this register and get [`soft_breq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_breq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_breq`] module"]
#[doc(alias = "SoftBReq")]
pub type SoftBreq = crate::Reg<soft_breq::SoftBreqSpec>;
#[doc = "Software Burst Request"]
pub mod soft_breq;
#[doc = "SoftSReq (rw) register accessor: Software Single Request\n\nYou can [`read`](crate::Reg::read) this register and get [`soft_sreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_sreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_sreq`] module"]
#[doc(alias = "SoftSReq")]
pub type SoftSreq = crate::Reg<soft_sreq::SoftSreqSpec>;
#[doc = "Software Single Request"]
pub mod soft_sreq;
#[doc = "SoftLBReq (rw) register accessor: Software Last Burst Request\n\nYou can [`read`](crate::Reg::read) this register and get [`soft_lbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_lbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_lbreq`] module"]
#[doc(alias = "SoftLBReq")]
pub type SoftLbreq = crate::Reg<soft_lbreq::SoftLbreqSpec>;
#[doc = "Software Last Burst Request"]
pub mod soft_lbreq;
#[doc = "SoftLSReq (rw) register accessor: Software Last Single Request\n\nYou can [`read`](crate::Reg::read) this register and get [`soft_lsreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_lsreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_lsreq`] module"]
#[doc(alias = "SoftLSReq")]
pub type SoftLsreq = crate::Reg<soft_lsreq::SoftLsreqSpec>;
#[doc = "Software Last Single Request"]
pub mod soft_lsreq;
#[doc = "Top_Config (rw) register accessor: DMA Top Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`top_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@top_config`] module"]
#[doc(alias = "Top_Config")]
pub type TopConfig = crate::Reg<top_config::TopConfigSpec>;
#[doc = "DMA Top Configuration Register"]
pub mod top_config;
#[doc = "Sync (rw) register accessor: DMA Synchronization Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`] module"]
pub type Sync = crate::Reg<sync::SyncSpec>;
#[doc = "DMA Synchronization Register"]
pub mod sync;
#[doc = "DMA Channel Registers"]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "DMA Channel Registers"]
pub mod ch;
