#[doc = "Register `SD_NORMAL_INT_STATUS_EN` reader"]
pub type R = crate::R<SdNormalIntStatusEnSpec>;
#[doc = "Register `SD_NORMAL_INT_STATUS_EN` writer"]
pub type W = crate::W<SdNormalIntStatusEnSpec>;
#[doc = "Field `CMD_COMPLETE_EN` reader - "]
pub type CmdCompleteEnR = crate::BitReader;
#[doc = "Field `CMD_COMPLETE_EN` writer - "]
pub type CmdCompleteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFER_COMPLETE_EN` reader - "]
pub type XferCompleteEnR = crate::BitReader;
#[doc = "Field `XFER_COMPLETE_EN` writer - "]
pub type XferCompleteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_GAP_EVT_EN` reader - "]
pub type BlockGapEvtEnR = crate::BitReader;
#[doc = "Field `BLOCK_GAP_EVT_EN` writer - "]
pub type BlockGapEvtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_INT_EN` reader - "]
pub type DmaIntEnR = crate::BitReader;
#[doc = "Field `DMA_INT_EN` writer - "]
pub type DmaIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_RDY_EN` reader - "]
pub type TxRdyEnR = crate::BitReader;
#[doc = "Field `TX_RDY_EN` writer - "]
pub type TxRdyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_RDY_EN` reader - "]
pub type RdRdyEnR = crate::BitReader;
#[doc = "Field `RD_RDY_EN` writer - "]
pub type RdRdyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INS_EN` reader - "]
pub type CardInsEnR = crate::BitReader;
#[doc = "Field `CARD_INS_EN` writer - "]
pub type CardInsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REM_EN` reader - "]
pub type CardRemEnR = crate::BitReader;
#[doc = "Field `CARD_REM_EN` writer - "]
pub type CardRemEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INT_EN` reader - "]
pub type CardIntEnR = crate::BitReader;
#[doc = "Field `CARD_INT_EN` writer - "]
pub type CardIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_A_INT_EN` reader - "]
pub type IntAIntEnR = crate::BitReader;
#[doc = "Field `INT_A_INT_EN` writer - "]
pub type IntAIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_B_INT_EN` reader - "]
pub type IntBIntEnR = crate::BitReader;
#[doc = "Field `INT_B_INT_EN` writer - "]
pub type IntBIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_C_INT_EN` reader - "]
pub type IntCIntEnR = crate::BitReader;
#[doc = "Field `INT_C_INT_EN` writer - "]
pub type IntCIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNE_INT_EN` reader - "]
pub type RetuneIntEnR = crate::BitReader;
#[doc = "Field `RETUNE_INT_EN` writer - "]
pub type RetuneIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_complete_en(&self) -> CmdCompleteEnR {
        CmdCompleteEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_complete_en(&self) -> XferCompleteEnR {
        XferCompleteEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn block_gap_evt_en(&self) -> BlockGapEvtEnR {
        BlockGapEvtEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_int_en(&self) -> DmaIntEnR {
        DmaIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_rdy_en(&self) -> TxRdyEnR {
        TxRdyEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rd_rdy_en(&self) -> RdRdyEnR {
        RdRdyEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_ins_en(&self) -> CardInsEnR {
        CardInsEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_rem_en(&self) -> CardRemEnR {
        CardRemEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn card_int_en(&self) -> CardIntEnR {
        CardIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn int_a_int_en(&self) -> IntAIntEnR {
        IntAIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn int_b_int_en(&self) -> IntBIntEnR {
        IntBIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn int_c_int_en(&self) -> IntCIntEnR {
        IntCIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn retune_int_en(&self) -> RetuneIntEnR {
        RetuneIntEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmd_complete_en(&mut self) -> CmdCompleteEnW<'_, SdNormalIntStatusEnSpec> {
        CmdCompleteEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_complete_en(&mut self) -> XferCompleteEnW<'_, SdNormalIntStatusEnSpec> {
        XferCompleteEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn block_gap_evt_en(&mut self) -> BlockGapEvtEnW<'_, SdNormalIntStatusEnSpec> {
        BlockGapEvtEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_int_en(&mut self) -> DmaIntEnW<'_, SdNormalIntStatusEnSpec> {
        DmaIntEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_rdy_en(&mut self) -> TxRdyEnW<'_, SdNormalIntStatusEnSpec> {
        TxRdyEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rd_rdy_en(&mut self) -> RdRdyEnW<'_, SdNormalIntStatusEnSpec> {
        RdRdyEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn card_ins_en(&mut self) -> CardInsEnW<'_, SdNormalIntStatusEnSpec> {
        CardInsEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn card_rem_en(&mut self) -> CardRemEnW<'_, SdNormalIntStatusEnSpec> {
        CardRemEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn card_int_en(&mut self) -> CardIntEnW<'_, SdNormalIntStatusEnSpec> {
        CardIntEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn int_a_int_en(&mut self) -> IntAIntEnW<'_, SdNormalIntStatusEnSpec> {
        IntAIntEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn int_b_int_en(&mut self) -> IntBIntEnW<'_, SdNormalIntStatusEnSpec> {
        IntBIntEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn int_c_int_en(&mut self) -> IntCIntEnW<'_, SdNormalIntStatusEnSpec> {
        IntCIntEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn retune_int_en(&mut self) -> RetuneIntEnW<'_, SdNormalIntStatusEnSpec> {
        RetuneIntEnW::new(self, 12)
    }
}
#[doc = "Normal Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_normal_int_status_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_normal_int_status_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdNormalIntStatusEnSpec;
impl crate::RegisterSpec for SdNormalIntStatusEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_normal_int_status_en::R`](R) reader structure"]
impl crate::Readable for SdNormalIntStatusEnSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_normal_int_status_en::W`](W) writer structure"]
impl crate::Writable for SdNormalIntStatusEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_NORMAL_INT_STATUS_EN to value 0"]
impl crate::Resettable for SdNormalIntStatusEnSpec {}
