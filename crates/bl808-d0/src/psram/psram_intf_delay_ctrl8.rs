#[doc = "Register `PSRAM_INTF_DELAY_CTRL8` reader"]
pub type R = crate::R<PsramIntfDelayCtrl8Spec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRL8` writer"]
pub type W = crate::W<PsramIntfDelayCtrl8Spec>;
#[doc = "Field `reg_delay_sel_i_adq9` reader - Delay select for input ADQ pin 9"]
pub type RegDelaySelIAdq9R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq9` writer - Delay select for input ADQ pin 9"]
pub type RegDelaySelIAdq9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_i_adq8` reader - Delay select for input ADQ pin 8"]
pub type RegDelaySelIAdq8R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq8` writer - Delay select for input ADQ pin 8"]
pub type RegDelaySelIAdq8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq15` reader - Delay select for output ADQ pin 15"]
pub type RegDelaySelOAdq15R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq15` writer - Delay select for output ADQ pin 15"]
pub type RegDelaySelOAdq15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq14` reader - Delay select for output ADQ pin 14"]
pub type RegDelaySelOAdq14R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq14` writer - Delay select for output ADQ pin 14"]
pub type RegDelaySelOAdq14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay select for input ADQ pin 9"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq9(&self) -> RegDelaySelIAdq9R {
        RegDelaySelIAdq9R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay select for input ADQ pin 8"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq8(&self) -> RegDelaySelIAdq8R {
        RegDelaySelIAdq8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ pin 15"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq15(&self) -> RegDelaySelOAdq15R {
        RegDelaySelOAdq15R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for output ADQ pin 14"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq14(&self) -> RegDelaySelOAdq14R {
        RegDelaySelOAdq14R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay select for input ADQ pin 9"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq9(&mut self) -> RegDelaySelIAdq9W<'_, PsramIntfDelayCtrl8Spec> {
        RegDelaySelIAdq9W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay select for input ADQ pin 8"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq8(&mut self) -> RegDelaySelIAdq8W<'_, PsramIntfDelayCtrl8Spec> {
        RegDelaySelIAdq8W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ pin 15"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq15(&mut self) -> RegDelaySelOAdq15W<'_, PsramIntfDelayCtrl8Spec> {
        RegDelaySelOAdq15W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for output ADQ pin 14"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq14(&mut self) -> RegDelaySelOAdq14W<'_, PsramIntfDelayCtrl8Spec> {
        RegDelaySelOAdq14W::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register 8 (ADQ9-8 In, ADQ15-14 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrl8Spec;
impl crate::RegisterSpec for PsramIntfDelayCtrl8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrl8::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrl8Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrl8::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrl8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRL8 to value 0"]
impl crate::Resettable for PsramIntfDelayCtrl8Spec {}
