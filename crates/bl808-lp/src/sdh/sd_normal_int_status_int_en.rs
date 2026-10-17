#[doc = "Register `SD_NORMAL_INT_STATUS_INT_EN` reader"]
pub type R = crate::R<SdNormalIntStatusIntEnSpec>;
#[doc = "Register `SD_NORMAL_INT_STATUS_INT_EN` writer"]
pub type W = crate::W<SdNormalIntStatusIntEnSpec>;
#[doc = "Field `CMD_COMPLETE_INT_EN` reader - "]
pub type CmdCompleteIntEnR = crate::BitReader;
#[doc = "Field `CMD_COMPLETE_INT_EN` writer - "]
pub type CmdCompleteIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFER_COMPLETE_INT_EN` reader - "]
pub type XferCompleteIntEnR = crate::BitReader;
#[doc = "Field `XFER_COMPLETE_INT_EN` writer - "]
pub type XferCompleteIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_GAP_EVT_INT_EN` reader - "]
pub type BlockGapEvtIntEnR = crate::BitReader;
#[doc = "Field `BLOCK_GAP_EVT_INT_EN` writer - "]
pub type BlockGapEvtIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_INT_INT_EN` reader - "]
pub type DmaIntIntEnR = crate::BitReader;
#[doc = "Field `DMA_INT_INT_EN` writer - "]
pub type DmaIntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_RDY_INT_EN` reader - "]
pub type TxRdyIntEnR = crate::BitReader;
#[doc = "Field `TX_RDY_INT_EN` writer - "]
pub type TxRdyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_RDY_INT_EN` reader - "]
pub type RxRdyIntEnR = crate::BitReader;
#[doc = "Field `RX_RDY_INT_EN` writer - "]
pub type RxRdyIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INS_INT_EN` reader - "]
pub type CardInsIntEnR = crate::BitReader;
#[doc = "Field `CARD_INS_INT_EN` writer - "]
pub type CardInsIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REM_INT_EN` reader - "]
pub type CardRemIntEnR = crate::BitReader;
#[doc = "Field `CARD_REM_INT_EN` writer - "]
pub type CardRemIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INT_INT_EN` reader - "]
pub type CardIntIntEnR = crate::BitReader;
#[doc = "Field `CARD_INT_INT_EN` writer - "]
pub type CardIntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_A_INT_INT_EN` reader - "]
pub type IntAIntIntEnR = crate::BitReader;
#[doc = "Field `INT_A_INT_INT_EN` writer - "]
pub type IntAIntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_B_INT_INT_EN` reader - "]
pub type IntBIntIntEnR = crate::BitReader;
#[doc = "Field `INT_B_INT_INT_EN` writer - "]
pub type IntBIntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_C_INT_INT_EN` reader - "]
pub type IntCIntIntEnR = crate::BitReader;
#[doc = "Field `INT_C_INT_INT_EN` writer - "]
pub type IntCIntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNE_INT_INT_EN` reader - "]
pub type RetuneIntIntEnR = crate::BitReader;
#[doc = "Field `RETUNE_INT_INT_EN` writer - "]
pub type RetuneIntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_complete_int_en(&self) -> CmdCompleteIntEnR {
        CmdCompleteIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_complete_int_en(&self) -> XferCompleteIntEnR {
        XferCompleteIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn block_gap_evt_int_en(&self) -> BlockGapEvtIntEnR {
        BlockGapEvtIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_int_int_en(&self) -> DmaIntIntEnR {
        DmaIntIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_rdy_int_en(&self) -> TxRdyIntEnR {
        TxRdyIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rdy_int_en(&self) -> RxRdyIntEnR {
        RxRdyIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_ins_int_en(&self) -> CardInsIntEnR {
        CardInsIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_rem_int_en(&self) -> CardRemIntEnR {
        CardRemIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn card_int_int_en(&self) -> CardIntIntEnR {
        CardIntIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn int_a_int_int_en(&self) -> IntAIntIntEnR {
        IntAIntIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn int_b_int_int_en(&self) -> IntBIntIntEnR {
        IntBIntIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn int_c_int_int_en(&self) -> IntCIntIntEnR {
        IntCIntIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn retune_int_int_en(&self) -> RetuneIntIntEnR {
        RetuneIntIntEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_complete_int_en(&mut self) -> CmdCompleteIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        CmdCompleteIntEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_complete_int_en(&mut self) -> XferCompleteIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        XferCompleteIntEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn block_gap_evt_int_en(&mut self) -> BlockGapEvtIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        BlockGapEvtIntEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_int_int_en(&mut self) -> DmaIntIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        DmaIntIntEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_rdy_int_en(&mut self) -> TxRdyIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        TxRdyIntEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_rdy_int_en(&mut self) -> RxRdyIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        RxRdyIntEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_ins_int_en(&mut self) -> CardInsIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        CardInsIntEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_rem_int_en(&mut self) -> CardRemIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        CardRemIntEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn card_int_int_en(&mut self) -> CardIntIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        CardIntIntEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn int_a_int_int_en(&mut self) -> IntAIntIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        IntAIntIntEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn int_b_int_int_en(&mut self) -> IntBIntIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        IntBIntIntEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn int_c_int_int_en(&mut self) -> IntCIntIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        IntCIntIntEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn retune_int_int_en(&mut self) -> RetuneIntIntEnW<'_, SdNormalIntStatusIntEnSpec> {
        RetuneIntIntEnW::new(self, 12)
    }
}
#[doc = "Normal Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_normal_int_status_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_normal_int_status_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdNormalIntStatusIntEnSpec;
impl crate::RegisterSpec for SdNormalIntStatusIntEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_normal_int_status_int_en::R`](R) reader structure"]
impl crate::Readable for SdNormalIntStatusIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_normal_int_status_int_en::W`](W) writer structure"]
impl crate::Writable for SdNormalIntStatusIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_NORMAL_INT_STATUS_INT_EN to value 0"]
impl crate::Resettable for SdNormalIntStatusIntEnSpec {}
