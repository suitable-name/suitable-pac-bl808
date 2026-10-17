#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL8` reader"]
pub type R = crate::R<PsramRoughDelayCtrl8Spec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL8` writer"]
pub type W = crate::W<PsramRoughDelayCtrl8Spec>;
#[doc = "Field `reg_rough_sel_i_adq9` reader - Rough delay select for input ADQ pin 9"]
pub type RegRoughSelIAdq9R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq9` writer - Rough delay select for input ADQ pin 9"]
pub type RegRoughSelIAdq9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_i_adq8` reader - Rough delay select for input ADQ pin 8"]
pub type RegRoughSelIAdq8R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq8` writer - Rough delay select for input ADQ pin 8"]
pub type RegRoughSelIAdq8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq15` reader - Rough delay select for output ADQ pin 15"]
pub type RegRoughSelOAdq15R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq15` writer - Rough delay select for output ADQ pin 15"]
pub type RegRoughSelOAdq15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq14` reader - Rough delay select for output ADQ pin 14"]
pub type RegRoughSelOAdq14R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq14` writer - Rough delay select for output ADQ pin 14"]
pub type RegRoughSelOAdq14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rough delay select for input ADQ pin 9"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq9(&self) -> RegRoughSelIAdq9R {
        RegRoughSelIAdq9R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rough delay select for input ADQ pin 8"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq8(&self) -> RegRoughSelIAdq8R {
        RegRoughSelIAdq8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ pin 15"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq15(&self) -> RegRoughSelOAdq15R {
        RegRoughSelOAdq15R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for output ADQ pin 14"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq14(&self) -> RegRoughSelOAdq14R {
        RegRoughSelOAdq14R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rough delay select for input ADQ pin 9"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq9(&mut self) -> RegRoughSelIAdq9W<'_, PsramRoughDelayCtrl8Spec> {
        RegRoughSelIAdq9W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rough delay select for input ADQ pin 8"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq8(&mut self) -> RegRoughSelIAdq8W<'_, PsramRoughDelayCtrl8Spec> {
        RegRoughSelIAdq8W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ pin 15"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq15(&mut self) -> RegRoughSelOAdq15W<'_, PsramRoughDelayCtrl8Spec> {
        RegRoughSelOAdq15W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for output ADQ pin 14"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq14(&mut self) -> RegRoughSelOAdq14W<'_, PsramRoughDelayCtrl8Spec> {
        RegRoughSelOAdq14W::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register 8 (ADQ9-8 In, ADQ15-14 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrl8Spec;
impl crate::RegisterSpec for PsramRoughDelayCtrl8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrl8::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrl8Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrl8::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrl8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRL8 to value 0"]
impl crate::Resettable for PsramRoughDelayCtrl8Spec {}
