#[doc = "Register `PSRAM_INTF_DELAY_CTRL3` reader"]
pub type R = crate::R<PsramIntfDelayCtrl3Spec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRL3` writer"]
pub type W = crate::W<PsramIntfDelayCtrl3Spec>;
#[doc = "Field `reg_delay_sel_i_adq1` reader - Delay select for input ADQ pin 1"]
pub type RegDelaySelIAdq1R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq1` writer - Delay select for input ADQ pin 1"]
pub type RegDelaySelIAdq1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_i_adq0` reader - Delay select for input ADQ pin 0"]
pub type RegDelaySelIAdq0R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq0` writer - Delay select for input ADQ pin 0"]
pub type RegDelaySelIAdq0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq7` reader - Delay select for output ADQ pin 7"]
pub type RegDelaySelOAdq7R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq7` writer - Delay select for output ADQ pin 7"]
pub type RegDelaySelOAdq7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq6` reader - Delay select for output ADQ pin 6"]
pub type RegDelaySelOAdq6R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq6` writer - Delay select for output ADQ pin 6"]
pub type RegDelaySelOAdq6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay select for input ADQ pin 1"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq1(&self) -> RegDelaySelIAdq1R {
        RegDelaySelIAdq1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay select for input ADQ pin 0"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq0(&self) -> RegDelaySelIAdq0R {
        RegDelaySelIAdq0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ pin 7"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq7(&self) -> RegDelaySelOAdq7R {
        RegDelaySelOAdq7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for output ADQ pin 6"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq6(&self) -> RegDelaySelOAdq6R {
        RegDelaySelOAdq6R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay select for input ADQ pin 1"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq1(&mut self) -> RegDelaySelIAdq1W<'_, PsramIntfDelayCtrl3Spec> {
        RegDelaySelIAdq1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay select for input ADQ pin 0"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq0(&mut self) -> RegDelaySelIAdq0W<'_, PsramIntfDelayCtrl3Spec> {
        RegDelaySelIAdq0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ pin 7"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq7(&mut self) -> RegDelaySelOAdq7W<'_, PsramIntfDelayCtrl3Spec> {
        RegDelaySelOAdq7W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for output ADQ pin 6"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq6(&mut self) -> RegDelaySelOAdq6W<'_, PsramIntfDelayCtrl3Spec> {
        RegDelaySelOAdq6W::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register 3 (ADQ1-0 In, ADQ7-6 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrl3Spec;
impl crate::RegisterSpec for PsramIntfDelayCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrl3::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrl3::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRL3 to value 0"]
impl crate::Resettable for PsramIntfDelayCtrl3Spec {}
