#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hccap: Hccap,
    hcsparams: Hcsparams,
    hccparams: Hccparams,
    _reserved3: [u8; 0x04],
    usbcmd: Usbcmd,
    usbsts: Usbsts,
    usbintr: Usbintr,
    frindex: Frindex,
    _reserved7: [u8; 0x04],
    periodiclistbase: Periodiclistbase,
    asynclistaddr: Asynclistaddr,
    _reserved9: [u8; 0x04],
    portsc: Portsc,
    _reserved10: [u8; 0x0c],
    hcmisc: Hcmisc,
    fs_eof: FsEof,
    hs_eof: HsEof,
    _reserved13: [u8; 0x34],
    otg_csr: OtgCsr,
    otg_isr: OtgIsr,
    otg_ier: OtgIer,
    _reserved16: [u8; 0x34],
    glb_isr: GlbIsr,
    glb_int: GlbInt,
    _reserved18: [u8; 0x18],
    revision: Revision,
    feature: Feature,
    axi_cr: AxiCr,
    _reserved21: [u8; 0x14],
    dev_ctl: DevCtl,
    dev_adr: DevAdr,
    dev_tst: DevTst,
    dev_sfn: DevSfn,
    dev_smt: DevSmt,
    phy_tst: PhyTst,
    dev_vctl: DevVctl,
    dev_cxcfg: DevCxcfg,
    dev_cxcfe: DevCxcfe,
    dev_icr: DevIcr,
    _reserved31: [u8; 0x08],
    dev_migr: DevMigr,
    dev_misg0: DevMisg0,
    dev_misg1: DevMisg1,
    dev_misg2: DevMisg2,
    dev_igr: DevIgr,
    dev_isg0: DevIsg0,
    dev_isg1: DevIsg1,
    dev_isg2: DevIsg2,
    dev_rxz: DevRxz,
    dev_txz: DevTxz,
    dev_ise: DevIse,
    _reserved42: [u8; 0x04],
    dev_inmps1: DevInmps1,
    dev_inmps2: DevInmps2,
    dev_inmps3: DevInmps3,
    dev_inmps4: DevInmps4,
    dev_inmps5: DevInmps5,
    dev_inmps6: DevInmps6,
    dev_inmps7: DevInmps7,
    dev_inmps8: DevInmps8,
    dev_outmps1: DevOutmps1,
    dev_outmps2: DevOutmps2,
    dev_outmps3: DevOutmps3,
    dev_outmps4: DevOutmps4,
    dev_outmps5: DevOutmps5,
    dev_outmps6: DevOutmps6,
    dev_outmps7: DevOutmps7,
    dev_outmps8: DevOutmps8,
    dev_epmap0: DevEpmap0,
    dev_epmap1: DevEpmap1,
    dev_fmap: DevFmap,
    dev_fcfg: DevFcfg,
    dev_fibc0: DevFibc0,
    dev_fibc1: DevFibc1,
    dev_fibc2: DevFibc2,
    dev_fibc3: DevFibc3,
    dma_tfn: DmaTfn,
    dma_cps0: DmaCps0,
    dma_cps1: DmaCps1,
    dma_cps2: DmaCps2,
    dma_cps3: DmaCps3,
    dma_cps4: DmaCps4,
    dev_fmap2: DevFmap2,
    dev_fcfg2: DevFcfg2,
    dev_fmap3: DevFmap3,
    dev_fcfg3: DevFcfg3,
    dev_fmap4: DevFmap4,
    dev_fcfg4: DevFcfg4,
    dev_fibc4: DevFibc4,
    dev_fibc5: DevFibc5,
    dev_fibc6: DevFibc6,
    dev_fibc7: DevFibc7,
    _reserved82: [u8; 0x0100],
    vdma_cxfps1: VdmaCxfps1,
    vdma_cxfps2: VdmaCxfps2,
    vdma_f0ps1: VdmaF0ps1,
    vdma_f0ps2: VdmaF0ps2,
    vdma_f1ps1: VdmaF1ps1,
    vdma_f1ps2: VdmaF1ps2,
    vdma_f2ps1: VdmaF2ps1,
    vdma_f2ps2: VdmaF2ps2,
    vdma_f3ps1: VdmaF3ps1,
    vdma_f3ps2: VdmaF3ps2,
    dev_isg3: DevIsg3,
    dev_misg3: DevMisg3,
    vdma_ctrl: VdmaCtrl,
    lpm_cap: LpmCap,
    dev_isg4: DevIsg4,
    dev_misg4: DevMisg4,
    _reserved98: [u8; 0x10],
    vdma_fn_ps1: VdmaFnPs1,
    vdma_fn_ps2: VdmaFnPs2,
}
impl RegisterBlock {
    #[doc = "0x00 - Host Controller Capability Register"]
    #[inline(always)]
    pub const fn hccap(&self) -> &Hccap {
        &self.hccap
    }
    #[doc = "0x04 - Host Controller Structural Parameters"]
    #[inline(always)]
    pub const fn hcsparams(&self) -> &Hcsparams {
        &self.hcsparams
    }
    #[doc = "0x08 - Host Controller Capability Parameters"]
    #[inline(always)]
    pub const fn hccparams(&self) -> &Hccparams {
        &self.hccparams
    }
    #[doc = "0x10 - USB Command Register"]
    #[inline(always)]
    pub const fn usbcmd(&self) -> &Usbcmd {
        &self.usbcmd
    }
    #[doc = "0x14 - USB Status Register"]
    #[inline(always)]
    pub const fn usbsts(&self) -> &Usbsts {
        &self.usbsts
    }
    #[doc = "0x18 - USB Interrupt Enable Register"]
    #[inline(always)]
    pub const fn usbintr(&self) -> &Usbintr {
        &self.usbintr
    }
    #[doc = "0x1c - Frame Index Register"]
    #[inline(always)]
    pub const fn frindex(&self) -> &Frindex {
        &self.frindex
    }
    #[doc = "0x24 - Periodic Frame List Base Address"]
    #[inline(always)]
    pub const fn periodiclistbase(&self) -> &Periodiclistbase {
        &self.periodiclistbase
    }
    #[doc = "0x28 - Asynchronous List Base Address"]
    #[inline(always)]
    pub const fn asynclistaddr(&self) -> &Asynclistaddr {
        &self.asynclistaddr
    }
    #[doc = "0x30 - Port Status and Control Register"]
    #[inline(always)]
    pub const fn portsc(&self) -> &Portsc {
        &self.portsc
    }
    #[doc = "0x40 - Host Controller Miscellaneous Control"]
    #[inline(always)]
    pub const fn hcmisc(&self) -> &Hcmisc {
        &self.hcmisc
    }
    #[doc = "0x44 - Full Speed EOF Control and Host SM Status"]
    #[inline(always)]
    pub const fn fs_eof(&self) -> &FsEof {
        &self.fs_eof
    }
    #[doc = "0x48 - High Speed EOF Control and Host SM Status"]
    #[inline(always)]
    pub const fn hs_eof(&self) -> &HsEof {
        &self.hs_eof
    }
    #[doc = "0x80 - OTG Control and Status Register"]
    #[inline(always)]
    pub const fn otg_csr(&self) -> &OtgCsr {
        &self.otg_csr
    }
    #[doc = "0x84 - OTG Interrupt Status Register"]
    #[inline(always)]
    pub const fn otg_isr(&self) -> &OtgIsr {
        &self.otg_isr
    }
    #[doc = "0x88 - OTG Interrupt Enable Register"]
    #[inline(always)]
    pub const fn otg_ier(&self) -> &OtgIer {
        &self.otg_ier
    }
    #[doc = "0xc0 - Global Interrupt Status Register"]
    #[inline(always)]
    pub const fn glb_isr(&self) -> &GlbIsr {
        &self.glb_isr
    }
    #[doc = "0xc4 - Global Interrupt Mask Register"]
    #[inline(always)]
    pub const fn glb_int(&self) -> &GlbInt {
        &self.glb_int
    }
    #[doc = "0xe0 - USB Revision Register"]
    #[inline(always)]
    pub const fn revision(&self) -> &Revision {
        &self.revision
    }
    #[doc = "0xe4 - USB Feature Register"]
    #[inline(always)]
    pub const fn feature(&self) -> &Feature {
        &self.feature
    }
    #[doc = "0xe8 - AXI Control Register"]
    #[inline(always)]
    pub const fn axi_cr(&self) -> &AxiCr {
        &self.axi_cr
    }
    #[doc = "0x100 - Device Control Register"]
    #[inline(always)]
    pub const fn dev_ctl(&self) -> &DevCtl {
        &self.dev_ctl
    }
    #[doc = "0x104 - Device Address Register"]
    #[inline(always)]
    pub const fn dev_adr(&self) -> &DevAdr {
        &self.dev_adr
    }
    #[doc = "0x108 - Device Test Control"]
    #[inline(always)]
    pub const fn dev_tst(&self) -> &DevTst {
        &self.dev_tst
    }
    #[doc = "0x10c - Device SOF Frame Number"]
    #[inline(always)]
    pub const fn dev_sfn(&self) -> &DevSfn {
        &self.dev_sfn
    }
    #[doc = "0x110 - Device SOF Micro Frame Timer"]
    #[inline(always)]
    pub const fn dev_smt(&self) -> &DevSmt {
        &self.dev_smt
    }
    #[doc = "0x114 - PHY Test Control"]
    #[inline(always)]
    pub const fn phy_tst(&self) -> &PhyTst {
        &self.phy_tst
    }
    #[doc = "0x118 - Device VBUS Control"]
    #[inline(always)]
    pub const fn dev_vctl(&self) -> &DevVctl {
        &self.dev_vctl
    }
    #[doc = "0x11c - Device Control Endpoint Configuration"]
    #[inline(always)]
    pub const fn dev_cxcfg(&self) -> &DevCxcfg {
        &self.dev_cxcfg
    }
    #[doc = "0x120 - Device Control Endpoint FIFO Status"]
    #[inline(always)]
    pub const fn dev_cxcfe(&self) -> &DevCxcfe {
        &self.dev_cxcfe
    }
    #[doc = "0x124 - Device Idle Counter"]
    #[inline(always)]
    pub const fn dev_icr(&self) -> &DevIcr {
        &self.dev_icr
    }
    #[doc = "0x130 - Device Master Interrupt Group Register"]
    #[inline(always)]
    pub const fn dev_migr(&self) -> &DevMigr {
        &self.dev_migr
    }
    #[doc = "0x134 - Device Masked Interrupt Status Group 0 (Control EP)"]
    #[inline(always)]
    pub const fn dev_misg0(&self) -> &DevMisg0 {
        &self.dev_misg0
    }
    #[doc = "0x138 - Device Masked Interrupt Status Group 1 (FIFO 0-7)"]
    #[inline(always)]
    pub const fn dev_misg1(&self) -> &DevMisg1 {
        &self.dev_misg1
    }
    #[doc = "0x13c - Device Masked Interrupt Status Group 2 (Events/DMA/FIFO 8-15)"]
    #[inline(always)]
    pub const fn dev_misg2(&self) -> &DevMisg2 {
        &self.dev_misg2
    }
    #[doc = "0x140 - Device Interrupt Group Register (Status)"]
    #[inline(always)]
    pub const fn dev_igr(&self) -> &DevIgr {
        &self.dev_igr
    }
    #[doc = "0x144 - Device Interrupt Status Group 0 (Control EP)"]
    #[inline(always)]
    pub const fn dev_isg0(&self) -> &DevIsg0 {
        &self.dev_isg0
    }
    #[doc = "0x148 - Device Interrupt Status Group 1 (FIFO 0-15)"]
    #[inline(always)]
    pub const fn dev_isg1(&self) -> &DevIsg1 {
        &self.dev_isg1
    }
    #[doc = "0x14c - Device Interrupt Status Group 2 (Events/DMA/FIFO 8-15)"]
    #[inline(always)]
    pub const fn dev_isg2(&self) -> &DevIsg2 {
        &self.dev_isg2
    }
    #[doc = "0x150 - Device RX 0 Byte Status"]
    #[inline(always)]
    pub const fn dev_rxz(&self) -> &DevRxz {
        &self.dev_rxz
    }
    #[doc = "0x154 - Device TX 0 Byte Status"]
    #[inline(always)]
    pub const fn dev_txz(&self) -> &DevTxz {
        &self.dev_txz
    }
    #[doc = "0x158 - Device Isochronous Sequence Error Status"]
    #[inline(always)]
    pub const fn dev_ise(&self) -> &DevIse {
        &self.dev_ise
    }
    #[doc = "0x160 - Device IN Endpoint 1 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_inmps1(&self) -> &DevInmps1 {
        &self.dev_inmps1
    }
    #[doc = "0x164 - Device IN Endpoint 2 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_inmps2(&self) -> &DevInmps2 {
        &self.dev_inmps2
    }
    #[doc = "0x168 - Device IN Endpoint 3 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_inmps3(&self) -> &DevInmps3 {
        &self.dev_inmps3
    }
    #[doc = "0x16c - Device IN Endpoint 4 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_inmps4(&self) -> &DevInmps4 {
        &self.dev_inmps4
    }
    #[doc = "0x170 - Device IN Endpoint 5 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_inmps5(&self) -> &DevInmps5 {
        &self.dev_inmps5
    }
    #[doc = "0x174 - Device IN Endpoint 6 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_inmps6(&self) -> &DevInmps6 {
        &self.dev_inmps6
    }
    #[doc = "0x178 - Device IN Endpoint 7 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_inmps7(&self) -> &DevInmps7 {
        &self.dev_inmps7
    }
    #[doc = "0x17c - Device IN Endpoint 8 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_inmps8(&self) -> &DevInmps8 {
        &self.dev_inmps8
    }
    #[doc = "0x180 - Device OUT Endpoint 1 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_outmps1(&self) -> &DevOutmps1 {
        &self.dev_outmps1
    }
    #[doc = "0x184 - Device OUT Endpoint 2 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_outmps2(&self) -> &DevOutmps2 {
        &self.dev_outmps2
    }
    #[doc = "0x188 - Device OUT Endpoint 3 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_outmps3(&self) -> &DevOutmps3 {
        &self.dev_outmps3
    }
    #[doc = "0x18c - Device OUT Endpoint 4 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_outmps4(&self) -> &DevOutmps4 {
        &self.dev_outmps4
    }
    #[doc = "0x190 - Device OUT Endpoint 5 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_outmps5(&self) -> &DevOutmps5 {
        &self.dev_outmps5
    }
    #[doc = "0x194 - Device OUT Endpoint 6 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_outmps6(&self) -> &DevOutmps6 {
        &self.dev_outmps6
    }
    #[doc = "0x198 - Device OUT Endpoint 7 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_outmps7(&self) -> &DevOutmps7 {
        &self.dev_outmps7
    }
    #[doc = "0x19c - Device OUT Endpoint 8 Max Packet Size"]
    #[inline(always)]
    pub const fn dev_outmps8(&self) -> &DevOutmps8 {
        &self.dev_outmps8
    }
    #[doc = "0x1a0 - Device Endpoint/FIFO Mapping 0 (EP1-EP4)"]
    #[inline(always)]
    pub const fn dev_epmap0(&self) -> &DevEpmap0 {
        &self.dev_epmap0
    }
    #[doc = "0x1a4 - Device Endpoint/FIFO Mapping 1 (EP5-EP8)"]
    #[inline(always)]
    pub const fn dev_epmap1(&self) -> &DevEpmap1 {
        &self.dev_epmap1
    }
    #[doc = "0x1a8 - Device FIFO Mapping (FIFO 0-3)"]
    #[inline(always)]
    pub const fn dev_fmap(&self) -> &DevFmap {
        &self.dev_fmap
    }
    #[doc = "0x1ac - Device FIFO Configuration (FIFO 0-3)"]
    #[inline(always)]
    pub const fn dev_fcfg(&self) -> &DevFcfg {
        &self.dev_fcfg
    }
    #[doc = "0x1b0 - Device FIFO Byte Count 0"]
    #[inline(always)]
    pub const fn dev_fibc0(&self) -> &DevFibc0 {
        &self.dev_fibc0
    }
    #[doc = "0x1b4 - Device FIFO Byte Count 1"]
    #[inline(always)]
    pub const fn dev_fibc1(&self) -> &DevFibc1 {
        &self.dev_fibc1
    }
    #[doc = "0x1b8 - Device FIFO Byte Count 2"]
    #[inline(always)]
    pub const fn dev_fibc2(&self) -> &DevFibc2 {
        &self.dev_fibc2
    }
    #[doc = "0x1bc - Device FIFO Byte Count 3"]
    #[inline(always)]
    pub const fn dev_fibc3(&self) -> &DevFibc3 {
        &self.dev_fibc3
    }
    #[doc = "0x1c0 - DMA Toggle/Frame Number"]
    #[inline(always)]
    pub const fn dma_tfn(&self) -> &DmaTfn {
        &self.dma_tfn
    }
    #[doc = "0x1c4 - DMA Control Packet Set 0"]
    #[inline(always)]
    pub const fn dma_cps0(&self) -> &DmaCps0 {
        &self.dma_cps0
    }
    #[doc = "0x1c8 - DMA Control Packet Set 1 (Control/Length)"]
    #[inline(always)]
    pub const fn dma_cps1(&self) -> &DmaCps1 {
        &self.dma_cps1
    }
    #[doc = "0x1cc - DMA Control Packet Set 2 (Memory Address)"]
    #[inline(always)]
    pub const fn dma_cps2(&self) -> &DmaCps2 {
        &self.dma_cps2
    }
    #[doc = "0x1d0 - DMA Control Packet Set 3 (Setup/Command)"]
    #[inline(always)]
    pub const fn dma_cps3(&self) -> &DmaCps3 {
        &self.dma_cps3
    }
    #[doc = "0x1d4 - DMA Control Packet Set 4 (Reserved)"]
    #[inline(always)]
    pub const fn dma_cps4(&self) -> &DmaCps4 {
        &self.dma_cps4
    }
    #[doc = "0x1d8 - Device FIFO Mapping 2 (FIFO 4-7)"]
    #[inline(always)]
    pub const fn dev_fmap2(&self) -> &DevFmap2 {
        &self.dev_fmap2
    }
    #[doc = "0x1dc - Device FIFO Configuration 2 (FIFO 4-7)"]
    #[inline(always)]
    pub const fn dev_fcfg2(&self) -> &DevFcfg2 {
        &self.dev_fcfg2
    }
    #[doc = "0x1e0 - Device FIFO Mapping 3 (FIFO 8-11)"]
    #[inline(always)]
    pub const fn dev_fmap3(&self) -> &DevFmap3 {
        &self.dev_fmap3
    }
    #[doc = "0x1e4 - Device FIFO Configuration 3 (FIFO 8-11)"]
    #[inline(always)]
    pub const fn dev_fcfg3(&self) -> &DevFcfg3 {
        &self.dev_fcfg3
    }
    #[doc = "0x1e8 - Device FIFO Mapping 4 (FIFO 12-15)"]
    #[inline(always)]
    pub const fn dev_fmap4(&self) -> &DevFmap4 {
        &self.dev_fmap4
    }
    #[doc = "0x1ec - Device FIFO Configuration 4 (FIFO 12-15)"]
    #[inline(always)]
    pub const fn dev_fcfg4(&self) -> &DevFcfg4 {
        &self.dev_fcfg4
    }
    #[doc = "0x1f0 - Device FIFO Byte Count 4"]
    #[inline(always)]
    pub const fn dev_fibc4(&self) -> &DevFibc4 {
        &self.dev_fibc4
    }
    #[doc = "0x1f4 - Device FIFO Byte Count 5"]
    #[inline(always)]
    pub const fn dev_fibc5(&self) -> &DevFibc5 {
        &self.dev_fibc5
    }
    #[doc = "0x1f8 - Device FIFO Byte Count 6"]
    #[inline(always)]
    pub const fn dev_fibc6(&self) -> &DevFibc6 {
        &self.dev_fibc6
    }
    #[doc = "0x1fc - Device FIFO Byte Count 7"]
    #[inline(always)]
    pub const fn dev_fibc7(&self) -> &DevFibc7 {
        &self.dev_fibc7
    }
    #[doc = "0x300 - VDMA Control Endpoint FIFO Parameter Set 1"]
    #[inline(always)]
    pub const fn vdma_cxfps1(&self) -> &VdmaCxfps1 {
        &self.vdma_cxfps1
    }
    #[doc = "0x304 - VDMA Control Endpoint FIFO Parameter Set 2 (Memory Address)"]
    #[inline(always)]
    pub const fn vdma_cxfps2(&self) -> &VdmaCxfps2 {
        &self.vdma_cxfps2
    }
    #[doc = "0x308 - VDMA FIFO 0 Parameter Set 1"]
    #[inline(always)]
    pub const fn vdma_f0ps1(&self) -> &VdmaF0ps1 {
        &self.vdma_f0ps1
    }
    #[doc = "0x30c - VDMA FIFO 0 Parameter Set 2 (Memory Address)"]
    #[inline(always)]
    pub const fn vdma_f0ps2(&self) -> &VdmaF0ps2 {
        &self.vdma_f0ps2
    }
    #[doc = "0x310 - VDMA FIFO 1 Parameter Set 1"]
    #[inline(always)]
    pub const fn vdma_f1ps1(&self) -> &VdmaF1ps1 {
        &self.vdma_f1ps1
    }
    #[doc = "0x314 - VDMA FIFO 1 Parameter Set 2 (Memory Address)"]
    #[inline(always)]
    pub const fn vdma_f1ps2(&self) -> &VdmaF1ps2 {
        &self.vdma_f1ps2
    }
    #[doc = "0x318 - VDMA FIFO 2 Parameter Set 1"]
    #[inline(always)]
    pub const fn vdma_f2ps1(&self) -> &VdmaF2ps1 {
        &self.vdma_f2ps1
    }
    #[doc = "0x31c - VDMA FIFO 2 Parameter Set 2 (Memory Address)"]
    #[inline(always)]
    pub const fn vdma_f2ps2(&self) -> &VdmaF2ps2 {
        &self.vdma_f2ps2
    }
    #[doc = "0x320 - VDMA FIFO 3 Parameter Set 1"]
    #[inline(always)]
    pub const fn vdma_f3ps1(&self) -> &VdmaF3ps1 {
        &self.vdma_f3ps1
    }
    #[doc = "0x324 - VDMA FIFO 3 Parameter Set 2 (Memory Address)"]
    #[inline(always)]
    pub const fn vdma_f3ps2(&self) -> &VdmaF3ps2 {
        &self.vdma_f3ps2
    }
    #[doc = "0x328 - Device Interrupt Status Group 3 (VDMA Completion/Error for F0-F14)"]
    #[inline(always)]
    pub const fn dev_isg3(&self) -> &DevIsg3 {
        &self.dev_isg3
    }
    #[doc = "0x32c - Device Masked Interrupt Status Group 3 (VDMA Completion/Error for F0-F14)"]
    #[inline(always)]
    pub const fn dev_misg3(&self) -> &DevMisg3 {
        &self.dev_misg3
    }
    #[doc = "0x330 - VDMA Control Register"]
    #[inline(always)]
    pub const fn vdma_ctrl(&self) -> &VdmaCtrl {
        &self.vdma_ctrl
    }
    #[doc = "0x334 - LPM Capability Register"]
    #[inline(always)]
    pub const fn lpm_cap(&self) -> &LpmCap {
        &self.lpm_cap
    }
    #[doc = "0x338 - Device Interrupt Status Group 4 (L1/VDMA F15)"]
    #[inline(always)]
    pub const fn dev_isg4(&self) -> &DevIsg4 {
        &self.dev_isg4
    }
    #[doc = "0x33c - Device Masked Interrupt Status Group 4 (L1/VDMA F15)"]
    #[inline(always)]
    pub const fn dev_misg4(&self) -> &DevMisg4 {
        &self.dev_misg4
    }
    #[doc = "0x350 - VDMA FIFO n Parameter Set 1 (General structure for F4-F15)"]
    #[inline(always)]
    pub const fn vdma_fn_ps1(&self) -> &VdmaFnPs1 {
        &self.vdma_fn_ps1
    }
    #[doc = "0x354 - VDMA FIFO n Parameter Set 2 (Memory Address) (General structure for F4-F15)"]
    #[inline(always)]
    pub const fn vdma_fn_ps2(&self) -> &VdmaFnPs2 {
        &self.vdma_fn_ps2
    }
}
#[doc = "HCCAP (r) register accessor: Host Controller Capability Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccap`] module"]
#[doc(alias = "HCCAP")]
pub type Hccap = crate::Reg<hccap::HccapSpec>;
#[doc = "Host Controller Capability Register"]
pub mod hccap;
#[doc = "HCSPARAMS (r) register accessor: Host Controller Structural Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsparams::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsparams`] module"]
#[doc(alias = "HCSPARAMS")]
pub type Hcsparams = crate::Reg<hcsparams::HcsparamsSpec>;
#[doc = "Host Controller Structural Parameters"]
pub mod hcsparams;
#[doc = "HCCPARAMS (r) register accessor: Host Controller Capability Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`hccparams::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccparams`] module"]
#[doc(alias = "HCCPARAMS")]
pub type Hccparams = crate::Reg<hccparams::HccparamsSpec>;
#[doc = "Host Controller Capability Parameters"]
pub mod hccparams;
#[doc = "USBCMD (rw) register accessor: USB Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbcmd`] module"]
#[doc(alias = "USBCMD")]
pub type Usbcmd = crate::Reg<usbcmd::UsbcmdSpec>;
#[doc = "USB Command Register"]
pub mod usbcmd;
#[doc = "USBSTS (rw) register accessor: USB Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbsts`] module"]
#[doc(alias = "USBSTS")]
pub type Usbsts = crate::Reg<usbsts::UsbstsSpec>;
#[doc = "USB Status Register"]
pub mod usbsts;
#[doc = "USBINTR (rw) register accessor: USB Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbintr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbintr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbintr`] module"]
#[doc(alias = "USBINTR")]
pub type Usbintr = crate::Reg<usbintr::UsbintrSpec>;
#[doc = "USB Interrupt Enable Register"]
pub mod usbintr;
#[doc = "FRINDEX (rw) register accessor: Frame Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frindex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frindex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frindex`] module"]
#[doc(alias = "FRINDEX")]
pub type Frindex = crate::Reg<frindex::FrindexSpec>;
#[doc = "Frame Index Register"]
pub mod frindex;
#[doc = "PERIODICLISTBASE (rw) register accessor: Periodic Frame List Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`periodiclistbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periodiclistbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periodiclistbase`] module"]
#[doc(alias = "PERIODICLISTBASE")]
pub type Periodiclistbase = crate::Reg<periodiclistbase::PeriodiclistbaseSpec>;
#[doc = "Periodic Frame List Base Address"]
pub mod periodiclistbase;
#[doc = "ASYNCLISTADDR (rw) register accessor: Asynchronous List Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`asynclistaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asynclistaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asynclistaddr`] module"]
#[doc(alias = "ASYNCLISTADDR")]
pub type Asynclistaddr = crate::Reg<asynclistaddr::AsynclistaddrSpec>;
#[doc = "Asynchronous List Base Address"]
pub mod asynclistaddr;
#[doc = "PORTSC (rw) register accessor: Port Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`portsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portsc`] module"]
#[doc(alias = "PORTSC")]
pub type Portsc = crate::Reg<portsc::PortscSpec>;
#[doc = "Port Status and Control Register"]
pub mod portsc;
#[doc = "HCMISC (rw) register accessor: Host Controller Miscellaneous Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hcmisc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcmisc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcmisc`] module"]
#[doc(alias = "HCMISC")]
pub type Hcmisc = crate::Reg<hcmisc::HcmiscSpec>;
#[doc = "Host Controller Miscellaneous Control"]
pub mod hcmisc;
#[doc = "FS_EOF (rw) register accessor: Full Speed EOF Control and Host SM Status\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_eof::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_eof::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_eof`] module"]
#[doc(alias = "FS_EOF")]
pub type FsEof = crate::Reg<fs_eof::FsEofSpec>;
#[doc = "Full Speed EOF Control and Host SM Status"]
pub mod fs_eof;
#[doc = "HS_EOF (rw) register accessor: High Speed EOF Control and Host SM Status\n\nYou can [`read`](crate::Reg::read) this register and get [`hs_eof::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hs_eof::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_eof`] module"]
#[doc(alias = "HS_EOF")]
pub type HsEof = crate::Reg<hs_eof::HsEofSpec>;
#[doc = "High Speed EOF Control and Host SM Status"]
pub mod hs_eof;
#[doc = "OTG_CSR (rw) register accessor: OTG Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_csr`] module"]
#[doc(alias = "OTG_CSR")]
pub type OtgCsr = crate::Reg<otg_csr::OtgCsrSpec>;
#[doc = "OTG Control and Status Register"]
pub mod otg_csr;
#[doc = "OTG_ISR (rw) register accessor: OTG Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_isr`] module"]
#[doc(alias = "OTG_ISR")]
pub type OtgIsr = crate::Reg<otg_isr::OtgIsrSpec>;
#[doc = "OTG Interrupt Status Register"]
pub mod otg_isr;
#[doc = "OTG_IER (rw) register accessor: OTG Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_ier`] module"]
#[doc(alias = "OTG_IER")]
pub type OtgIer = crate::Reg<otg_ier::OtgIerSpec>;
#[doc = "OTG Interrupt Enable Register"]
pub mod otg_ier;
#[doc = "GLB_ISR (r) register accessor: Global Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`glb_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_isr`] module"]
#[doc(alias = "GLB_ISR")]
pub type GlbIsr = crate::Reg<glb_isr::GlbIsrSpec>;
#[doc = "Global Interrupt Status Register"]
pub mod glb_isr;
#[doc = "GLB_INT (rw) register accessor: Global Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`glb_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glb_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_int`] module"]
#[doc(alias = "GLB_INT")]
pub type GlbInt = crate::Reg<glb_int::GlbIntSpec>;
#[doc = "Global Interrupt Mask Register"]
pub mod glb_int;
#[doc = "REVISION (r) register accessor: USB Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`] module"]
#[doc(alias = "REVISION")]
pub type Revision = crate::Reg<revision::RevisionSpec>;
#[doc = "USB Revision Register"]
pub mod revision;
#[doc = "FEATURE (r) register accessor: USB Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`feature::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feature`] module"]
#[doc(alias = "FEATURE")]
pub type Feature = crate::Reg<feature::FeatureSpec>;
#[doc = "USB Feature Register"]
pub mod feature;
#[doc = "AXI_CR (rw) register accessor: AXI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_cr`] module"]
#[doc(alias = "AXI_CR")]
pub type AxiCr = crate::Reg<axi_cr::AxiCrSpec>;
#[doc = "AXI Control Register"]
pub mod axi_cr;
#[doc = "DEV_CTL (rw) register accessor: Device Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_ctl`] module"]
#[doc(alias = "DEV_CTL")]
pub type DevCtl = crate::Reg<dev_ctl::DevCtlSpec>;
#[doc = "Device Control Register"]
pub mod dev_ctl;
#[doc = "DEV_ADR (rw) register accessor: Device Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_adr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_adr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_adr`] module"]
#[doc(alias = "DEV_ADR")]
pub type DevAdr = crate::Reg<dev_adr::DevAdrSpec>;
#[doc = "Device Address Register"]
pub mod dev_adr;
#[doc = "DEV_TST (rw) register accessor: Device Test Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_tst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_tst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_tst`] module"]
#[doc(alias = "DEV_TST")]
pub type DevTst = crate::Reg<dev_tst::DevTstSpec>;
#[doc = "Device Test Control"]
pub mod dev_tst;
#[doc = "DEV_SFN (r) register accessor: Device SOF Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_sfn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_sfn`] module"]
#[doc(alias = "DEV_SFN")]
pub type DevSfn = crate::Reg<dev_sfn::DevSfnSpec>;
#[doc = "Device SOF Frame Number"]
pub mod dev_sfn;
#[doc = "DEV_SMT (r) register accessor: Device SOF Micro Frame Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_smt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_smt`] module"]
#[doc(alias = "DEV_SMT")]
pub type DevSmt = crate::Reg<dev_smt::DevSmtSpec>;
#[doc = "Device SOF Micro Frame Timer"]
pub mod dev_smt;
#[doc = "PHY_TST (rw) register accessor: PHY Test Control\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_tst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_tst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_tst`] module"]
#[doc(alias = "PHY_TST")]
pub type PhyTst = crate::Reg<phy_tst::PhyTstSpec>;
#[doc = "PHY Test Control"]
pub mod phy_tst;
#[doc = "DEV_VCTL (rw) register accessor: Device VBUS Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_vctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_vctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_vctl`] module"]
#[doc(alias = "DEV_VCTL")]
pub type DevVctl = crate::Reg<dev_vctl::DevVctlSpec>;
#[doc = "Device VBUS Control"]
pub mod dev_vctl;
#[doc = "DEV_CXCFG (rw) register accessor: Device Control Endpoint Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_cxcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_cxcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_cxcfg`] module"]
#[doc(alias = "DEV_CXCFG")]
pub type DevCxcfg = crate::Reg<dev_cxcfg::DevCxcfgSpec>;
#[doc = "Device Control Endpoint Configuration"]
pub mod dev_cxcfg;
#[doc = "DEV_CXCFE (rw) register accessor: Device Control Endpoint FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_cxcfe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_cxcfe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_cxcfe`] module"]
#[doc(alias = "DEV_CXCFE")]
pub type DevCxcfe = crate::Reg<dev_cxcfe::DevCxcfeSpec>;
#[doc = "Device Control Endpoint FIFO Status"]
pub mod dev_cxcfe;
#[doc = "DEV_ICR (rw) register accessor: Device Idle Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_icr`] module"]
#[doc(alias = "DEV_ICR")]
pub type DevIcr = crate::Reg<dev_icr::DevIcrSpec>;
#[doc = "Device Idle Counter"]
pub mod dev_icr;
#[doc = "DEV_MIGR (rw) register accessor: Device Master Interrupt Group Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_migr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_migr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_migr`] module"]
#[doc(alias = "DEV_MIGR")]
pub type DevMigr = crate::Reg<dev_migr::DevMigrSpec>;
#[doc = "Device Master Interrupt Group Register"]
pub mod dev_migr;
#[doc = "DEV_MISG0 (r) register accessor: Device Masked Interrupt Status Group 0 (Control EP)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_misg0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_misg0`] module"]
#[doc(alias = "DEV_MISG0")]
pub type DevMisg0 = crate::Reg<dev_misg0::DevMisg0Spec>;
#[doc = "Device Masked Interrupt Status Group 0 (Control EP)"]
pub mod dev_misg0;
#[doc = "DEV_MISG1 (r) register accessor: Device Masked Interrupt Status Group 1 (FIFO 0-7)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_misg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_misg1`] module"]
#[doc(alias = "DEV_MISG1")]
pub type DevMisg1 = crate::Reg<dev_misg1::DevMisg1Spec>;
#[doc = "Device Masked Interrupt Status Group 1 (FIFO 0-7)"]
pub mod dev_misg1;
#[doc = "DEV_MISG2 (r) register accessor: Device Masked Interrupt Status Group 2 (Events/DMA/FIFO 8-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_misg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_misg2`] module"]
#[doc(alias = "DEV_MISG2")]
pub type DevMisg2 = crate::Reg<dev_misg2::DevMisg2Spec>;
#[doc = "Device Masked Interrupt Status Group 2 (Events/DMA/FIFO 8-15)"]
pub mod dev_misg2;
#[doc = "DEV_IGR (r) register accessor: Device Interrupt Group Register (Status)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_igr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_igr`] module"]
#[doc(alias = "DEV_IGR")]
pub type DevIgr = crate::Reg<dev_igr::DevIgrSpec>;
#[doc = "Device Interrupt Group Register (Status)"]
pub mod dev_igr;
#[doc = "DEV_ISG0 (r) register accessor: Device Interrupt Status Group 0 (Control EP)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_isg0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_isg0`] module"]
#[doc(alias = "DEV_ISG0")]
pub type DevIsg0 = crate::Reg<dev_isg0::DevIsg0Spec>;
#[doc = "Device Interrupt Status Group 0 (Control EP)"]
pub mod dev_isg0;
#[doc = "DEV_ISG1 (r) register accessor: Device Interrupt Status Group 1 (FIFO 0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_isg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_isg1`] module"]
#[doc(alias = "DEV_ISG1")]
pub type DevIsg1 = crate::Reg<dev_isg1::DevIsg1Spec>;
#[doc = "Device Interrupt Status Group 1 (FIFO 0-15)"]
pub mod dev_isg1;
#[doc = "DEV_ISG2 (r) register accessor: Device Interrupt Status Group 2 (Events/DMA/FIFO 8-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_isg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_isg2`] module"]
#[doc(alias = "DEV_ISG2")]
pub type DevIsg2 = crate::Reg<dev_isg2::DevIsg2Spec>;
#[doc = "Device Interrupt Status Group 2 (Events/DMA/FIFO 8-15)"]
pub mod dev_isg2;
#[doc = "DEV_RXZ (r) register accessor: Device RX 0 Byte Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_rxz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_rxz`] module"]
#[doc(alias = "DEV_RXZ")]
pub type DevRxz = crate::Reg<dev_rxz::DevRxzSpec>;
#[doc = "Device RX 0 Byte Status"]
pub mod dev_rxz;
#[doc = "DEV_TXZ (r) register accessor: Device TX 0 Byte Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_txz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_txz`] module"]
#[doc(alias = "DEV_TXZ")]
pub type DevTxz = crate::Reg<dev_txz::DevTxzSpec>;
#[doc = "Device TX 0 Byte Status"]
pub mod dev_txz;
#[doc = "DEV_ISE (r) register accessor: Device Isochronous Sequence Error Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_ise::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_ise`] module"]
#[doc(alias = "DEV_ISE")]
pub type DevIse = crate::Reg<dev_ise::DevIseSpec>;
#[doc = "Device Isochronous Sequence Error Status"]
pub mod dev_ise;
#[doc = "DEV_INMPS1 (rw) register accessor: Device IN Endpoint 1 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_inmps1`] module"]
#[doc(alias = "DEV_INMPS1")]
pub type DevInmps1 = crate::Reg<dev_inmps1::DevInmps1Spec>;
#[doc = "Device IN Endpoint 1 Max Packet Size"]
pub mod dev_inmps1;
#[doc = "DEV_INMPS2 (rw) register accessor: Device IN Endpoint 2 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_inmps2`] module"]
#[doc(alias = "DEV_INMPS2")]
pub type DevInmps2 = crate::Reg<dev_inmps2::DevInmps2Spec>;
#[doc = "Device IN Endpoint 2 Max Packet Size"]
pub mod dev_inmps2;
#[doc = "DEV_INMPS3 (rw) register accessor: Device IN Endpoint 3 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_inmps3`] module"]
#[doc(alias = "DEV_INMPS3")]
pub type DevInmps3 = crate::Reg<dev_inmps3::DevInmps3Spec>;
#[doc = "Device IN Endpoint 3 Max Packet Size"]
pub mod dev_inmps3;
#[doc = "DEV_INMPS4 (rw) register accessor: Device IN Endpoint 4 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_inmps4`] module"]
#[doc(alias = "DEV_INMPS4")]
pub type DevInmps4 = crate::Reg<dev_inmps4::DevInmps4Spec>;
#[doc = "Device IN Endpoint 4 Max Packet Size"]
pub mod dev_inmps4;
#[doc = "DEV_INMPS5 (rw) register accessor: Device IN Endpoint 5 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_inmps5`] module"]
#[doc(alias = "DEV_INMPS5")]
pub type DevInmps5 = crate::Reg<dev_inmps5::DevInmps5Spec>;
#[doc = "Device IN Endpoint 5 Max Packet Size"]
pub mod dev_inmps5;
#[doc = "DEV_INMPS6 (rw) register accessor: Device IN Endpoint 6 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_inmps6`] module"]
#[doc(alias = "DEV_INMPS6")]
pub type DevInmps6 = crate::Reg<dev_inmps6::DevInmps6Spec>;
#[doc = "Device IN Endpoint 6 Max Packet Size"]
pub mod dev_inmps6;
#[doc = "DEV_INMPS7 (rw) register accessor: Device IN Endpoint 7 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_inmps7`] module"]
#[doc(alias = "DEV_INMPS7")]
pub type DevInmps7 = crate::Reg<dev_inmps7::DevInmps7Spec>;
#[doc = "Device IN Endpoint 7 Max Packet Size"]
pub mod dev_inmps7;
#[doc = "DEV_INMPS8 (rw) register accessor: Device IN Endpoint 8 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_inmps8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_inmps8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_inmps8`] module"]
#[doc(alias = "DEV_INMPS8")]
pub type DevInmps8 = crate::Reg<dev_inmps8::DevInmps8Spec>;
#[doc = "Device IN Endpoint 8 Max Packet Size"]
pub mod dev_inmps8;
#[doc = "DEV_OUTMPS1 (rw) register accessor: Device OUT Endpoint 1 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_outmps1`] module"]
#[doc(alias = "DEV_OUTMPS1")]
pub type DevOutmps1 = crate::Reg<dev_outmps1::DevOutmps1Spec>;
#[doc = "Device OUT Endpoint 1 Max Packet Size"]
pub mod dev_outmps1;
#[doc = "DEV_OUTMPS2 (rw) register accessor: Device OUT Endpoint 2 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_outmps2`] module"]
#[doc(alias = "DEV_OUTMPS2")]
pub type DevOutmps2 = crate::Reg<dev_outmps2::DevOutmps2Spec>;
#[doc = "Device OUT Endpoint 2 Max Packet Size"]
pub mod dev_outmps2;
#[doc = "DEV_OUTMPS3 (rw) register accessor: Device OUT Endpoint 3 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_outmps3`] module"]
#[doc(alias = "DEV_OUTMPS3")]
pub type DevOutmps3 = crate::Reg<dev_outmps3::DevOutmps3Spec>;
#[doc = "Device OUT Endpoint 3 Max Packet Size"]
pub mod dev_outmps3;
#[doc = "DEV_OUTMPS4 (rw) register accessor: Device OUT Endpoint 4 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_outmps4`] module"]
#[doc(alias = "DEV_OUTMPS4")]
pub type DevOutmps4 = crate::Reg<dev_outmps4::DevOutmps4Spec>;
#[doc = "Device OUT Endpoint 4 Max Packet Size"]
pub mod dev_outmps4;
#[doc = "DEV_OUTMPS5 (rw) register accessor: Device OUT Endpoint 5 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_outmps5`] module"]
#[doc(alias = "DEV_OUTMPS5")]
pub type DevOutmps5 = crate::Reg<dev_outmps5::DevOutmps5Spec>;
#[doc = "Device OUT Endpoint 5 Max Packet Size"]
pub mod dev_outmps5;
#[doc = "DEV_OUTMPS6 (rw) register accessor: Device OUT Endpoint 6 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_outmps6`] module"]
#[doc(alias = "DEV_OUTMPS6")]
pub type DevOutmps6 = crate::Reg<dev_outmps6::DevOutmps6Spec>;
#[doc = "Device OUT Endpoint 6 Max Packet Size"]
pub mod dev_outmps6;
#[doc = "DEV_OUTMPS7 (rw) register accessor: Device OUT Endpoint 7 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_outmps7`] module"]
#[doc(alias = "DEV_OUTMPS7")]
pub type DevOutmps7 = crate::Reg<dev_outmps7::DevOutmps7Spec>;
#[doc = "Device OUT Endpoint 7 Max Packet Size"]
pub mod dev_outmps7;
#[doc = "DEV_OUTMPS8 (rw) register accessor: Device OUT Endpoint 8 Max Packet Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_outmps8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_outmps8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_outmps8`] module"]
#[doc(alias = "DEV_OUTMPS8")]
pub type DevOutmps8 = crate::Reg<dev_outmps8::DevOutmps8Spec>;
#[doc = "Device OUT Endpoint 8 Max Packet Size"]
pub mod dev_outmps8;
#[doc = "DEV_EPMAP0 (rw) register accessor: Device Endpoint/FIFO Mapping 0 (EP1-EP4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_epmap0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_epmap0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_epmap0`] module"]
#[doc(alias = "DEV_EPMAP0")]
pub type DevEpmap0 = crate::Reg<dev_epmap0::DevEpmap0Spec>;
#[doc = "Device Endpoint/FIFO Mapping 0 (EP1-EP4)"]
pub mod dev_epmap0;
#[doc = "DEV_EPMAP1 (rw) register accessor: Device Endpoint/FIFO Mapping 1 (EP5-EP8)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_epmap1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_epmap1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_epmap1`] module"]
#[doc(alias = "DEV_EPMAP1")]
pub type DevEpmap1 = crate::Reg<dev_epmap1::DevEpmap1Spec>;
#[doc = "Device Endpoint/FIFO Mapping 1 (EP5-EP8)"]
pub mod dev_epmap1;
#[doc = "DEV_FMAP (rw) register accessor: Device FIFO Mapping (FIFO 0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fmap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fmap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fmap`] module"]
#[doc(alias = "DEV_FMAP")]
pub type DevFmap = crate::Reg<dev_fmap::DevFmapSpec>;
#[doc = "Device FIFO Mapping (FIFO 0-3)"]
pub mod dev_fmap;
#[doc = "DEV_FCFG (rw) register accessor: Device FIFO Configuration (FIFO 0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fcfg`] module"]
#[doc(alias = "DEV_FCFG")]
pub type DevFcfg = crate::Reg<dev_fcfg::DevFcfgSpec>;
#[doc = "Device FIFO Configuration (FIFO 0-3)"]
pub mod dev_fcfg;
#[doc = "DEV_FIBC0 (rw) register accessor: Device FIFO Byte Count 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fibc0`] module"]
#[doc(alias = "DEV_FIBC0")]
pub type DevFibc0 = crate::Reg<dev_fibc0::DevFibc0Spec>;
#[doc = "Device FIFO Byte Count 0"]
pub mod dev_fibc0;
#[doc = "DEV_FIBC1 (rw) register accessor: Device FIFO Byte Count 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fibc1`] module"]
#[doc(alias = "DEV_FIBC1")]
pub type DevFibc1 = crate::Reg<dev_fibc1::DevFibc1Spec>;
#[doc = "Device FIFO Byte Count 1"]
pub mod dev_fibc1;
#[doc = "DEV_FIBC2 (rw) register accessor: Device FIFO Byte Count 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fibc2`] module"]
#[doc(alias = "DEV_FIBC2")]
pub type DevFibc2 = crate::Reg<dev_fibc2::DevFibc2Spec>;
#[doc = "Device FIFO Byte Count 2"]
pub mod dev_fibc2;
#[doc = "DEV_FIBC3 (rw) register accessor: Device FIFO Byte Count 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fibc3`] module"]
#[doc(alias = "DEV_FIBC3")]
pub type DevFibc3 = crate::Reg<dev_fibc3::DevFibc3Spec>;
#[doc = "Device FIFO Byte Count 3"]
pub mod dev_fibc3;
#[doc = "DMA_TFN (rw) register accessor: DMA Toggle/Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tfn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tfn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_tfn`] module"]
#[doc(alias = "DMA_TFN")]
pub type DmaTfn = crate::Reg<dma_tfn::DmaTfnSpec>;
#[doc = "DMA Toggle/Frame Number"]
pub mod dma_tfn;
#[doc = "DMA_CPS0 (rw) register accessor: DMA Control Packet Set 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cps0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cps0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cps0`] module"]
#[doc(alias = "DMA_CPS0")]
pub type DmaCps0 = crate::Reg<dma_cps0::DmaCps0Spec>;
#[doc = "DMA Control Packet Set 0"]
pub mod dma_cps0;
#[doc = "DMA_CPS1 (rw) register accessor: DMA Control Packet Set 1 (Control/Length)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cps1`] module"]
#[doc(alias = "DMA_CPS1")]
pub type DmaCps1 = crate::Reg<dma_cps1::DmaCps1Spec>;
#[doc = "DMA Control Packet Set 1 (Control/Length)"]
pub mod dma_cps1;
#[doc = "DMA_CPS2 (rw) register accessor: DMA Control Packet Set 2 (Memory Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cps2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cps2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cps2`] module"]
#[doc(alias = "DMA_CPS2")]
pub type DmaCps2 = crate::Reg<dma_cps2::DmaCps2Spec>;
#[doc = "DMA Control Packet Set 2 (Memory Address)"]
pub mod dma_cps2;
#[doc = "DMA_CPS3 (rw) register accessor: DMA Control Packet Set 3 (Setup/Command)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cps3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cps3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cps3`] module"]
#[doc(alias = "DMA_CPS3")]
pub type DmaCps3 = crate::Reg<dma_cps3::DmaCps3Spec>;
#[doc = "DMA Control Packet Set 3 (Setup/Command)"]
pub mod dma_cps3;
#[doc = "DMA_CPS4 (r) register accessor: DMA Control Packet Set 4 (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cps4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_cps4`] module"]
#[doc(alias = "DMA_CPS4")]
pub type DmaCps4 = crate::Reg<dma_cps4::DmaCps4Spec>;
#[doc = "DMA Control Packet Set 4 (Reserved)"]
pub mod dma_cps4;
#[doc = "DEV_FMAP2 (rw) register accessor: Device FIFO Mapping 2 (FIFO 4-7)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fmap2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fmap2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fmap2`] module"]
#[doc(alias = "DEV_FMAP2")]
pub type DevFmap2 = crate::Reg<dev_fmap2::DevFmap2Spec>;
#[doc = "Device FIFO Mapping 2 (FIFO 4-7)"]
pub mod dev_fmap2;
#[doc = "DEV_FCFG2 (rw) register accessor: Device FIFO Configuration 2 (FIFO 4-7)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fcfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fcfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fcfg2`] module"]
#[doc(alias = "DEV_FCFG2")]
pub type DevFcfg2 = crate::Reg<dev_fcfg2::DevFcfg2Spec>;
#[doc = "Device FIFO Configuration 2 (FIFO 4-7)"]
pub mod dev_fcfg2;
#[doc = "DEV_FMAP3 (rw) register accessor: Device FIFO Mapping 3 (FIFO 8-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fmap3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fmap3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fmap3`] module"]
#[doc(alias = "DEV_FMAP3")]
pub type DevFmap3 = crate::Reg<dev_fmap3::DevFmap3Spec>;
#[doc = "Device FIFO Mapping 3 (FIFO 8-11)"]
pub mod dev_fmap3;
#[doc = "DEV_FCFG3 (rw) register accessor: Device FIFO Configuration 3 (FIFO 8-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fcfg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fcfg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fcfg3`] module"]
#[doc(alias = "DEV_FCFG3")]
pub type DevFcfg3 = crate::Reg<dev_fcfg3::DevFcfg3Spec>;
#[doc = "Device FIFO Configuration 3 (FIFO 8-11)"]
pub mod dev_fcfg3;
#[doc = "DEV_FMAP4 (rw) register accessor: Device FIFO Mapping 4 (FIFO 12-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fmap4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fmap4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fmap4`] module"]
#[doc(alias = "DEV_FMAP4")]
pub type DevFmap4 = crate::Reg<dev_fmap4::DevFmap4Spec>;
#[doc = "Device FIFO Mapping 4 (FIFO 12-15)"]
pub mod dev_fmap4;
#[doc = "DEV_FCFG4 (rw) register accessor: Device FIFO Configuration 4 (FIFO 12-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fcfg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fcfg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fcfg4`] module"]
#[doc(alias = "DEV_FCFG4")]
pub type DevFcfg4 = crate::Reg<dev_fcfg4::DevFcfg4Spec>;
#[doc = "Device FIFO Configuration 4 (FIFO 12-15)"]
pub mod dev_fcfg4;
#[doc = "DEV_FIBC4 (rw) register accessor: Device FIFO Byte Count 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fibc4`] module"]
#[doc(alias = "DEV_FIBC4")]
pub type DevFibc4 = crate::Reg<dev_fibc4::DevFibc4Spec>;
#[doc = "Device FIFO Byte Count 4"]
pub mod dev_fibc4;
#[doc = "DEV_FIBC5 (rw) register accessor: Device FIFO Byte Count 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fibc5`] module"]
#[doc(alias = "DEV_FIBC5")]
pub type DevFibc5 = crate::Reg<dev_fibc5::DevFibc5Spec>;
#[doc = "Device FIFO Byte Count 5"]
pub mod dev_fibc5;
#[doc = "DEV_FIBC6 (rw) register accessor: Device FIFO Byte Count 6\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fibc6`] module"]
#[doc(alias = "DEV_FIBC6")]
pub type DevFibc6 = crate::Reg<dev_fibc6::DevFibc6Spec>;
#[doc = "Device FIFO Byte Count 6"]
pub mod dev_fibc6;
#[doc = "DEV_FIBC7 (rw) register accessor: Device FIFO Byte Count 7\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fibc7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fibc7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_fibc7`] module"]
#[doc(alias = "DEV_FIBC7")]
pub type DevFibc7 = crate::Reg<dev_fibc7::DevFibc7Spec>;
#[doc = "Device FIFO Byte Count 7"]
pub mod dev_fibc7;
#[doc = "VDMA_CXFPS1 (rw) register accessor: VDMA Control Endpoint FIFO Parameter Set 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_cxfps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_cxfps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_cxfps1`] module"]
#[doc(alias = "VDMA_CXFPS1")]
pub type VdmaCxfps1 = crate::Reg<vdma_cxfps1::VdmaCxfps1Spec>;
#[doc = "VDMA Control Endpoint FIFO Parameter Set 1"]
pub mod vdma_cxfps1;
#[doc = "VDMA_CXFPS2 (rw) register accessor: VDMA Control Endpoint FIFO Parameter Set 2 (Memory Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_cxfps2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_cxfps2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_cxfps2`] module"]
#[doc(alias = "VDMA_CXFPS2")]
pub type VdmaCxfps2 = crate::Reg<vdma_cxfps2::VdmaCxfps2Spec>;
#[doc = "VDMA Control Endpoint FIFO Parameter Set 2 (Memory Address)"]
pub mod vdma_cxfps2;
#[doc = "VDMA_F0PS1 (rw) register accessor: VDMA FIFO 0 Parameter Set 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f0ps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f0ps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_f0ps1`] module"]
#[doc(alias = "VDMA_F0PS1")]
pub type VdmaF0ps1 = crate::Reg<vdma_f0ps1::VdmaF0ps1Spec>;
#[doc = "VDMA FIFO 0 Parameter Set 1"]
pub mod vdma_f0ps1;
#[doc = "VDMA_F0PS2 (rw) register accessor: VDMA FIFO 0 Parameter Set 2 (Memory Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f0ps2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f0ps2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_f0ps2`] module"]
#[doc(alias = "VDMA_F0PS2")]
pub type VdmaF0ps2 = crate::Reg<vdma_f0ps2::VdmaF0ps2Spec>;
#[doc = "VDMA FIFO 0 Parameter Set 2 (Memory Address)"]
pub mod vdma_f0ps2;
#[doc = "VDMA_F1PS1 (rw) register accessor: VDMA FIFO 1 Parameter Set 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f1ps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f1ps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_f1ps1`] module"]
#[doc(alias = "VDMA_F1PS1")]
pub type VdmaF1ps1 = crate::Reg<vdma_f1ps1::VdmaF1ps1Spec>;
#[doc = "VDMA FIFO 1 Parameter Set 1"]
pub mod vdma_f1ps1;
#[doc = "VDMA_F1PS2 (rw) register accessor: VDMA FIFO 1 Parameter Set 2 (Memory Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f1ps2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f1ps2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_f1ps2`] module"]
#[doc(alias = "VDMA_F1PS2")]
pub type VdmaF1ps2 = crate::Reg<vdma_f1ps2::VdmaF1ps2Spec>;
#[doc = "VDMA FIFO 1 Parameter Set 2 (Memory Address)"]
pub mod vdma_f1ps2;
#[doc = "VDMA_F2PS1 (rw) register accessor: VDMA FIFO 2 Parameter Set 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f2ps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f2ps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_f2ps1`] module"]
#[doc(alias = "VDMA_F2PS1")]
pub type VdmaF2ps1 = crate::Reg<vdma_f2ps1::VdmaF2ps1Spec>;
#[doc = "VDMA FIFO 2 Parameter Set 1"]
pub mod vdma_f2ps1;
#[doc = "VDMA_F2PS2 (rw) register accessor: VDMA FIFO 2 Parameter Set 2 (Memory Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f2ps2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f2ps2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_f2ps2`] module"]
#[doc(alias = "VDMA_F2PS2")]
pub type VdmaF2ps2 = crate::Reg<vdma_f2ps2::VdmaF2ps2Spec>;
#[doc = "VDMA FIFO 2 Parameter Set 2 (Memory Address)"]
pub mod vdma_f2ps2;
#[doc = "VDMA_F3PS1 (rw) register accessor: VDMA FIFO 3 Parameter Set 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f3ps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f3ps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_f3ps1`] module"]
#[doc(alias = "VDMA_F3PS1")]
pub type VdmaF3ps1 = crate::Reg<vdma_f3ps1::VdmaF3ps1Spec>;
#[doc = "VDMA FIFO 3 Parameter Set 1"]
pub mod vdma_f3ps1;
#[doc = "VDMA_F3PS2 (rw) register accessor: VDMA FIFO 3 Parameter Set 2 (Memory Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_f3ps2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_f3ps2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_f3ps2`] module"]
#[doc(alias = "VDMA_F3PS2")]
pub type VdmaF3ps2 = crate::Reg<vdma_f3ps2::VdmaF3ps2Spec>;
#[doc = "VDMA FIFO 3 Parameter Set 2 (Memory Address)"]
pub mod vdma_f3ps2;
#[doc = "DEV_ISG3 (r) register accessor: Device Interrupt Status Group 3 (VDMA Completion/Error for F0-F14)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_isg3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_isg3`] module"]
#[doc(alias = "DEV_ISG3")]
pub type DevIsg3 = crate::Reg<dev_isg3::DevIsg3Spec>;
#[doc = "Device Interrupt Status Group 3 (VDMA Completion/Error for F0-F14)"]
pub mod dev_isg3;
#[doc = "DEV_MISG3 (r) register accessor: Device Masked Interrupt Status Group 3 (VDMA Completion/Error for F0-F14)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_misg3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_misg3`] module"]
#[doc(alias = "DEV_MISG3")]
pub type DevMisg3 = crate::Reg<dev_misg3::DevMisg3Spec>;
#[doc = "Device Masked Interrupt Status Group 3 (VDMA Completion/Error for F0-F14)"]
pub mod dev_misg3;
#[doc = "VDMA_CTRL (rw) register accessor: VDMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_ctrl`] module"]
#[doc(alias = "VDMA_CTRL")]
pub type VdmaCtrl = crate::Reg<vdma_ctrl::VdmaCtrlSpec>;
#[doc = "VDMA Control Register"]
pub mod vdma_ctrl;
#[doc = "LPM_CAP (rw) register accessor: LPM Capability Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpm_cap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpm_cap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpm_cap`] module"]
#[doc(alias = "LPM_CAP")]
pub type LpmCap = crate::Reg<lpm_cap::LpmCapSpec>;
#[doc = "LPM Capability Register"]
pub mod lpm_cap;
#[doc = "DEV_ISG4 (r) register accessor: Device Interrupt Status Group 4 (L1/VDMA F15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_isg4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_isg4`] module"]
#[doc(alias = "DEV_ISG4")]
pub type DevIsg4 = crate::Reg<dev_isg4::DevIsg4Spec>;
#[doc = "Device Interrupt Status Group 4 (L1/VDMA F15)"]
pub mod dev_isg4;
#[doc = "DEV_MISG4 (r) register accessor: Device Masked Interrupt Status Group 4 (L1/VDMA F15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_misg4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_misg4`] module"]
#[doc(alias = "DEV_MISG4")]
pub type DevMisg4 = crate::Reg<dev_misg4::DevMisg4Spec>;
#[doc = "Device Masked Interrupt Status Group 4 (L1/VDMA F15)"]
pub mod dev_misg4;
#[doc = "VDMA_FnPS1 (rw) register accessor: VDMA FIFO n Parameter Set 1 (General structure for F4-F15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_fn_ps1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_fn_ps1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_fn_ps1`] module"]
#[doc(alias = "VDMA_FnPS1")]
pub type VdmaFnPs1 = crate::Reg<vdma_fn_ps1::VdmaFnPs1Spec>;
#[doc = "VDMA FIFO n Parameter Set 1 (General structure for F4-F15)"]
pub mod vdma_fn_ps1;
#[doc = "VDMA_FnPS2 (rw) register accessor: VDMA FIFO n Parameter Set 2 (Memory Address) (General structure for F4-F15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_fn_ps2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_fn_ps2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdma_fn_ps2`] module"]
#[doc(alias = "VDMA_FnPS2")]
pub type VdmaFnPs2 = crate::Reg<vdma_fn_ps2::VdmaFnPs2Spec>;
#[doc = "VDMA FIFO n Parameter Set 2 (Memory Address) (General structure for F4-F15)"]
pub mod vdma_fn_ps2;
