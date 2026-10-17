#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ef_cfg_0: EfCfg0,
    ef_dbg_pwd_low: EfDbgPwdLow,
    ef_dbg_pwd_high: EfDbgPwdHigh,
    ef_dbg_pwd2_low: EfDbgPwd2Low,
    ef_dbg_pwd2_high: EfDbgPwd2High,
    ef_wifi_mac_low: EfWifiMacLow,
    ef_wifi_mac_high: EfWifiMacHigh,
    ef_key_slot_0_w0: EfKeySlot0W0,
    ef_key_slot_0_w1: EfKeySlot0W1,
    ef_key_slot_0_w2: EfKeySlot0W2,
    ef_key_slot_0_w3: EfKeySlot0W3,
    ef_key_slot_1_w0: EfKeySlot1W0,
    ef_key_slot_1_w1: EfKeySlot1W1,
    ef_key_slot_1_w2: EfKeySlot1W2,
    ef_key_slot_1_w3: EfKeySlot1W3,
    ef_key_slot_2_w0: EfKeySlot2W0,
    ef_key_slot_2_w1: EfKeySlot2W1,
    ef_key_slot_2_w2: EfKeySlot2W2,
    ef_key_slot_2_w3: EfKeySlot2W3,
    ef_key_slot_3_w0: EfKeySlot3W0,
    ef_key_slot_3_w1: EfKeySlot3W1,
    ef_key_slot_3_w2: EfKeySlot3W2,
    ef_key_slot_3_w3: EfKeySlot3W3,
    ef_sw_usage_0: EfSwUsage0,
    ef_sw_usage_1: EfSwUsage1,
    ef_sw_usage_2: EfSwUsage2,
    ef_sw_usage_3: EfSwUsage3,
    ef_key_slot_11_w0: EfKeySlot11W0,
    ef_key_slot_11_w1: EfKeySlot11W1,
    ef_key_slot_11_w2: EfKeySlot11W2,
    ef_key_slot_11_w3: EfKeySlot11W3,
    ef_data_0_lock: EfData0Lock,
    ef_key_slot_4_w0: EfKeySlot4W0,
    ef_key_slot_4_w1: EfKeySlot4W1,
    ef_key_slot_4_w2: EfKeySlot4W2,
    ef_key_slot_4_w3: EfKeySlot4W3,
    ef_key_slot_5_w0: EfKeySlot5W0,
    ef_key_slot_5_w1: EfKeySlot5W1,
    ef_key_slot_5_w2: EfKeySlot5W2,
    ef_key_slot_5_w3: EfKeySlot5W3,
    ef_key_slot_6_w0: EfKeySlot6W0,
    ef_key_slot_6_w1: EfKeySlot6W1,
    ef_key_slot_6_w2: EfKeySlot6W2,
    ef_key_slot_6_w3: EfKeySlot6W3,
    ef_key_slot_7_w0: EfKeySlot7W0,
    ef_key_slot_7_w1: EfKeySlot7W1,
    ef_key_slot_7_w2: EfKeySlot7W2,
    ef_key_slot_7_w3: EfKeySlot7W3,
    ef_key_slot_8_w0: EfKeySlot8W0,
    ef_key_slot_8_w1: EfKeySlot8W1,
    ef_key_slot_8_w2: EfKeySlot8W2,
    ef_key_slot_8_w3: EfKeySlot8W3,
    ef_key_slot_9_w0: EfKeySlot9W0,
    ef_key_slot_9_w1: EfKeySlot9W1,
    ef_key_slot_9_w2: EfKeySlot9W2,
    ef_key_slot_9_w3: EfKeySlot9W3,
    ef_key_slot_10_w0: EfKeySlot10W0,
    ef_key_slot_10_w1: EfKeySlot10W1,
    ef_key_slot_10_w2: EfKeySlot10W2,
    ef_key_slot_10_w3: EfKeySlot10W3,
    ef_dat_1_rsvd_0: EfDat1Rsvd0,
    ef_dat_1_rsvd_1: EfDat1Rsvd1,
    ef_dat_1_rsvd_2: EfDat1Rsvd2,
    ef_data_1_lock: EfData1Lock,
    _reserved64: [u8; 0x0700],
    ef_ctrl_if_ctrl_0: EfCtrlIfCtrl0,
    ef_ctrl_if_cyc_0: EfCtrlIfCyc0,
    ef_ctrl_if_cyc_1: EfCtrlIfCyc1,
    ef_ctrl_if_0_manual: EfCtrlIf0Manual,
    ef_ctrl_if_0_status: EfCtrlIf0Status,
    ef_ctrl_if_cfg_0: EfCtrlIfCfg0,
    ef_ctrl_sw_cfg_0: EfCtrlSwCfg0,
    ef_ctrl_ef_reserved: EfCtrlEfReserved,
    ef_ctrl_if_sw_usage_0: EfCtrlIfSwUsage0,
    ef_ctrl_if_sw_usage_1: EfCtrlIfSwUsage1,
    _reserved74: [u8; 0xd8],
    ef_ctrl_if_ctrl_1: EfCtrlIfCtrl1,
    ef_ctrl_if_1_manual: EfCtrlIf1Manual,
    ef_ctrl_if_1_status: EfCtrlIf1Status,
    _reserved77: [u8; 0x04],
    ef_ctrl_if_ctrl_2: EfCtrlIfCtrl2,
    ef_ctrl_if_2_manual: EfCtrlIf2Manual,
    ef_ctrl_if_2_status: EfCtrlIf2Status,
    _reserved80: [u8; 0xe4],
    ef_crc_ctrl_0: EfCrcCtrl0,
    ef_crc_ctrl_1: EfCrcCtrl1,
    ef_crc_ctrl_2: EfCrcCtrl2,
    ef_crc_ctrl_3: EfCrcCtrl3,
    ef_crc_ctrl_4: EfCrcCtrl4,
    ef_crc_ctrl_5: EfCrcCtrl5,
}
impl RegisterBlock {
    #[doc = "0x00 - eFuse Configuration Settings (Bitstream Data)"]
    #[inline(always)]
    pub const fn ef_cfg_0(&self) -> &EfCfg0 {
        &self.ef_cfg_0
    }
    #[doc = "0x04 - Debug Password Low Word"]
    #[inline(always)]
    pub const fn ef_dbg_pwd_low(&self) -> &EfDbgPwdLow {
        &self.ef_dbg_pwd_low
    }
    #[doc = "0x08 - Debug Password High Word"]
    #[inline(always)]
    pub const fn ef_dbg_pwd_high(&self) -> &EfDbgPwdHigh {
        &self.ef_dbg_pwd_high
    }
    #[doc = "0x0c - Debug Password 2 Low Word"]
    #[inline(always)]
    pub const fn ef_dbg_pwd2_low(&self) -> &EfDbgPwd2Low {
        &self.ef_dbg_pwd2_low
    }
    #[doc = "0x10 - Debug Password 2 High Word"]
    #[inline(always)]
    pub const fn ef_dbg_pwd2_high(&self) -> &EfDbgPwd2High {
        &self.ef_dbg_pwd2_high
    }
    #[doc = "0x14 - Wi-Fi MAC Address Low Word (Bits 31:0)"]
    #[inline(always)]
    pub const fn ef_wifi_mac_low(&self) -> &EfWifiMacLow {
        &self.ef_wifi_mac_low
    }
    #[doc = "0x18 - Wi-Fi MAC Address High Word (Bits 47:32)"]
    #[inline(always)]
    pub const fn ef_wifi_mac_high(&self) -> &EfWifiMacHigh {
        &self.ef_wifi_mac_high
    }
    #[doc = "0x1c - Key Slot 0 Word 0"]
    #[inline(always)]
    pub const fn ef_key_slot_0_w0(&self) -> &EfKeySlot0W0 {
        &self.ef_key_slot_0_w0
    }
    #[doc = "0x20 - Key Slot 0 Word 1"]
    #[inline(always)]
    pub const fn ef_key_slot_0_w1(&self) -> &EfKeySlot0W1 {
        &self.ef_key_slot_0_w1
    }
    #[doc = "0x24 - Key Slot 0 Word 2"]
    #[inline(always)]
    pub const fn ef_key_slot_0_w2(&self) -> &EfKeySlot0W2 {
        &self.ef_key_slot_0_w2
    }
    #[doc = "0x28 - Key Slot 0 Word 3"]
    #[inline(always)]
    pub const fn ef_key_slot_0_w3(&self) -> &EfKeySlot0W3 {
        &self.ef_key_slot_0_w3
    }
    #[doc = "0x2c - Key Slot 1 Word 0"]
    #[inline(always)]
    pub const fn ef_key_slot_1_w0(&self) -> &EfKeySlot1W0 {
        &self.ef_key_slot_1_w0
    }
    #[doc = "0x30 - Key Slot 1 Word 1"]
    #[inline(always)]
    pub const fn ef_key_slot_1_w1(&self) -> &EfKeySlot1W1 {
        &self.ef_key_slot_1_w1
    }
    #[doc = "0x34 - Key Slot 1 Word 2"]
    #[inline(always)]
    pub const fn ef_key_slot_1_w2(&self) -> &EfKeySlot1W2 {
        &self.ef_key_slot_1_w2
    }
    #[doc = "0x38 - Key Slot 1 Word 3"]
    #[inline(always)]
    pub const fn ef_key_slot_1_w3(&self) -> &EfKeySlot1W3 {
        &self.ef_key_slot_1_w3
    }
    #[doc = "0x3c - Key Slot 2 Word 0"]
    #[inline(always)]
    pub const fn ef_key_slot_2_w0(&self) -> &EfKeySlot2W0 {
        &self.ef_key_slot_2_w0
    }
    #[doc = "0x40 - Key Slot 2 Word 1"]
    #[inline(always)]
    pub const fn ef_key_slot_2_w1(&self) -> &EfKeySlot2W1 {
        &self.ef_key_slot_2_w1
    }
    #[doc = "0x44 - Key Slot 2 Word 2"]
    #[inline(always)]
    pub const fn ef_key_slot_2_w2(&self) -> &EfKeySlot2W2 {
        &self.ef_key_slot_2_w2
    }
    #[doc = "0x48 - Key Slot 2 Word 3"]
    #[inline(always)]
    pub const fn ef_key_slot_2_w3(&self) -> &EfKeySlot2W3 {
        &self.ef_key_slot_2_w3
    }
    #[doc = "0x4c - Key Slot 3 Word 0"]
    #[inline(always)]
    pub const fn ef_key_slot_3_w0(&self) -> &EfKeySlot3W0 {
        &self.ef_key_slot_3_w0
    }
    #[doc = "0x50 - Key Slot 3 Word 1"]
    #[inline(always)]
    pub const fn ef_key_slot_3_w1(&self) -> &EfKeySlot3W1 {
        &self.ef_key_slot_3_w1
    }
    #[doc = "0x54 - Key Slot 3 Word 2"]
    #[inline(always)]
    pub const fn ef_key_slot_3_w2(&self) -> &EfKeySlot3W2 {
        &self.ef_key_slot_3_w2
    }
    #[doc = "0x58 - Key Slot 3 Word 3"]
    #[inline(always)]
    pub const fn ef_key_slot_3_w3(&self) -> &EfKeySlot3W3 {
        &self.ef_key_slot_3_w3
    }
    #[doc = "0x5c - Software Usage Slot 0 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_sw_usage_0(&self) -> &EfSwUsage0 {
        &self.ef_sw_usage_0
    }
    #[doc = "0x60 - Software Usage Slot 1 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_sw_usage_1(&self) -> &EfSwUsage1 {
        &self.ef_sw_usage_1
    }
    #[doc = "0x64 - Software Usage Slot 2 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_sw_usage_2(&self) -> &EfSwUsage2 {
        &self.ef_sw_usage_2
    }
    #[doc = "0x68 - Software Usage Slot 3 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_sw_usage_3(&self) -> &EfSwUsage3 {
        &self.ef_sw_usage_3
    }
    #[doc = "0x6c - Key Slot 11 Word 0 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_11_w0(&self) -> &EfKeySlot11W0 {
        &self.ef_key_slot_11_w0
    }
    #[doc = "0x70 - Key Slot 11 Word 1 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_11_w1(&self) -> &EfKeySlot11W1 {
        &self.ef_key_slot_11_w1
    }
    #[doc = "0x74 - Key Slot 11 Word 2 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_11_w2(&self) -> &EfKeySlot11W2 {
        &self.ef_key_slot_11_w2
    }
    #[doc = "0x78 - Key Slot 11 Word 3 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_11_w3(&self) -> &EfKeySlot11W3 {
        &self.ef_key_slot_11_w3
    }
    #[doc = "0x7c - eFuse Data 0 Write/Read Lock Register"]
    #[inline(always)]
    pub const fn ef_data_0_lock(&self) -> &EfData0Lock {
        &self.ef_data_0_lock
    }
    #[doc = "0x80 - Key Slot 4 Word 0 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_4_w0(&self) -> &EfKeySlot4W0 {
        &self.ef_key_slot_4_w0
    }
    #[doc = "0x84 - Key Slot 4 Word 1 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_4_w1(&self) -> &EfKeySlot4W1 {
        &self.ef_key_slot_4_w1
    }
    #[doc = "0x88 - Key Slot 4 Word 2 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_4_w2(&self) -> &EfKeySlot4W2 {
        &self.ef_key_slot_4_w2
    }
    #[doc = "0x8c - Key Slot 4 Word 3 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_4_w3(&self) -> &EfKeySlot4W3 {
        &self.ef_key_slot_4_w3
    }
    #[doc = "0x90 - Key Slot 5 Word 0 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_5_w0(&self) -> &EfKeySlot5W0 {
        &self.ef_key_slot_5_w0
    }
    #[doc = "0x94 - Key Slot 5 Word 1 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_5_w1(&self) -> &EfKeySlot5W1 {
        &self.ef_key_slot_5_w1
    }
    #[doc = "0x98 - Key Slot 5 Word 2 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_5_w2(&self) -> &EfKeySlot5W2 {
        &self.ef_key_slot_5_w2
    }
    #[doc = "0x9c - Key Slot 5 Word 3 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_5_w3(&self) -> &EfKeySlot5W3 {
        &self.ef_key_slot_5_w3
    }
    #[doc = "0xa0 - Key Slot 6 Word 0 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_6_w0(&self) -> &EfKeySlot6W0 {
        &self.ef_key_slot_6_w0
    }
    #[doc = "0xa4 - Key Slot 6 Word 1 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_6_w1(&self) -> &EfKeySlot6W1 {
        &self.ef_key_slot_6_w1
    }
    #[doc = "0xa8 - Key Slot 6 Word 2 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_6_w2(&self) -> &EfKeySlot6W2 {
        &self.ef_key_slot_6_w2
    }
    #[doc = "0xac - Key Slot 6 Word 3 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_6_w3(&self) -> &EfKeySlot6W3 {
        &self.ef_key_slot_6_w3
    }
    #[doc = "0xb0 - Key Slot 7 Word 0 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_7_w0(&self) -> &EfKeySlot7W0 {
        &self.ef_key_slot_7_w0
    }
    #[doc = "0xb4 - Key Slot 7 Word 1 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_7_w1(&self) -> &EfKeySlot7W1 {
        &self.ef_key_slot_7_w1
    }
    #[doc = "0xb8 - Key Slot 7 Word 2 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_7_w2(&self) -> &EfKeySlot7W2 {
        &self.ef_key_slot_7_w2
    }
    #[doc = "0xbc - Key Slot 7 Word 3 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_7_w3(&self) -> &EfKeySlot7W3 {
        &self.ef_key_slot_7_w3
    }
    #[doc = "0xc0 - Key Slot 8 Word 0 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_8_w0(&self) -> &EfKeySlot8W0 {
        &self.ef_key_slot_8_w0
    }
    #[doc = "0xc4 - Key Slot 8 Word 1 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_8_w1(&self) -> &EfKeySlot8W1 {
        &self.ef_key_slot_8_w1
    }
    #[doc = "0xc8 - Key Slot 8 Word 2 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_8_w2(&self) -> &EfKeySlot8W2 {
        &self.ef_key_slot_8_w2
    }
    #[doc = "0xcc - Key Slot 8 Word 3 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_8_w3(&self) -> &EfKeySlot8W3 {
        &self.ef_key_slot_8_w3
    }
    #[doc = "0xd0 - Key Slot 9 Word 0 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_9_w0(&self) -> &EfKeySlot9W0 {
        &self.ef_key_slot_9_w0
    }
    #[doc = "0xd4 - Key Slot 9 Word 1 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_9_w1(&self) -> &EfKeySlot9W1 {
        &self.ef_key_slot_9_w1
    }
    #[doc = "0xd8 - Key Slot 9 Word 2 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_9_w2(&self) -> &EfKeySlot9W2 {
        &self.ef_key_slot_9_w2
    }
    #[doc = "0xdc - Key Slot 9 Word 3 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_9_w3(&self) -> &EfKeySlot9W3 {
        &self.ef_key_slot_9_w3
    }
    #[doc = "0xe0 - Key Slot 10 Word 0 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_10_w0(&self) -> &EfKeySlot10W0 {
        &self.ef_key_slot_10_w0
    }
    #[doc = "0xe4 - Key Slot 10 Word 1 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_10_w1(&self) -> &EfKeySlot10W1 {
        &self.ef_key_slot_10_w1
    }
    #[doc = "0xe8 - Key Slot 10 Word 2 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_10_w2(&self) -> &EfKeySlot10W2 {
        &self.ef_key_slot_10_w2
    }
    #[doc = "0xec - Key Slot 10 Word 3 (eFuse Data)"]
    #[inline(always)]
    pub const fn ef_key_slot_10_w3(&self) -> &EfKeySlot10W3 {
        &self.ef_key_slot_10_w3
    }
    #[doc = "0xf0 - eFuse Data 1 Reserved Word 0"]
    #[inline(always)]
    pub const fn ef_dat_1_rsvd_0(&self) -> &EfDat1Rsvd0 {
        &self.ef_dat_1_rsvd_0
    }
    #[doc = "0xf4 - eFuse Data 1 Reserved Word 1"]
    #[inline(always)]
    pub const fn ef_dat_1_rsvd_1(&self) -> &EfDat1Rsvd1 {
        &self.ef_dat_1_rsvd_1
    }
    #[doc = "0xf8 - eFuse Data 1 Reserved Word 2"]
    #[inline(always)]
    pub const fn ef_dat_1_rsvd_2(&self) -> &EfDat1Rsvd2 {
        &self.ef_dat_1_rsvd_2
    }
    #[doc = "0xfc - eFuse Data 1 Write/Read Lock Register"]
    #[inline(always)]
    pub const fn ef_data_1_lock(&self) -> &EfData1Lock {
        &self.ef_data_1_lock
    }
    #[doc = "0x800 - eFuse Interface 0 Control Register"]
    #[inline(always)]
    pub const fn ef_ctrl_if_ctrl_0(&self) -> &EfCtrlIfCtrl0 {
        &self.ef_ctrl_if_ctrl_0
    }
    #[doc = "0x804 - eFuse Interface Cycle Timing 0"]
    #[inline(always)]
    pub const fn ef_ctrl_if_cyc_0(&self) -> &EfCtrlIfCyc0 {
        &self.ef_ctrl_if_cyc_0
    }
    #[doc = "0x808 - eFuse Interface Cycle Timing 1"]
    #[inline(always)]
    pub const fn ef_ctrl_if_cyc_1(&self) -> &EfCtrlIfCyc1 {
        &self.ef_ctrl_if_cyc_1
    }
    #[doc = "0x80c - eFuse Interface 0 Manual Operation Register"]
    #[inline(always)]
    pub const fn ef_ctrl_if_0_manual(&self) -> &EfCtrlIf0Manual {
        &self.ef_ctrl_if_0_manual
    }
    #[doc = "0x810 - eFuse Interface 0 Status Register"]
    #[inline(always)]
    pub const fn ef_ctrl_if_0_status(&self) -> &EfCtrlIf0Status {
        &self.ef_ctrl_if_0_status
    }
    #[doc = "0x814 - eFuse Interface Configuration (Read values from array)"]
    #[inline(always)]
    pub const fn ef_ctrl_if_cfg_0(&self) -> &EfCtrlIfCfg0 {
        &self.ef_ctrl_if_cfg_0
    }
    #[doc = "0x818 - eFuse Software Override Configuration"]
    #[inline(always)]
    pub const fn ef_ctrl_sw_cfg_0(&self) -> &EfCtrlSwCfg0 {
        &self.ef_ctrl_sw_cfg_0
    }
    #[doc = "0x81c - eFuse Reserved Register (Read-Write)"]
    #[inline(always)]
    pub const fn ef_ctrl_ef_reserved(&self) -> &EfCtrlEfReserved {
        &self.ef_ctrl_ef_reserved
    }
    #[doc = "0x820 - eFuse Software Usage Register 0 (Read from array)"]
    #[inline(always)]
    pub const fn ef_ctrl_if_sw_usage_0(&self) -> &EfCtrlIfSwUsage0 {
        &self.ef_ctrl_if_sw_usage_0
    }
    #[doc = "0x824 - eFuse Software Usage Register 1 (Read from array)"]
    #[inline(always)]
    pub const fn ef_ctrl_if_sw_usage_1(&self) -> &EfCtrlIfSwUsage1 {
        &self.ef_ctrl_if_sw_usage_1
    }
    #[doc = "0x900 - eFuse Interface 1 Control Register"]
    #[inline(always)]
    pub const fn ef_ctrl_if_ctrl_1(&self) -> &EfCtrlIfCtrl1 {
        &self.ef_ctrl_if_ctrl_1
    }
    #[doc = "0x904 - eFuse Interface 1 Manual Operation Register"]
    #[inline(always)]
    pub const fn ef_ctrl_if_1_manual(&self) -> &EfCtrlIf1Manual {
        &self.ef_ctrl_if_1_manual
    }
    #[doc = "0x908 - eFuse Interface 1 Status Register"]
    #[inline(always)]
    pub const fn ef_ctrl_if_1_status(&self) -> &EfCtrlIf1Status {
        &self.ef_ctrl_if_1_status
    }
    #[doc = "0x910 - eFuse Interface 2 Control Register"]
    #[inline(always)]
    pub const fn ef_ctrl_if_ctrl_2(&self) -> &EfCtrlIfCtrl2 {
        &self.ef_ctrl_if_ctrl_2
    }
    #[doc = "0x914 - eFuse Interface 2 Manual Operation Register"]
    #[inline(always)]
    pub const fn ef_ctrl_if_2_manual(&self) -> &EfCtrlIf2Manual {
        &self.ef_ctrl_if_2_manual
    }
    #[doc = "0x918 - eFuse Interface 2 Status Register"]
    #[inline(always)]
    pub const fn ef_ctrl_if_2_status(&self) -> &EfCtrlIf2Status {
        &self.ef_ctrl_if_2_status
    }
    #[doc = "0xa00 - eFuse CRC Control Register 0"]
    #[inline(always)]
    pub const fn ef_crc_ctrl_0(&self) -> &EfCrcCtrl0 {
        &self.ef_crc_ctrl_0
    }
    #[doc = "0xa04 - eFuse CRC Data Enable Mask 0 (Bits 31:0)"]
    #[inline(always)]
    pub const fn ef_crc_ctrl_1(&self) -> &EfCrcCtrl1 {
        &self.ef_crc_ctrl_1
    }
    #[doc = "0xa08 - eFuse CRC Data Enable Mask 1 (Bits 63:32)"]
    #[inline(always)]
    pub const fn ef_crc_ctrl_2(&self) -> &EfCrcCtrl2 {
        &self.ef_crc_ctrl_2
    }
    #[doc = "0xa0c - eFuse CRC Initial Value (IV)"]
    #[inline(always)]
    pub const fn ef_crc_ctrl_3(&self) -> &EfCrcCtrl3 {
        &self.ef_crc_ctrl_3
    }
    #[doc = "0xa10 - eFuse CRC Golden/Expected Value"]
    #[inline(always)]
    pub const fn ef_crc_ctrl_4(&self) -> &EfCrcCtrl4 {
        &self.ef_crc_ctrl_4
    }
    #[doc = "0xa14 - eFuse CRC Output Data"]
    #[inline(always)]
    pub const fn ef_crc_ctrl_5(&self) -> &EfCrcCtrl5 {
        &self.ef_crc_ctrl_5
    }
}
#[doc = "EF_CFG_0 (rw) register accessor: eFuse Configuration Settings (Bitstream Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_cfg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_cfg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_cfg_0`] module"]
#[doc(alias = "EF_CFG_0")]
pub type EfCfg0 = crate::Reg<ef_cfg_0::EfCfg0Spec>;
#[doc = "eFuse Configuration Settings (Bitstream Data)"]
pub mod ef_cfg_0;
#[doc = "EF_DBG_PWD_LOW (rw) register accessor: Debug Password Low Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dbg_pwd_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dbg_pwd_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_dbg_pwd_low`] module"]
#[doc(alias = "EF_DBG_PWD_LOW")]
pub type EfDbgPwdLow = crate::Reg<ef_dbg_pwd_low::EfDbgPwdLowSpec>;
#[doc = "Debug Password Low Word"]
pub mod ef_dbg_pwd_low;
#[doc = "EF_DBG_PWD_HIGH (rw) register accessor: Debug Password High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dbg_pwd_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dbg_pwd_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_dbg_pwd_high`] module"]
#[doc(alias = "EF_DBG_PWD_HIGH")]
pub type EfDbgPwdHigh = crate::Reg<ef_dbg_pwd_high::EfDbgPwdHighSpec>;
#[doc = "Debug Password High Word"]
pub mod ef_dbg_pwd_high;
#[doc = "EF_DBG_PWD2_LOW (rw) register accessor: Debug Password 2 Low Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dbg_pwd2_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dbg_pwd2_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_dbg_pwd2_low`] module"]
#[doc(alias = "EF_DBG_PWD2_LOW")]
pub type EfDbgPwd2Low = crate::Reg<ef_dbg_pwd2_low::EfDbgPwd2LowSpec>;
#[doc = "Debug Password 2 Low Word"]
pub mod ef_dbg_pwd2_low;
#[doc = "EF_DBG_PWD2_HIGH (rw) register accessor: Debug Password 2 High Word\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dbg_pwd2_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dbg_pwd2_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_dbg_pwd2_high`] module"]
#[doc(alias = "EF_DBG_PWD2_HIGH")]
pub type EfDbgPwd2High = crate::Reg<ef_dbg_pwd2_high::EfDbgPwd2HighSpec>;
#[doc = "Debug Password 2 High Word"]
pub mod ef_dbg_pwd2_high;
#[doc = "EF_WIFI_MAC_LOW (rw) register accessor: Wi-Fi MAC Address Low Word (Bits 31:0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_wifi_mac_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_wifi_mac_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_wifi_mac_low`] module"]
#[doc(alias = "EF_WIFI_MAC_LOW")]
pub type EfWifiMacLow = crate::Reg<ef_wifi_mac_low::EfWifiMacLowSpec>;
#[doc = "Wi-Fi MAC Address Low Word (Bits 31:0)"]
pub mod ef_wifi_mac_low;
#[doc = "EF_WIFI_MAC_HIGH (rw) register accessor: Wi-Fi MAC Address High Word (Bits 47:32)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_wifi_mac_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_wifi_mac_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_wifi_mac_high`] module"]
#[doc(alias = "EF_WIFI_MAC_HIGH")]
pub type EfWifiMacHigh = crate::Reg<ef_wifi_mac_high::EfWifiMacHighSpec>;
#[doc = "Wi-Fi MAC Address High Word (Bits 47:32)"]
pub mod ef_wifi_mac_high;
#[doc = "EF_KEY_SLOT_0_W0 (rw) register accessor: Key Slot 0 Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_0_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_0_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_0_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_0_W0")]
pub type EfKeySlot0W0 = crate::Reg<ef_key_slot_0_w0::EfKeySlot0W0Spec>;
#[doc = "Key Slot 0 Word 0"]
pub mod ef_key_slot_0_w0;
#[doc = "EF_KEY_SLOT_0_W1 (rw) register accessor: Key Slot 0 Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_0_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_0_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_0_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_0_W1")]
pub type EfKeySlot0W1 = crate::Reg<ef_key_slot_0_w1::EfKeySlot0W1Spec>;
#[doc = "Key Slot 0 Word 1"]
pub mod ef_key_slot_0_w1;
#[doc = "EF_KEY_SLOT_0_W2 (rw) register accessor: Key Slot 0 Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_0_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_0_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_0_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_0_W2")]
pub type EfKeySlot0W2 = crate::Reg<ef_key_slot_0_w2::EfKeySlot0W2Spec>;
#[doc = "Key Slot 0 Word 2"]
pub mod ef_key_slot_0_w2;
#[doc = "EF_KEY_SLOT_0_W3 (rw) register accessor: Key Slot 0 Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_0_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_0_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_0_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_0_W3")]
pub type EfKeySlot0W3 = crate::Reg<ef_key_slot_0_w3::EfKeySlot0W3Spec>;
#[doc = "Key Slot 0 Word 3"]
pub mod ef_key_slot_0_w3;
#[doc = "EF_KEY_SLOT_1_W0 (rw) register accessor: Key Slot 1 Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_1_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_1_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_1_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_1_W0")]
pub type EfKeySlot1W0 = crate::Reg<ef_key_slot_1_w0::EfKeySlot1W0Spec>;
#[doc = "Key Slot 1 Word 0"]
pub mod ef_key_slot_1_w0;
#[doc = "EF_KEY_SLOT_1_W1 (rw) register accessor: Key Slot 1 Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_1_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_1_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_1_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_1_W1")]
pub type EfKeySlot1W1 = crate::Reg<ef_key_slot_1_w1::EfKeySlot1W1Spec>;
#[doc = "Key Slot 1 Word 1"]
pub mod ef_key_slot_1_w1;
#[doc = "EF_KEY_SLOT_1_W2 (rw) register accessor: Key Slot 1 Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_1_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_1_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_1_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_1_W2")]
pub type EfKeySlot1W2 = crate::Reg<ef_key_slot_1_w2::EfKeySlot1W2Spec>;
#[doc = "Key Slot 1 Word 2"]
pub mod ef_key_slot_1_w2;
#[doc = "EF_KEY_SLOT_1_W3 (rw) register accessor: Key Slot 1 Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_1_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_1_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_1_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_1_W3")]
pub type EfKeySlot1W3 = crate::Reg<ef_key_slot_1_w3::EfKeySlot1W3Spec>;
#[doc = "Key Slot 1 Word 3"]
pub mod ef_key_slot_1_w3;
#[doc = "EF_KEY_SLOT_2_W0 (rw) register accessor: Key Slot 2 Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_2_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_2_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_2_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_2_W0")]
pub type EfKeySlot2W0 = crate::Reg<ef_key_slot_2_w0::EfKeySlot2W0Spec>;
#[doc = "Key Slot 2 Word 0"]
pub mod ef_key_slot_2_w0;
#[doc = "EF_KEY_SLOT_2_W1 (rw) register accessor: Key Slot 2 Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_2_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_2_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_2_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_2_W1")]
pub type EfKeySlot2W1 = crate::Reg<ef_key_slot_2_w1::EfKeySlot2W1Spec>;
#[doc = "Key Slot 2 Word 1"]
pub mod ef_key_slot_2_w1;
#[doc = "EF_KEY_SLOT_2_W2 (rw) register accessor: Key Slot 2 Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_2_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_2_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_2_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_2_W2")]
pub type EfKeySlot2W2 = crate::Reg<ef_key_slot_2_w2::EfKeySlot2W2Spec>;
#[doc = "Key Slot 2 Word 2"]
pub mod ef_key_slot_2_w2;
#[doc = "EF_KEY_SLOT_2_W3 (rw) register accessor: Key Slot 2 Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_2_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_2_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_2_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_2_W3")]
pub type EfKeySlot2W3 = crate::Reg<ef_key_slot_2_w3::EfKeySlot2W3Spec>;
#[doc = "Key Slot 2 Word 3"]
pub mod ef_key_slot_2_w3;
#[doc = "EF_KEY_SLOT_3_W0 (rw) register accessor: Key Slot 3 Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_3_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_3_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_3_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_3_W0")]
pub type EfKeySlot3W0 = crate::Reg<ef_key_slot_3_w0::EfKeySlot3W0Spec>;
#[doc = "Key Slot 3 Word 0"]
pub mod ef_key_slot_3_w0;
#[doc = "EF_KEY_SLOT_3_W1 (rw) register accessor: Key Slot 3 Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_3_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_3_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_3_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_3_W1")]
pub type EfKeySlot3W1 = crate::Reg<ef_key_slot_3_w1::EfKeySlot3W1Spec>;
#[doc = "Key Slot 3 Word 1"]
pub mod ef_key_slot_3_w1;
#[doc = "EF_KEY_SLOT_3_W2 (rw) register accessor: Key Slot 3 Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_3_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_3_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_3_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_3_W2")]
pub type EfKeySlot3W2 = crate::Reg<ef_key_slot_3_w2::EfKeySlot3W2Spec>;
#[doc = "Key Slot 3 Word 2"]
pub mod ef_key_slot_3_w2;
#[doc = "EF_KEY_SLOT_3_W3 (rw) register accessor: Key Slot 3 Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_3_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_3_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_3_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_3_W3")]
pub type EfKeySlot3W3 = crate::Reg<ef_key_slot_3_w3::EfKeySlot3W3Spec>;
#[doc = "Key Slot 3 Word 3"]
pub mod ef_key_slot_3_w3;
#[doc = "EF_SW_USAGE_0 (rw) register accessor: Software Usage Slot 0 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_sw_usage_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_sw_usage_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_sw_usage_0`] module"]
#[doc(alias = "EF_SW_USAGE_0")]
pub type EfSwUsage0 = crate::Reg<ef_sw_usage_0::EfSwUsage0Spec>;
#[doc = "Software Usage Slot 0 (eFuse Data)"]
pub mod ef_sw_usage_0;
#[doc = "EF_SW_USAGE_1 (rw) register accessor: Software Usage Slot 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_sw_usage_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_sw_usage_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_sw_usage_1`] module"]
#[doc(alias = "EF_SW_USAGE_1")]
pub type EfSwUsage1 = crate::Reg<ef_sw_usage_1::EfSwUsage1Spec>;
#[doc = "Software Usage Slot 1 (eFuse Data)"]
pub mod ef_sw_usage_1;
#[doc = "EF_SW_USAGE_2 (rw) register accessor: Software Usage Slot 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_sw_usage_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_sw_usage_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_sw_usage_2`] module"]
#[doc(alias = "EF_SW_USAGE_2")]
pub type EfSwUsage2 = crate::Reg<ef_sw_usage_2::EfSwUsage2Spec>;
#[doc = "Software Usage Slot 2 (eFuse Data)"]
pub mod ef_sw_usage_2;
#[doc = "EF_SW_USAGE_3 (rw) register accessor: Software Usage Slot 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_sw_usage_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_sw_usage_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_sw_usage_3`] module"]
#[doc(alias = "EF_SW_USAGE_3")]
pub type EfSwUsage3 = crate::Reg<ef_sw_usage_3::EfSwUsage3Spec>;
#[doc = "Software Usage Slot 3 (eFuse Data)"]
pub mod ef_sw_usage_3;
#[doc = "EF_KEY_SLOT_11_W0 (rw) register accessor: Key Slot 11 Word 0 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_11_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_11_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_11_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_11_W0")]
pub type EfKeySlot11W0 = crate::Reg<ef_key_slot_11_w0::EfKeySlot11W0Spec>;
#[doc = "Key Slot 11 Word 0 (eFuse Data)"]
pub mod ef_key_slot_11_w0;
#[doc = "EF_KEY_SLOT_11_W1 (rw) register accessor: Key Slot 11 Word 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_11_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_11_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_11_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_11_W1")]
pub type EfKeySlot11W1 = crate::Reg<ef_key_slot_11_w1::EfKeySlot11W1Spec>;
#[doc = "Key Slot 11 Word 1 (eFuse Data)"]
pub mod ef_key_slot_11_w1;
#[doc = "EF_KEY_SLOT_11_W2 (rw) register accessor: Key Slot 11 Word 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_11_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_11_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_11_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_11_W2")]
pub type EfKeySlot11W2 = crate::Reg<ef_key_slot_11_w2::EfKeySlot11W2Spec>;
#[doc = "Key Slot 11 Word 2 (eFuse Data)"]
pub mod ef_key_slot_11_w2;
#[doc = "EF_KEY_SLOT_11_W3 (rw) register accessor: Key Slot 11 Word 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_11_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_11_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_11_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_11_W3")]
pub type EfKeySlot11W3 = crate::Reg<ef_key_slot_11_w3::EfKeySlot11W3Spec>;
#[doc = "Key Slot 11 Word 3 (eFuse Data)"]
pub mod ef_key_slot_11_w3;
#[doc = "EF_DATA_0_LOCK (rw) register accessor: eFuse Data 0 Write/Read Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_data_0_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_data_0_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_data_0_lock`] module"]
#[doc(alias = "EF_DATA_0_LOCK")]
pub type EfData0Lock = crate::Reg<ef_data_0_lock::EfData0LockSpec>;
#[doc = "eFuse Data 0 Write/Read Lock Register"]
pub mod ef_data_0_lock;
#[doc = "EF_KEY_SLOT_4_W0 (rw) register accessor: Key Slot 4 Word 0 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_4_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_4_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_4_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_4_W0")]
pub type EfKeySlot4W0 = crate::Reg<ef_key_slot_4_w0::EfKeySlot4W0Spec>;
#[doc = "Key Slot 4 Word 0 (eFuse Data)"]
pub mod ef_key_slot_4_w0;
#[doc = "EF_KEY_SLOT_4_W1 (rw) register accessor: Key Slot 4 Word 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_4_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_4_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_4_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_4_W1")]
pub type EfKeySlot4W1 = crate::Reg<ef_key_slot_4_w1::EfKeySlot4W1Spec>;
#[doc = "Key Slot 4 Word 1 (eFuse Data)"]
pub mod ef_key_slot_4_w1;
#[doc = "EF_KEY_SLOT_4_W2 (rw) register accessor: Key Slot 4 Word 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_4_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_4_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_4_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_4_W2")]
pub type EfKeySlot4W2 = crate::Reg<ef_key_slot_4_w2::EfKeySlot4W2Spec>;
#[doc = "Key Slot 4 Word 2 (eFuse Data)"]
pub mod ef_key_slot_4_w2;
#[doc = "EF_KEY_SLOT_4_W3 (rw) register accessor: Key Slot 4 Word 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_4_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_4_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_4_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_4_W3")]
pub type EfKeySlot4W3 = crate::Reg<ef_key_slot_4_w3::EfKeySlot4W3Spec>;
#[doc = "Key Slot 4 Word 3 (eFuse Data)"]
pub mod ef_key_slot_4_w3;
#[doc = "EF_KEY_SLOT_5_W0 (rw) register accessor: Key Slot 5 Word 0 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_5_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_5_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_5_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_5_W0")]
pub type EfKeySlot5W0 = crate::Reg<ef_key_slot_5_w0::EfKeySlot5W0Spec>;
#[doc = "Key Slot 5 Word 0 (eFuse Data)"]
pub mod ef_key_slot_5_w0;
#[doc = "EF_KEY_SLOT_5_W1 (rw) register accessor: Key Slot 5 Word 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_5_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_5_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_5_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_5_W1")]
pub type EfKeySlot5W1 = crate::Reg<ef_key_slot_5_w1::EfKeySlot5W1Spec>;
#[doc = "Key Slot 5 Word 1 (eFuse Data)"]
pub mod ef_key_slot_5_w1;
#[doc = "EF_KEY_SLOT_5_W2 (rw) register accessor: Key Slot 5 Word 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_5_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_5_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_5_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_5_W2")]
pub type EfKeySlot5W2 = crate::Reg<ef_key_slot_5_w2::EfKeySlot5W2Spec>;
#[doc = "Key Slot 5 Word 2 (eFuse Data)"]
pub mod ef_key_slot_5_w2;
#[doc = "EF_KEY_SLOT_5_W3 (rw) register accessor: Key Slot 5 Word 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_5_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_5_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_5_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_5_W3")]
pub type EfKeySlot5W3 = crate::Reg<ef_key_slot_5_w3::EfKeySlot5W3Spec>;
#[doc = "Key Slot 5 Word 3 (eFuse Data)"]
pub mod ef_key_slot_5_w3;
#[doc = "EF_KEY_SLOT_6_W0 (rw) register accessor: Key Slot 6 Word 0 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_6_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_6_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_6_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_6_W0")]
pub type EfKeySlot6W0 = crate::Reg<ef_key_slot_6_w0::EfKeySlot6W0Spec>;
#[doc = "Key Slot 6 Word 0 (eFuse Data)"]
pub mod ef_key_slot_6_w0;
#[doc = "EF_KEY_SLOT_6_W1 (rw) register accessor: Key Slot 6 Word 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_6_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_6_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_6_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_6_W1")]
pub type EfKeySlot6W1 = crate::Reg<ef_key_slot_6_w1::EfKeySlot6W1Spec>;
#[doc = "Key Slot 6 Word 1 (eFuse Data)"]
pub mod ef_key_slot_6_w1;
#[doc = "EF_KEY_SLOT_6_W2 (rw) register accessor: Key Slot 6 Word 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_6_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_6_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_6_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_6_W2")]
pub type EfKeySlot6W2 = crate::Reg<ef_key_slot_6_w2::EfKeySlot6W2Spec>;
#[doc = "Key Slot 6 Word 2 (eFuse Data)"]
pub mod ef_key_slot_6_w2;
#[doc = "EF_KEY_SLOT_6_W3 (rw) register accessor: Key Slot 6 Word 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_6_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_6_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_6_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_6_W3")]
pub type EfKeySlot6W3 = crate::Reg<ef_key_slot_6_w3::EfKeySlot6W3Spec>;
#[doc = "Key Slot 6 Word 3 (eFuse Data)"]
pub mod ef_key_slot_6_w3;
#[doc = "EF_KEY_SLOT_7_W0 (rw) register accessor: Key Slot 7 Word 0 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_7_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_7_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_7_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_7_W0")]
pub type EfKeySlot7W0 = crate::Reg<ef_key_slot_7_w0::EfKeySlot7W0Spec>;
#[doc = "Key Slot 7 Word 0 (eFuse Data)"]
pub mod ef_key_slot_7_w0;
#[doc = "EF_KEY_SLOT_7_W1 (rw) register accessor: Key Slot 7 Word 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_7_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_7_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_7_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_7_W1")]
pub type EfKeySlot7W1 = crate::Reg<ef_key_slot_7_w1::EfKeySlot7W1Spec>;
#[doc = "Key Slot 7 Word 1 (eFuse Data)"]
pub mod ef_key_slot_7_w1;
#[doc = "EF_KEY_SLOT_7_W2 (rw) register accessor: Key Slot 7 Word 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_7_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_7_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_7_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_7_W2")]
pub type EfKeySlot7W2 = crate::Reg<ef_key_slot_7_w2::EfKeySlot7W2Spec>;
#[doc = "Key Slot 7 Word 2 (eFuse Data)"]
pub mod ef_key_slot_7_w2;
#[doc = "EF_KEY_SLOT_7_W3 (rw) register accessor: Key Slot 7 Word 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_7_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_7_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_7_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_7_W3")]
pub type EfKeySlot7W3 = crate::Reg<ef_key_slot_7_w3::EfKeySlot7W3Spec>;
#[doc = "Key Slot 7 Word 3 (eFuse Data)"]
pub mod ef_key_slot_7_w3;
#[doc = "EF_KEY_SLOT_8_W0 (rw) register accessor: Key Slot 8 Word 0 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_8_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_8_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_8_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_8_W0")]
pub type EfKeySlot8W0 = crate::Reg<ef_key_slot_8_w0::EfKeySlot8W0Spec>;
#[doc = "Key Slot 8 Word 0 (eFuse Data)"]
pub mod ef_key_slot_8_w0;
#[doc = "EF_KEY_SLOT_8_W1 (rw) register accessor: Key Slot 8 Word 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_8_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_8_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_8_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_8_W1")]
pub type EfKeySlot8W1 = crate::Reg<ef_key_slot_8_w1::EfKeySlot8W1Spec>;
#[doc = "Key Slot 8 Word 1 (eFuse Data)"]
pub mod ef_key_slot_8_w1;
#[doc = "EF_KEY_SLOT_8_W2 (rw) register accessor: Key Slot 8 Word 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_8_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_8_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_8_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_8_W2")]
pub type EfKeySlot8W2 = crate::Reg<ef_key_slot_8_w2::EfKeySlot8W2Spec>;
#[doc = "Key Slot 8 Word 2 (eFuse Data)"]
pub mod ef_key_slot_8_w2;
#[doc = "EF_KEY_SLOT_8_W3 (rw) register accessor: Key Slot 8 Word 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_8_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_8_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_8_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_8_W3")]
pub type EfKeySlot8W3 = crate::Reg<ef_key_slot_8_w3::EfKeySlot8W3Spec>;
#[doc = "Key Slot 8 Word 3 (eFuse Data)"]
pub mod ef_key_slot_8_w3;
#[doc = "EF_KEY_SLOT_9_W0 (rw) register accessor: Key Slot 9 Word 0 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_9_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_9_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_9_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_9_W0")]
pub type EfKeySlot9W0 = crate::Reg<ef_key_slot_9_w0::EfKeySlot9W0Spec>;
#[doc = "Key Slot 9 Word 0 (eFuse Data)"]
pub mod ef_key_slot_9_w0;
#[doc = "EF_KEY_SLOT_9_W1 (rw) register accessor: Key Slot 9 Word 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_9_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_9_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_9_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_9_W1")]
pub type EfKeySlot9W1 = crate::Reg<ef_key_slot_9_w1::EfKeySlot9W1Spec>;
#[doc = "Key Slot 9 Word 1 (eFuse Data)"]
pub mod ef_key_slot_9_w1;
#[doc = "EF_KEY_SLOT_9_W2 (rw) register accessor: Key Slot 9 Word 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_9_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_9_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_9_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_9_W2")]
pub type EfKeySlot9W2 = crate::Reg<ef_key_slot_9_w2::EfKeySlot9W2Spec>;
#[doc = "Key Slot 9 Word 2 (eFuse Data)"]
pub mod ef_key_slot_9_w2;
#[doc = "EF_KEY_SLOT_9_W3 (rw) register accessor: Key Slot 9 Word 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_9_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_9_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_9_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_9_W3")]
pub type EfKeySlot9W3 = crate::Reg<ef_key_slot_9_w3::EfKeySlot9W3Spec>;
#[doc = "Key Slot 9 Word 3 (eFuse Data)"]
pub mod ef_key_slot_9_w3;
#[doc = "EF_KEY_SLOT_10_W0 (rw) register accessor: Key Slot 10 Word 0 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_10_w0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_10_w0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_10_w0`] module"]
#[doc(alias = "EF_KEY_SLOT_10_W0")]
pub type EfKeySlot10W0 = crate::Reg<ef_key_slot_10_w0::EfKeySlot10W0Spec>;
#[doc = "Key Slot 10 Word 0 (eFuse Data)"]
pub mod ef_key_slot_10_w0;
#[doc = "EF_KEY_SLOT_10_W1 (rw) register accessor: Key Slot 10 Word 1 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_10_w1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_10_w1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_10_w1`] module"]
#[doc(alias = "EF_KEY_SLOT_10_W1")]
pub type EfKeySlot10W1 = crate::Reg<ef_key_slot_10_w1::EfKeySlot10W1Spec>;
#[doc = "Key Slot 10 Word 1 (eFuse Data)"]
pub mod ef_key_slot_10_w1;
#[doc = "EF_KEY_SLOT_10_W2 (rw) register accessor: Key Slot 10 Word 2 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_10_w2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_10_w2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_10_w2`] module"]
#[doc(alias = "EF_KEY_SLOT_10_W2")]
pub type EfKeySlot10W2 = crate::Reg<ef_key_slot_10_w2::EfKeySlot10W2Spec>;
#[doc = "Key Slot 10 Word 2 (eFuse Data)"]
pub mod ef_key_slot_10_w2;
#[doc = "EF_KEY_SLOT_10_W3 (rw) register accessor: Key Slot 10 Word 3 (eFuse Data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_key_slot_10_w3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_key_slot_10_w3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_key_slot_10_w3`] module"]
#[doc(alias = "EF_KEY_SLOT_10_W3")]
pub type EfKeySlot10W3 = crate::Reg<ef_key_slot_10_w3::EfKeySlot10W3Spec>;
#[doc = "Key Slot 10 Word 3 (eFuse Data)"]
pub mod ef_key_slot_10_w3;
#[doc = "EF_DAT_1_RSVD_0 (rw) register accessor: eFuse Data 1 Reserved Word 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dat_1_rsvd_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dat_1_rsvd_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_dat_1_rsvd_0`] module"]
#[doc(alias = "EF_DAT_1_RSVD_0")]
pub type EfDat1Rsvd0 = crate::Reg<ef_dat_1_rsvd_0::EfDat1Rsvd0Spec>;
#[doc = "eFuse Data 1 Reserved Word 0"]
pub mod ef_dat_1_rsvd_0;
#[doc = "EF_DAT_1_RSVD_1 (rw) register accessor: eFuse Data 1 Reserved Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dat_1_rsvd_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dat_1_rsvd_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_dat_1_rsvd_1`] module"]
#[doc(alias = "EF_DAT_1_RSVD_1")]
pub type EfDat1Rsvd1 = crate::Reg<ef_dat_1_rsvd_1::EfDat1Rsvd1Spec>;
#[doc = "eFuse Data 1 Reserved Word 1"]
pub mod ef_dat_1_rsvd_1;
#[doc = "EF_DAT_1_RSVD_2 (rw) register accessor: eFuse Data 1 Reserved Word 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dat_1_rsvd_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dat_1_rsvd_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_dat_1_rsvd_2`] module"]
#[doc(alias = "EF_DAT_1_RSVD_2")]
pub type EfDat1Rsvd2 = crate::Reg<ef_dat_1_rsvd_2::EfDat1Rsvd2Spec>;
#[doc = "eFuse Data 1 Reserved Word 2"]
pub mod ef_dat_1_rsvd_2;
#[doc = "EF_DATA_1_LOCK (rw) register accessor: eFuse Data 1 Write/Read Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_data_1_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_data_1_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_data_1_lock`] module"]
#[doc(alias = "EF_DATA_1_LOCK")]
pub type EfData1Lock = crate::Reg<ef_data_1_lock::EfData1LockSpec>;
#[doc = "eFuse Data 1 Write/Read Lock Register"]
pub mod ef_data_1_lock;
#[doc = "EF_CTRL_IF_CTRL_0 (rw) register accessor: eFuse Interface 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_ctrl_0`] module"]
#[doc(alias = "EF_CTRL_IF_CTRL_0")]
pub type EfCtrlIfCtrl0 = crate::Reg<ef_ctrl_if_ctrl_0::EfCtrlIfCtrl0Spec>;
#[doc = "eFuse Interface 0 Control Register"]
pub mod ef_ctrl_if_ctrl_0;
#[doc = "EF_CTRL_IF_CYC_0 (rw) register accessor: eFuse Interface Cycle Timing 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_cyc_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_cyc_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_cyc_0`] module"]
#[doc(alias = "EF_CTRL_IF_CYC_0")]
pub type EfCtrlIfCyc0 = crate::Reg<ef_ctrl_if_cyc_0::EfCtrlIfCyc0Spec>;
#[doc = "eFuse Interface Cycle Timing 0"]
pub mod ef_ctrl_if_cyc_0;
#[doc = "EF_CTRL_IF_CYC_1 (rw) register accessor: eFuse Interface Cycle Timing 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_cyc_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_cyc_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_cyc_1`] module"]
#[doc(alias = "EF_CTRL_IF_CYC_1")]
pub type EfCtrlIfCyc1 = crate::Reg<ef_ctrl_if_cyc_1::EfCtrlIfCyc1Spec>;
#[doc = "eFuse Interface Cycle Timing 1"]
pub mod ef_ctrl_if_cyc_1;
#[doc = "EF_CTRL_IF_0_MANUAL (rw) register accessor: eFuse Interface 0 Manual Operation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_0_manual::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_0_manual::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_0_manual`] module"]
#[doc(alias = "EF_CTRL_IF_0_MANUAL")]
pub type EfCtrlIf0Manual = crate::Reg<ef_ctrl_if_0_manual::EfCtrlIf0ManualSpec>;
#[doc = "eFuse Interface 0 Manual Operation Register"]
pub mod ef_ctrl_if_0_manual;
#[doc = "EF_CTRL_IF_0_STATUS (rw) register accessor: eFuse Interface 0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_0_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_0_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_0_status`] module"]
#[doc(alias = "EF_CTRL_IF_0_STATUS")]
pub type EfCtrlIf0Status = crate::Reg<ef_ctrl_if_0_status::EfCtrlIf0StatusSpec>;
#[doc = "eFuse Interface 0 Status Register"]
pub mod ef_ctrl_if_0_status;
#[doc = "EF_CTRL_IF_CFG_0 (rw) register accessor: eFuse Interface Configuration (Read values from array)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_cfg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_cfg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_cfg_0`] module"]
#[doc(alias = "EF_CTRL_IF_CFG_0")]
pub type EfCtrlIfCfg0 = crate::Reg<ef_ctrl_if_cfg_0::EfCtrlIfCfg0Spec>;
#[doc = "eFuse Interface Configuration (Read values from array)"]
pub mod ef_ctrl_if_cfg_0;
#[doc = "EF_CTRL_SW_CFG_0 (rw) register accessor: eFuse Software Override Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_sw_cfg_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_sw_cfg_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_sw_cfg_0`] module"]
#[doc(alias = "EF_CTRL_SW_CFG_0")]
pub type EfCtrlSwCfg0 = crate::Reg<ef_ctrl_sw_cfg_0::EfCtrlSwCfg0Spec>;
#[doc = "eFuse Software Override Configuration"]
pub mod ef_ctrl_sw_cfg_0;
#[doc = "EF_CTRL_EF_RESERVED (rw) register accessor: eFuse Reserved Register (Read-Write)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_ef_reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_ef_reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_ef_reserved`] module"]
#[doc(alias = "EF_CTRL_EF_RESERVED")]
pub type EfCtrlEfReserved = crate::Reg<ef_ctrl_ef_reserved::EfCtrlEfReservedSpec>;
#[doc = "eFuse Reserved Register (Read-Write)"]
pub mod ef_ctrl_ef_reserved;
#[doc = "EF_CTRL_IF_SW_USAGE_0 (rw) register accessor: eFuse Software Usage Register 0 (Read from array)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_sw_usage_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_sw_usage_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_sw_usage_0`] module"]
#[doc(alias = "EF_CTRL_IF_SW_USAGE_0")]
pub type EfCtrlIfSwUsage0 = crate::Reg<ef_ctrl_if_sw_usage_0::EfCtrlIfSwUsage0Spec>;
#[doc = "eFuse Software Usage Register 0 (Read from array)"]
pub mod ef_ctrl_if_sw_usage_0;
#[doc = "EF_CTRL_IF_SW_USAGE_1 (rw) register accessor: eFuse Software Usage Register 1 (Read from array)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_sw_usage_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_sw_usage_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_sw_usage_1`] module"]
#[doc(alias = "EF_CTRL_IF_SW_USAGE_1")]
pub type EfCtrlIfSwUsage1 = crate::Reg<ef_ctrl_if_sw_usage_1::EfCtrlIfSwUsage1Spec>;
#[doc = "eFuse Software Usage Register 1 (Read from array)"]
pub mod ef_ctrl_if_sw_usage_1;
#[doc = "EF_CTRL_IF_CTRL_1 (rw) register accessor: eFuse Interface 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_ctrl_1`] module"]
#[doc(alias = "EF_CTRL_IF_CTRL_1")]
pub type EfCtrlIfCtrl1 = crate::Reg<ef_ctrl_if_ctrl_1::EfCtrlIfCtrl1Spec>;
#[doc = "eFuse Interface 1 Control Register"]
pub mod ef_ctrl_if_ctrl_1;
#[doc = "EF_CTRL_IF_1_MANUAL (rw) register accessor: eFuse Interface 1 Manual Operation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_1_manual::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_1_manual::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_1_manual`] module"]
#[doc(alias = "EF_CTRL_IF_1_MANUAL")]
pub type EfCtrlIf1Manual = crate::Reg<ef_ctrl_if_1_manual::EfCtrlIf1ManualSpec>;
#[doc = "eFuse Interface 1 Manual Operation Register"]
pub mod ef_ctrl_if_1_manual;
#[doc = "EF_CTRL_IF_1_STATUS (rw) register accessor: eFuse Interface 1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_1_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_1_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_1_status`] module"]
#[doc(alias = "EF_CTRL_IF_1_STATUS")]
pub type EfCtrlIf1Status = crate::Reg<ef_ctrl_if_1_status::EfCtrlIf1StatusSpec>;
#[doc = "eFuse Interface 1 Status Register"]
pub mod ef_ctrl_if_1_status;
#[doc = "EF_CTRL_IF_CTRL_2 (rw) register accessor: eFuse Interface 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_ctrl_2`] module"]
#[doc(alias = "EF_CTRL_IF_CTRL_2")]
pub type EfCtrlIfCtrl2 = crate::Reg<ef_ctrl_if_ctrl_2::EfCtrlIfCtrl2Spec>;
#[doc = "eFuse Interface 2 Control Register"]
pub mod ef_ctrl_if_ctrl_2;
#[doc = "EF_CTRL_IF_2_MANUAL (rw) register accessor: eFuse Interface 2 Manual Operation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_2_manual::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_2_manual::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_2_manual`] module"]
#[doc(alias = "EF_CTRL_IF_2_MANUAL")]
pub type EfCtrlIf2Manual = crate::Reg<ef_ctrl_if_2_manual::EfCtrlIf2ManualSpec>;
#[doc = "eFuse Interface 2 Manual Operation Register"]
pub mod ef_ctrl_if_2_manual;
#[doc = "EF_CTRL_IF_2_STATUS (rw) register accessor: eFuse Interface 2 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_2_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_2_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_ctrl_if_2_status`] module"]
#[doc(alias = "EF_CTRL_IF_2_STATUS")]
pub type EfCtrlIf2Status = crate::Reg<ef_ctrl_if_2_status::EfCtrlIf2StatusSpec>;
#[doc = "eFuse Interface 2 Status Register"]
pub mod ef_ctrl_if_2_status;
#[doc = "EF_CRC_CTRL_0 (rw) register accessor: eFuse CRC Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_0`] module"]
#[doc(alias = "EF_CRC_CTRL_0")]
pub type EfCrcCtrl0 = crate::Reg<ef_crc_ctrl_0::EfCrcCtrl0Spec>;
#[doc = "eFuse CRC Control Register 0"]
pub mod ef_crc_ctrl_0;
#[doc = "EF_CRC_CTRL_1 (rw) register accessor: eFuse CRC Data Enable Mask 0 (Bits 31:0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_1`] module"]
#[doc(alias = "EF_CRC_CTRL_1")]
pub type EfCrcCtrl1 = crate::Reg<ef_crc_ctrl_1::EfCrcCtrl1Spec>;
#[doc = "eFuse CRC Data Enable Mask 0 (Bits 31:0)"]
pub mod ef_crc_ctrl_1;
#[doc = "EF_CRC_CTRL_2 (rw) register accessor: eFuse CRC Data Enable Mask 1 (Bits 63:32)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_2`] module"]
#[doc(alias = "EF_CRC_CTRL_2")]
pub type EfCrcCtrl2 = crate::Reg<ef_crc_ctrl_2::EfCrcCtrl2Spec>;
#[doc = "eFuse CRC Data Enable Mask 1 (Bits 63:32)"]
pub mod ef_crc_ctrl_2;
#[doc = "EF_CRC_CTRL_3 (rw) register accessor: eFuse CRC Initial Value (IV)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_3`] module"]
#[doc(alias = "EF_CRC_CTRL_3")]
pub type EfCrcCtrl3 = crate::Reg<ef_crc_ctrl_3::EfCrcCtrl3Spec>;
#[doc = "eFuse CRC Initial Value (IV)"]
pub mod ef_crc_ctrl_3;
#[doc = "EF_CRC_CTRL_4 (rw) register accessor: eFuse CRC Golden/Expected Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_4`] module"]
#[doc(alias = "EF_CRC_CTRL_4")]
pub type EfCrcCtrl4 = crate::Reg<ef_crc_ctrl_4::EfCrcCtrl4Spec>;
#[doc = "eFuse CRC Golden/Expected Value"]
pub mod ef_crc_ctrl_4;
#[doc = "EF_CRC_CTRL_5 (rw) register accessor: eFuse CRC Output Data\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ef_crc_ctrl_5`] module"]
#[doc(alias = "EF_CRC_CTRL_5")]
pub type EfCrcCtrl5 = crate::Reg<ef_crc_ctrl_5::EfCrcCtrl5Spec>;
#[doc = "eFuse CRC Output Data"]
pub mod ef_crc_ctrl_5;
