#[doc = "Register `PSRAM_INTF_DELAY_CTRL2` reader"]
pub type R = crate::R<PsramIntfDelayCtrl2Spec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRL2` writer"]
pub type W = crate::W<PsramIntfDelayCtrl2Spec>;
#[doc = "Field `reg_delay_sel_o_adq5` reader - Delay select for output ADQ pin 5"]
pub type RegDelaySelOAdq5R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq5` writer - Delay select for output ADQ pin 5"]
pub type RegDelaySelOAdq5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq4` reader - Delay select for output ADQ pin 4"]
pub type RegDelaySelOAdq4R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq4` writer - Delay select for output ADQ pin 4"]
pub type RegDelaySelOAdq4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq3` reader - Delay select for output ADQ pin 3"]
pub type RegDelaySelOAdq3R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq3` writer - Delay select for output ADQ pin 3"]
pub type RegDelaySelOAdq3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq2` reader - Delay select for output ADQ pin 2"]
pub type RegDelaySelOAdq2R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq2` writer - Delay select for output ADQ pin 2"]
pub type RegDelaySelOAdq2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay select for output ADQ pin 5"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq5(&self) -> RegDelaySelOAdq5R {
        RegDelaySelOAdq5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay select for output ADQ pin 4"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq4(&self) -> RegDelaySelOAdq4R {
        RegDelaySelOAdq4R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ pin 3"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq3(&self) -> RegDelaySelOAdq3R {
        RegDelaySelOAdq3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for output ADQ pin 2"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq2(&self) -> RegDelaySelOAdq2R {
        RegDelaySelOAdq2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay select for output ADQ pin 5"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq5(&mut self) -> RegDelaySelOAdq5W<'_, PsramIntfDelayCtrl2Spec> {
        RegDelaySelOAdq5W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay select for output ADQ pin 4"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq4(&mut self) -> RegDelaySelOAdq4W<'_, PsramIntfDelayCtrl2Spec> {
        RegDelaySelOAdq4W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ pin 3"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq3(&mut self) -> RegDelaySelOAdq3W<'_, PsramIntfDelayCtrl2Spec> {
        RegDelaySelOAdq3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for output ADQ pin 2"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq2(&mut self) -> RegDelaySelOAdq2W<'_, PsramIntfDelayCtrl2Spec> {
        RegDelaySelOAdq2W::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register 2 (ADQ5-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrl2Spec;
impl crate::RegisterSpec for PsramIntfDelayCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrl2::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrl2::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRL2 to value 0"]
impl crate::Resettable for PsramIntfDelayCtrl2Spec {}
