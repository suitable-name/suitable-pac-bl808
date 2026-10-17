#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dma_int_status: DmaIntStatus,
    dma_int_tcstatus: DmaIntTcstatus,
    dma_int_tcclear: DmaIntTcclear,
    dma_enbld_chns: DmaEnbldChns,
    dma_config: DmaConfig,
    dma_sync: DmaSync,
    dma_soft_breq: DmaSoftBreq,
    dma_soft_lbreq: DmaSoftLbreq,
    dma_soft_sreq: DmaSoftSreq,
    dma_soft_lsreq: DmaSoftLsreq,
    _reserved10: [u8; 0xd8],
    lane: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Status of the DMA interrupts after masking"]
    #[inline(always)]
    pub const fn dma_int_status(&self) -> &DmaIntStatus {
        &self.dma_int_status
    }
    #[doc = "0x04 - Interrupt terminal count request status"]
    #[inline(always)]
    pub const fn dma_int_tcstatus(&self) -> &DmaIntTcstatus {
        &self.dma_int_tcstatus
    }
    #[doc = "0x08 - Terminal count request clear"]
    #[inline(always)]
    pub const fn dma_int_tcclear(&self) -> &DmaIntTcclear {
        &self.dma_int_tcclear
    }
    #[doc = "0x0c - Channel enable status"]
    #[inline(always)]
    pub const fn dma_enbld_chns(&self) -> &DmaEnbldChns {
        &self.dma_enbld_chns
    }
    #[doc = "0x10 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn dma_config(&self) -> &DmaConfig {
        &self.dma_config
    }
    #[doc = "0x14 - DMA Synchronization Logic"]
    #[inline(always)]
    pub const fn dma_sync(&self) -> &DmaSync {
        &self.dma_sync
    }
    #[doc = "0x18 - DMA Software Burst Request"]
    #[inline(always)]
    pub const fn dma_soft_breq(&self) -> &DmaSoftBreq {
        &self.dma_soft_breq
    }
    #[doc = "0x1c - DMA Software Last Burst Request"]
    #[inline(always)]
    pub const fn dma_soft_lbreq(&self) -> &DmaSoftLbreq {
        &self.dma_soft_lbreq
    }
    #[doc = "0x20 - DMA Software Single Request"]
    #[inline(always)]
    pub const fn dma_soft_sreq(&self) -> &DmaSoftSreq {
        &self.dma_soft_sreq
    }
    #[doc = "0x24 - DMA Software Last Single Request"]
    #[inline(always)]
    pub const fn dma_soft_lsreq(&self) -> &DmaSoftLsreq {
        &self.dma_soft_lsreq
    }
    #[doc = "0x100..0x200 - 2D DMA Lane Registers"]
    #[inline(always)]
    pub const fn lane(&self, n: usize) -> &Lane {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x200 - 2D DMA Lane Registers"]
    #[inline(always)]
    pub fn lane_iter(&self) -> impl Iterator<Item = &Lane> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        })
    }
}
#[doc = "DMA_IntStatus (r) register accessor: Status of the DMA interrupts after masking\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_status`] module"]
#[doc(alias = "DMA_IntStatus")]
pub type DmaIntStatus = crate::Reg<dma_int_status::DmaIntStatusSpec>;
#[doc = "Status of the DMA interrupts after masking"]
pub mod dma_int_status;
#[doc = "DMA_IntTCStatus (r) register accessor: Interrupt terminal count request status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_tcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_tcstatus`] module"]
#[doc(alias = "DMA_IntTCStatus")]
pub type DmaIntTcstatus = crate::Reg<dma_int_tcstatus::DmaIntTcstatusSpec>;
#[doc = "Interrupt terminal count request status"]
pub mod dma_int_tcstatus;
#[doc = "DMA_IntTCClear (w) register accessor: Terminal count request clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_tcclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_int_tcclear`] module"]
#[doc(alias = "DMA_IntTCClear")]
pub type DmaIntTcclear = crate::Reg<dma_int_tcclear::DmaIntTcclearSpec>;
#[doc = "Terminal count request clear"]
pub mod dma_int_tcclear;
#[doc = "DMA_EnbldChns (r) register accessor: Channel enable status\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_enbld_chns::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_enbld_chns`] module"]
#[doc(alias = "DMA_EnbldChns")]
pub type DmaEnbldChns = crate::Reg<dma_enbld_chns::DmaEnbldChnsSpec>;
#[doc = "Channel enable status"]
pub mod dma_enbld_chns;
#[doc = "DMA_Config (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_config`] module"]
#[doc(alias = "DMA_Config")]
pub type DmaConfig = crate::Reg<dma_config::DmaConfigSpec>;
#[doc = "DMA Configuration Register"]
pub mod dma_config;
#[doc = "DMA_Sync (rw) register accessor: DMA Synchronization Logic\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_sync`] module"]
#[doc(alias = "DMA_Sync")]
pub type DmaSync = crate::Reg<dma_sync::DmaSyncSpec>;
#[doc = "DMA Synchronization Logic"]
pub mod dma_sync;
#[doc = "DMA_SoftBReq (rw) register accessor: DMA Software Burst Request\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_breq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_breq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_breq`] module"]
#[doc(alias = "DMA_SoftBReq")]
pub type DmaSoftBreq = crate::Reg<dma_soft_breq::DmaSoftBreqSpec>;
#[doc = "DMA Software Burst Request"]
pub mod dma_soft_breq;
#[doc = "DMA_SoftLBReq (rw) register accessor: DMA Software Last Burst Request\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_lbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_lbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_lbreq`] module"]
#[doc(alias = "DMA_SoftLBReq")]
pub type DmaSoftLbreq = crate::Reg<dma_soft_lbreq::DmaSoftLbreqSpec>;
#[doc = "DMA Software Last Burst Request"]
pub mod dma_soft_lbreq;
#[doc = "DMA_SoftSReq (rw) register accessor: DMA Software Single Request\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_sreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_sreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_sreq`] module"]
#[doc(alias = "DMA_SoftSReq")]
pub type DmaSoftSreq = crate::Reg<dma_soft_sreq::DmaSoftSreqSpec>;
#[doc = "DMA Software Single Request"]
pub mod dma_soft_sreq;
#[doc = "DMA_SoftLSReq (rw) register accessor: DMA Software Last Single Request\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_soft_lsreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_soft_lsreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_soft_lsreq`] module"]
#[doc(alias = "DMA_SoftLSReq")]
pub type DmaSoftLsreq = crate::Reg<dma_soft_lsreq::DmaSoftLsreqSpec>;
#[doc = "DMA Software Last Single Request"]
pub mod dma_soft_lsreq;
#[doc = "2D DMA Lane Registers"]
pub use self::lane::Lane;
#[doc = r"Cluster"]
#[doc = "2D DMA Lane Registers"]
pub mod lane;
