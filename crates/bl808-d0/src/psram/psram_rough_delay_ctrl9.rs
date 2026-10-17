#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL9` reader"]
pub type R = crate::R<PsramRoughDelayCtrl9Spec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL9` writer"]
pub type W = crate::W<PsramRoughDelayCtrl9Spec>;
#[doc = "Field `reg_rough_sel_i_adq13` reader - Rough delay select for input ADQ pin 13"]
pub type RegRoughSelIAdq13R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq13` writer - Rough delay select for input ADQ pin 13"]
pub type RegRoughSelIAdq13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_i_adq12` reader - Rough delay select for input ADQ pin 12"]
pub type RegRoughSelIAdq12R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq12` writer - Rough delay select for input ADQ pin 12"]
pub type RegRoughSelIAdq12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_i_adq11` reader - Rough delay select for input ADQ pin 11"]
pub type RegRoughSelIAdq11R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq11` writer - Rough delay select for input ADQ pin 11"]
pub type RegRoughSelIAdq11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_i_adq10` reader - Rough delay select for input ADQ pin 10"]
pub type RegRoughSelIAdq10R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq10` writer - Rough delay select for input ADQ pin 10"]
pub type RegRoughSelIAdq10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rough delay select for input ADQ pin 13"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq13(&self) -> RegRoughSelIAdq13R {
        RegRoughSelIAdq13R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rough delay select for input ADQ pin 12"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq12(&self) -> RegRoughSelIAdq12R {
        RegRoughSelIAdq12R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Rough delay select for input ADQ pin 11"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq11(&self) -> RegRoughSelIAdq11R {
        RegRoughSelIAdq11R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for input ADQ pin 10"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq10(&self) -> RegRoughSelIAdq10R {
        RegRoughSelIAdq10R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rough delay select for input ADQ pin 13"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq13(&mut self) -> RegRoughSelIAdq13W<'_, PsramRoughDelayCtrl9Spec> {
        RegRoughSelIAdq13W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rough delay select for input ADQ pin 12"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq12(&mut self) -> RegRoughSelIAdq12W<'_, PsramRoughDelayCtrl9Spec> {
        RegRoughSelIAdq12W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Rough delay select for input ADQ pin 11"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq11(&mut self) -> RegRoughSelIAdq11W<'_, PsramRoughDelayCtrl9Spec> {
        RegRoughSelIAdq11W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for input ADQ pin 10"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq10(&mut self) -> RegRoughSelIAdq10W<'_, PsramRoughDelayCtrl9Spec> {
        RegRoughSelIAdq10W::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register 9 (ADQ13-10 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrl9Spec;
impl crate::RegisterSpec for PsramRoughDelayCtrl9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrl9::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrl9Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrl9::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrl9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRL9 to value 0"]
impl crate::Resettable for PsramRoughDelayCtrl9Spec {}
