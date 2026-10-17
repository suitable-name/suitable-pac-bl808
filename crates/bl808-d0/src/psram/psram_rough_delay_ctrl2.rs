#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL2` reader"]
pub type R = crate::R<PsramRoughDelayCtrl2Spec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL2` writer"]
pub type W = crate::W<PsramRoughDelayCtrl2Spec>;
#[doc = "Field `reg_rough_sel_o_adq5` reader - Rough delay select for output ADQ pin 5"]
pub type RegRoughSelOAdq5R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq5` writer - Rough delay select for output ADQ pin 5"]
pub type RegRoughSelOAdq5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq4` reader - Rough delay select for output ADQ pin 4"]
pub type RegRoughSelOAdq4R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq4` writer - Rough delay select for output ADQ pin 4"]
pub type RegRoughSelOAdq4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq3` reader - Rough delay select for output ADQ pin 3"]
pub type RegRoughSelOAdq3R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq3` writer - Rough delay select for output ADQ pin 3"]
pub type RegRoughSelOAdq3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq2` reader - Rough delay select for output ADQ pin 2"]
pub type RegRoughSelOAdq2R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq2` writer - Rough delay select for output ADQ pin 2"]
pub type RegRoughSelOAdq2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rough delay select for output ADQ pin 5"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq5(&self) -> RegRoughSelOAdq5R {
        RegRoughSelOAdq5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rough delay select for output ADQ pin 4"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq4(&self) -> RegRoughSelOAdq4R {
        RegRoughSelOAdq4R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ pin 3"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq3(&self) -> RegRoughSelOAdq3R {
        RegRoughSelOAdq3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for output ADQ pin 2"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq2(&self) -> RegRoughSelOAdq2R {
        RegRoughSelOAdq2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rough delay select for output ADQ pin 5"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq5(&mut self) -> RegRoughSelOAdq5W<'_, PsramRoughDelayCtrl2Spec> {
        RegRoughSelOAdq5W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rough delay select for output ADQ pin 4"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq4(&mut self) -> RegRoughSelOAdq4W<'_, PsramRoughDelayCtrl2Spec> {
        RegRoughSelOAdq4W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ pin 3"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq3(&mut self) -> RegRoughSelOAdq3W<'_, PsramRoughDelayCtrl2Spec> {
        RegRoughSelOAdq3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for output ADQ pin 2"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq2(&mut self) -> RegRoughSelOAdq2W<'_, PsramRoughDelayCtrl2Spec> {
        RegRoughSelOAdq2W::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register 2 (ADQ5-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrl2Spec;
impl crate::RegisterSpec for PsramRoughDelayCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrl2::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrl2::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRL2 to value 0"]
impl crate::Resettable for PsramRoughDelayCtrl2Spec {}
