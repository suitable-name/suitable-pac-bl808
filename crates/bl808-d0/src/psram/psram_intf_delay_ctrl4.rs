#[doc = "Register `PSRAM_INTF_DELAY_CTRL4` reader"]
pub type R = crate::R<PsramIntfDelayCtrl4Spec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRL4` writer"]
pub type W = crate::W<PsramIntfDelayCtrl4Spec>;
#[doc = "Field `reg_delay_sel_i_adq5` reader - Delay select for input ADQ pin 5"]
pub type RegDelaySelIAdq5R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq5` writer - Delay select for input ADQ pin 5"]
pub type RegDelaySelIAdq5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_i_adq4` reader - Delay select for input ADQ pin 4"]
pub type RegDelaySelIAdq4R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq4` writer - Delay select for input ADQ pin 4"]
pub type RegDelaySelIAdq4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_i_adq3` reader - Delay select for input ADQ pin 3"]
pub type RegDelaySelIAdq3R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq3` writer - Delay select for input ADQ pin 3"]
pub type RegDelaySelIAdq3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_i_adq2` reader - Delay select for input ADQ pin 2"]
pub type RegDelaySelIAdq2R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq2` writer - Delay select for input ADQ pin 2"]
pub type RegDelaySelIAdq2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay select for input ADQ pin 5"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq5(&self) -> RegDelaySelIAdq5R {
        RegDelaySelIAdq5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay select for input ADQ pin 4"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq4(&self) -> RegDelaySelIAdq4R {
        RegDelaySelIAdq4R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay select for input ADQ pin 3"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq3(&self) -> RegDelaySelIAdq3R {
        RegDelaySelIAdq3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for input ADQ pin 2"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq2(&self) -> RegDelaySelIAdq2R {
        RegDelaySelIAdq2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay select for input ADQ pin 5"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq5(&mut self) -> RegDelaySelIAdq5W<'_, PsramIntfDelayCtrl4Spec> {
        RegDelaySelIAdq5W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay select for input ADQ pin 4"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq4(&mut self) -> RegDelaySelIAdq4W<'_, PsramIntfDelayCtrl4Spec> {
        RegDelaySelIAdq4W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay select for input ADQ pin 3"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq3(&mut self) -> RegDelaySelIAdq3W<'_, PsramIntfDelayCtrl4Spec> {
        RegDelaySelIAdq3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for input ADQ pin 2"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq2(&mut self) -> RegDelaySelIAdq2W<'_, PsramIntfDelayCtrl4Spec> {
        RegDelaySelIAdq2W::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register 4 (ADQ5-2 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrl4Spec;
impl crate::RegisterSpec for PsramIntfDelayCtrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrl4::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrl4::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrl4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRL4 to value 0"]
impl crate::Resettable for PsramIntfDelayCtrl4Spec {}
