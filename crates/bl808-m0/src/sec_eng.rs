#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    se_sha_0_ctrl: SeSha0Ctrl,
    se_sha_0_msa: SeSha0Msa,
    se_sha_0_status: SeSha0Status,
    se_sha_0_endian: SeSha0Endian,
    se_sha_0_hash_l_: [SeSha0HashL_; 8],
    se_sha_0_hash_h_: [SeSha0HashH_; 8],
    se_sha_0_link: SeSha0Link,
    _reserved7: [u8; 0xa8],
    se_sha_0_ctrl_prot: SeSha0CtrlProt,
    se_aes_0_ctrl: SeAes0Ctrl,
    se_aes_0_msa: SeAes0Msa,
    se_aes_0_mda: SeAes0Mda,
    se_aes_0_status: SeAes0Status,
    se_aes_0_iv_: [SeAes0Iv_; 4],
    se_aes_0_key_: [SeAes0Key_; 8],
    se_aes_0_key_sel: SeAes0KeySel,
    se_aes_1_key_sel: SeAes1KeySel,
    se_aes_0_endian: SeAes0Endian,
    se_aes_sboot: SeAesSboot,
    se_aes_0_link: SeAes0Link,
    _reserved19: [u8; 0xa8],
    se_aes_0_ctrl_prot: SeAes0CtrlProt,
    se_trng_0_ctrl_0: SeTrng0Ctrl0,
    se_trng_0_status: SeTrng0Status,
    se_trng_0_dout_: [SeTrng0Dout_; 8],
    se_trng_0_test: SeTrng0Test,
    se_trng_0_ctrl_1: SeTrng0Ctrl1,
    se_trng_0_ctrl_2: SeTrng0Ctrl2,
    se_trng_0_ctrl_3: SeTrng0Ctrl3,
    _reserved27: [u8; 0x08],
    se_trng_0_test_out_: [SeTrng0TestOut_; 4],
    _reserved28: [u8; 0xac],
    se_trng_0_ctrl_prot: SeTrng0CtrlProt,
    se_pka_0_ctrl_0: SePka0Ctrl0,
    _reserved30: [u8; 0x08],
    se_pka_0_seed: SePka0Seed,
    se_pka_0_ctrl_1: SePka0Ctrl1,
    _reserved32: [u8; 0x2c],
    se_pka_0_rw: SePka0Rw,
    _reserved33: [u8; 0x1c],
    se_pka_0_rw_burst: SePka0RwBurst,
    _reserved34: [u8; 0x98],
    se_pka_0_ctrl_prot: SePka0CtrlProt,
    se_cdet_0_ctrl_0: SeCdet0Ctrl0,
    se_cdet_0_ctrl_1: SeCdet0Ctrl1,
    _reserved37: [u8; 0xf4],
    se_cdet_0_ctrl_prot: SeCdet0CtrlProt,
    se_gmac_0_ctrl_0: SeGmac0Ctrl0,
    se_gmac_0_lca: SeGmac0Lca,
    se_gmac_0_status: SeGmac0Status,
    _reserved41: [u8; 0xf0],
    se_gmac_0_ctrl_prot: SeGmac0CtrlProt,
    _reserved42: [u8; 0x0900],
    se_ctrl_prot_rd: SeCtrlProtRd,
    se_ctrl_reserved_0: SeCtrlReserved0,
    se_ctrl_reserved_1: SeCtrlReserved1,
    se_ctrl_reserved_2: SeCtrlReserved2,
}
impl RegisterBlock {
    #[doc = "0x00 - SHA Control Register"]
    #[inline(always)]
    pub const fn se_sha_0_ctrl(&self) -> &SeSha0Ctrl {
        &self.se_sha_0_ctrl
    }
    #[doc = "0x04 - SHA Message Source Address"]
    #[inline(always)]
    pub const fn se_sha_0_msa(&self) -> &SeSha0Msa {
        &self.se_sha_0_msa
    }
    #[doc = "0x08 - SHA Engine Status"]
    #[inline(always)]
    pub const fn se_sha_0_status(&self) -> &SeSha0Status {
        &self.se_sha_0_status
    }
    #[doc = "0x0c - SHA Output Endian Control"]
    #[inline(always)]
    pub const fn se_sha_0_endian(&self) -> &SeSha0Endian {
        &self.se_sha_0_endian
    }
    #[doc = "0x10..0x30 - SHA Hash Result Low \\[255:0\\]"]
    #[inline(always)]
    pub const fn se_sha_0_hash_l_(&self, n: usize) -> &SeSha0HashL_ {
        &self.se_sha_0_hash_l_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x30 - SHA Hash Result Low \\[255:0\\]"]
    #[inline(always)]
    pub fn se_sha_0_hash_l__iter(&self) -> impl Iterator<Item = &SeSha0HashL_> {
        self.se_sha_0_hash_l_.iter()
    }
    #[doc = "0x30..0x50 - SHA Hash Result High \\[511:256\\]"]
    #[inline(always)]
    pub const fn se_sha_0_hash_h_(&self, n: usize) -> &SeSha0HashH_ {
        &self.se_sha_0_hash_h_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x50 - SHA Hash Result High \\[511:256\\]"]
    #[inline(always)]
    pub fn se_sha_0_hash_h__iter(&self) -> impl Iterator<Item = &SeSha0HashH_> {
        self.se_sha_0_hash_h_.iter()
    }
    #[doc = "0x50 - SHA Link List Address"]
    #[inline(always)]
    pub const fn se_sha_0_link(&self) -> &SeSha0Link {
        &self.se_sha_0_link
    }
    #[doc = "0xfc - SHA Engine Protection"]
    #[inline(always)]
    pub const fn se_sha_0_ctrl_prot(&self) -> &SeSha0CtrlProt {
        &self.se_sha_0_ctrl_prot
    }
    #[doc = "0x100 - AES Control Register"]
    #[inline(always)]
    pub const fn se_aes_0_ctrl(&self) -> &SeAes0Ctrl {
        &self.se_aes_0_ctrl
    }
    #[doc = "0x104 - AES Message Source Address"]
    #[inline(always)]
    pub const fn se_aes_0_msa(&self) -> &SeAes0Msa {
        &self.se_aes_0_msa
    }
    #[doc = "0x108 - AES Message Destination Address"]
    #[inline(always)]
    pub const fn se_aes_0_mda(&self) -> &SeAes0Mda {
        &self.se_aes_0_mda
    }
    #[doc = "0x10c - AES Engine Status"]
    #[inline(always)]
    pub const fn se_aes_0_status(&self) -> &SeAes0Status {
        &self.se_aes_0_status
    }
    #[doc = "0x110..0x120 - AES IV Register %s"]
    #[inline(always)]
    pub const fn se_aes_0_iv_(&self, n: usize) -> &SeAes0Iv_ {
        &self.se_aes_0_iv_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x120 - AES IV Register %s"]
    #[inline(always)]
    pub fn se_aes_0_iv__iter(&self) -> impl Iterator<Item = &SeAes0Iv_> {
        self.se_aes_0_iv_.iter()
    }
    #[doc = "0x120..0x140 - AES Key Register %s"]
    #[inline(always)]
    pub const fn se_aes_0_key_(&self, n: usize) -> &SeAes0Key_ {
        &self.se_aes_0_key_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x140 - AES Key Register %s"]
    #[inline(always)]
    pub fn se_aes_0_key__iter(&self) -> impl Iterator<Item = &SeAes0Key_> {
        self.se_aes_0_key_.iter()
    }
    #[doc = "0x140 - AES-0 HW Key Selection"]
    #[inline(always)]
    pub const fn se_aes_0_key_sel(&self) -> &SeAes0KeySel {
        &self.se_aes_0_key_sel
    }
    #[doc = "0x144 - AES-1 HW Key Selection"]
    #[inline(always)]
    pub const fn se_aes_1_key_sel(&self) -> &SeAes1KeySel {
        &self.se_aes_1_key_sel
    }
    #[doc = "0x148 - AES Endian and CTR Configuration"]
    #[inline(always)]
    pub const fn se_aes_0_endian(&self) -> &SeAes0Endian {
        &self.se_aes_0_endian
    }
    #[doc = "0x14c - AES Secure Boot Configuration"]
    #[inline(always)]
    pub const fn se_aes_sboot(&self) -> &SeAesSboot {
        &self.se_aes_sboot
    }
    #[doc = "0x150 - AES Link List Address"]
    #[inline(always)]
    pub const fn se_aes_0_link(&self) -> &SeAes0Link {
        &self.se_aes_0_link
    }
    #[doc = "0x1fc - AES Engine Protection"]
    #[inline(always)]
    pub const fn se_aes_0_ctrl_prot(&self) -> &SeAes0CtrlProt {
        &self.se_aes_0_ctrl_prot
    }
    #[doc = "0x200 - TRNG Control Register 0"]
    #[inline(always)]
    pub const fn se_trng_0_ctrl_0(&self) -> &SeTrng0Ctrl0 {
        &self.se_trng_0_ctrl_0
    }
    #[doc = "0x204 - TRNG Engine Status"]
    #[inline(always)]
    pub const fn se_trng_0_status(&self) -> &SeTrng0Status {
        &self.se_trng_0_status
    }
    #[doc = "0x208..0x228 - TRNG Output Data %s"]
    #[inline(always)]
    pub const fn se_trng_0_dout_(&self, n: usize) -> &SeTrng0Dout_ {
        &self.se_trng_0_dout_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x208..0x228 - TRNG Output Data %s"]
    #[inline(always)]
    pub fn se_trng_0_dout__iter(&self) -> impl Iterator<Item = &SeTrng0Dout_> {
        self.se_trng_0_dout_.iter()
    }
    #[doc = "0x228 - TRNG Test Configuration"]
    #[inline(always)]
    pub const fn se_trng_0_test(&self) -> &SeTrng0Test {
        &self.se_trng_0_test
    }
    #[doc = "0x22c - TRNG Reseed Parameter LSB"]
    #[inline(always)]
    pub const fn se_trng_0_ctrl_1(&self) -> &SeTrng0Ctrl1 {
        &self.se_trng_0_ctrl_1
    }
    #[doc = "0x230 - TRNG Reseed Parameter MSB"]
    #[inline(always)]
    pub const fn se_trng_0_ctrl_2(&self) -> &SeTrng0Ctrl2 {
        &self.se_trng_0_ctrl_2
    }
    #[doc = "0x234 - TRNG HT and ROSC Control"]
    #[inline(always)]
    pub const fn se_trng_0_ctrl_3(&self) -> &SeTrng0Ctrl3 {
        &self.se_trng_0_ctrl_3
    }
    #[doc = "0x240..0x250 - TRNG Test Output %s"]
    #[inline(always)]
    pub const fn se_trng_0_test_out_(&self, n: usize) -> &SeTrng0TestOut_ {
        &self.se_trng_0_test_out_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x240..0x250 - TRNG Test Output %s"]
    #[inline(always)]
    pub fn se_trng_0_test_out__iter(&self) -> impl Iterator<Item = &SeTrng0TestOut_> {
        self.se_trng_0_test_out_.iter()
    }
    #[doc = "0x2fc - TRNG Engine Protection"]
    #[inline(always)]
    pub const fn se_trng_0_ctrl_prot(&self) -> &SeTrng0CtrlProt {
        &self.se_trng_0_ctrl_prot
    }
    #[doc = "0x300 - PKA Control Register 0"]
    #[inline(always)]
    pub const fn se_pka_0_ctrl_0(&self) -> &SePka0Ctrl0 {
        &self.se_pka_0_ctrl_0
    }
    #[doc = "0x30c - PKA Random Seed"]
    #[inline(always)]
    pub const fn se_pka_0_seed(&self) -> &SePka0Seed {
        &self.se_pka_0_seed
    }
    #[doc = "0x310 - PKA Burst Configuration"]
    #[inline(always)]
    pub const fn se_pka_0_ctrl_1(&self) -> &SePka0Ctrl1 {
        &self.se_pka_0_ctrl_1
    }
    #[doc = "0x340 - PKA Register Read/Write"]
    #[inline(always)]
    pub const fn se_pka_0_rw(&self) -> &SePka0Rw {
        &self.se_pka_0_rw
    }
    #[doc = "0x360 - PKA Register Burst Read/Write"]
    #[inline(always)]
    pub const fn se_pka_0_rw_burst(&self) -> &SePka0RwBurst {
        &self.se_pka_0_rw_burst
    }
    #[doc = "0x3fc - PKA Engine Protection"]
    #[inline(always)]
    pub const fn se_pka_0_ctrl_prot(&self) -> &SePka0CtrlProt {
        &self.se_pka_0_ctrl_prot
    }
    #[doc = "0x400 - Clock Detection Control 0"]
    #[inline(always)]
    pub const fn se_cdet_0_ctrl_0(&self) -> &SeCdet0Ctrl0 {
        &self.se_cdet_0_ctrl_0
    }
    #[doc = "0x404 - Clock Detection Control 1"]
    #[inline(always)]
    pub const fn se_cdet_0_ctrl_1(&self) -> &SeCdet0Ctrl1 {
        &self.se_cdet_0_ctrl_1
    }
    #[doc = "0x4fc - CDET Engine Protection"]
    #[inline(always)]
    pub const fn se_cdet_0_ctrl_prot(&self) -> &SeCdet0CtrlProt {
        &self.se_cdet_0_ctrl_prot
    }
    #[doc = "0x500 - GMAC Control Register 0"]
    #[inline(always)]
    pub const fn se_gmac_0_ctrl_0(&self) -> &SeGmac0Ctrl0 {
        &self.se_gmac_0_ctrl_0
    }
    #[doc = "0x504 - GMAC Link List Address"]
    #[inline(always)]
    pub const fn se_gmac_0_lca(&self) -> &SeGmac0Lca {
        &self.se_gmac_0_lca
    }
    #[doc = "0x508 - GMAC Status Register"]
    #[inline(always)]
    pub const fn se_gmac_0_status(&self) -> &SeGmac0Status {
        &self.se_gmac_0_status
    }
    #[doc = "0x5fc - GMAC Engine Protection"]
    #[inline(always)]
    pub const fn se_gmac_0_ctrl_prot(&self) -> &SeGmac0CtrlProt {
        &self.se_gmac_0_ctrl_prot
    }
    #[doc = "0xf00 - Global Sub-engine Access Right Status"]
    #[inline(always)]
    pub const fn se_ctrl_prot_rd(&self) -> &SeCtrlProtRd {
        &self.se_ctrl_prot_rd
    }
    #[doc = "0xf04 - Security engine internal bus configuration. Reserved for hardware-specific arbitration and timing tuning."]
    #[inline(always)]
    pub const fn se_ctrl_reserved_0(&self) -> &SeCtrlReserved0 {
        &self.se_ctrl_reserved_0
    }
    #[doc = "0xf08 - Security engine internal status and control. Reserved for factory-specific hardware configuration."]
    #[inline(always)]
    pub const fn se_ctrl_reserved_1(&self) -> &SeCtrlReserved1 {
        &self.se_ctrl_reserved_1
    }
    #[doc = "0xf0c - Security engine hardware revision identification. Returns the silicon-specific version code for the encryption accelerators."]
    #[inline(always)]
    pub const fn se_ctrl_reserved_2(&self) -> &SeCtrlReserved2 {
        &self.se_ctrl_reserved_2
    }
}
#[doc = "SE_SHA_0_CTRL (rw) register accessor: SHA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_ctrl`] module"]
#[doc(alias = "SE_SHA_0_CTRL")]
pub type SeSha0Ctrl = crate::Reg<se_sha_0_ctrl::SeSha0CtrlSpec>;
#[doc = "SHA Control Register"]
pub mod se_sha_0_ctrl;
#[doc = "SE_SHA_0_MSA (rw) register accessor: SHA Message Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_msa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_msa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_msa`] module"]
#[doc(alias = "SE_SHA_0_MSA")]
pub type SeSha0Msa = crate::Reg<se_sha_0_msa::SeSha0MsaSpec>;
#[doc = "SHA Message Source Address"]
pub mod se_sha_0_msa;
#[doc = "SE_SHA_0_STATUS (r) register accessor: SHA Engine Status\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_status`] module"]
#[doc(alias = "SE_SHA_0_STATUS")]
pub type SeSha0Status = crate::Reg<se_sha_0_status::SeSha0StatusSpec>;
#[doc = "SHA Engine Status"]
pub mod se_sha_0_status;
#[doc = "SE_SHA_0_ENDIAN (rw) register accessor: SHA Output Endian Control\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_endian::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_endian::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_endian`] module"]
#[doc(alias = "SE_SHA_0_ENDIAN")]
pub type SeSha0Endian = crate::Reg<se_sha_0_endian::SeSha0EndianSpec>;
#[doc = "SHA Output Endian Control"]
pub mod se_sha_0_endian;
#[doc = "SE_SHA_0_HASH_L_ (r) register accessor: SHA Hash Result Low \\[255:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_l_`] module"]
#[doc(alias = "SE_SHA_0_HASH_L_")]
pub type SeSha0HashL_ = crate::Reg<se_sha_0_hash_l_::SeSha0HashL_Spec>;
#[doc = "SHA Hash Result Low \\[255:0\\]"]
pub mod se_sha_0_hash_l_;
#[doc = "SE_SHA_0_HASH_H_ (r) register accessor: SHA Hash Result High \\[511:256\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_hash_h_`] module"]
#[doc(alias = "SE_SHA_0_HASH_H_")]
pub type SeSha0HashH_ = crate::Reg<se_sha_0_hash_h_::SeSha0HashH_Spec>;
#[doc = "SHA Hash Result High \\[511:256\\]"]
pub mod se_sha_0_hash_h_;
#[doc = "SE_SHA_0_LINK (rw) register accessor: SHA Link List Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_link`] module"]
#[doc(alias = "SE_SHA_0_LINK")]
pub type SeSha0Link = crate::Reg<se_sha_0_link::SeSha0LinkSpec>;
#[doc = "SHA Link List Address"]
pub mod se_sha_0_link;
#[doc = "SE_SHA_0_CTRL_PROT (rw) register accessor: SHA Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_sha_0_ctrl_prot`] module"]
#[doc(alias = "SE_SHA_0_CTRL_PROT")]
pub type SeSha0CtrlProt = crate::Reg<se_sha_0_ctrl_prot::SeSha0CtrlProtSpec>;
#[doc = "SHA Engine Protection"]
pub mod se_sha_0_ctrl_prot;
#[doc = "SE_AES_0_CTRL (rw) register accessor: AES Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_ctrl`] module"]
#[doc(alias = "SE_AES_0_CTRL")]
pub type SeAes0Ctrl = crate::Reg<se_aes_0_ctrl::SeAes0CtrlSpec>;
#[doc = "AES Control Register"]
pub mod se_aes_0_ctrl;
#[doc = "SE_AES_0_MSA (rw) register accessor: AES Message Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_msa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_msa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_msa`] module"]
#[doc(alias = "SE_AES_0_MSA")]
pub type SeAes0Msa = crate::Reg<se_aes_0_msa::SeAes0MsaSpec>;
#[doc = "AES Message Source Address"]
pub mod se_aes_0_msa;
#[doc = "SE_AES_0_MDA (rw) register accessor: AES Message Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_mda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_mda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_mda`] module"]
#[doc(alias = "SE_AES_0_MDA")]
pub type SeAes0Mda = crate::Reg<se_aes_0_mda::SeAes0MdaSpec>;
#[doc = "AES Message Destination Address"]
pub mod se_aes_0_mda;
#[doc = "SE_AES_0_STATUS (r) register accessor: AES Engine Status\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_status`] module"]
#[doc(alias = "SE_AES_0_STATUS")]
pub type SeAes0Status = crate::Reg<se_aes_0_status::SeAes0StatusSpec>;
#[doc = "AES Engine Status"]
pub mod se_aes_0_status;
#[doc = "SE_AES_0_IV_ (rw) register accessor: AES IV Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_iv_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_iv_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_iv_`] module"]
#[doc(alias = "SE_AES_0_IV_")]
pub type SeAes0Iv_ = crate::Reg<se_aes_0_iv_::SeAes0Iv_Spec>;
#[doc = "AES IV Register %s"]
pub mod se_aes_0_iv_;
#[doc = "SE_AES_0_KEY_ (rw) register accessor: AES Key Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_`] module"]
#[doc(alias = "SE_AES_0_KEY_")]
pub type SeAes0Key_ = crate::Reg<se_aes_0_key_::SeAes0Key_Spec>;
#[doc = "AES Key Register %s"]
pub mod se_aes_0_key_;
#[doc = "SE_AES_0_KEY_SEL (rw) register accessor: AES-0 HW Key Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_key_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_key_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_key_sel`] module"]
#[doc(alias = "SE_AES_0_KEY_SEL")]
pub type SeAes0KeySel = crate::Reg<se_aes_0_key_sel::SeAes0KeySelSpec>;
#[doc = "AES-0 HW Key Selection"]
pub mod se_aes_0_key_sel;
#[doc = "SE_AES_1_KEY_SEL (rw) register accessor: AES-1 HW Key Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_1_key_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_1_key_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_1_key_sel`] module"]
#[doc(alias = "SE_AES_1_KEY_SEL")]
pub type SeAes1KeySel = crate::Reg<se_aes_1_key_sel::SeAes1KeySelSpec>;
#[doc = "AES-1 HW Key Selection"]
pub mod se_aes_1_key_sel;
#[doc = "SE_AES_0_ENDIAN (rw) register accessor: AES Endian and CTR Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_endian::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_endian::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_endian`] module"]
#[doc(alias = "SE_AES_0_ENDIAN")]
pub type SeAes0Endian = crate::Reg<se_aes_0_endian::SeAes0EndianSpec>;
#[doc = "AES Endian and CTR Configuration"]
pub mod se_aes_0_endian;
#[doc = "SE_AES_SBOOT (rw) register accessor: AES Secure Boot Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_sboot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_sboot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_sboot`] module"]
#[doc(alias = "SE_AES_SBOOT")]
pub type SeAesSboot = crate::Reg<se_aes_sboot::SeAesSbootSpec>;
#[doc = "AES Secure Boot Configuration"]
pub mod se_aes_sboot;
#[doc = "SE_AES_0_LINK (rw) register accessor: AES Link List Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_link::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_link::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_link`] module"]
#[doc(alias = "SE_AES_0_LINK")]
pub type SeAes0Link = crate::Reg<se_aes_0_link::SeAes0LinkSpec>;
#[doc = "AES Link List Address"]
pub mod se_aes_0_link;
#[doc = "SE_AES_0_CTRL_PROT (rw) register accessor: AES Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_aes_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_aes_0_ctrl_prot`] module"]
#[doc(alias = "SE_AES_0_CTRL_PROT")]
pub type SeAes0CtrlProt = crate::Reg<se_aes_0_ctrl_prot::SeAes0CtrlProtSpec>;
#[doc = "AES Engine Protection"]
pub mod se_aes_0_ctrl_prot;
#[doc = "SE_TRNG_0_CTRL_0 (rw) register accessor: TRNG Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_ctrl_0`] module"]
#[doc(alias = "SE_TRNG_0_CTRL_0")]
pub type SeTrng0Ctrl0 = crate::Reg<se_trng_0_ctrl_0::SeTrng0Ctrl0Spec>;
#[doc = "TRNG Control Register 0"]
pub mod se_trng_0_ctrl_0;
#[doc = "SE_TRNG_0_STATUS (r) register accessor: TRNG Engine Status\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_status`] module"]
#[doc(alias = "SE_TRNG_0_STATUS")]
pub type SeTrng0Status = crate::Reg<se_trng_0_status::SeTrng0StatusSpec>;
#[doc = "TRNG Engine Status"]
pub mod se_trng_0_status;
#[doc = "SE_TRNG_0_DOUT_ (r) register accessor: TRNG Output Data %s\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_dout_`] module"]
#[doc(alias = "SE_TRNG_0_DOUT_")]
pub type SeTrng0Dout_ = crate::Reg<se_trng_0_dout_::SeTrng0Dout_Spec>;
#[doc = "TRNG Output Data %s"]
pub mod se_trng_0_dout_;
#[doc = "SE_TRNG_0_TEST (rw) register accessor: TRNG Test Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_test`] module"]
#[doc(alias = "SE_TRNG_0_TEST")]
pub type SeTrng0Test = crate::Reg<se_trng_0_test::SeTrng0TestSpec>;
#[doc = "TRNG Test Configuration"]
pub mod se_trng_0_test;
#[doc = "SE_TRNG_0_CTRL_1 (rw) register accessor: TRNG Reseed Parameter LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_ctrl_1`] module"]
#[doc(alias = "SE_TRNG_0_CTRL_1")]
pub type SeTrng0Ctrl1 = crate::Reg<se_trng_0_ctrl_1::SeTrng0Ctrl1Spec>;
#[doc = "TRNG Reseed Parameter LSB"]
pub mod se_trng_0_ctrl_1;
#[doc = "SE_TRNG_0_CTRL_2 (rw) register accessor: TRNG Reseed Parameter MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_ctrl_2`] module"]
#[doc(alias = "SE_TRNG_0_CTRL_2")]
pub type SeTrng0Ctrl2 = crate::Reg<se_trng_0_ctrl_2::SeTrng0Ctrl2Spec>;
#[doc = "TRNG Reseed Parameter MSB"]
pub mod se_trng_0_ctrl_2;
#[doc = "SE_TRNG_0_CTRL_3 (rw) register accessor: TRNG HT and ROSC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_ctrl_3`] module"]
#[doc(alias = "SE_TRNG_0_CTRL_3")]
pub type SeTrng0Ctrl3 = crate::Reg<se_trng_0_ctrl_3::SeTrng0Ctrl3Spec>;
#[doc = "TRNG HT and ROSC Control"]
pub mod se_trng_0_ctrl_3;
#[doc = "SE_TRNG_0_TEST_OUT_ (r) register accessor: TRNG Test Output %s\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_test_out_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_test_out_`] module"]
#[doc(alias = "SE_TRNG_0_TEST_OUT_")]
pub type SeTrng0TestOut_ = crate::Reg<se_trng_0_test_out_::SeTrng0TestOut_Spec>;
#[doc = "TRNG Test Output %s"]
pub mod se_trng_0_test_out_;
#[doc = "SE_TRNG_0_CTRL_PROT (rw) register accessor: TRNG Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_trng_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_trng_0_ctrl_prot`] module"]
#[doc(alias = "SE_TRNG_0_CTRL_PROT")]
pub type SeTrng0CtrlProt = crate::Reg<se_trng_0_ctrl_prot::SeTrng0CtrlProtSpec>;
#[doc = "TRNG Engine Protection"]
pub mod se_trng_0_ctrl_prot;
#[doc = "SE_PKA_0_CTRL_0 (rw) register accessor: PKA Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_ctrl_0`] module"]
#[doc(alias = "SE_PKA_0_CTRL_0")]
pub type SePka0Ctrl0 = crate::Reg<se_pka_0_ctrl_0::SePka0Ctrl0Spec>;
#[doc = "PKA Control Register 0"]
pub mod se_pka_0_ctrl_0;
#[doc = "SE_PKA_0_SEED (rw) register accessor: PKA Random Seed\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_seed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_seed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_seed`] module"]
#[doc(alias = "SE_PKA_0_SEED")]
pub type SePka0Seed = crate::Reg<se_pka_0_seed::SePka0SeedSpec>;
#[doc = "PKA Random Seed"]
pub mod se_pka_0_seed;
#[doc = "SE_PKA_0_CTRL_1 (rw) register accessor: PKA Burst Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_ctrl_1`] module"]
#[doc(alias = "SE_PKA_0_CTRL_1")]
pub type SePka0Ctrl1 = crate::Reg<se_pka_0_ctrl_1::SePka0Ctrl1Spec>;
#[doc = "PKA Burst Configuration"]
pub mod se_pka_0_ctrl_1;
#[doc = "SE_PKA_0_RW (rw) register accessor: PKA Register Read/Write\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_rw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_rw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_rw`] module"]
#[doc(alias = "SE_PKA_0_RW")]
pub type SePka0Rw = crate::Reg<se_pka_0_rw::SePka0RwSpec>;
#[doc = "PKA Register Read/Write"]
pub mod se_pka_0_rw;
#[doc = "SE_PKA_0_RW_BURST (rw) register accessor: PKA Register Burst Read/Write\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_rw_burst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_rw_burst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_rw_burst`] module"]
#[doc(alias = "SE_PKA_0_RW_BURST")]
pub type SePka0RwBurst = crate::Reg<se_pka_0_rw_burst::SePka0RwBurstSpec>;
#[doc = "PKA Register Burst Read/Write"]
pub mod se_pka_0_rw_burst;
#[doc = "SE_PKA_0_CTRL_PROT (rw) register accessor: PKA Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_pka_0_ctrl_prot`] module"]
#[doc(alias = "SE_PKA_0_CTRL_PROT")]
pub type SePka0CtrlProt = crate::Reg<se_pka_0_ctrl_prot::SePka0CtrlProtSpec>;
#[doc = "PKA Engine Protection"]
pub mod se_pka_0_ctrl_prot;
#[doc = "SE_CDET_0_CTRL_0 (rw) register accessor: Clock Detection Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`se_cdet_0_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_cdet_0_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_cdet_0_ctrl_0`] module"]
#[doc(alias = "SE_CDET_0_CTRL_0")]
pub type SeCdet0Ctrl0 = crate::Reg<se_cdet_0_ctrl_0::SeCdet0Ctrl0Spec>;
#[doc = "Clock Detection Control 0"]
pub mod se_cdet_0_ctrl_0;
#[doc = "SE_CDET_0_CTRL_1 (rw) register accessor: Clock Detection Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`se_cdet_0_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_cdet_0_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_cdet_0_ctrl_1`] module"]
#[doc(alias = "SE_CDET_0_CTRL_1")]
pub type SeCdet0Ctrl1 = crate::Reg<se_cdet_0_ctrl_1::SeCdet0Ctrl1Spec>;
#[doc = "Clock Detection Control 1"]
pub mod se_cdet_0_ctrl_1;
#[doc = "SE_CDET_0_CTRL_PROT (rw) register accessor: CDET Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_cdet_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_cdet_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_cdet_0_ctrl_prot`] module"]
#[doc(alias = "SE_CDET_0_CTRL_PROT")]
pub type SeCdet0CtrlProt = crate::Reg<se_cdet_0_ctrl_prot::SeCdet0CtrlProtSpec>;
#[doc = "CDET Engine Protection"]
pub mod se_cdet_0_ctrl_prot;
#[doc = "SE_GMAC_0_CTRL_0 (rw) register accessor: GMAC Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_gmac_0_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_gmac_0_ctrl_0`] module"]
#[doc(alias = "SE_GMAC_0_CTRL_0")]
pub type SeGmac0Ctrl0 = crate::Reg<se_gmac_0_ctrl_0::SeGmac0Ctrl0Spec>;
#[doc = "GMAC Control Register 0"]
pub mod se_gmac_0_ctrl_0;
#[doc = "SE_GMAC_0_LCA (rw) register accessor: GMAC Link List Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_lca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_gmac_0_lca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_gmac_0_lca`] module"]
#[doc(alias = "SE_GMAC_0_LCA")]
pub type SeGmac0Lca = crate::Reg<se_gmac_0_lca::SeGmac0LcaSpec>;
#[doc = "GMAC Link List Address"]
pub mod se_gmac_0_lca;
#[doc = "SE_GMAC_0_STATUS (r) register accessor: GMAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_gmac_0_status`] module"]
#[doc(alias = "SE_GMAC_0_STATUS")]
pub type SeGmac0Status = crate::Reg<se_gmac_0_status::SeGmac0StatusSpec>;
#[doc = "GMAC Status Register"]
pub mod se_gmac_0_status;
#[doc = "SE_GMAC_0_CTRL_PROT (rw) register accessor: GMAC Engine Protection\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_ctrl_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_gmac_0_ctrl_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_gmac_0_ctrl_prot`] module"]
#[doc(alias = "SE_GMAC_0_CTRL_PROT")]
pub type SeGmac0CtrlProt = crate::Reg<se_gmac_0_ctrl_prot::SeGmac0CtrlProtSpec>;
#[doc = "GMAC Engine Protection"]
pub mod se_gmac_0_ctrl_prot;
#[doc = "SE_CTRL_PROT_RD (r) register accessor: Global Sub-engine Access Right Status\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_prot_rd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_ctrl_prot_rd`] module"]
#[doc(alias = "SE_CTRL_PROT_RD")]
pub type SeCtrlProtRd = crate::Reg<se_ctrl_prot_rd::SeCtrlProtRdSpec>;
#[doc = "Global Sub-engine Access Right Status"]
pub mod se_ctrl_prot_rd;
#[doc = "SE_CTRL_RESERVED_0 (rw) register accessor: Security engine internal bus configuration. Reserved for hardware-specific arbitration and timing tuning.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_reserved_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_ctrl_reserved_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_ctrl_reserved_0`] module"]
#[doc(alias = "SE_CTRL_RESERVED_0")]
pub type SeCtrlReserved0 = crate::Reg<se_ctrl_reserved_0::SeCtrlReserved0Spec>;
#[doc = "Security engine internal bus configuration. Reserved for hardware-specific arbitration and timing tuning."]
pub mod se_ctrl_reserved_0;
#[doc = "SE_CTRL_RESERVED_1 (rw) register accessor: Security engine internal status and control. Reserved for factory-specific hardware configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_reserved_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_ctrl_reserved_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_ctrl_reserved_1`] module"]
#[doc(alias = "SE_CTRL_RESERVED_1")]
pub type SeCtrlReserved1 = crate::Reg<se_ctrl_reserved_1::SeCtrlReserved1Spec>;
#[doc = "Security engine internal status and control. Reserved for factory-specific hardware configuration."]
pub mod se_ctrl_reserved_1;
#[doc = "SE_CTRL_RESERVED_2 (r) register accessor: Security engine hardware revision identification. Returns the silicon-specific version code for the encryption accelerators.\n\nYou can [`read`](crate::Reg::read) this register and get [`se_ctrl_reserved_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@se_ctrl_reserved_2`] module"]
#[doc(alias = "SE_CTRL_RESERVED_2")]
pub type SeCtrlReserved2 = crate::Reg<se_ctrl_reserved_2::SeCtrlReserved2Spec>;
#[doc = "Security engine hardware revision identification. Returns the silicon-specific version code for the encryption accelerators."]
pub mod se_ctrl_reserved_2;
