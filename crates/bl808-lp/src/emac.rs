#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: Mode,
    int_source: IntSource,
    int_mask: IntMask,
    ipgt: Ipgt,
    _reserved4: [u8; 0x08],
    packetlen: Packetlen,
    collconfig: Collconfig,
    tx_bd_num: TxBdNum,
    _reserved7: [u8; 0x04],
    miimode: Miimode,
    miicommand: Miicommand,
    miiaddress: Miiaddress,
    miitx_data: MiitxData,
    miirx_data: MiirxData,
    miistatus: Miistatus,
    mac_addr0: MacAddr0,
    mac_addr1: MacAddr1,
    hash0_addr: Hash0Addr,
    hash1_addr: Hash1Addr,
    txctrl: Txctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC Mode Configuration Register"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x04 - Interrupt Source Register"]
    #[inline(always)]
    pub const fn int_source(&self) -> &IntSource {
        &self.int_source
    }
    #[doc = "0x08 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0x0c - Back-to-Back Inter-Packet-Gap Register"]
    #[inline(always)]
    pub const fn ipgt(&self) -> &Ipgt {
        &self.ipgt
    }
    #[doc = "0x18 - Packet Length Register"]
    #[inline(always)]
    pub const fn packetlen(&self) -> &Packetlen {
        &self.packetlen
    }
    #[doc = "0x1c - Collision Configuration Register"]
    #[inline(always)]
    pub const fn collconfig(&self) -> &Collconfig {
        &self.collconfig
    }
    #[doc = "0x20 - Transmit Buffer Descriptor Number and Pointer Register"]
    #[inline(always)]
    pub const fn tx_bd_num(&self) -> &TxBdNum {
        &self.tx_bd_num
    }
    #[doc = "0x28 - MII Management Mode Register"]
    #[inline(always)]
    pub const fn miimode(&self) -> &Miimode {
        &self.miimode
    }
    #[doc = "0x2c - MII Management Command Register"]
    #[inline(always)]
    pub const fn miicommand(&self) -> &Miicommand {
        &self.miicommand
    }
    #[doc = "0x30 - MII Management Address Register"]
    #[inline(always)]
    pub const fn miiaddress(&self) -> &Miiaddress {
        &self.miiaddress
    }
    #[doc = "0x34 - MII Management Transmit Data Register"]
    #[inline(always)]
    pub const fn miitx_data(&self) -> &MiitxData {
        &self.miitx_data
    }
    #[doc = "0x38 - MII Management Receive Data Register"]
    #[inline(always)]
    pub const fn miirx_data(&self) -> &MiirxData {
        &self.miirx_data
    }
    #[doc = "0x3c - MII Management Status Register"]
    #[inline(always)]
    pub const fn miistatus(&self) -> &Miistatus {
        &self.miistatus
    }
    #[doc = "0x40 - MAC Individual Address Register 0 (Bytes 5-2)"]
    #[inline(always)]
    pub const fn mac_addr0(&self) -> &MacAddr0 {
        &self.mac_addr0
    }
    #[doc = "0x44 - MAC Individual Address Register 1 (Bytes 1-0)"]
    #[inline(always)]
    pub const fn mac_addr1(&self) -> &MacAddr1 {
        &self.mac_addr1
    }
    #[doc = "0x48 - Hash Table Register 0"]
    #[inline(always)]
    pub const fn hash0_addr(&self) -> &Hash0Addr {
        &self.hash0_addr
    }
    #[doc = "0x4c - Hash Table Register 1"]
    #[inline(always)]
    pub const fn hash1_addr(&self) -> &Hash1Addr {
        &self.hash1_addr
    }
    #[doc = "0x50 - Transmit Control (Flow Control) Register"]
    #[inline(always)]
    pub const fn txctrl(&self) -> &Txctrl {
        &self.txctrl
    }
}
#[doc = "MODE (rw) register accessor: MAC Mode Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "MAC Mode Configuration Register"]
pub mod mode;
#[doc = "INT_SOURCE (rw) register accessor: Interrupt Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_source::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_source::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_source`] module"]
#[doc(alias = "INT_SOURCE")]
pub type IntSource = crate::Reg<int_source::IntSourceSpec>;
#[doc = "Interrupt Source Register"]
pub mod int_source;
#[doc = "INT_MASK (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`] module"]
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "Interrupt Mask Register"]
pub mod int_mask;
#[doc = "IPGT (rw) register accessor: Back-to-Back Inter-Packet-Gap Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipgt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipgt`] module"]
#[doc(alias = "IPGT")]
pub type Ipgt = crate::Reg<ipgt::IpgtSpec>;
#[doc = "Back-to-Back Inter-Packet-Gap Register"]
pub mod ipgt;
#[doc = "PACKETLEN (rw) register accessor: Packet Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`packetlen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packetlen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@packetlen`] module"]
#[doc(alias = "PACKETLEN")]
pub type Packetlen = crate::Reg<packetlen::PacketlenSpec>;
#[doc = "Packet Length Register"]
pub mod packetlen;
#[doc = "COLLCONFIG (rw) register accessor: Collision Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`collconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`collconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@collconfig`] module"]
#[doc(alias = "COLLCONFIG")]
pub type Collconfig = crate::Reg<collconfig::CollconfigSpec>;
#[doc = "Collision Configuration Register"]
pub mod collconfig;
#[doc = "TX_BD_NUM (rw) register accessor: Transmit Buffer Descriptor Number and Pointer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_bd_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_bd_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_bd_num`] module"]
#[doc(alias = "TX_BD_NUM")]
pub type TxBdNum = crate::Reg<tx_bd_num::TxBdNumSpec>;
#[doc = "Transmit Buffer Descriptor Number and Pointer Register"]
pub mod tx_bd_num;
#[doc = "MIIMODE (rw) register accessor: MII Management Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miimode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miimode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miimode`] module"]
#[doc(alias = "MIIMODE")]
pub type Miimode = crate::Reg<miimode::MiimodeSpec>;
#[doc = "MII Management Mode Register"]
pub mod miimode;
#[doc = "MIICOMMAND (rw) register accessor: MII Management Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miicommand::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miicommand::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miicommand`] module"]
#[doc(alias = "MIICOMMAND")]
pub type Miicommand = crate::Reg<miicommand::MiicommandSpec>;
#[doc = "MII Management Command Register"]
pub mod miicommand;
#[doc = "MIIADDRESS (rw) register accessor: MII Management Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miiaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miiaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miiaddress`] module"]
#[doc(alias = "MIIADDRESS")]
pub type Miiaddress = crate::Reg<miiaddress::MiiaddressSpec>;
#[doc = "MII Management Address Register"]
pub mod miiaddress;
#[doc = "MIITX_DATA (rw) register accessor: MII Management Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miitx_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miitx_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miitx_data`] module"]
#[doc(alias = "MIITX_DATA")]
pub type MiitxData = crate::Reg<miitx_data::MiitxDataSpec>;
#[doc = "MII Management Transmit Data Register"]
pub mod miitx_data;
#[doc = "MIIRX_DATA (r) register accessor: MII Management Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miirx_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miirx_data`] module"]
#[doc(alias = "MIIRX_DATA")]
pub type MiirxData = crate::Reg<miirx_data::MiirxDataSpec>;
#[doc = "MII Management Receive Data Register"]
pub mod miirx_data;
#[doc = "MIISTATUS (r) register accessor: MII Management Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miistatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miistatus`] module"]
#[doc(alias = "MIISTATUS")]
pub type Miistatus = crate::Reg<miistatus::MiistatusSpec>;
#[doc = "MII Management Status Register"]
pub mod miistatus;
#[doc = "MAC_ADDR0 (rw) register accessor: MAC Individual Address Register 0 (Bytes 5-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_addr0`] module"]
#[doc(alias = "MAC_ADDR0")]
pub type MacAddr0 = crate::Reg<mac_addr0::MacAddr0Spec>;
#[doc = "MAC Individual Address Register 0 (Bytes 5-2)"]
pub mod mac_addr0;
#[doc = "MAC_ADDR1 (rw) register accessor: MAC Individual Address Register 1 (Bytes 1-0)\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_addr1`] module"]
#[doc(alias = "MAC_ADDR1")]
pub type MacAddr1 = crate::Reg<mac_addr1::MacAddr1Spec>;
#[doc = "MAC Individual Address Register 1 (Bytes 1-0)"]
pub mod mac_addr1;
#[doc = "HASH0_ADDR (rw) register accessor: Hash Table Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hash0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash0_addr`] module"]
#[doc(alias = "HASH0_ADDR")]
pub type Hash0Addr = crate::Reg<hash0_addr::Hash0AddrSpec>;
#[doc = "Hash Table Register 0"]
pub mod hash0_addr;
#[doc = "HASH1_ADDR (rw) register accessor: Hash Table Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hash1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash1_addr`] module"]
#[doc(alias = "HASH1_ADDR")]
pub type Hash1Addr = crate::Reg<hash1_addr::Hash1AddrSpec>;
#[doc = "Hash Table Register 1"]
pub mod hash1_addr;
#[doc = "TXCTRL (rw) register accessor: Transmit Control (Flow Control) Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctrl`] module"]
#[doc(alias = "TXCTRL")]
pub type Txctrl = crate::Reg<txctrl::TxctrlSpec>;
#[doc = "Transmit Control (Flow Control) Register"]
pub mod txctrl;
