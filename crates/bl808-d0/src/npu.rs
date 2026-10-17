#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bali_general_cfg: BaliGeneralCfg,
    blai_int_cfg: BlaiIntCfg,
    _reserved2: [u8; 0x08],
    blai_weight_addr: BlaiWeightAddr,
    blai_bias_addr: BlaiBiasAddr,
    blai_inst_addr: BlaiInstAddr,
    apu_dm0_addr: ApuDm0Addr,
    apu_dm1_addr: ApuDm1Addr,
    blai_tf_cfg0: BlaiTfCfg0,
    _reserved8: [u8; 0xd8],
    blai_act_table_03_00: BlaiActTable03_00,
    blai_act_table_07_04: BlaiActTable07_04,
    blai_act_table_0b_08: BlaiActTable0b08,
    blai_act_table_0f_0c: BlaiActTable0f0c,
    blai_act_table_13_10: BlaiActTable13_10,
    blai_act_table_17_14: BlaiActTable17_14,
    blai_act_table_1b_18: BlaiActTable1b18,
    blai_act_table_1f_1c: BlaiActTable1f1c,
    blai_act_table_23_20: BlaiActTable23_20,
    blai_act_table_27_24: BlaiActTable27_24,
    blai_act_table_2b_28: BlaiActTable2b28,
    blai_act_table_2f_2c: BlaiActTable2f2c,
    blai_act_table_33_30: BlaiActTable33_30,
    blai_act_table_37_34: BlaiActTable37_34,
    blai_act_table_3b_38: BlaiActTable3b38,
    blai_act_table_3f_3c: BlaiActTable3f3c,
    blai_act_table_43_40: BlaiActTable43_40,
    blai_act_table_47_44: BlaiActTable47_44,
    blai_act_table_4b_48: BlaiActTable4b48,
    blai_act_table_4f_4c: BlaiActTable4f4c,
    blai_act_table_53_50: BlaiActTable53_50,
    blai_act_table_57_54: BlaiActTable57_54,
    blai_act_table_5b_58: BlaiActTable5b58,
    blai_act_table_5f_5c: BlaiActTable5f5c,
    blai_act_table_63_60: BlaiActTable63_60,
    blai_act_table_67_64: BlaiActTable67_64,
    blai_act_table_6b_68: BlaiActTable6b68,
    blai_act_table_6f_6c: BlaiActTable6f6c,
    blai_act_table_73_70: BlaiActTable73_70,
    blai_act_table_77_74: BlaiActTable77_74,
    blai_act_table_7b_78: BlaiActTable7b78,
    blai_act_table_7f_7c: BlaiActTable7f7c,
    blai_act_table_83_80: BlaiActTable83_80,
    blai_act_table_87_84: BlaiActTable87_84,
    blai_act_table_8b_88: BlaiActTable8b88,
    blai_act_table_8f_8c: BlaiActTable8f8c,
    blai_act_table_93_90: BlaiActTable93_90,
    blai_act_table_97_94: BlaiActTable97_94,
    blai_act_table_9b_98: BlaiActTable9b98,
    blai_act_table_9f_9c: BlaiActTable9f9c,
    blai_act_table_a3_a0: BlaiActTableA3A0,
    blai_act_table_a7_a4: BlaiActTableA7A4,
    blai_act_table_ab_a8: BlaiActTableAbA8,
    blai_act_table_af_ac: BlaiActTableAfAc,
    blai_act_table_b3_b0: BlaiActTableB3B0,
    blai_act_table_b7_b4: BlaiActTableB7B4,
    blai_act_table_bb_b8: BlaiActTableBbB8,
    blai_act_table_bf_bc: BlaiActTableBfBc,
    blai_act_table_c3_c0: BlaiActTableC3C0,
    blai_act_table_c7_c4: BlaiActTableC7C4,
    blai_act_table_cb_c8: BlaiActTableCbC8,
    blai_act_table_cf_cc: BlaiActTableCfCc,
    blai_act_table_d3_d0: BlaiActTableD3D0,
    blai_act_table_d7_d4: BlaiActTableD7D4,
    blai_act_table_db_d8: BlaiActTableDbD8,
    blai_act_table_df_dc: BlaiActTableDfDc,
    blai_act_table_e3_e0: BlaiActTableE3E0,
    blai_act_table_e7_e4: BlaiActTableE7E4,
    blai_act_table_eb_e8: BlaiActTableEbE8,
    blai_act_table_ef_ec: BlaiActTableEfEc,
    blai_act_table_f3_f0: BlaiActTableF3F0,
    blai_act_table_f7_f4: BlaiActTableF7F4,
    blai_act_table_fb_f8: BlaiActTableFbF8,
    blai_act_table_ff_fc: BlaiActTableFfFc,
}
impl RegisterBlock {
    #[doc = "0x00 - BLAI General Configuration Register"]
    #[inline(always)]
    pub const fn bali_general_cfg(&self) -> &BaliGeneralCfg {
        &self.bali_general_cfg
    }
    #[doc = "0x04 - BLAI Interrupt and Control Configuration"]
    #[inline(always)]
    pub const fn blai_int_cfg(&self) -> &BlaiIntCfg {
        &self.blai_int_cfg
    }
    #[doc = "0x10 - Weight Data Base Address Pointer"]
    #[inline(always)]
    pub const fn blai_weight_addr(&self) -> &BlaiWeightAddr {
        &self.blai_weight_addr
    }
    #[doc = "0x14 - Bias Data Base Address Pointer"]
    #[inline(always)]
    pub const fn blai_bias_addr(&self) -> &BlaiBiasAddr {
        &self.blai_bias_addr
    }
    #[doc = "0x18 - Instruction Command Base Address Pointer"]
    #[inline(always)]
    pub const fn blai_inst_addr(&self) -> &BlaiInstAddr {
        &self.blai_inst_addr
    }
    #[doc = "0x1c - Image Data (Input/Output) Base Address Pointer (DM0)"]
    #[inline(always)]
    pub const fn apu_dm0_addr(&self) -> &ApuDm0Addr {
        &self.apu_dm0_addr
    }
    #[doc = "0x20 - Image Memory Segment Configuration (DM1)"]
    #[inline(always)]
    pub const fn apu_dm1_addr(&self) -> &ApuDm1Addr {
        &self.apu_dm1_addr
    }
    #[doc = "0x24 - BLAI TensorFlow Compatibility Configuration 0"]
    #[inline(always)]
    pub const fn blai_tf_cfg0(&self) -> &BlaiTfCfg0 {
        &self.blai_tf_cfg0
    }
    #[doc = "0x100 - BLAI Activation Table (Indices 0x00-0x03)"]
    #[inline(always)]
    pub const fn blai_act_table_03_00(&self) -> &BlaiActTable03_00 {
        &self.blai_act_table_03_00
    }
    #[doc = "0x104 - BLAI Activation Table (Indices 0x04-0x07)"]
    #[inline(always)]
    pub const fn blai_act_table_07_04(&self) -> &BlaiActTable07_04 {
        &self.blai_act_table_07_04
    }
    #[doc = "0x108 - BLAI Activation Table (Indices 0x08-0x0B)"]
    #[inline(always)]
    pub const fn blai_act_table_0b_08(&self) -> &BlaiActTable0b08 {
        &self.blai_act_table_0b_08
    }
    #[doc = "0x10c - BLAI Activation Table (Indices 0x0C-0x0F)"]
    #[inline(always)]
    pub const fn blai_act_table_0f_0c(&self) -> &BlaiActTable0f0c {
        &self.blai_act_table_0f_0c
    }
    #[doc = "0x110 - BLAI Activation Table (Indices 0x10-0x13)"]
    #[inline(always)]
    pub const fn blai_act_table_13_10(&self) -> &BlaiActTable13_10 {
        &self.blai_act_table_13_10
    }
    #[doc = "0x114 - BLAI Activation Table (Indices 0x14-0x17)"]
    #[inline(always)]
    pub const fn blai_act_table_17_14(&self) -> &BlaiActTable17_14 {
        &self.blai_act_table_17_14
    }
    #[doc = "0x118 - BLAI Activation Table (Indices 0x18-0x1B)"]
    #[inline(always)]
    pub const fn blai_act_table_1b_18(&self) -> &BlaiActTable1b18 {
        &self.blai_act_table_1b_18
    }
    #[doc = "0x11c - BLAI Activation Table (Indices 0x1C-0x1F)"]
    #[inline(always)]
    pub const fn blai_act_table_1f_1c(&self) -> &BlaiActTable1f1c {
        &self.blai_act_table_1f_1c
    }
    #[doc = "0x120 - BLAI Activation Table (Indices 0x20-0x23)"]
    #[inline(always)]
    pub const fn blai_act_table_23_20(&self) -> &BlaiActTable23_20 {
        &self.blai_act_table_23_20
    }
    #[doc = "0x124 - BLAI Activation Table (Indices 0x24-0x27)"]
    #[inline(always)]
    pub const fn blai_act_table_27_24(&self) -> &BlaiActTable27_24 {
        &self.blai_act_table_27_24
    }
    #[doc = "0x128 - BLAI Activation Table (Indices 0x28-0x2B)"]
    #[inline(always)]
    pub const fn blai_act_table_2b_28(&self) -> &BlaiActTable2b28 {
        &self.blai_act_table_2b_28
    }
    #[doc = "0x12c - BLAI Activation Table (Indices 0x2C-0x2F)"]
    #[inline(always)]
    pub const fn blai_act_table_2f_2c(&self) -> &BlaiActTable2f2c {
        &self.blai_act_table_2f_2c
    }
    #[doc = "0x130 - BLAI Activation Table (Indices 0x30-0x33)"]
    #[inline(always)]
    pub const fn blai_act_table_33_30(&self) -> &BlaiActTable33_30 {
        &self.blai_act_table_33_30
    }
    #[doc = "0x134 - BLAI Activation Table (Indices 0x34-0x37)"]
    #[inline(always)]
    pub const fn blai_act_table_37_34(&self) -> &BlaiActTable37_34 {
        &self.blai_act_table_37_34
    }
    #[doc = "0x138 - BLAI Activation Table (Indices 0x38-0x3B)"]
    #[inline(always)]
    pub const fn blai_act_table_3b_38(&self) -> &BlaiActTable3b38 {
        &self.blai_act_table_3b_38
    }
    #[doc = "0x13c - BLAI Activation Table (Indices 0x3C-0x3F)"]
    #[inline(always)]
    pub const fn blai_act_table_3f_3c(&self) -> &BlaiActTable3f3c {
        &self.blai_act_table_3f_3c
    }
    #[doc = "0x140 - BLAI Activation Table (Indices 0x40-0x43)"]
    #[inline(always)]
    pub const fn blai_act_table_43_40(&self) -> &BlaiActTable43_40 {
        &self.blai_act_table_43_40
    }
    #[doc = "0x144 - BLAI Activation Table (Indices 0x44-0x47)"]
    #[inline(always)]
    pub const fn blai_act_table_47_44(&self) -> &BlaiActTable47_44 {
        &self.blai_act_table_47_44
    }
    #[doc = "0x148 - BLAI Activation Table (Indices 0x48-0x4B)"]
    #[inline(always)]
    pub const fn blai_act_table_4b_48(&self) -> &BlaiActTable4b48 {
        &self.blai_act_table_4b_48
    }
    #[doc = "0x14c - BLAI Activation Table (Indices 0x4C-0x4F)"]
    #[inline(always)]
    pub const fn blai_act_table_4f_4c(&self) -> &BlaiActTable4f4c {
        &self.blai_act_table_4f_4c
    }
    #[doc = "0x150 - BLAI Activation Table (Indices 0x50-0x53)"]
    #[inline(always)]
    pub const fn blai_act_table_53_50(&self) -> &BlaiActTable53_50 {
        &self.blai_act_table_53_50
    }
    #[doc = "0x154 - BLAI Activation Table (Indices 0x54-0x57)"]
    #[inline(always)]
    pub const fn blai_act_table_57_54(&self) -> &BlaiActTable57_54 {
        &self.blai_act_table_57_54
    }
    #[doc = "0x158 - BLAI Activation Table (Indices 0x58-0x5B)"]
    #[inline(always)]
    pub const fn blai_act_table_5b_58(&self) -> &BlaiActTable5b58 {
        &self.blai_act_table_5b_58
    }
    #[doc = "0x15c - BLAI Activation Table (Indices 0x5C-0x5F)"]
    #[inline(always)]
    pub const fn blai_act_table_5f_5c(&self) -> &BlaiActTable5f5c {
        &self.blai_act_table_5f_5c
    }
    #[doc = "0x160 - BLAI Activation Table (Indices 0x60-0x63)"]
    #[inline(always)]
    pub const fn blai_act_table_63_60(&self) -> &BlaiActTable63_60 {
        &self.blai_act_table_63_60
    }
    #[doc = "0x164 - BLAI Activation Table (Indices 0x64-0x67)"]
    #[inline(always)]
    pub const fn blai_act_table_67_64(&self) -> &BlaiActTable67_64 {
        &self.blai_act_table_67_64
    }
    #[doc = "0x168 - BLAI Activation Table (Indices 0x68-0x6B)"]
    #[inline(always)]
    pub const fn blai_act_table_6b_68(&self) -> &BlaiActTable6b68 {
        &self.blai_act_table_6b_68
    }
    #[doc = "0x16c - BLAI Activation Table (Indices 0x6C-0x6F)"]
    #[inline(always)]
    pub const fn blai_act_table_6f_6c(&self) -> &BlaiActTable6f6c {
        &self.blai_act_table_6f_6c
    }
    #[doc = "0x170 - BLAI Activation Table (Indices 0x70-0x73)"]
    #[inline(always)]
    pub const fn blai_act_table_73_70(&self) -> &BlaiActTable73_70 {
        &self.blai_act_table_73_70
    }
    #[doc = "0x174 - BLAI Activation Table (Indices 0x74-0x77)"]
    #[inline(always)]
    pub const fn blai_act_table_77_74(&self) -> &BlaiActTable77_74 {
        &self.blai_act_table_77_74
    }
    #[doc = "0x178 - BLAI Activation Table (Indices 0x78-0x7B)"]
    #[inline(always)]
    pub const fn blai_act_table_7b_78(&self) -> &BlaiActTable7b78 {
        &self.blai_act_table_7b_78
    }
    #[doc = "0x17c - BLAI Activation Table (Indices 0x7C-0x7F)"]
    #[inline(always)]
    pub const fn blai_act_table_7f_7c(&self) -> &BlaiActTable7f7c {
        &self.blai_act_table_7f_7c
    }
    #[doc = "0x180 - BLAI Activation Table (Indices 0x80-0x83)"]
    #[inline(always)]
    pub const fn blai_act_table_83_80(&self) -> &BlaiActTable83_80 {
        &self.blai_act_table_83_80
    }
    #[doc = "0x184 - BLAI Activation Table (Indices 0x84-0x87)"]
    #[inline(always)]
    pub const fn blai_act_table_87_84(&self) -> &BlaiActTable87_84 {
        &self.blai_act_table_87_84
    }
    #[doc = "0x188 - BLAI Activation Table (Indices 0x88-0x8B)"]
    #[inline(always)]
    pub const fn blai_act_table_8b_88(&self) -> &BlaiActTable8b88 {
        &self.blai_act_table_8b_88
    }
    #[doc = "0x18c - BLAI Activation Table (Indices 0x8C-0x8F)"]
    #[inline(always)]
    pub const fn blai_act_table_8f_8c(&self) -> &BlaiActTable8f8c {
        &self.blai_act_table_8f_8c
    }
    #[doc = "0x190 - BLAI Activation Table (Indices 0x90-0x93)"]
    #[inline(always)]
    pub const fn blai_act_table_93_90(&self) -> &BlaiActTable93_90 {
        &self.blai_act_table_93_90
    }
    #[doc = "0x194 - BLAI Activation Table (Indices 0x94-0x97)"]
    #[inline(always)]
    pub const fn blai_act_table_97_94(&self) -> &BlaiActTable97_94 {
        &self.blai_act_table_97_94
    }
    #[doc = "0x198 - BLAI Activation Table (Indices 0x98-0x9B)"]
    #[inline(always)]
    pub const fn blai_act_table_9b_98(&self) -> &BlaiActTable9b98 {
        &self.blai_act_table_9b_98
    }
    #[doc = "0x19c - BLAI Activation Table (Indices 0x9C-0x9F)"]
    #[inline(always)]
    pub const fn blai_act_table_9f_9c(&self) -> &BlaiActTable9f9c {
        &self.blai_act_table_9f_9c
    }
    #[doc = "0x1a0 - BLAI Activation Table (Indices 0xA0-0xA3)"]
    #[inline(always)]
    pub const fn blai_act_table_a3_a0(&self) -> &BlaiActTableA3A0 {
        &self.blai_act_table_a3_a0
    }
    #[doc = "0x1a4 - BLAI Activation Table (Indices 0xA4-0xA7)"]
    #[inline(always)]
    pub const fn blai_act_table_a7_a4(&self) -> &BlaiActTableA7A4 {
        &self.blai_act_table_a7_a4
    }
    #[doc = "0x1a8 - BLAI Activation Table (Indices 0xA8-0xAB)"]
    #[inline(always)]
    pub const fn blai_act_table_ab_a8(&self) -> &BlaiActTableAbA8 {
        &self.blai_act_table_ab_a8
    }
    #[doc = "0x1ac - BLAI Activation Table (Indices 0xAC-0xAF)"]
    #[inline(always)]
    pub const fn blai_act_table_af_ac(&self) -> &BlaiActTableAfAc {
        &self.blai_act_table_af_ac
    }
    #[doc = "0x1b0 - BLAI Activation Table (Indices 0xB0-0xB3)"]
    #[inline(always)]
    pub const fn blai_act_table_b3_b0(&self) -> &BlaiActTableB3B0 {
        &self.blai_act_table_b3_b0
    }
    #[doc = "0x1b4 - BLAI Activation Table (Indices 0xB4-0xB7)"]
    #[inline(always)]
    pub const fn blai_act_table_b7_b4(&self) -> &BlaiActTableB7B4 {
        &self.blai_act_table_b7_b4
    }
    #[doc = "0x1b8 - BLAI Activation Table (Indices 0xB8-0xBB)"]
    #[inline(always)]
    pub const fn blai_act_table_bb_b8(&self) -> &BlaiActTableBbB8 {
        &self.blai_act_table_bb_b8
    }
    #[doc = "0x1bc - BLAI Activation Table (Indices 0xBC-0xBF)"]
    #[inline(always)]
    pub const fn blai_act_table_bf_bc(&self) -> &BlaiActTableBfBc {
        &self.blai_act_table_bf_bc
    }
    #[doc = "0x1c0 - BLAI Activation Table (Indices 0xC0-0xC3)"]
    #[inline(always)]
    pub const fn blai_act_table_c3_c0(&self) -> &BlaiActTableC3C0 {
        &self.blai_act_table_c3_c0
    }
    #[doc = "0x1c4 - BLAI Activation Table (Indices 0xC4-0xC7)"]
    #[inline(always)]
    pub const fn blai_act_table_c7_c4(&self) -> &BlaiActTableC7C4 {
        &self.blai_act_table_c7_c4
    }
    #[doc = "0x1c8 - BLAI Activation Table (Indices 0xC8-0xCB)"]
    #[inline(always)]
    pub const fn blai_act_table_cb_c8(&self) -> &BlaiActTableCbC8 {
        &self.blai_act_table_cb_c8
    }
    #[doc = "0x1cc - BLAI Activation Table (Indices 0xCC-0xCF)"]
    #[inline(always)]
    pub const fn blai_act_table_cf_cc(&self) -> &BlaiActTableCfCc {
        &self.blai_act_table_cf_cc
    }
    #[doc = "0x1d0 - BLAI Activation Table (Indices 0xD0-0xD3)"]
    #[inline(always)]
    pub const fn blai_act_table_d3_d0(&self) -> &BlaiActTableD3D0 {
        &self.blai_act_table_d3_d0
    }
    #[doc = "0x1d4 - BLAI Activation Table (Indices 0xD4-0xD7)"]
    #[inline(always)]
    pub const fn blai_act_table_d7_d4(&self) -> &BlaiActTableD7D4 {
        &self.blai_act_table_d7_d4
    }
    #[doc = "0x1d8 - BLAI Activation Table (Indices 0xD8-0xDB)"]
    #[inline(always)]
    pub const fn blai_act_table_db_d8(&self) -> &BlaiActTableDbD8 {
        &self.blai_act_table_db_d8
    }
    #[doc = "0x1dc - BLAI Activation Table (Indices 0xDC-0xDF)"]
    #[inline(always)]
    pub const fn blai_act_table_df_dc(&self) -> &BlaiActTableDfDc {
        &self.blai_act_table_df_dc
    }
    #[doc = "0x1e0 - BLAI Activation Table (Indices 0xE0-0xE3)"]
    #[inline(always)]
    pub const fn blai_act_table_e3_e0(&self) -> &BlaiActTableE3E0 {
        &self.blai_act_table_e3_e0
    }
    #[doc = "0x1e4 - BLAI Activation Table (Indices 0xE4-0xE7)"]
    #[inline(always)]
    pub const fn blai_act_table_e7_e4(&self) -> &BlaiActTableE7E4 {
        &self.blai_act_table_e7_e4
    }
    #[doc = "0x1e8 - BLAI Activation Table (Indices 0xE8-0xEB)"]
    #[inline(always)]
    pub const fn blai_act_table_eb_e8(&self) -> &BlaiActTableEbE8 {
        &self.blai_act_table_eb_e8
    }
    #[doc = "0x1ec - BLAI Activation Table (Indices 0xEC-0xEF)"]
    #[inline(always)]
    pub const fn blai_act_table_ef_ec(&self) -> &BlaiActTableEfEc {
        &self.blai_act_table_ef_ec
    }
    #[doc = "0x1f0 - BLAI Activation Table (Indices 0xF0-0xF3)"]
    #[inline(always)]
    pub const fn blai_act_table_f3_f0(&self) -> &BlaiActTableF3F0 {
        &self.blai_act_table_f3_f0
    }
    #[doc = "0x1f4 - BLAI Activation Table (Indices 0xF4-0xF7)"]
    #[inline(always)]
    pub const fn blai_act_table_f7_f4(&self) -> &BlaiActTableF7F4 {
        &self.blai_act_table_f7_f4
    }
    #[doc = "0x1f8 - BLAI Activation Table (Indices 0xF8-0xFB)"]
    #[inline(always)]
    pub const fn blai_act_table_fb_f8(&self) -> &BlaiActTableFbF8 {
        &self.blai_act_table_fb_f8
    }
    #[doc = "0x1fc - BLAI Activation Table (Indices 0xFC-0xFF)"]
    #[inline(always)]
    pub const fn blai_act_table_ff_fc(&self) -> &BlaiActTableFfFc {
        &self.blai_act_table_ff_fc
    }
}
#[doc = "BALI_GENERAL_CFG (rw) register accessor: BLAI General Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bali_general_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bali_general_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bali_general_cfg`] module"]
#[doc(alias = "BALI_GENERAL_CFG")]
pub type BaliGeneralCfg = crate::Reg<bali_general_cfg::BaliGeneralCfgSpec>;
#[doc = "BLAI General Configuration Register"]
pub mod bali_general_cfg;
#[doc = "BLAI_INT_CFG (rw) register accessor: BLAI Interrupt and Control Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_int_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_int_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_int_cfg`] module"]
#[doc(alias = "BLAI_INT_CFG")]
pub type BlaiIntCfg = crate::Reg<blai_int_cfg::BlaiIntCfgSpec>;
#[doc = "BLAI Interrupt and Control Configuration"]
pub mod blai_int_cfg;
#[doc = "BLAI_WEIGHT_ADDR (rw) register accessor: Weight Data Base Address Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_weight_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_weight_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_weight_addr`] module"]
#[doc(alias = "BLAI_WEIGHT_ADDR")]
pub type BlaiWeightAddr = crate::Reg<blai_weight_addr::BlaiWeightAddrSpec>;
#[doc = "Weight Data Base Address Pointer"]
pub mod blai_weight_addr;
#[doc = "BLAI_BIAS_ADDR (rw) register accessor: Bias Data Base Address Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_bias_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_bias_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_bias_addr`] module"]
#[doc(alias = "BLAI_BIAS_ADDR")]
pub type BlaiBiasAddr = crate::Reg<blai_bias_addr::BlaiBiasAddrSpec>;
#[doc = "Bias Data Base Address Pointer"]
pub mod blai_bias_addr;
#[doc = "BLAI_INST_ADDR (rw) register accessor: Instruction Command Base Address Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_inst_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_inst_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_inst_addr`] module"]
#[doc(alias = "BLAI_INST_ADDR")]
pub type BlaiInstAddr = crate::Reg<blai_inst_addr::BlaiInstAddrSpec>;
#[doc = "Instruction Command Base Address Pointer"]
pub mod blai_inst_addr;
#[doc = "APU_DM0_ADDR (rw) register accessor: Image Data (Input/Output) Base Address Pointer (DM0)\n\nYou can [`read`](crate::Reg::read) this register and get [`apu_dm0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apu_dm0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apu_dm0_addr`] module"]
#[doc(alias = "APU_DM0_ADDR")]
pub type ApuDm0Addr = crate::Reg<apu_dm0_addr::ApuDm0AddrSpec>;
#[doc = "Image Data (Input/Output) Base Address Pointer (DM0)"]
pub mod apu_dm0_addr;
#[doc = "APU_DM1_ADDR (rw) register accessor: Image Memory Segment Configuration (DM1)\n\nYou can [`read`](crate::Reg::read) this register and get [`apu_dm1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apu_dm1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apu_dm1_addr`] module"]
#[doc(alias = "APU_DM1_ADDR")]
pub type ApuDm1Addr = crate::Reg<apu_dm1_addr::ApuDm1AddrSpec>;
#[doc = "Image Memory Segment Configuration (DM1)"]
pub mod apu_dm1_addr;
#[doc = "BLAI_TF_CFG0 (rw) register accessor: BLAI TensorFlow Compatibility Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_tf_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_tf_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_tf_cfg0`] module"]
#[doc(alias = "BLAI_TF_CFG0")]
pub type BlaiTfCfg0 = crate::Reg<blai_tf_cfg0::BlaiTfCfg0Spec>;
#[doc = "BLAI TensorFlow Compatibility Configuration 0"]
pub mod blai_tf_cfg0;
#[doc = "BLAI_ACT_TABLE_03_00 (rw) register accessor: BLAI Activation Table (Indices 0x00-0x03)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_03_00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_03_00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_03_00`] module"]
#[doc(alias = "BLAI_ACT_TABLE_03_00")]
pub type BlaiActTable03_00 = crate::Reg<blai_act_table_03_00::BlaiActTable03_00Spec>;
#[doc = "BLAI Activation Table (Indices 0x00-0x03)"]
pub mod blai_act_table_03_00;
#[doc = "BLAI_ACT_TABLE_07_04 (rw) register accessor: BLAI Activation Table (Indices 0x04-0x07)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_07_04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_07_04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_07_04`] module"]
#[doc(alias = "BLAI_ACT_TABLE_07_04")]
pub type BlaiActTable07_04 = crate::Reg<blai_act_table_07_04::BlaiActTable07_04Spec>;
#[doc = "BLAI Activation Table (Indices 0x04-0x07)"]
pub mod blai_act_table_07_04;
#[doc = "BLAI_ACT_TABLE_0B_08 (rw) register accessor: BLAI Activation Table (Indices 0x08-0x0B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_0b_08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_0b_08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_0b_08`] module"]
#[doc(alias = "BLAI_ACT_TABLE_0B_08")]
pub type BlaiActTable0b08 = crate::Reg<blai_act_table_0b_08::BlaiActTable0b08Spec>;
#[doc = "BLAI Activation Table (Indices 0x08-0x0B)"]
pub mod blai_act_table_0b_08;
#[doc = "BLAI_ACT_TABLE_0F_0C (rw) register accessor: BLAI Activation Table (Indices 0x0C-0x0F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_0f_0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_0f_0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_0f_0c`] module"]
#[doc(alias = "BLAI_ACT_TABLE_0F_0C")]
pub type BlaiActTable0f0c = crate::Reg<blai_act_table_0f_0c::BlaiActTable0f0cSpec>;
#[doc = "BLAI Activation Table (Indices 0x0C-0x0F)"]
pub mod blai_act_table_0f_0c;
#[doc = "BLAI_ACT_TABLE_13_10 (rw) register accessor: BLAI Activation Table (Indices 0x10-0x13)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_13_10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_13_10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_13_10`] module"]
#[doc(alias = "BLAI_ACT_TABLE_13_10")]
pub type BlaiActTable13_10 = crate::Reg<blai_act_table_13_10::BlaiActTable13_10Spec>;
#[doc = "BLAI Activation Table (Indices 0x10-0x13)"]
pub mod blai_act_table_13_10;
#[doc = "BLAI_ACT_TABLE_17_14 (rw) register accessor: BLAI Activation Table (Indices 0x14-0x17)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_17_14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_17_14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_17_14`] module"]
#[doc(alias = "BLAI_ACT_TABLE_17_14")]
pub type BlaiActTable17_14 = crate::Reg<blai_act_table_17_14::BlaiActTable17_14Spec>;
#[doc = "BLAI Activation Table (Indices 0x14-0x17)"]
pub mod blai_act_table_17_14;
#[doc = "BLAI_ACT_TABLE_1B_18 (rw) register accessor: BLAI Activation Table (Indices 0x18-0x1B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_1b_18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_1b_18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_1b_18`] module"]
#[doc(alias = "BLAI_ACT_TABLE_1B_18")]
pub type BlaiActTable1b18 = crate::Reg<blai_act_table_1b_18::BlaiActTable1b18Spec>;
#[doc = "BLAI Activation Table (Indices 0x18-0x1B)"]
pub mod blai_act_table_1b_18;
#[doc = "BLAI_ACT_TABLE_1F_1C (rw) register accessor: BLAI Activation Table (Indices 0x1C-0x1F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_1f_1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_1f_1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_1f_1c`] module"]
#[doc(alias = "BLAI_ACT_TABLE_1F_1C")]
pub type BlaiActTable1f1c = crate::Reg<blai_act_table_1f_1c::BlaiActTable1f1cSpec>;
#[doc = "BLAI Activation Table (Indices 0x1C-0x1F)"]
pub mod blai_act_table_1f_1c;
#[doc = "BLAI_ACT_TABLE_23_20 (rw) register accessor: BLAI Activation Table (Indices 0x20-0x23)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_23_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_23_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_23_20`] module"]
#[doc(alias = "BLAI_ACT_TABLE_23_20")]
pub type BlaiActTable23_20 = crate::Reg<blai_act_table_23_20::BlaiActTable23_20Spec>;
#[doc = "BLAI Activation Table (Indices 0x20-0x23)"]
pub mod blai_act_table_23_20;
#[doc = "BLAI_ACT_TABLE_27_24 (rw) register accessor: BLAI Activation Table (Indices 0x24-0x27)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_27_24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_27_24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_27_24`] module"]
#[doc(alias = "BLAI_ACT_TABLE_27_24")]
pub type BlaiActTable27_24 = crate::Reg<blai_act_table_27_24::BlaiActTable27_24Spec>;
#[doc = "BLAI Activation Table (Indices 0x24-0x27)"]
pub mod blai_act_table_27_24;
#[doc = "BLAI_ACT_TABLE_2B_28 (rw) register accessor: BLAI Activation Table (Indices 0x28-0x2B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_2b_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_2b_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_2b_28`] module"]
#[doc(alias = "BLAI_ACT_TABLE_2B_28")]
pub type BlaiActTable2b28 = crate::Reg<blai_act_table_2b_28::BlaiActTable2b28Spec>;
#[doc = "BLAI Activation Table (Indices 0x28-0x2B)"]
pub mod blai_act_table_2b_28;
#[doc = "BLAI_ACT_TABLE_2F_2C (rw) register accessor: BLAI Activation Table (Indices 0x2C-0x2F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_2f_2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_2f_2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_2f_2c`] module"]
#[doc(alias = "BLAI_ACT_TABLE_2F_2C")]
pub type BlaiActTable2f2c = crate::Reg<blai_act_table_2f_2c::BlaiActTable2f2cSpec>;
#[doc = "BLAI Activation Table (Indices 0x2C-0x2F)"]
pub mod blai_act_table_2f_2c;
#[doc = "BLAI_ACT_TABLE_33_30 (rw) register accessor: BLAI Activation Table (Indices 0x30-0x33)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_33_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_33_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_33_30`] module"]
#[doc(alias = "BLAI_ACT_TABLE_33_30")]
pub type BlaiActTable33_30 = crate::Reg<blai_act_table_33_30::BlaiActTable33_30Spec>;
#[doc = "BLAI Activation Table (Indices 0x30-0x33)"]
pub mod blai_act_table_33_30;
#[doc = "BLAI_ACT_TABLE_37_34 (rw) register accessor: BLAI Activation Table (Indices 0x34-0x37)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_37_34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_37_34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_37_34`] module"]
#[doc(alias = "BLAI_ACT_TABLE_37_34")]
pub type BlaiActTable37_34 = crate::Reg<blai_act_table_37_34::BlaiActTable37_34Spec>;
#[doc = "BLAI Activation Table (Indices 0x34-0x37)"]
pub mod blai_act_table_37_34;
#[doc = "BLAI_ACT_TABLE_3B_38 (rw) register accessor: BLAI Activation Table (Indices 0x38-0x3B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_3b_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_3b_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_3b_38`] module"]
#[doc(alias = "BLAI_ACT_TABLE_3B_38")]
pub type BlaiActTable3b38 = crate::Reg<blai_act_table_3b_38::BlaiActTable3b38Spec>;
#[doc = "BLAI Activation Table (Indices 0x38-0x3B)"]
pub mod blai_act_table_3b_38;
#[doc = "BLAI_ACT_TABLE_3F_3C (rw) register accessor: BLAI Activation Table (Indices 0x3C-0x3F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_3f_3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_3f_3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_3f_3c`] module"]
#[doc(alias = "BLAI_ACT_TABLE_3F_3C")]
pub type BlaiActTable3f3c = crate::Reg<blai_act_table_3f_3c::BlaiActTable3f3cSpec>;
#[doc = "BLAI Activation Table (Indices 0x3C-0x3F)"]
pub mod blai_act_table_3f_3c;
#[doc = "BLAI_ACT_TABLE_43_40 (rw) register accessor: BLAI Activation Table (Indices 0x40-0x43)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_43_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_43_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_43_40`] module"]
#[doc(alias = "BLAI_ACT_TABLE_43_40")]
pub type BlaiActTable43_40 = crate::Reg<blai_act_table_43_40::BlaiActTable43_40Spec>;
#[doc = "BLAI Activation Table (Indices 0x40-0x43)"]
pub mod blai_act_table_43_40;
#[doc = "BLAI_ACT_TABLE_47_44 (rw) register accessor: BLAI Activation Table (Indices 0x44-0x47)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_47_44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_47_44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_47_44`] module"]
#[doc(alias = "BLAI_ACT_TABLE_47_44")]
pub type BlaiActTable47_44 = crate::Reg<blai_act_table_47_44::BlaiActTable47_44Spec>;
#[doc = "BLAI Activation Table (Indices 0x44-0x47)"]
pub mod blai_act_table_47_44;
#[doc = "BLAI_ACT_TABLE_4B_48 (rw) register accessor: BLAI Activation Table (Indices 0x48-0x4B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_4b_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_4b_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_4b_48`] module"]
#[doc(alias = "BLAI_ACT_TABLE_4B_48")]
pub type BlaiActTable4b48 = crate::Reg<blai_act_table_4b_48::BlaiActTable4b48Spec>;
#[doc = "BLAI Activation Table (Indices 0x48-0x4B)"]
pub mod blai_act_table_4b_48;
#[doc = "BLAI_ACT_TABLE_4F_4C (rw) register accessor: BLAI Activation Table (Indices 0x4C-0x4F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_4f_4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_4f_4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_4f_4c`] module"]
#[doc(alias = "BLAI_ACT_TABLE_4F_4C")]
pub type BlaiActTable4f4c = crate::Reg<blai_act_table_4f_4c::BlaiActTable4f4cSpec>;
#[doc = "BLAI Activation Table (Indices 0x4C-0x4F)"]
pub mod blai_act_table_4f_4c;
#[doc = "BLAI_ACT_TABLE_53_50 (rw) register accessor: BLAI Activation Table (Indices 0x50-0x53)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_53_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_53_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_53_50`] module"]
#[doc(alias = "BLAI_ACT_TABLE_53_50")]
pub type BlaiActTable53_50 = crate::Reg<blai_act_table_53_50::BlaiActTable53_50Spec>;
#[doc = "BLAI Activation Table (Indices 0x50-0x53)"]
pub mod blai_act_table_53_50;
#[doc = "BLAI_ACT_TABLE_57_54 (rw) register accessor: BLAI Activation Table (Indices 0x54-0x57)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_57_54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_57_54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_57_54`] module"]
#[doc(alias = "BLAI_ACT_TABLE_57_54")]
pub type BlaiActTable57_54 = crate::Reg<blai_act_table_57_54::BlaiActTable57_54Spec>;
#[doc = "BLAI Activation Table (Indices 0x54-0x57)"]
pub mod blai_act_table_57_54;
#[doc = "BLAI_ACT_TABLE_5B_58 (rw) register accessor: BLAI Activation Table (Indices 0x58-0x5B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_5b_58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_5b_58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_5b_58`] module"]
#[doc(alias = "BLAI_ACT_TABLE_5B_58")]
pub type BlaiActTable5b58 = crate::Reg<blai_act_table_5b_58::BlaiActTable5b58Spec>;
#[doc = "BLAI Activation Table (Indices 0x58-0x5B)"]
pub mod blai_act_table_5b_58;
#[doc = "BLAI_ACT_TABLE_5F_5C (rw) register accessor: BLAI Activation Table (Indices 0x5C-0x5F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_5f_5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_5f_5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_5f_5c`] module"]
#[doc(alias = "BLAI_ACT_TABLE_5F_5C")]
pub type BlaiActTable5f5c = crate::Reg<blai_act_table_5f_5c::BlaiActTable5f5cSpec>;
#[doc = "BLAI Activation Table (Indices 0x5C-0x5F)"]
pub mod blai_act_table_5f_5c;
#[doc = "BLAI_ACT_TABLE_63_60 (rw) register accessor: BLAI Activation Table (Indices 0x60-0x63)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_63_60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_63_60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_63_60`] module"]
#[doc(alias = "BLAI_ACT_TABLE_63_60")]
pub type BlaiActTable63_60 = crate::Reg<blai_act_table_63_60::BlaiActTable63_60Spec>;
#[doc = "BLAI Activation Table (Indices 0x60-0x63)"]
pub mod blai_act_table_63_60;
#[doc = "BLAI_ACT_TABLE_67_64 (rw) register accessor: BLAI Activation Table (Indices 0x64-0x67)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_67_64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_67_64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_67_64`] module"]
#[doc(alias = "BLAI_ACT_TABLE_67_64")]
pub type BlaiActTable67_64 = crate::Reg<blai_act_table_67_64::BlaiActTable67_64Spec>;
#[doc = "BLAI Activation Table (Indices 0x64-0x67)"]
pub mod blai_act_table_67_64;
#[doc = "BLAI_ACT_TABLE_6B_68 (rw) register accessor: BLAI Activation Table (Indices 0x68-0x6B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_6b_68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_6b_68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_6b_68`] module"]
#[doc(alias = "BLAI_ACT_TABLE_6B_68")]
pub type BlaiActTable6b68 = crate::Reg<blai_act_table_6b_68::BlaiActTable6b68Spec>;
#[doc = "BLAI Activation Table (Indices 0x68-0x6B)"]
pub mod blai_act_table_6b_68;
#[doc = "BLAI_ACT_TABLE_6F_6C (rw) register accessor: BLAI Activation Table (Indices 0x6C-0x6F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_6f_6c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_6f_6c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_6f_6c`] module"]
#[doc(alias = "BLAI_ACT_TABLE_6F_6C")]
pub type BlaiActTable6f6c = crate::Reg<blai_act_table_6f_6c::BlaiActTable6f6cSpec>;
#[doc = "BLAI Activation Table (Indices 0x6C-0x6F)"]
pub mod blai_act_table_6f_6c;
#[doc = "BLAI_ACT_TABLE_73_70 (rw) register accessor: BLAI Activation Table (Indices 0x70-0x73)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_73_70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_73_70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_73_70`] module"]
#[doc(alias = "BLAI_ACT_TABLE_73_70")]
pub type BlaiActTable73_70 = crate::Reg<blai_act_table_73_70::BlaiActTable73_70Spec>;
#[doc = "BLAI Activation Table (Indices 0x70-0x73)"]
pub mod blai_act_table_73_70;
#[doc = "BLAI_ACT_TABLE_77_74 (rw) register accessor: BLAI Activation Table (Indices 0x74-0x77)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_77_74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_77_74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_77_74`] module"]
#[doc(alias = "BLAI_ACT_TABLE_77_74")]
pub type BlaiActTable77_74 = crate::Reg<blai_act_table_77_74::BlaiActTable77_74Spec>;
#[doc = "BLAI Activation Table (Indices 0x74-0x77)"]
pub mod blai_act_table_77_74;
#[doc = "BLAI_ACT_TABLE_7B_78 (rw) register accessor: BLAI Activation Table (Indices 0x78-0x7B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_7b_78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_7b_78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_7b_78`] module"]
#[doc(alias = "BLAI_ACT_TABLE_7B_78")]
pub type BlaiActTable7b78 = crate::Reg<blai_act_table_7b_78::BlaiActTable7b78Spec>;
#[doc = "BLAI Activation Table (Indices 0x78-0x7B)"]
pub mod blai_act_table_7b_78;
#[doc = "BLAI_ACT_TABLE_7F_7C (rw) register accessor: BLAI Activation Table (Indices 0x7C-0x7F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_7f_7c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_7f_7c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_7f_7c`] module"]
#[doc(alias = "BLAI_ACT_TABLE_7F_7C")]
pub type BlaiActTable7f7c = crate::Reg<blai_act_table_7f_7c::BlaiActTable7f7cSpec>;
#[doc = "BLAI Activation Table (Indices 0x7C-0x7F)"]
pub mod blai_act_table_7f_7c;
#[doc = "BLAI_ACT_TABLE_83_80 (rw) register accessor: BLAI Activation Table (Indices 0x80-0x83)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_83_80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_83_80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_83_80`] module"]
#[doc(alias = "BLAI_ACT_TABLE_83_80")]
pub type BlaiActTable83_80 = crate::Reg<blai_act_table_83_80::BlaiActTable83_80Spec>;
#[doc = "BLAI Activation Table (Indices 0x80-0x83)"]
pub mod blai_act_table_83_80;
#[doc = "BLAI_ACT_TABLE_87_84 (rw) register accessor: BLAI Activation Table (Indices 0x84-0x87)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_87_84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_87_84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_87_84`] module"]
#[doc(alias = "BLAI_ACT_TABLE_87_84")]
pub type BlaiActTable87_84 = crate::Reg<blai_act_table_87_84::BlaiActTable87_84Spec>;
#[doc = "BLAI Activation Table (Indices 0x84-0x87)"]
pub mod blai_act_table_87_84;
#[doc = "BLAI_ACT_TABLE_8B_88 (rw) register accessor: BLAI Activation Table (Indices 0x88-0x8B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_8b_88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_8b_88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_8b_88`] module"]
#[doc(alias = "BLAI_ACT_TABLE_8B_88")]
pub type BlaiActTable8b88 = crate::Reg<blai_act_table_8b_88::BlaiActTable8b88Spec>;
#[doc = "BLAI Activation Table (Indices 0x88-0x8B)"]
pub mod blai_act_table_8b_88;
#[doc = "BLAI_ACT_TABLE_8F_8C (rw) register accessor: BLAI Activation Table (Indices 0x8C-0x8F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_8f_8c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_8f_8c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_8f_8c`] module"]
#[doc(alias = "BLAI_ACT_TABLE_8F_8C")]
pub type BlaiActTable8f8c = crate::Reg<blai_act_table_8f_8c::BlaiActTable8f8cSpec>;
#[doc = "BLAI Activation Table (Indices 0x8C-0x8F)"]
pub mod blai_act_table_8f_8c;
#[doc = "BLAI_ACT_TABLE_93_90 (rw) register accessor: BLAI Activation Table (Indices 0x90-0x93)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_93_90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_93_90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_93_90`] module"]
#[doc(alias = "BLAI_ACT_TABLE_93_90")]
pub type BlaiActTable93_90 = crate::Reg<blai_act_table_93_90::BlaiActTable93_90Spec>;
#[doc = "BLAI Activation Table (Indices 0x90-0x93)"]
pub mod blai_act_table_93_90;
#[doc = "BLAI_ACT_TABLE_97_94 (rw) register accessor: BLAI Activation Table (Indices 0x94-0x97)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_97_94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_97_94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_97_94`] module"]
#[doc(alias = "BLAI_ACT_TABLE_97_94")]
pub type BlaiActTable97_94 = crate::Reg<blai_act_table_97_94::BlaiActTable97_94Spec>;
#[doc = "BLAI Activation Table (Indices 0x94-0x97)"]
pub mod blai_act_table_97_94;
#[doc = "BLAI_ACT_TABLE_9B_98 (rw) register accessor: BLAI Activation Table (Indices 0x98-0x9B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_9b_98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_9b_98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_9b_98`] module"]
#[doc(alias = "BLAI_ACT_TABLE_9B_98")]
pub type BlaiActTable9b98 = crate::Reg<blai_act_table_9b_98::BlaiActTable9b98Spec>;
#[doc = "BLAI Activation Table (Indices 0x98-0x9B)"]
pub mod blai_act_table_9b_98;
#[doc = "BLAI_ACT_TABLE_9F_9C (rw) register accessor: BLAI Activation Table (Indices 0x9C-0x9F)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_9f_9c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_9f_9c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_9f_9c`] module"]
#[doc(alias = "BLAI_ACT_TABLE_9F_9C")]
pub type BlaiActTable9f9c = crate::Reg<blai_act_table_9f_9c::BlaiActTable9f9cSpec>;
#[doc = "BLAI Activation Table (Indices 0x9C-0x9F)"]
pub mod blai_act_table_9f_9c;
#[doc = "BLAI_ACT_TABLE_A3_A0 (rw) register accessor: BLAI Activation Table (Indices 0xA0-0xA3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_a3_a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_a3_a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_a3_a0`] module"]
#[doc(alias = "BLAI_ACT_TABLE_A3_A0")]
pub type BlaiActTableA3A0 = crate::Reg<blai_act_table_a3_a0::BlaiActTableA3A0Spec>;
#[doc = "BLAI Activation Table (Indices 0xA0-0xA3)"]
pub mod blai_act_table_a3_a0;
#[doc = "BLAI_ACT_TABLE_A7_A4 (rw) register accessor: BLAI Activation Table (Indices 0xA4-0xA7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_a7_a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_a7_a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_a7_a4`] module"]
#[doc(alias = "BLAI_ACT_TABLE_A7_A4")]
pub type BlaiActTableA7A4 = crate::Reg<blai_act_table_a7_a4::BlaiActTableA7A4Spec>;
#[doc = "BLAI Activation Table (Indices 0xA4-0xA7)"]
pub mod blai_act_table_a7_a4;
#[doc = "BLAI_ACT_TABLE_AB_A8 (rw) register accessor: BLAI Activation Table (Indices 0xA8-0xAB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_ab_a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_ab_a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_ab_a8`] module"]
#[doc(alias = "BLAI_ACT_TABLE_AB_A8")]
pub type BlaiActTableAbA8 = crate::Reg<blai_act_table_ab_a8::BlaiActTableAbA8Spec>;
#[doc = "BLAI Activation Table (Indices 0xA8-0xAB)"]
pub mod blai_act_table_ab_a8;
#[doc = "BLAI_ACT_TABLE_AF_AC (rw) register accessor: BLAI Activation Table (Indices 0xAC-0xAF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_af_ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_af_ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_af_ac`] module"]
#[doc(alias = "BLAI_ACT_TABLE_AF_AC")]
pub type BlaiActTableAfAc = crate::Reg<blai_act_table_af_ac::BlaiActTableAfAcSpec>;
#[doc = "BLAI Activation Table (Indices 0xAC-0xAF)"]
pub mod blai_act_table_af_ac;
#[doc = "BLAI_ACT_TABLE_B3_B0 (rw) register accessor: BLAI Activation Table (Indices 0xB0-0xB3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_b3_b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_b3_b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_b3_b0`] module"]
#[doc(alias = "BLAI_ACT_TABLE_B3_B0")]
pub type BlaiActTableB3B0 = crate::Reg<blai_act_table_b3_b0::BlaiActTableB3B0Spec>;
#[doc = "BLAI Activation Table (Indices 0xB0-0xB3)"]
pub mod blai_act_table_b3_b0;
#[doc = "BLAI_ACT_TABLE_B7_B4 (rw) register accessor: BLAI Activation Table (Indices 0xB4-0xB7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_b7_b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_b7_b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_b7_b4`] module"]
#[doc(alias = "BLAI_ACT_TABLE_B7_B4")]
pub type BlaiActTableB7B4 = crate::Reg<blai_act_table_b7_b4::BlaiActTableB7B4Spec>;
#[doc = "BLAI Activation Table (Indices 0xB4-0xB7)"]
pub mod blai_act_table_b7_b4;
#[doc = "BLAI_ACT_TABLE_BB_B8 (rw) register accessor: BLAI Activation Table (Indices 0xB8-0xBB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_bb_b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_bb_b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_bb_b8`] module"]
#[doc(alias = "BLAI_ACT_TABLE_BB_B8")]
pub type BlaiActTableBbB8 = crate::Reg<blai_act_table_bb_b8::BlaiActTableBbB8Spec>;
#[doc = "BLAI Activation Table (Indices 0xB8-0xBB)"]
pub mod blai_act_table_bb_b8;
#[doc = "BLAI_ACT_TABLE_BF_BC (rw) register accessor: BLAI Activation Table (Indices 0xBC-0xBF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_bf_bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_bf_bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_bf_bc`] module"]
#[doc(alias = "BLAI_ACT_TABLE_BF_BC")]
pub type BlaiActTableBfBc = crate::Reg<blai_act_table_bf_bc::BlaiActTableBfBcSpec>;
#[doc = "BLAI Activation Table (Indices 0xBC-0xBF)"]
pub mod blai_act_table_bf_bc;
#[doc = "BLAI_ACT_TABLE_C3_C0 (rw) register accessor: BLAI Activation Table (Indices 0xC0-0xC3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_c3_c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_c3_c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_c3_c0`] module"]
#[doc(alias = "BLAI_ACT_TABLE_C3_C0")]
pub type BlaiActTableC3C0 = crate::Reg<blai_act_table_c3_c0::BlaiActTableC3C0Spec>;
#[doc = "BLAI Activation Table (Indices 0xC0-0xC3)"]
pub mod blai_act_table_c3_c0;
#[doc = "BLAI_ACT_TABLE_C7_C4 (rw) register accessor: BLAI Activation Table (Indices 0xC4-0xC7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_c7_c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_c7_c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_c7_c4`] module"]
#[doc(alias = "BLAI_ACT_TABLE_C7_C4")]
pub type BlaiActTableC7C4 = crate::Reg<blai_act_table_c7_c4::BlaiActTableC7C4Spec>;
#[doc = "BLAI Activation Table (Indices 0xC4-0xC7)"]
pub mod blai_act_table_c7_c4;
#[doc = "BLAI_ACT_TABLE_CB_C8 (rw) register accessor: BLAI Activation Table (Indices 0xC8-0xCB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_cb_c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_cb_c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_cb_c8`] module"]
#[doc(alias = "BLAI_ACT_TABLE_CB_C8")]
pub type BlaiActTableCbC8 = crate::Reg<blai_act_table_cb_c8::BlaiActTableCbC8Spec>;
#[doc = "BLAI Activation Table (Indices 0xC8-0xCB)"]
pub mod blai_act_table_cb_c8;
#[doc = "BLAI_ACT_TABLE_CF_CC (rw) register accessor: BLAI Activation Table (Indices 0xCC-0xCF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_cf_cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_cf_cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_cf_cc`] module"]
#[doc(alias = "BLAI_ACT_TABLE_CF_CC")]
pub type BlaiActTableCfCc = crate::Reg<blai_act_table_cf_cc::BlaiActTableCfCcSpec>;
#[doc = "BLAI Activation Table (Indices 0xCC-0xCF)"]
pub mod blai_act_table_cf_cc;
#[doc = "BLAI_ACT_TABLE_D3_D0 (rw) register accessor: BLAI Activation Table (Indices 0xD0-0xD3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_d3_d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_d3_d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_d3_d0`] module"]
#[doc(alias = "BLAI_ACT_TABLE_D3_D0")]
pub type BlaiActTableD3D0 = crate::Reg<blai_act_table_d3_d0::BlaiActTableD3D0Spec>;
#[doc = "BLAI Activation Table (Indices 0xD0-0xD3)"]
pub mod blai_act_table_d3_d0;
#[doc = "BLAI_ACT_TABLE_D7_D4 (rw) register accessor: BLAI Activation Table (Indices 0xD4-0xD7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_d7_d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_d7_d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_d7_d4`] module"]
#[doc(alias = "BLAI_ACT_TABLE_D7_D4")]
pub type BlaiActTableD7D4 = crate::Reg<blai_act_table_d7_d4::BlaiActTableD7D4Spec>;
#[doc = "BLAI Activation Table (Indices 0xD4-0xD7)"]
pub mod blai_act_table_d7_d4;
#[doc = "BLAI_ACT_TABLE_DB_D8 (rw) register accessor: BLAI Activation Table (Indices 0xD8-0xDB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_db_d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_db_d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_db_d8`] module"]
#[doc(alias = "BLAI_ACT_TABLE_DB_D8")]
pub type BlaiActTableDbD8 = crate::Reg<blai_act_table_db_d8::BlaiActTableDbD8Spec>;
#[doc = "BLAI Activation Table (Indices 0xD8-0xDB)"]
pub mod blai_act_table_db_d8;
#[doc = "BLAI_ACT_TABLE_DF_DC (rw) register accessor: BLAI Activation Table (Indices 0xDC-0xDF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_df_dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_df_dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_df_dc`] module"]
#[doc(alias = "BLAI_ACT_TABLE_DF_DC")]
pub type BlaiActTableDfDc = crate::Reg<blai_act_table_df_dc::BlaiActTableDfDcSpec>;
#[doc = "BLAI Activation Table (Indices 0xDC-0xDF)"]
pub mod blai_act_table_df_dc;
#[doc = "BLAI_ACT_TABLE_E3_E0 (rw) register accessor: BLAI Activation Table (Indices 0xE0-0xE3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_e3_e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_e3_e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_e3_e0`] module"]
#[doc(alias = "BLAI_ACT_TABLE_E3_E0")]
pub type BlaiActTableE3E0 = crate::Reg<blai_act_table_e3_e0::BlaiActTableE3E0Spec>;
#[doc = "BLAI Activation Table (Indices 0xE0-0xE3)"]
pub mod blai_act_table_e3_e0;
#[doc = "BLAI_ACT_TABLE_E7_E4 (rw) register accessor: BLAI Activation Table (Indices 0xE4-0xE7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_e7_e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_e7_e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_e7_e4`] module"]
#[doc(alias = "BLAI_ACT_TABLE_E7_E4")]
pub type BlaiActTableE7E4 = crate::Reg<blai_act_table_e7_e4::BlaiActTableE7E4Spec>;
#[doc = "BLAI Activation Table (Indices 0xE4-0xE7)"]
pub mod blai_act_table_e7_e4;
#[doc = "BLAI_ACT_TABLE_EB_E8 (rw) register accessor: BLAI Activation Table (Indices 0xE8-0xEB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_eb_e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_eb_e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_eb_e8`] module"]
#[doc(alias = "BLAI_ACT_TABLE_EB_E8")]
pub type BlaiActTableEbE8 = crate::Reg<blai_act_table_eb_e8::BlaiActTableEbE8Spec>;
#[doc = "BLAI Activation Table (Indices 0xE8-0xEB)"]
pub mod blai_act_table_eb_e8;
#[doc = "BLAI_ACT_TABLE_EF_EC (rw) register accessor: BLAI Activation Table (Indices 0xEC-0xEF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_ef_ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_ef_ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_ef_ec`] module"]
#[doc(alias = "BLAI_ACT_TABLE_EF_EC")]
pub type BlaiActTableEfEc = crate::Reg<blai_act_table_ef_ec::BlaiActTableEfEcSpec>;
#[doc = "BLAI Activation Table (Indices 0xEC-0xEF)"]
pub mod blai_act_table_ef_ec;
#[doc = "BLAI_ACT_TABLE_F3_F0 (rw) register accessor: BLAI Activation Table (Indices 0xF0-0xF3)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_f3_f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_f3_f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_f3_f0`] module"]
#[doc(alias = "BLAI_ACT_TABLE_F3_F0")]
pub type BlaiActTableF3F0 = crate::Reg<blai_act_table_f3_f0::BlaiActTableF3F0Spec>;
#[doc = "BLAI Activation Table (Indices 0xF0-0xF3)"]
pub mod blai_act_table_f3_f0;
#[doc = "BLAI_ACT_TABLE_F7_F4 (rw) register accessor: BLAI Activation Table (Indices 0xF4-0xF7)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_f7_f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_f7_f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_f7_f4`] module"]
#[doc(alias = "BLAI_ACT_TABLE_F7_F4")]
pub type BlaiActTableF7F4 = crate::Reg<blai_act_table_f7_f4::BlaiActTableF7F4Spec>;
#[doc = "BLAI Activation Table (Indices 0xF4-0xF7)"]
pub mod blai_act_table_f7_f4;
#[doc = "BLAI_ACT_TABLE_FB_F8 (rw) register accessor: BLAI Activation Table (Indices 0xF8-0xFB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_fb_f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_fb_f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_fb_f8`] module"]
#[doc(alias = "BLAI_ACT_TABLE_FB_F8")]
pub type BlaiActTableFbF8 = crate::Reg<blai_act_table_fb_f8::BlaiActTableFbF8Spec>;
#[doc = "BLAI Activation Table (Indices 0xF8-0xFB)"]
pub mod blai_act_table_fb_f8;
#[doc = "BLAI_ACT_TABLE_FF_FC (rw) register accessor: BLAI Activation Table (Indices 0xFC-0xFF)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_ff_fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_ff_fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blai_act_table_ff_fc`] module"]
#[doc(alias = "BLAI_ACT_TABLE_FF_FC")]
pub type BlaiActTableFfFc = crate::Reg<blai_act_table_ff_fc::BlaiActTableFfFcSpec>;
#[doc = "BLAI Activation Table (Indices 0xFC-0xFF)"]
pub mod blai_act_table_ff_fc;
