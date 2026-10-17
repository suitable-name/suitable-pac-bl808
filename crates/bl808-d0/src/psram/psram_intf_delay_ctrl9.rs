#[doc = "Register `PSRAM_INTF_DELAY_CTRL9` reader"]
pub type R = crate::R<PsramIntfDelayCtrl9Spec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRL9` writer"]
pub type W = crate::W<PsramIntfDelayCtrl9Spec>;
#[doc = "Field `reg_delay_sel_i_adq13` reader - Delay select for input ADQ pin 13"]
pub type RegDelaySelIAdq13R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq13` writer - Delay select for input ADQ pin 13"]
pub type RegDelaySelIAdq13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_i_adq12` reader - Delay select for input ADQ pin 12"]
pub type RegDelaySelIAdq12R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq12` writer - Delay select for input ADQ pin 12"]
pub type RegDelaySelIAdq12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_i_adq11` reader - Delay select for input ADQ pin 11"]
pub type RegDelaySelIAdq11R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq11` writer - Delay select for input ADQ pin 11"]
pub type RegDelaySelIAdq11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_i_adq10` reader - Delay select for input ADQ pin 10"]
pub type RegDelaySelIAdq10R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq10` writer - Delay select for input ADQ pin 10"]
pub type RegDelaySelIAdq10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay select for input ADQ pin 13"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq13(&self) -> RegDelaySelIAdq13R {
        RegDelaySelIAdq13R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay select for input ADQ pin 12"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq12(&self) -> RegDelaySelIAdq12R {
        RegDelaySelIAdq12R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay select for input ADQ pin 11"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq11(&self) -> RegDelaySelIAdq11R {
        RegDelaySelIAdq11R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for input ADQ pin 10"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq10(&self) -> RegDelaySelIAdq10R {
        RegDelaySelIAdq10R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay select for input ADQ pin 13"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq13(&mut self) -> RegDelaySelIAdq13W<'_, PsramIntfDelayCtrl9Spec> {
        RegDelaySelIAdq13W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay select for input ADQ pin 12"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq12(&mut self) -> RegDelaySelIAdq12W<'_, PsramIntfDelayCtrl9Spec> {
        RegDelaySelIAdq12W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay select for input ADQ pin 11"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq11(&mut self) -> RegDelaySelIAdq11W<'_, PsramIntfDelayCtrl9Spec> {
        RegDelaySelIAdq11W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for input ADQ pin 10"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq10(&mut self) -> RegDelaySelIAdq10W<'_, PsramIntfDelayCtrl9Spec> {
        RegDelaySelIAdq10W::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register 9 (ADQ13-10 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrl9Spec;
impl crate::RegisterSpec for PsramIntfDelayCtrl9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrl9::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrl9Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrl9::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrl9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRL9 to value 0"]
impl crate::Resettable for PsramIntfDelayCtrl9Spec {}
