#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL7` reader"]
pub type R = crate::R<PsramRoughDelayCtrl7Spec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL7` writer"]
pub type W = crate::W<PsramRoughDelayCtrl7Spec>;
#[doc = "Field `reg_rough_sel_o_adq13` reader - Rough delay select for output ADQ pin 13"]
pub type RegRoughSelOAdq13R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq13` writer - Rough delay select for output ADQ pin 13"]
pub type RegRoughSelOAdq13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq12` reader - Rough delay select for output ADQ pin 12"]
pub type RegRoughSelOAdq12R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq12` writer - Rough delay select for output ADQ pin 12"]
pub type RegRoughSelOAdq12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq11` reader - Rough delay select for output ADQ pin 11"]
pub type RegRoughSelOAdq11R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq11` writer - Rough delay select for output ADQ pin 11"]
pub type RegRoughSelOAdq11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq10` reader - Rough delay select for output ADQ pin 10"]
pub type RegRoughSelOAdq10R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq10` writer - Rough delay select for output ADQ pin 10"]
pub type RegRoughSelOAdq10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rough delay select for output ADQ pin 13"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq13(&self) -> RegRoughSelOAdq13R {
        RegRoughSelOAdq13R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rough delay select for output ADQ pin 12"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq12(&self) -> RegRoughSelOAdq12R {
        RegRoughSelOAdq12R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ pin 11"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq11(&self) -> RegRoughSelOAdq11R {
        RegRoughSelOAdq11R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for output ADQ pin 10"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq10(&self) -> RegRoughSelOAdq10R {
        RegRoughSelOAdq10R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rough delay select for output ADQ pin 13"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq13(&mut self) -> RegRoughSelOAdq13W<'_, PsramRoughDelayCtrl7Spec> {
        RegRoughSelOAdq13W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rough delay select for output ADQ pin 12"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq12(&mut self) -> RegRoughSelOAdq12W<'_, PsramRoughDelayCtrl7Spec> {
        RegRoughSelOAdq12W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ pin 11"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq11(&mut self) -> RegRoughSelOAdq11W<'_, PsramRoughDelayCtrl7Spec> {
        RegRoughSelOAdq11W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for output ADQ pin 10"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq10(&mut self) -> RegRoughSelOAdq10W<'_, PsramRoughDelayCtrl7Spec> {
        RegRoughSelOAdq10W::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register 7 (ADQ13-10 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrl7Spec;
impl crate::RegisterSpec for PsramRoughDelayCtrl7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrl7::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrl7Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrl7::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrl7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRL7 to value 0"]
impl crate::Resettable for PsramRoughDelayCtrl7Spec {}
