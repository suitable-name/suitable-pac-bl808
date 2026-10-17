#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sf_ctrl_0: SfCtrl0,
    sf_ctrl_1: SfCtrl1,
    sf_if_sahb_0: SfIfSahb0,
    sf_if_sahb_1: SfIfSahb1,
    sf_if_sahb_2: SfIfSahb2,
    sf_if_iahb_0: SfIfIahb0,
    sf_if_iahb_1: SfIfIahb1,
    sf_if_iahb_2: SfIfIahb2,
    sf_if_status_0: SfIfStatus0,
    sf_if_status_1: SfIfStatus1,
    sf_aes: SfAes,
    sf_ahb2sif_status: SfAhb2sifStatus,
    sf_if_io_dly_0: SfIfIoDly0,
    sf_if_io_dly_1: SfIfIoDly1,
    sf_if_io_dly_2: SfIfIoDly2,
    sf_if_io_dly_3: SfIfIoDly3,
    sf_if_io_dly_4: SfIfIoDly4,
    sf_reserved: SfReserved,
    sf2_if_io_dly_0: Sf2IfIoDly0,
    sf2_if_io_dly_1: Sf2IfIoDly1,
    sf2_if_io_dly_2: Sf2IfIoDly2,
    sf2_if_io_dly_3: Sf2IfIoDly3,
    sf2_if_io_dly_4: Sf2IfIoDly4,
    sf3_if_io_dly_0: Sf3IfIoDly0,
    sf3_if_io_dly_1: Sf3IfIoDly1,
    sf3_if_io_dly_2: Sf3IfIoDly2,
    sf3_if_io_dly_3: Sf3IfIoDly3,
    sf3_if_io_dly_4: Sf3IfIoDly4,
    sf_ctrl_2: SfCtrl2,
    sf_ctrl_3: SfCtrl3,
    sf_if_iahb_3: SfIfIahb3,
    sf_if_iahb_4: SfIfIahb4,
    sf_if_iahb_5: SfIfIahb5,
    sf_if_iahb_6: SfIfIahb6,
    sf_if_iahb_7: SfIfIahb7,
    sf_if_iahb_8: SfIfIahb8,
    sf_if_iahb_9: SfIfIahb9,
    sf_if_iahb_10: SfIfIahb10,
    sf_if_iahb_11: SfIfIahb11,
    sf_if_iahb_12: SfIfIahb12,
    sf_id0_offset: SfId0Offset,
    sf_id1_offset: SfId1Offset,
    sf_bk2_id0_offset: SfBk2Id0Offset,
    sf_bk2_id1_offset: SfBk2Id1Offset,
    sf_dbg: SfDbg,
    _reserved45: [u8; 0x0c],
    sf_if2_ctrl_0: SfIf2Ctrl0,
    sf_if2_ctrl_1: SfIf2Ctrl1,
    sf_if2_sahb_0: SfIf2Sahb0,
    sf_if2_sahb_1: SfIf2Sahb1,
    sf_if2_sahb_2: SfIf2Sahb2,
    _reserved50: [u8; 0x2c],
    sf_ctrl_prot_en_rd: SfCtrlProtEnRd,
    sf_ctrl_prot_en: SfCtrlProtEn,
    _reserved52: [u8; 0xf8],
    aes_region: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Serial Flash Control Register 0"]
    #[inline(always)]
    pub const fn sf_ctrl_0(&self) -> &SfCtrl0 {
        &self.sf_ctrl_0
    }
    #[doc = "0x04 - Serial Flash Control Register 1"]
    #[inline(always)]
    pub const fn sf_ctrl_1(&self) -> &SfCtrl1 {
        &self.sf_ctrl_1
    }
    #[doc = "0x08 - System AHB Interface Register 0"]
    #[inline(always)]
    pub const fn sf_if_sahb_0(&self) -> &SfIfSahb0 {
        &self.sf_if_sahb_0
    }
    #[doc = "0x0c - System AHB Command Buffer 0"]
    #[inline(always)]
    pub const fn sf_if_sahb_1(&self) -> &SfIfSahb1 {
        &self.sf_if_sahb_1
    }
    #[doc = "0x10 - System AHB Command Buffer 1"]
    #[inline(always)]
    pub const fn sf_if_sahb_2(&self) -> &SfIfSahb2 {
        &self.sf_if_sahb_2
    }
    #[doc = "0x14 - Instruction AHB Control 1"]
    #[inline(always)]
    pub const fn sf_if_iahb_0(&self) -> &SfIfIahb0 {
        &self.sf_if_iahb_0
    }
    #[doc = "0x18 - Instruction AHB Command Buffer 1-0"]
    #[inline(always)]
    pub const fn sf_if_iahb_1(&self) -> &SfIfIahb1 {
        &self.sf_if_iahb_1
    }
    #[doc = "0x1c - Instruction AHB Command Buffer 1-1"]
    #[inline(always)]
    pub const fn sf_if_iahb_2(&self) -> &SfIfIahb2 {
        &self.sf_if_iahb_2
    }
    #[doc = "0x20 - Interface Status 0"]
    #[inline(always)]
    pub const fn sf_if_status_0(&self) -> &SfIfStatus0 {
        &self.sf_if_status_0
    }
    #[doc = "0x24 - Interface Status 1"]
    #[inline(always)]
    pub const fn sf_if_status_1(&self) -> &SfIfStatus1 {
        &self.sf_if_status_1
    }
    #[doc = "0x28 - AES Global Control and Status"]
    #[inline(always)]
    pub const fn sf_aes(&self) -> &SfAes {
        &self.sf_aes
    }
    #[doc = "0x2c - AHB to Serial Interface Status"]
    #[inline(always)]
    pub const fn sf_ahb2sif_status(&self) -> &SfAhb2sifStatus {
        &self.sf_ahb2sif_status
    }
    #[doc = "0x30 - SF IO Delay Configuration 0"]
    #[inline(always)]
    pub const fn sf_if_io_dly_0(&self) -> &SfIfIoDly0 {
        &self.sf_if_io_dly_0
    }
    #[doc = "0x34 - SF IO 0 Delay Tuning"]
    #[inline(always)]
    pub const fn sf_if_io_dly_1(&self) -> &SfIfIoDly1 {
        &self.sf_if_io_dly_1
    }
    #[doc = "0x38 - SF IO 1 Delay Tuning"]
    #[inline(always)]
    pub const fn sf_if_io_dly_2(&self) -> &SfIfIoDly2 {
        &self.sf_if_io_dly_2
    }
    #[doc = "0x3c - SF IO 2 Delay Tuning"]
    #[inline(always)]
    pub const fn sf_if_io_dly_3(&self) -> &SfIfIoDly3 {
        &self.sf_if_io_dly_3
    }
    #[doc = "0x40 - SF IO 3 Delay Tuning"]
    #[inline(always)]
    pub const fn sf_if_io_dly_4(&self) -> &SfIfIoDly4 {
        &self.sf_if_io_dly_4
    }
    #[doc = "0x44 - SF Reserved Register"]
    #[inline(always)]
    pub const fn sf_reserved(&self) -> &SfReserved {
        &self.sf_reserved
    }
    #[doc = "0x48 - SF2 IO Delay Configuration 0"]
    #[inline(always)]
    pub const fn sf2_if_io_dly_0(&self) -> &Sf2IfIoDly0 {
        &self.sf2_if_io_dly_0
    }
    #[doc = "0x4c - SF2 IO 0 Delay Tuning"]
    #[inline(always)]
    pub const fn sf2_if_io_dly_1(&self) -> &Sf2IfIoDly1 {
        &self.sf2_if_io_dly_1
    }
    #[doc = "0x50 - SF2 IO 1 Delay Tuning"]
    #[inline(always)]
    pub const fn sf2_if_io_dly_2(&self) -> &Sf2IfIoDly2 {
        &self.sf2_if_io_dly_2
    }
    #[doc = "0x54 - SF2 IO 2 Delay Tuning"]
    #[inline(always)]
    pub const fn sf2_if_io_dly_3(&self) -> &Sf2IfIoDly3 {
        &self.sf2_if_io_dly_3
    }
    #[doc = "0x58 - SF2 IO 3 Delay Tuning"]
    #[inline(always)]
    pub const fn sf2_if_io_dly_4(&self) -> &Sf2IfIoDly4 {
        &self.sf2_if_io_dly_4
    }
    #[doc = "0x5c - SF3 IO Delay Configuration 0"]
    #[inline(always)]
    pub const fn sf3_if_io_dly_0(&self) -> &Sf3IfIoDly0 {
        &self.sf3_if_io_dly_0
    }
    #[doc = "0x60 - SF3 IO 0 Delay Tuning"]
    #[inline(always)]
    pub const fn sf3_if_io_dly_1(&self) -> &Sf3IfIoDly1 {
        &self.sf3_if_io_dly_1
    }
    #[doc = "0x64 - SF3 IO 1 Delay Tuning"]
    #[inline(always)]
    pub const fn sf3_if_io_dly_2(&self) -> &Sf3IfIoDly2 {
        &self.sf3_if_io_dly_2
    }
    #[doc = "0x68 - SF3 IO 2 Delay Tuning"]
    #[inline(always)]
    pub const fn sf3_if_io_dly_3(&self) -> &Sf3IfIoDly3 {
        &self.sf3_if_io_dly_3
    }
    #[doc = "0x6c - SF3 IO 3 Delay Tuning"]
    #[inline(always)]
    pub const fn sf3_if_io_dly_4(&self) -> &Sf3IfIoDly4 {
        &self.sf3_if_io_dly_4
    }
    #[doc = "0x70 - Serial Flash Control Register 2"]
    #[inline(always)]
    pub const fn sf_ctrl_2(&self) -> &SfCtrl2 {
        &self.sf_ctrl_2
    }
    #[doc = "0x74 - Serial Flash Control Register 3"]
    #[inline(always)]
    pub const fn sf_ctrl_3(&self) -> &SfCtrl3 {
        &self.sf_ctrl_3
    }
    #[doc = "0x78 - Instruction AHB Control 2"]
    #[inline(always)]
    pub const fn sf_if_iahb_3(&self) -> &SfIfIahb3 {
        &self.sf_if_iahb_3
    }
    #[doc = "0x7c - Instruction AHB Command Buffer 2-0"]
    #[inline(always)]
    pub const fn sf_if_iahb_4(&self) -> &SfIfIahb4 {
        &self.sf_if_iahb_4
    }
    #[doc = "0x80 - Instruction AHB Command Buffer 2-1"]
    #[inline(always)]
    pub const fn sf_if_iahb_5(&self) -> &SfIfIahb5 {
        &self.sf_if_iahb_5
    }
    #[doc = "0x84 - Instruction AHB Control 3"]
    #[inline(always)]
    pub const fn sf_if_iahb_6(&self) -> &SfIfIahb6 {
        &self.sf_if_iahb_6
    }
    #[doc = "0x88 - Instruction AHB Command Buffer 3-0"]
    #[inline(always)]
    pub const fn sf_if_iahb_7(&self) -> &SfIfIahb7 {
        &self.sf_if_iahb_7
    }
    #[doc = "0x8c - Instruction AHB Command Buffer 3-1"]
    #[inline(always)]
    pub const fn sf_if_iahb_8(&self) -> &SfIfIahb8 {
        &self.sf_if_iahb_8
    }
    #[doc = "0x90 - Instruction AHB Control 4"]
    #[inline(always)]
    pub const fn sf_if_iahb_9(&self) -> &SfIfIahb9 {
        &self.sf_if_iahb_9
    }
    #[doc = "0x94 - Instruction AHB Command Buffer 4-0"]
    #[inline(always)]
    pub const fn sf_if_iahb_10(&self) -> &SfIfIahb10 {
        &self.sf_if_iahb_10
    }
    #[doc = "0x98 - Instruction AHB Command Buffer 4-1"]
    #[inline(always)]
    pub const fn sf_if_iahb_11(&self) -> &SfIfIahb11 {
        &self.sf_if_iahb_11
    }
    #[doc = "0x9c - SF2/SF3 Clock and Read Delay Control"]
    #[inline(always)]
    pub const fn sf_if_iahb_12(&self) -> &SfIfIahb12 {
        &self.sf_if_iahb_12
    }
    #[doc = "0xa0 - Remap Offset for Region 0"]
    #[inline(always)]
    pub const fn sf_id0_offset(&self) -> &SfId0Offset {
        &self.sf_id0_offset
    }
    #[doc = "0xa4 - Remap Offset for Region 1"]
    #[inline(always)]
    pub const fn sf_id1_offset(&self) -> &SfId1Offset {
        &self.sf_id1_offset
    }
    #[doc = "0xa8 - Remap Offset for Bank 2 Region 0"]
    #[inline(always)]
    pub const fn sf_bk2_id0_offset(&self) -> &SfBk2Id0Offset {
        &self.sf_bk2_id0_offset
    }
    #[doc = "0xac - Remap Offset for Bank 2 Region 1"]
    #[inline(always)]
    pub const fn sf_bk2_id1_offset(&self) -> &SfBk2Id1Offset {
        &self.sf_bk2_id1_offset
    }
    #[doc = "0xb0 - Autoload Status and Debug"]
    #[inline(always)]
    pub const fn sf_dbg(&self) -> &SfDbg {
        &self.sf_dbg
    }
    #[doc = "0xc0 - Serial Flash 2 Control Register 0"]
    #[inline(always)]
    pub const fn sf_if2_ctrl_0(&self) -> &SfIf2Ctrl0 {
        &self.sf_if2_ctrl_0
    }
    #[doc = "0xc4 - Serial Flash 2 Control Register 1"]
    #[inline(always)]
    pub const fn sf_if2_ctrl_1(&self) -> &SfIf2Ctrl1 {
        &self.sf_if2_ctrl_1
    }
    #[doc = "0xc8 - SF2 System AHB Interface Register 0"]
    #[inline(always)]
    pub const fn sf_if2_sahb_0(&self) -> &SfIf2Sahb0 {
        &self.sf_if2_sahb_0
    }
    #[doc = "0xcc - SF2 System AHB Command Buffer 0"]
    #[inline(always)]
    pub const fn sf_if2_sahb_1(&self) -> &SfIf2Sahb1 {
        &self.sf_if2_sahb_1
    }
    #[doc = "0xd0 - SF2 System AHB Command Buffer 1"]
    #[inline(always)]
    pub const fn sf_if2_sahb_2(&self) -> &SfIf2Sahb2 {
        &self.sf_if2_sahb_2
    }
    #[doc = "0x100 - TrustZone Protection Read Status"]
    #[inline(always)]
    pub const fn sf_ctrl_prot_en_rd(&self) -> &SfCtrlProtEnRd {
        &self.sf_ctrl_prot_en_rd
    }
    #[doc = "0x104 - TrustZone Protection Configuration"]
    #[inline(always)]
    pub const fn sf_ctrl_prot_en(&self) -> &SfCtrlProtEn {
        &self.sf_ctrl_prot_en
    }
    #[doc = "0x200..0x2a8 - AES Encryption Regions for Flash"]
    #[inline(always)]
    pub const fn aes_region(&self, n: usize) -> &AesRegion {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x2a8 - AES Encryption Regions for Flash"]
    #[inline(always)]
    pub fn aes_region_iter(&self) -> impl Iterator<Item = &AesRegion> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(512)
                .add(128 * n)
                .cast()
        })
    }
}
#[doc = "SF_CTRL_0 (rw) register accessor: Serial Flash Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_0`] module"]
#[doc(alias = "SF_CTRL_0")]
pub type SfCtrl0 = crate::Reg<sf_ctrl_0::SfCtrl0Spec>;
#[doc = "Serial Flash Control Register 0"]
pub mod sf_ctrl_0;
#[doc = "SF_CTRL_1 (rw) register accessor: Serial Flash Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_1`] module"]
#[doc(alias = "SF_CTRL_1")]
pub type SfCtrl1 = crate::Reg<sf_ctrl_1::SfCtrl1Spec>;
#[doc = "Serial Flash Control Register 1"]
pub mod sf_ctrl_1;
#[doc = "SF_IF_SAHB_0 (rw) register accessor: System AHB Interface Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_sahb_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_sahb_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_sahb_0`] module"]
#[doc(alias = "SF_IF_SAHB_0")]
pub type SfIfSahb0 = crate::Reg<sf_if_sahb_0::SfIfSahb0Spec>;
#[doc = "System AHB Interface Register 0"]
pub mod sf_if_sahb_0;
#[doc = "SF_IF_SAHB_1 (rw) register accessor: System AHB Command Buffer 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_sahb_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_sahb_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_sahb_1`] module"]
#[doc(alias = "SF_IF_SAHB_1")]
pub type SfIfSahb1 = crate::Reg<sf_if_sahb_1::SfIfSahb1Spec>;
#[doc = "System AHB Command Buffer 0"]
pub mod sf_if_sahb_1;
#[doc = "SF_IF_SAHB_2 (rw) register accessor: System AHB Command Buffer 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_sahb_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_sahb_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_sahb_2`] module"]
#[doc(alias = "SF_IF_SAHB_2")]
pub type SfIfSahb2 = crate::Reg<sf_if_sahb_2::SfIfSahb2Spec>;
#[doc = "System AHB Command Buffer 1"]
pub mod sf_if_sahb_2;
#[doc = "SF_IF_IAHB_0 (rw) register accessor: Instruction AHB Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_0`] module"]
#[doc(alias = "SF_IF_IAHB_0")]
pub type SfIfIahb0 = crate::Reg<sf_if_iahb_0::SfIfIahb0Spec>;
#[doc = "Instruction AHB Control 1"]
pub mod sf_if_iahb_0;
#[doc = "SF_IF_IAHB_1 (rw) register accessor: Instruction AHB Command Buffer 1-0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_1`] module"]
#[doc(alias = "SF_IF_IAHB_1")]
pub type SfIfIahb1 = crate::Reg<sf_if_iahb_1::SfIfIahb1Spec>;
#[doc = "Instruction AHB Command Buffer 1-0"]
pub mod sf_if_iahb_1;
#[doc = "SF_IF_IAHB_2 (rw) register accessor: Instruction AHB Command Buffer 1-1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_2`] module"]
#[doc(alias = "SF_IF_IAHB_2")]
pub type SfIfIahb2 = crate::Reg<sf_if_iahb_2::SfIfIahb2Spec>;
#[doc = "Instruction AHB Command Buffer 1-1"]
pub mod sf_if_iahb_2;
#[doc = "SF_IF_STATUS_0 (r) register accessor: Interface Status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_status_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_status_0`] module"]
#[doc(alias = "SF_IF_STATUS_0")]
pub type SfIfStatus0 = crate::Reg<sf_if_status_0::SfIfStatus0Spec>;
#[doc = "Interface Status 0"]
pub mod sf_if_status_0;
#[doc = "SF_IF_STATUS_1 (r) register accessor: Interface Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_status_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_status_1`] module"]
#[doc(alias = "SF_IF_STATUS_1")]
pub type SfIfStatus1 = crate::Reg<sf_if_status_1::SfIfStatus1Spec>;
#[doc = "Interface Status 1"]
pub mod sf_if_status_1;
#[doc = "SF_AES (rw) register accessor: AES Global Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_aes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_aes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_aes`] module"]
#[doc(alias = "SF_AES")]
pub type SfAes = crate::Reg<sf_aes::SfAesSpec>;
#[doc = "AES Global Control and Status"]
pub mod sf_aes;
#[doc = "SF_AHB2SIF_STATUS (r) register accessor: AHB to Serial Interface Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ahb2sif_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ahb2sif_status`] module"]
#[doc(alias = "SF_AHB2SIF_STATUS")]
pub type SfAhb2sifStatus = crate::Reg<sf_ahb2sif_status::SfAhb2sifStatusSpec>;
#[doc = "AHB to Serial Interface Status"]
pub mod sf_ahb2sif_status;
#[doc = "SF_IF_IO_DLY_0 (rw) register accessor: SF IO Delay Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_io_dly_0`] module"]
#[doc(alias = "SF_IF_IO_DLY_0")]
pub type SfIfIoDly0 = crate::Reg<sf_if_io_dly_0::SfIfIoDly0Spec>;
#[doc = "SF IO Delay Configuration 0"]
pub mod sf_if_io_dly_0;
#[doc = "SF_IF_IO_DLY_1 (rw) register accessor: SF IO 0 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_io_dly_1`] module"]
#[doc(alias = "SF_IF_IO_DLY_1")]
pub type SfIfIoDly1 = crate::Reg<sf_if_io_dly_1::SfIfIoDly1Spec>;
#[doc = "SF IO 0 Delay Tuning"]
pub mod sf_if_io_dly_1;
#[doc = "SF_IF_IO_DLY_2 (rw) register accessor: SF IO 1 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_io_dly_2`] module"]
#[doc(alias = "SF_IF_IO_DLY_2")]
pub type SfIfIoDly2 = crate::Reg<sf_if_io_dly_2::SfIfIoDly2Spec>;
#[doc = "SF IO 1 Delay Tuning"]
pub mod sf_if_io_dly_2;
#[doc = "SF_IF_IO_DLY_3 (rw) register accessor: SF IO 2 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_io_dly_3`] module"]
#[doc(alias = "SF_IF_IO_DLY_3")]
pub type SfIfIoDly3 = crate::Reg<sf_if_io_dly_3::SfIfIoDly3Spec>;
#[doc = "SF IO 2 Delay Tuning"]
pub mod sf_if_io_dly_3;
#[doc = "SF_IF_IO_DLY_4 (rw) register accessor: SF IO 3 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_io_dly_4`] module"]
#[doc(alias = "SF_IF_IO_DLY_4")]
pub type SfIfIoDly4 = crate::Reg<sf_if_io_dly_4::SfIfIoDly4Spec>;
#[doc = "SF IO 3 Delay Tuning"]
pub mod sf_if_io_dly_4;
#[doc = "SF_RESERVED (rw) register accessor: SF Reserved Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_reserved`] module"]
#[doc(alias = "SF_RESERVED")]
pub type SfReserved = crate::Reg<sf_reserved::SfReservedSpec>;
#[doc = "SF Reserved Register"]
pub mod sf_reserved;
#[doc = "SF2_IF_IO_DLY_0 (rw) register accessor: SF2 IO Delay Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf2_if_io_dly_0`] module"]
#[doc(alias = "SF2_IF_IO_DLY_0")]
pub type Sf2IfIoDly0 = crate::Reg<sf2_if_io_dly_0::Sf2IfIoDly0Spec>;
#[doc = "SF2 IO Delay Configuration 0"]
pub mod sf2_if_io_dly_0;
#[doc = "SF2_IF_IO_DLY_1 (rw) register accessor: SF2 IO 0 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf2_if_io_dly_1`] module"]
#[doc(alias = "SF2_IF_IO_DLY_1")]
pub type Sf2IfIoDly1 = crate::Reg<sf2_if_io_dly_1::Sf2IfIoDly1Spec>;
#[doc = "SF2 IO 0 Delay Tuning"]
pub mod sf2_if_io_dly_1;
#[doc = "SF2_IF_IO_DLY_2 (rw) register accessor: SF2 IO 1 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf2_if_io_dly_2`] module"]
#[doc(alias = "SF2_IF_IO_DLY_2")]
pub type Sf2IfIoDly2 = crate::Reg<sf2_if_io_dly_2::Sf2IfIoDly2Spec>;
#[doc = "SF2 IO 1 Delay Tuning"]
pub mod sf2_if_io_dly_2;
#[doc = "SF2_IF_IO_DLY_3 (rw) register accessor: SF2 IO 2 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf2_if_io_dly_3`] module"]
#[doc(alias = "SF2_IF_IO_DLY_3")]
pub type Sf2IfIoDly3 = crate::Reg<sf2_if_io_dly_3::Sf2IfIoDly3Spec>;
#[doc = "SF2 IO 2 Delay Tuning"]
pub mod sf2_if_io_dly_3;
#[doc = "SF2_IF_IO_DLY_4 (rw) register accessor: SF2 IO 3 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf2_if_io_dly_4`] module"]
#[doc(alias = "SF2_IF_IO_DLY_4")]
pub type Sf2IfIoDly4 = crate::Reg<sf2_if_io_dly_4::Sf2IfIoDly4Spec>;
#[doc = "SF2 IO 3 Delay Tuning"]
pub mod sf2_if_io_dly_4;
#[doc = "SF3_IF_IO_DLY_0 (rw) register accessor: SF3 IO Delay Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf3_if_io_dly_0`] module"]
#[doc(alias = "SF3_IF_IO_DLY_0")]
pub type Sf3IfIoDly0 = crate::Reg<sf3_if_io_dly_0::Sf3IfIoDly0Spec>;
#[doc = "SF3 IO Delay Configuration 0"]
pub mod sf3_if_io_dly_0;
#[doc = "SF3_IF_IO_DLY_1 (rw) register accessor: SF3 IO 0 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf3_if_io_dly_1`] module"]
#[doc(alias = "SF3_IF_IO_DLY_1")]
pub type Sf3IfIoDly1 = crate::Reg<sf3_if_io_dly_1::Sf3IfIoDly1Spec>;
#[doc = "SF3 IO 0 Delay Tuning"]
pub mod sf3_if_io_dly_1;
#[doc = "SF3_IF_IO_DLY_2 (rw) register accessor: SF3 IO 1 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf3_if_io_dly_2`] module"]
#[doc(alias = "SF3_IF_IO_DLY_2")]
pub type Sf3IfIoDly2 = crate::Reg<sf3_if_io_dly_2::Sf3IfIoDly2Spec>;
#[doc = "SF3 IO 1 Delay Tuning"]
pub mod sf3_if_io_dly_2;
#[doc = "SF3_IF_IO_DLY_3 (rw) register accessor: SF3 IO 2 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf3_if_io_dly_3`] module"]
#[doc(alias = "SF3_IF_IO_DLY_3")]
pub type Sf3IfIoDly3 = crate::Reg<sf3_if_io_dly_3::Sf3IfIoDly3Spec>;
#[doc = "SF3 IO 2 Delay Tuning"]
pub mod sf3_if_io_dly_3;
#[doc = "SF3_IF_IO_DLY_4 (rw) register accessor: SF3 IO 3 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf3_if_io_dly_4`] module"]
#[doc(alias = "SF3_IF_IO_DLY_4")]
pub type Sf3IfIoDly4 = crate::Reg<sf3_if_io_dly_4::Sf3IfIoDly4Spec>;
#[doc = "SF3 IO 3 Delay Tuning"]
pub mod sf3_if_io_dly_4;
#[doc = "SF_CTRL_2 (rw) register accessor: Serial Flash Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_2`] module"]
#[doc(alias = "SF_CTRL_2")]
pub type SfCtrl2 = crate::Reg<sf_ctrl_2::SfCtrl2Spec>;
#[doc = "Serial Flash Control Register 2"]
pub mod sf_ctrl_2;
#[doc = "SF_CTRL_3 (rw) register accessor: Serial Flash Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_3`] module"]
#[doc(alias = "SF_CTRL_3")]
pub type SfCtrl3 = crate::Reg<sf_ctrl_3::SfCtrl3Spec>;
#[doc = "Serial Flash Control Register 3"]
pub mod sf_ctrl_3;
#[doc = "SF_IF_IAHB_3 (rw) register accessor: Instruction AHB Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_3`] module"]
#[doc(alias = "SF_IF_IAHB_3")]
pub type SfIfIahb3 = crate::Reg<sf_if_iahb_3::SfIfIahb3Spec>;
#[doc = "Instruction AHB Control 2"]
pub mod sf_if_iahb_3;
#[doc = "SF_IF_IAHB_4 (rw) register accessor: Instruction AHB Command Buffer 2-0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_4`] module"]
#[doc(alias = "SF_IF_IAHB_4")]
pub type SfIfIahb4 = crate::Reg<sf_if_iahb_4::SfIfIahb4Spec>;
#[doc = "Instruction AHB Command Buffer 2-0"]
pub mod sf_if_iahb_4;
#[doc = "SF_IF_IAHB_5 (rw) register accessor: Instruction AHB Command Buffer 2-1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_5`] module"]
#[doc(alias = "SF_IF_IAHB_5")]
pub type SfIfIahb5 = crate::Reg<sf_if_iahb_5::SfIfIahb5Spec>;
#[doc = "Instruction AHB Command Buffer 2-1"]
pub mod sf_if_iahb_5;
#[doc = "SF_IF_IAHB_6 (rw) register accessor: Instruction AHB Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_6`] module"]
#[doc(alias = "SF_IF_IAHB_6")]
pub type SfIfIahb6 = crate::Reg<sf_if_iahb_6::SfIfIahb6Spec>;
#[doc = "Instruction AHB Control 3"]
pub mod sf_if_iahb_6;
#[doc = "SF_IF_IAHB_7 (rw) register accessor: Instruction AHB Command Buffer 3-0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_7`] module"]
#[doc(alias = "SF_IF_IAHB_7")]
pub type SfIfIahb7 = crate::Reg<sf_if_iahb_7::SfIfIahb7Spec>;
#[doc = "Instruction AHB Command Buffer 3-0"]
pub mod sf_if_iahb_7;
#[doc = "SF_IF_IAHB_8 (rw) register accessor: Instruction AHB Command Buffer 3-1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_8`] module"]
#[doc(alias = "SF_IF_IAHB_8")]
pub type SfIfIahb8 = crate::Reg<sf_if_iahb_8::SfIfIahb8Spec>;
#[doc = "Instruction AHB Command Buffer 3-1"]
pub mod sf_if_iahb_8;
#[doc = "SF_IF_IAHB_9 (rw) register accessor: Instruction AHB Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_9`] module"]
#[doc(alias = "SF_IF_IAHB_9")]
pub type SfIfIahb9 = crate::Reg<sf_if_iahb_9::SfIfIahb9Spec>;
#[doc = "Instruction AHB Control 4"]
pub mod sf_if_iahb_9;
#[doc = "SF_IF_IAHB_10 (rw) register accessor: Instruction AHB Command Buffer 4-0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_10`] module"]
#[doc(alias = "SF_IF_IAHB_10")]
pub type SfIfIahb10 = crate::Reg<sf_if_iahb_10::SfIfIahb10Spec>;
#[doc = "Instruction AHB Command Buffer 4-0"]
pub mod sf_if_iahb_10;
#[doc = "SF_IF_IAHB_11 (rw) register accessor: Instruction AHB Command Buffer 4-1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_11`] module"]
#[doc(alias = "SF_IF_IAHB_11")]
pub type SfIfIahb11 = crate::Reg<sf_if_iahb_11::SfIfIahb11Spec>;
#[doc = "Instruction AHB Command Buffer 4-1"]
pub mod sf_if_iahb_11;
#[doc = "SF_IF_IAHB_12 (rw) register accessor: SF2/SF3 Clock and Read Delay Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if_iahb_12`] module"]
#[doc(alias = "SF_IF_IAHB_12")]
pub type SfIfIahb12 = crate::Reg<sf_if_iahb_12::SfIfIahb12Spec>;
#[doc = "SF2/SF3 Clock and Read Delay Control"]
pub mod sf_if_iahb_12;
#[doc = "SF_ID0_OFFSET (rw) register accessor: Remap Offset for Region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_id0_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_id0_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_id0_offset`] module"]
#[doc(alias = "SF_ID0_OFFSET")]
pub type SfId0Offset = crate::Reg<sf_id0_offset::SfId0OffsetSpec>;
#[doc = "Remap Offset for Region 0"]
pub mod sf_id0_offset;
#[doc = "SF_ID1_OFFSET (rw) register accessor: Remap Offset for Region 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_id1_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_id1_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_id1_offset`] module"]
#[doc(alias = "SF_ID1_OFFSET")]
pub type SfId1Offset = crate::Reg<sf_id1_offset::SfId1OffsetSpec>;
#[doc = "Remap Offset for Region 1"]
pub mod sf_id1_offset;
#[doc = "SF_BK2_ID0_OFFSET (rw) register accessor: Remap Offset for Bank 2 Region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_bk2_id0_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_bk2_id0_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_bk2_id0_offset`] module"]
#[doc(alias = "SF_BK2_ID0_OFFSET")]
pub type SfBk2Id0Offset = crate::Reg<sf_bk2_id0_offset::SfBk2Id0OffsetSpec>;
#[doc = "Remap Offset for Bank 2 Region 0"]
pub mod sf_bk2_id0_offset;
#[doc = "SF_BK2_ID1_OFFSET (rw) register accessor: Remap Offset for Bank 2 Region 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_bk2_id1_offset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_bk2_id1_offset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_bk2_id1_offset`] module"]
#[doc(alias = "SF_BK2_ID1_OFFSET")]
pub type SfBk2Id1Offset = crate::Reg<sf_bk2_id1_offset::SfBk2Id1OffsetSpec>;
#[doc = "Remap Offset for Bank 2 Region 1"]
pub mod sf_bk2_id1_offset;
#[doc = "SF_DBG (r) register accessor: Autoload Status and Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_dbg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_dbg`] module"]
#[doc(alias = "SF_DBG")]
pub type SfDbg = crate::Reg<sf_dbg::SfDbgSpec>;
#[doc = "Autoload Status and Debug"]
pub mod sf_dbg;
#[doc = "SF_IF2_CTRL_0 (rw) register accessor: Serial Flash 2 Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if2_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if2_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if2_ctrl_0`] module"]
#[doc(alias = "SF_IF2_CTRL_0")]
pub type SfIf2Ctrl0 = crate::Reg<sf_if2_ctrl_0::SfIf2Ctrl0Spec>;
#[doc = "Serial Flash 2 Control Register 0"]
pub mod sf_if2_ctrl_0;
#[doc = "SF_IF2_CTRL_1 (rw) register accessor: Serial Flash 2 Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if2_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if2_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if2_ctrl_1`] module"]
#[doc(alias = "SF_IF2_CTRL_1")]
pub type SfIf2Ctrl1 = crate::Reg<sf_if2_ctrl_1::SfIf2Ctrl1Spec>;
#[doc = "Serial Flash 2 Control Register 1"]
pub mod sf_if2_ctrl_1;
#[doc = "SF_IF2_SAHB_0 (rw) register accessor: SF2 System AHB Interface Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if2_sahb_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if2_sahb_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if2_sahb_0`] module"]
#[doc(alias = "SF_IF2_SAHB_0")]
pub type SfIf2Sahb0 = crate::Reg<sf_if2_sahb_0::SfIf2Sahb0Spec>;
#[doc = "SF2 System AHB Interface Register 0"]
pub mod sf_if2_sahb_0;
#[doc = "SF_IF2_SAHB_1 (rw) register accessor: SF2 System AHB Command Buffer 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if2_sahb_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if2_sahb_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if2_sahb_1`] module"]
#[doc(alias = "SF_IF2_SAHB_1")]
pub type SfIf2Sahb1 = crate::Reg<sf_if2_sahb_1::SfIf2Sahb1Spec>;
#[doc = "SF2 System AHB Command Buffer 0"]
pub mod sf_if2_sahb_1;
#[doc = "SF_IF2_SAHB_2 (rw) register accessor: SF2 System AHB Command Buffer 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if2_sahb_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if2_sahb_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_if2_sahb_2`] module"]
#[doc(alias = "SF_IF2_SAHB_2")]
pub type SfIf2Sahb2 = crate::Reg<sf_if2_sahb_2::SfIf2Sahb2Spec>;
#[doc = "SF2 System AHB Command Buffer 1"]
pub mod sf_if2_sahb_2;
#[doc = "SF_CTRL_PROT_EN_RD (r) register accessor: TrustZone Protection Read Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_prot_en_rd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_prot_en_rd`] module"]
#[doc(alias = "SF_CTRL_PROT_EN_RD")]
pub type SfCtrlProtEnRd = crate::Reg<sf_ctrl_prot_en_rd::SfCtrlProtEnRdSpec>;
#[doc = "TrustZone Protection Read Status"]
pub mod sf_ctrl_prot_en_rd;
#[doc = "SF_CTRL_PROT_EN (rw) register accessor: TrustZone Protection Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_prot_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_prot_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sf_ctrl_prot_en`] module"]
#[doc(alias = "SF_CTRL_PROT_EN")]
pub type SfCtrlProtEn = crate::Reg<sf_ctrl_prot_en::SfCtrlProtEnSpec>;
#[doc = "TrustZone Protection Configuration"]
pub mod sf_ctrl_prot_en;
#[doc = "AES Encryption Regions for Flash"]
pub use self::aes_region::AesRegion;
#[doc = r"Cluster"]
#[doc = "AES Encryption Regions for Flash"]
pub mod aes_region;
