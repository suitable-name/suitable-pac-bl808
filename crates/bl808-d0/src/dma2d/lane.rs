#[repr(C)]
#[doc = "2D DMA Lane Registers"]
#[doc(alias = "LANE")]
pub struct Lane {
    src_addr: SrcAddr,
    dst_addr: DstAddr,
    lli: Lli,
    bus: Bus,
    src_cnt: SrcCnt,
    src_xic: SrcXic,
    src_yic: SrcYic,
    dst_cnt: DstCnt,
    dst_xic: DstXic,
    dst_yic: DstYic,
    _reserved10: [u8; 0x4c],
    key: Key,
    key_en: KeyEn,
    cfg: Cfg,
}
impl Lane {
    #[doc = "0x00 - Source start address"]
    #[inline(always)]
    pub const fn src_addr(&self) -> &SrcAddr {
        &self.src_addr
    }
    #[doc = "0x04 - Destination start address"]
    #[inline(always)]
    pub const fn dst_addr(&self) -> &DstAddr {
        &self.dst_addr
    }
    #[doc = "0x08 - Next LLI Address and control"]
    #[inline(always)]
    pub const fn lli(&self) -> &Lli {
        &self.lli
    }
    #[doc = "0x0c - Lane Bus and Burst Control"]
    #[inline(always)]
    pub const fn bus(&self) -> &Bus {
        &self.bus
    }
    #[doc = "0x10 - Source 2D Dimensions"]
    #[inline(always)]
    pub const fn src_cnt(&self) -> &SrcCnt {
        &self.src_cnt
    }
    #[doc = "0x14 - Source X Stride (Increment)"]
    #[inline(always)]
    pub const fn src_xic(&self) -> &SrcXic {
        &self.src_xic
    }
    #[doc = "0x18 - Source Y Stride (Increment)"]
    #[inline(always)]
    pub const fn src_yic(&self) -> &SrcYic {
        &self.src_yic
    }
    #[doc = "0x1c - Destination 2D Width"]
    #[inline(always)]
    pub const fn dst_cnt(&self) -> &DstCnt {
        &self.dst_cnt
    }
    #[doc = "0x20 - Destination X Stride (Increment)"]
    #[inline(always)]
    pub const fn dst_xic(&self) -> &DstXic {
        &self.dst_xic
    }
    #[doc = "0x24 - Destination Y Stride (Increment)"]
    #[inline(always)]
    pub const fn dst_yic(&self) -> &DstYic {
        &self.dst_yic
    }
    #[doc = "0x74 - Color Key Value"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x78 - Color Key Control"]
    #[inline(always)]
    pub const fn key_en(&self) -> &KeyEn {
        &self.key_en
    }
    #[doc = "0x7c - Lane Configuration and Flow Control"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
}
#[doc = "SrcAddr (rw) register accessor: Source start address\n\nYou can [`read`](crate::Reg::read) this register and get [`src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr`] module"]
pub type SrcAddr = crate::Reg<src_addr::SrcAddrSpec>;
#[doc = "Source start address"]
pub mod src_addr;
#[doc = "DstAddr (rw) register accessor: Destination start address\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr`] module"]
pub type DstAddr = crate::Reg<dst_addr::DstAddrSpec>;
#[doc = "Destination start address"]
pub mod dst_addr;
#[doc = "LLI (rw) register accessor: Next LLI Address and control\n\nYou can [`read`](crate::Reg::read) this register and get [`lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lli`] module"]
#[doc(alias = "LLI")]
pub type Lli = crate::Reg<lli::LliSpec>;
#[doc = "Next LLI Address and control"]
pub mod lli;
#[doc = "BUS (rw) register accessor: Lane Bus and Burst Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus`] module"]
#[doc(alias = "BUS")]
pub type Bus = crate::Reg<bus::BusSpec>;
#[doc = "Lane Bus and Burst Control"]
pub mod bus;
#[doc = "SRC_CNT (rw) register accessor: Source 2D Dimensions\n\nYou can [`read`](crate::Reg::read) this register and get [`src_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_cnt`] module"]
#[doc(alias = "SRC_CNT")]
pub type SrcCnt = crate::Reg<src_cnt::SrcCntSpec>;
#[doc = "Source 2D Dimensions"]
pub mod src_cnt;
#[doc = "SRC_XIC (rw) register accessor: Source X Stride (Increment)\n\nYou can [`read`](crate::Reg::read) this register and get [`src_xic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_xic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_xic`] module"]
#[doc(alias = "SRC_XIC")]
pub type SrcXic = crate::Reg<src_xic::SrcXicSpec>;
#[doc = "Source X Stride (Increment)"]
pub mod src_xic;
#[doc = "SRC_YIC (rw) register accessor: Source Y Stride (Increment)\n\nYou can [`read`](crate::Reg::read) this register and get [`src_yic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_yic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_yic`] module"]
#[doc(alias = "SRC_YIC")]
pub type SrcYic = crate::Reg<src_yic::SrcYicSpec>;
#[doc = "Source Y Stride (Increment)"]
pub mod src_yic;
#[doc = "DST_CNT (rw) register accessor: Destination 2D Width\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_cnt`] module"]
#[doc(alias = "DST_CNT")]
pub type DstCnt = crate::Reg<dst_cnt::DstCntSpec>;
#[doc = "Destination 2D Width"]
pub mod dst_cnt;
#[doc = "DST_XIC (rw) register accessor: Destination X Stride (Increment)\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_xic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_xic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_xic`] module"]
#[doc(alias = "DST_XIC")]
pub type DstXic = crate::Reg<dst_xic::DstXicSpec>;
#[doc = "Destination X Stride (Increment)"]
pub mod dst_xic;
#[doc = "DST_YIC (rw) register accessor: Destination Y Stride (Increment)\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_yic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_yic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_yic`] module"]
#[doc(alias = "DST_YIC")]
pub type DstYic = crate::Reg<dst_yic::DstYicSpec>;
#[doc = "Destination Y Stride (Increment)"]
pub mod dst_yic;
#[doc = "KEY (rw) register accessor: Color Key Value\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "Color Key Value"]
pub mod key;
#[doc = "KEY_EN (rw) register accessor: Color Key Control\n\nYou can [`read`](crate::Reg::read) this register and get [`key_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_en`] module"]
#[doc(alias = "KEY_EN")]
pub type KeyEn = crate::Reg<key_en::KeyEnSpec>;
#[doc = "Color Key Control"]
pub mod key_en;
#[doc = "CFG (rw) register accessor: Lane Configuration and Flow Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Lane Configuration and Flow Control"]
pub mod cfg;
