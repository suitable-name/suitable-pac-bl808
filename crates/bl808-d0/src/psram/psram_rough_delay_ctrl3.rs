#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL3` reader"]
pub type R = crate::R<PsramRoughDelayCtrl3Spec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL3` writer"]
pub type W = crate::W<PsramRoughDelayCtrl3Spec>;
#[doc = "Field `reg_rough_sel_i_adq1` reader - Rough delay select for input ADQ pin 1"]
pub type RegRoughSelIAdq1R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq1` writer - Rough delay select for input ADQ pin 1"]
pub type RegRoughSelIAdq1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_i_adq0` reader - Rough delay select for input ADQ pin 0"]
pub type RegRoughSelIAdq0R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq0` writer - Rough delay select for input ADQ pin 0"]
pub type RegRoughSelIAdq0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq7` reader - Rough delay select for output ADQ pin 7"]
pub type RegRoughSelOAdq7R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq7` writer - Rough delay select for output ADQ pin 7"]
pub type RegRoughSelOAdq7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_o_adq6` reader - Rough delay select for output ADQ pin 6"]
pub type RegRoughSelOAdq6R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_o_adq6` writer - Rough delay select for output ADQ pin 6"]
pub type RegRoughSelOAdq6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rough delay select for input ADQ pin 1"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq1(&self) -> RegRoughSelIAdq1R {
        RegRoughSelIAdq1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rough delay select for input ADQ pin 0"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq0(&self) -> RegRoughSelIAdq0R {
        RegRoughSelIAdq0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ pin 7"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq7(&self) -> RegRoughSelOAdq7R {
        RegRoughSelOAdq7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for output ADQ pin 6"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq6(&self) -> RegRoughSelOAdq6R {
        RegRoughSelOAdq6R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rough delay select for input ADQ pin 1"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq1(&mut self) -> RegRoughSelIAdq1W<'_, PsramRoughDelayCtrl3Spec> {
        RegRoughSelIAdq1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rough delay select for input ADQ pin 0"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq0(&mut self) -> RegRoughSelIAdq0W<'_, PsramRoughDelayCtrl3Spec> {
        RegRoughSelIAdq0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Rough delay select for output ADQ pin 7"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq7(&mut self) -> RegRoughSelOAdq7W<'_, PsramRoughDelayCtrl3Spec> {
        RegRoughSelOAdq7W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for output ADQ pin 6"]
    #[inline(always)]
    pub fn reg_rough_sel_o_adq6(&mut self) -> RegRoughSelOAdq6W<'_, PsramRoughDelayCtrl3Spec> {
        RegRoughSelOAdq6W::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register 3 (ADQ1-0 In, ADQ7-6 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrl3Spec;
impl crate::RegisterSpec for PsramRoughDelayCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrl3::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrl3::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRL3 to value 0"]
impl crate::Resettable for PsramRoughDelayCtrl3Spec {}
