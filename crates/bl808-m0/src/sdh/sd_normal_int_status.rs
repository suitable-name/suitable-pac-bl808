#[doc = "Register `SD_NORMAL_INT_STATUS` reader"]
pub type R = crate::R<SdNormalIntStatusSpec>;
#[doc = "Register `SD_NORMAL_INT_STATUS` writer"]
pub type W = crate::W<SdNormalIntStatusSpec>;
#[doc = "Field `CMD_COMPLETE` reader - "]
pub type CmdCompleteR = crate::BitReader;
#[doc = "Field `CMD_COMPLETE` writer - "]
pub type CmdCompleteW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `XFER_COMPLETE` reader - "]
pub type XferCompleteR = crate::BitReader;
#[doc = "Field `XFER_COMPLETE` writer - "]
pub type XferCompleteW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BLOCK_GAP_EVT` reader - "]
pub type BlockGapEvtR = crate::BitReader;
#[doc = "Field `BLOCK_GAP_EVT` writer - "]
pub type BlockGapEvtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMA_INT` reader - "]
pub type DmaIntR = crate::BitReader;
#[doc = "Field `DMA_INT` writer - "]
pub type DmaIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_RDY` reader - "]
pub type TxRdyR = crate::BitReader;
#[doc = "Field `TX_RDY` writer - "]
pub type TxRdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_RDY` reader - "]
pub type RxRdyR = crate::BitReader;
#[doc = "Field `RX_RDY` writer - "]
pub type RxRdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CARD_INS_INT` reader - "]
pub type CardInsIntR = crate::BitReader;
#[doc = "Field `CARD_INS_INT` writer - "]
pub type CardInsIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CARD_REM_INT` reader - "]
pub type CardRemIntR = crate::BitReader;
#[doc = "Field `CARD_REM_INT` writer - "]
pub type CardRemIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CARD_INT` reader - "]
pub type CardIntR = crate::BitReader;
#[doc = "Field `INT_A` reader - "]
pub type IntAR = crate::BitReader;
#[doc = "Field `INT_A` writer - "]
pub type IntAW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_B` reader - "]
pub type IntBR = crate::BitReader;
#[doc = "Field `INT_B` writer - "]
pub type IntBW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_C` reader - "]
pub type IntCR = crate::BitReader;
#[doc = "Field `INT_C` writer - "]
pub type IntCW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RETUNING_INT` reader - "]
pub type RetuningIntR = crate::BitReader;
#[doc = "Field `RETUNING_INT` writer - "]
pub type RetuningIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERR_INT` reader - "]
pub type ErrIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_complete(&self) -> CmdCompleteR {
        CmdCompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_complete(&self) -> XferCompleteR {
        XferCompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn block_gap_evt(&self) -> BlockGapEvtR {
        BlockGapEvtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_int(&self) -> DmaIntR {
        DmaIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_rdy(&self) -> TxRdyR {
        TxRdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rdy(&self) -> RxRdyR {
        RxRdyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_ins_int(&self) -> CardInsIntR {
        CardInsIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_rem_int(&self) -> CardRemIntR {
        CardRemIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn card_int(&self) -> CardIntR {
        CardIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn int_a(&self) -> IntAR {
        IntAR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn int_b(&self) -> IntBR {
        IntBR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn int_c(&self) -> IntCR {
        IntCR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn retuning_int(&self) -> RetuningIntR {
        RetuningIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn err_int(&self) -> ErrIntR {
        ErrIntR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_complete(&mut self) -> CmdCompleteW<'_, SdNormalIntStatusSpec> {
        CmdCompleteW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_complete(&mut self) -> XferCompleteW<'_, SdNormalIntStatusSpec> {
        XferCompleteW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn block_gap_evt(&mut self) -> BlockGapEvtW<'_, SdNormalIntStatusSpec> {
        BlockGapEvtW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_int(&mut self) -> DmaIntW<'_, SdNormalIntStatusSpec> {
        DmaIntW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_rdy(&mut self) -> TxRdyW<'_, SdNormalIntStatusSpec> {
        TxRdyW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rdy(&mut self) -> RxRdyW<'_, SdNormalIntStatusSpec> {
        RxRdyW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_ins_int(&mut self) -> CardInsIntW<'_, SdNormalIntStatusSpec> {
        CardInsIntW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_rem_int(&mut self) -> CardRemIntW<'_, SdNormalIntStatusSpec> {
        CardRemIntW::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn int_a(&mut self) -> IntAW<'_, SdNormalIntStatusSpec> {
        IntAW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn int_b(&mut self) -> IntBW<'_, SdNormalIntStatusSpec> {
        IntBW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn int_c(&mut self) -> IntCW<'_, SdNormalIntStatusSpec> {
        IntCW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn retuning_int(&mut self) -> RetuningIntW<'_, SdNormalIntStatusSpec> {
        RetuningIntW::new(self, 12)
    }
}
#[doc = "Normal Interrupt Status Register (Read/Write 1 to Clear)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_normal_int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_normal_int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdNormalIntStatusSpec;
impl crate::RegisterSpec for SdNormalIntStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_normal_int_status::R`](R) reader structure"]
impl crate::Readable for SdNormalIntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_normal_int_status::W`](W) writer structure"]
impl crate::Writable for SdNormalIntStatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0x1eff;
}
#[doc = "`reset()` method sets SD_NORMAL_INT_STATUS to value 0"]
impl crate::Resettable for SdNormalIntStatusSpec {}
