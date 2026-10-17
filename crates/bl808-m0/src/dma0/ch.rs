#[repr(C)]
#[doc = "DMA Channel Registers"]
#[doc(alias = "CH")]
pub struct Ch {
    src_addr: SrcAddr,
    dst_addr: DstAddr,
    lli: Lli,
    control: Control,
    config: Config,
    _reserved5: [u8; 0x08],
    rsvd: Rsvd,
}
impl Ch {
    #[doc = "0x00 - DMA source address"]
    #[inline(always)]
    pub const fn src_addr(&self) -> &SrcAddr {
        &self.src_addr
    }
    #[doc = "0x04 - DMA destination address"]
    #[inline(always)]
    pub const fn dst_addr(&self) -> &DstAddr {
        &self.dst_addr
    }
    #[doc = "0x08 - Next linked list item address"]
    #[inline(always)]
    pub const fn lli(&self) -> &Lli {
        &self.lli
    }
    #[doc = "0x0c - Channel Control Register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x10 - Channel Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x1c - Channel Reserved Register"]
    #[inline(always)]
    pub const fn rsvd(&self) -> &Rsvd {
        &self.rsvd
    }
}
#[doc = "SrcAddr (rw) register accessor: DMA source address\n\nYou can [`read`](crate::Reg::read) this register and get [`src_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr`] module"]
pub type SrcAddr = crate::Reg<src_addr::SrcAddrSpec>;
#[doc = "DMA source address"]
pub mod src_addr;
#[doc = "DstAddr (rw) register accessor: DMA destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr`] module"]
pub type DstAddr = crate::Reg<dst_addr::DstAddrSpec>;
#[doc = "DMA destination address"]
pub mod dst_addr;
#[doc = "LLI (rw) register accessor: Next linked list item address\n\nYou can [`read`](crate::Reg::read) this register and get [`lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lli`] module"]
#[doc(alias = "LLI")]
pub type Lli = crate::Reg<lli::LliSpec>;
#[doc = "Next linked list item address"]
pub mod lli;
#[doc = "Control (rw) register accessor: Channel Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Channel Control Register"]
pub mod control;
#[doc = "Config (rw) register accessor: Channel Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Channel Configuration Register"]
pub mod config;
#[doc = "RSVD (rw) register accessor: Channel Reserved Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsvd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsvd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsvd`] module"]
#[doc(alias = "RSVD")]
pub type Rsvd = crate::Reg<rsvd::RsvdSpec>;
#[doc = "Channel Reserved Register"]
pub mod rsvd;
