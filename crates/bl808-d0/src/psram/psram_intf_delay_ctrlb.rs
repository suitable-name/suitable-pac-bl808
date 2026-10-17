#[doc = "Register `PSRAM_INTF_DELAY_CTRLB` reader"]
pub type R = crate::R<PsramIntfDelayCtrlbSpec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRLB` writer"]
pub type W = crate::W<PsramIntfDelayCtrlbSpec>;
#[doc = "Field `reg_delay_sel_o_dqs_mask` reader - Delay select for output DQS mask"]
pub type RegDelaySelODqsMaskR = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_dqs_mask` writer - Delay select for output DQS mask"]
pub type RegDelaySelODqsMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_dqs_oen1` reader - Delay select for output DQS enable for slice 1"]
pub type RegDelaySelODqsOen1R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_dqs_oen1` writer - Delay select for output DQS enable for slice 1"]
pub type RegDelaySelODqsOen1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay select for output DQS mask"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs_mask(&self) -> RegDelaySelODqsMaskR {
        RegDelaySelODqsMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay select for output DQS enable for slice 1"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs_oen1(&self) -> RegDelaySelODqsOen1R {
        RegDelaySelODqsOen1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay select for output DQS mask"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs_mask(
        &mut self,
    ) -> RegDelaySelODqsMaskW<'_, PsramIntfDelayCtrlbSpec> {
        RegDelaySelODqsMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay select for output DQS enable for slice 1"]
    #[inline(always)]
    pub fn reg_delay_sel_o_dqs_oen1(
        &mut self,
    ) -> RegDelaySelODqsOen1W<'_, PsramIntfDelayCtrlbSpec> {
        RegDelaySelODqsOen1W::new(self, 8)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register B (DQS Mask, DQS_OEN1)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrlbSpec;
impl crate::RegisterSpec for PsramIntfDelayCtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrlb::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrlb::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRLB to value 0"]
impl crate::Resettable for PsramIntfDelayCtrlbSpec {}
