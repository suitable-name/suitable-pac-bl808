#[doc = "Register `PSRAM_ROUGH_DELAY_CTRLB` reader"]
pub type R = crate::R<PsramRoughDelayCtrlbSpec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRLB` writer"]
pub type W = crate::W<PsramRoughDelayCtrlbSpec>;
#[doc = "Field `reg_rough_sel_o_dqs_mask` reader - Rough delay select for output DQS mask"]
pub type RegRoughSelODqsMaskR = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_dqs_mask` writer - Rough delay select for output DQS mask"]
pub type RegRoughSelODqsMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_dqs_oen1` reader - Rough delay select for output DQS enable for slice 1"]
pub type RegRoughSelODqsOen1R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_dqs_oen1` writer - Rough delay select for output DQS enable for slice 1"]
pub type RegRoughSelODqsOen1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rough delay select for output DQS mask"]
    #[inline(always)]
    pub fn reg_rough_sel_o_dqs_mask(&self) -> RegRoughSelODqsMaskR {
        RegRoughSelODqsMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rough delay select for output DQS enable for slice 1"]
    #[inline(always)]
    pub fn reg_rough_sel_o_dqs_oen1(&self) -> RegRoughSelODqsOen1R {
        RegRoughSelODqsOen1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rough delay select for output DQS mask"]
    #[inline(always)]
    pub fn reg_rough_sel_o_dqs_mask(
        &mut self,
    ) -> RegRoughSelODqsMaskW<'_, PsramRoughDelayCtrlbSpec> {
        RegRoughSelODqsMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rough delay select for output DQS enable for slice 1"]
    #[inline(always)]
    pub fn reg_rough_sel_o_dqs_oen1(
        &mut self,
    ) -> RegRoughSelODqsOen1W<'_, PsramRoughDelayCtrlbSpec> {
        RegRoughSelODqsOen1W::new(self, 8)
    }
}
#[doc = "PSRAM Rough Delay Control Register B (DQS Mask, DQS_OEN1)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrlbSpec;
impl crate::RegisterSpec for PsramRoughDelayCtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrlb::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrlb::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRLB to value 0"]
impl crate::Resettable for PsramRoughDelayCtrlbSpec {}
