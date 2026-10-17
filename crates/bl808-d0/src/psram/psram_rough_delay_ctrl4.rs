#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL4` reader"]
pub type R = crate::R<PsramRoughDelayCtrl4Spec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL4` writer"]
pub type W = crate::W<PsramRoughDelayCtrl4Spec>;
#[doc = "Field `reg_rough_sel_i_adq5` reader - Rough delay select for input ADQ pin 5"]
pub type RegRoughSelIAdq5R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq5` writer - Rough delay select for input ADQ pin 5"]
pub type RegRoughSelIAdq5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_i_adq4` reader - Rough delay select for input ADQ pin 4"]
pub type RegRoughSelIAdq4R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq4` writer - Rough delay select for input ADQ pin 4"]
pub type RegRoughSelIAdq4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_i_adq3` reader - Rough delay select for input ADQ pin 3"]
pub type RegRoughSelIAdq3R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq3` writer - Rough delay select for input ADQ pin 3"]
pub type RegRoughSelIAdq3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_i_adq2` reader - Rough delay select for input ADQ pin 2"]
pub type RegRoughSelIAdq2R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq2` writer - Rough delay select for input ADQ pin 2"]
pub type RegRoughSelIAdq2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rough delay select for input ADQ pin 5"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq5(&self) -> RegRoughSelIAdq5R {
        RegRoughSelIAdq5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rough delay select for input ADQ pin 4"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq4(&self) -> RegRoughSelIAdq4R {
        RegRoughSelIAdq4R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Rough delay select for input ADQ pin 3"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq3(&self) -> RegRoughSelIAdq3R {
        RegRoughSelIAdq3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for input ADQ pin 2"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq2(&self) -> RegRoughSelIAdq2R {
        RegRoughSelIAdq2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rough delay select for input ADQ pin 5"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq5(&mut self) -> RegRoughSelIAdq5W<'_, PsramRoughDelayCtrl4Spec> {
        RegRoughSelIAdq5W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rough delay select for input ADQ pin 4"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq4(&mut self) -> RegRoughSelIAdq4W<'_, PsramRoughDelayCtrl4Spec> {
        RegRoughSelIAdq4W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Rough delay select for input ADQ pin 3"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq3(&mut self) -> RegRoughSelIAdq3W<'_, PsramRoughDelayCtrl4Spec> {
        RegRoughSelIAdq3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for input ADQ pin 2"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq2(&mut self) -> RegRoughSelIAdq2W<'_, PsramRoughDelayCtrl4Spec> {
        RegRoughSelIAdq2W::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register 4 (ADQ5-2 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrl4Spec;
impl crate::RegisterSpec for PsramRoughDelayCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrl4::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrl4::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRL4 to value 0"]
impl crate::Resettable for PsramRoughDelayCtrl4Spec {}
