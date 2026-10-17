#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lz4_config: Lz4Config,
    lz4_src_fix: Lz4SrcFix,
    lz4_dst_fix: Lz4DstFix,
    _reserved3: [u8; 0x04],
    lz4_src_start: Lz4SrcStart,
    lz4_src_end: Lz4SrcEnd,
    lz4_dst_start: Lz4DstStart,
    lz4_dst_end: Lz4DstEnd,
    lz4_int_en: Lz4IntEn,
    lz4_int_sta: Lz4IntSta,
    lz4_monitor: Lz4Monitor,
}
impl RegisterBlock {
    #[doc = "0x00 - LZ4 Configuration Register"]
    #[inline(always)]
    pub const fn lz4_config(&self) -> &Lz4Config {
        &self.lz4_config
    }
    #[doc = "0x04 - Source Address Fix Configuration"]
    #[inline(always)]
    pub const fn lz4_src_fix(&self) -> &Lz4SrcFix {
        &self.lz4_src_fix
    }
    #[doc = "0x08 - Destination Address Fix Configuration"]
    #[inline(always)]
    pub const fn lz4_dst_fix(&self) -> &Lz4DstFix {
        &self.lz4_dst_fix
    }
    #[doc = "0x10 - Source Start Address"]
    #[inline(always)]
    pub const fn lz4_src_start(&self) -> &Lz4SrcStart {
        &self.lz4_src_start
    }
    #[doc = "0x14 - Source End Address Status"]
    #[inline(always)]
    pub const fn lz4_src_end(&self) -> &Lz4SrcEnd {
        &self.lz4_src_end
    }
    #[doc = "0x18 - Destination Start Address"]
    #[inline(always)]
    pub const fn lz4_dst_start(&self) -> &Lz4DstStart {
        &self.lz4_dst_start
    }
    #[doc = "0x1c - Destination End Address Status"]
    #[inline(always)]
    pub const fn lz4_dst_end(&self) -> &Lz4DstEnd {
        &self.lz4_dst_end
    }
    #[doc = "0x20 - LZ4 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn lz4_int_en(&self) -> &Lz4IntEn {
        &self.lz4_int_en
    }
    #[doc = "0x24 - LZ4 Interrupt Status Register"]
    #[inline(always)]
    pub const fn lz4_int_sta(&self) -> &Lz4IntSta {
        &self.lz4_int_sta
    }
    #[doc = "0x28 - LZ4 Engine Monitor Register"]
    #[inline(always)]
    pub const fn lz4_monitor(&self) -> &Lz4Monitor {
        &self.lz4_monitor
    }
}
#[doc = "LZ4_CONFIG (rw) register accessor: LZ4 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lz4_config`] module"]
#[doc(alias = "LZ4_CONFIG")]
pub type Lz4Config = crate::Reg<lz4_config::Lz4ConfigSpec>;
#[doc = "LZ4 Configuration Register"]
pub mod lz4_config;
#[doc = "LZ4_SRC_FIX (rw) register accessor: Source Address Fix Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_src_fix::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_src_fix::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lz4_src_fix`] module"]
#[doc(alias = "LZ4_SRC_FIX")]
pub type Lz4SrcFix = crate::Reg<lz4_src_fix::Lz4SrcFixSpec>;
#[doc = "Source Address Fix Configuration"]
pub mod lz4_src_fix;
#[doc = "LZ4_DST_FIX (rw) register accessor: Destination Address Fix Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_dst_fix::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_dst_fix::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lz4_dst_fix`] module"]
#[doc(alias = "LZ4_DST_FIX")]
pub type Lz4DstFix = crate::Reg<lz4_dst_fix::Lz4DstFixSpec>;
#[doc = "Destination Address Fix Configuration"]
pub mod lz4_dst_fix;
#[doc = "LZ4_SRC_START (rw) register accessor: Source Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_src_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_src_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lz4_src_start`] module"]
#[doc(alias = "LZ4_SRC_START")]
pub type Lz4SrcStart = crate::Reg<lz4_src_start::Lz4SrcStartSpec>;
#[doc = "Source Start Address"]
pub mod lz4_src_start;
#[doc = "LZ4_SRC_END (r) register accessor: Source End Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_src_end::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lz4_src_end`] module"]
#[doc(alias = "LZ4_SRC_END")]
pub type Lz4SrcEnd = crate::Reg<lz4_src_end::Lz4SrcEndSpec>;
#[doc = "Source End Address Status"]
pub mod lz4_src_end;
#[doc = "LZ4_DST_START (rw) register accessor: Destination Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_dst_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_dst_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lz4_dst_start`] module"]
#[doc(alias = "LZ4_DST_START")]
pub type Lz4DstStart = crate::Reg<lz4_dst_start::Lz4DstStartSpec>;
#[doc = "Destination Start Address"]
pub mod lz4_dst_start;
#[doc = "LZ4_DST_END (r) register accessor: Destination End Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_dst_end::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lz4_dst_end`] module"]
#[doc(alias = "LZ4_DST_END")]
pub type Lz4DstEnd = crate::Reg<lz4_dst_end::Lz4DstEndSpec>;
#[doc = "Destination End Address Status"]
pub mod lz4_dst_end;
#[doc = "LZ4_INT_EN (rw) register accessor: LZ4 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lz4_int_en`] module"]
#[doc(alias = "LZ4_INT_EN")]
pub type Lz4IntEn = crate::Reg<lz4_int_en::Lz4IntEnSpec>;
#[doc = "LZ4 Interrupt Enable Register"]
pub mod lz4_int_en;
#[doc = "LZ4_INT_STA (r) register accessor: LZ4 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_int_sta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lz4_int_sta`] module"]
#[doc(alias = "LZ4_INT_STA")]
pub type Lz4IntSta = crate::Reg<lz4_int_sta::Lz4IntStaSpec>;
#[doc = "LZ4 Interrupt Status Register"]
pub mod lz4_int_sta;
#[doc = "LZ4_MONITOR (r) register accessor: LZ4 Engine Monitor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_monitor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lz4_monitor`] module"]
#[doc(alias = "LZ4_MONITOR")]
pub type Lz4Monitor = crate::Reg<lz4_monitor::Lz4MonitorSpec>;
#[doc = "LZ4 Engine Monitor Register"]
pub mod lz4_monitor;
