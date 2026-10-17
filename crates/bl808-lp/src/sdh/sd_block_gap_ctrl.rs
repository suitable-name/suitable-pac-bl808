#[doc = "Register `SD_BLOCK_GAP_CTRL` reader"]
pub type R = crate::R<SdBlockGapCtrlSpec>;
#[doc = "Register `SD_BLOCK_GAP_CTRL` writer"]
pub type W = crate::W<SdBlockGapCtrlSpec>;
#[doc = "Field `STOP_AT_BLOCK_GAP_REQ` reader - "]
pub type StopAtBlockGapReqR = crate::BitReader;
#[doc = "Field `STOP_AT_BLOCK_GAP_REQ` writer - "]
pub type StopAtBlockGapReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT_REQ` reader - "]
pub type ContReqR = crate::BitReader;
#[doc = "Field `CONT_REQ` writer - "]
pub type ContReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_WAIT_CTL` reader - "]
pub type RdWaitCtlR = crate::BitReader;
#[doc = "Field `RD_WAIT_CTL` writer - "]
pub type RdWaitCtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_BLK_GAP` reader - "]
pub type IntBlkGapR = crate::BitReader;
#[doc = "Field `INT_BLK_GAP` writer - "]
pub type IntBlkGapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_CARD_INT` reader - "]
pub type WCardIntR = crate::BitReader;
#[doc = "Field `W_CARD_INT` writer - "]
pub type WCardIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_INSERTION` reader - "]
pub type WInsertionR = crate::BitReader;
#[doc = "Field `W_INSERTION` writer - "]
pub type WInsertionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_REMOVAL` reader - "]
pub type WRemovalR = crate::BitReader;
#[doc = "Field `W_REMOVAL` writer - "]
pub type WRemovalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stop_at_block_gap_req(&self) -> StopAtBlockGapReqR {
        StopAtBlockGapReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cont_req(&self) -> ContReqR {
        ContReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rd_wait_ctl(&self) -> RdWaitCtlR {
        RdWaitCtlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int_blk_gap(&self) -> IntBlkGapR {
        IntBlkGapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn w_card_int(&self) -> WCardIntR {
        WCardIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn w_insertion(&self) -> WInsertionR {
        WInsertionR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn w_removal(&self) -> WRemovalR {
        WRemovalR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stop_at_block_gap_req(&mut self) -> StopAtBlockGapReqW<'_, SdBlockGapCtrlSpec> {
        StopAtBlockGapReqW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cont_req(&mut self) -> ContReqW<'_, SdBlockGapCtrlSpec> {
        ContReqW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rd_wait_ctl(&mut self) -> RdWaitCtlW<'_, SdBlockGapCtrlSpec> {
        RdWaitCtlW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int_blk_gap(&mut self) -> IntBlkGapW<'_, SdBlockGapCtrlSpec> {
        IntBlkGapW::new(self, 3)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn w_card_int(&mut self) -> WCardIntW<'_, SdBlockGapCtrlSpec> {
        WCardIntW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn w_insertion(&mut self) -> WInsertionW<'_, SdBlockGapCtrlSpec> {
        WInsertionW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn w_removal(&mut self) -> WRemovalW<'_, SdBlockGapCtrlSpec> {
        WRemovalW::new(self, 10)
    }
}
#[doc = "Block Gap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_block_gap_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_block_gap_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdBlockGapCtrlSpec;
impl crate::RegisterSpec for SdBlockGapCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_block_gap_ctrl::R`](R) reader structure"]
impl crate::Readable for SdBlockGapCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_block_gap_ctrl::W`](W) writer structure"]
impl crate::Writable for SdBlockGapCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_BLOCK_GAP_CTRL to value 0"]
impl crate::Resettable for SdBlockGapCtrlSpec {}
