#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL6` reader"]
pub type R = crate::R<PsramRoughDelayCtrl6Spec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL6` writer"]
pub type W = crate::W<PsramRoughDelayCtrl6Spec>;
#[doc = "Field `reg_rough_sel_o_adq9` reader - Rough delay select for output ADQ pin 9"]
pub type RegRoughSelOAdq9R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq9` writer - Rough delay select for output ADQ pin 9"]
pub type RegRoughSelOAdq9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq8` reader - Rough delay select for output ADQ pin 8"]
pub type RegRoughSelOAdq8R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq8` writer - Rough delay select for output ADQ pin 8"]
pub type RegRoughSelOAdq8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq_oen1` reader - Rough delay select for output ADQ enable for slice 1"]
pub type RegRoughSelOAdqOen1R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq_oen1` writer - Rough delay select for output ADQ enable for slice 1"]
pub type RegRoughSelOAdqOen1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_dqs1` reader - Rough delay select for output DQS pin 1"]
pub type RegRoughSelODqs1R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_dqs1` writer - Rough delay select for output DQS pin 1"]
pub type RegRoughSelODqs1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rough delay select for output ADQ pin 9"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq9(&self) -> RegRoughSelOAdq9R {
        RegRoughSelOAdq9R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rough delay select for output ADQ pin 8"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq8(&self) -> RegRoughSelOAdq8R {
        RegRoughSelOAdq8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ enable for slice 1"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq_oen1(&self) -> RegRoughSelOAdqOen1R {
        RegRoughSelOAdqOen1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for output DQS pin 1"]
    #[inline(always)]
    pub fn reg_rough_sel_o_dqs1(&self) -> RegRoughSelODqs1R {
        RegRoughSelODqs1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rough delay select for output ADQ pin 9"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq9(&mut self) -> RegRoughSelOAdq9W<'_, PsramRoughDelayCtrl6Spec> {
        RegRoughSelOAdq9W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rough delay select for output ADQ pin 8"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq8(&mut self) -> RegRoughSelOAdq8W<'_, PsramRoughDelayCtrl6Spec> {
        RegRoughSelOAdq8W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ enable for slice 1"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq_oen1(
        &mut self,
    ) -> RegRoughSelOAdqOen1W<'_, PsramRoughDelayCtrl6Spec> {
        RegRoughSelOAdqOen1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for output DQS pin 1"]
    #[inline(always)]
    pub fn reg_rough_sel_o_dqs1(&mut self) -> RegRoughSelODqs1W<'_, PsramRoughDelayCtrl6Spec> {
        RegRoughSelODqs1W::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register 6 (ADQ9-8 Out, ADQ_OEN1, DQS1 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrl6Spec;
impl crate::RegisterSpec for PsramRoughDelayCtrl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrl6::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrl6Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrl6::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrl6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRL6 to value 0"]
impl crate::Resettable for PsramRoughDelayCtrl6Spec {}
