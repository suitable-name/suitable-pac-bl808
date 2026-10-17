#[doc = "Register `PSRAM_INTF_DELAY_CTRL7` reader"]
pub type R = crate::R<PsramIntfDelayCtrl7Spec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRL7` writer"]
pub type W = crate::W<PsramIntfDelayCtrl7Spec>;
#[doc = "Field `reg_delay_sel_o_adq13` reader - Delay select for output ADQ pin 13"]
pub type RegDelaySelOAdq13R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq13` writer - Delay select for output ADQ pin 13"]
pub type RegDelaySelOAdq13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq12` reader - Delay select for output ADQ pin 12"]
pub type RegDelaySelOAdq12R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq12` writer - Delay select for output ADQ pin 12"]
pub type RegDelaySelOAdq12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq11` reader - Delay select for output ADQ pin 11"]
pub type RegDelaySelOAdq11R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq11` writer - Delay select for output ADQ pin 11"]
pub type RegDelaySelOAdq11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_o_adq10` reader - Delay select for output ADQ pin 10"]
pub type RegDelaySelOAdq10R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_o_adq10` writer - Delay select for output ADQ pin 10"]
pub type RegDelaySelOAdq10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Delay select for output ADQ pin 13"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq13(&self) -> RegDelaySelOAdq13R {
        RegDelaySelOAdq13R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay select for output ADQ pin 12"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq12(&self) -> RegDelaySelOAdq12R {
        RegDelaySelOAdq12R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ pin 11"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq11(&self) -> RegDelaySelOAdq11R {
        RegDelaySelOAdq11R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for output ADQ pin 10"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq10(&self) -> RegDelaySelOAdq10R {
        RegDelaySelOAdq10R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay select for output ADQ pin 13"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq13(&mut self) -> RegDelaySelOAdq13W<'_, PsramIntfDelayCtrl7Spec> {
        RegDelaySelOAdq13W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Delay select for output ADQ pin 12"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq12(&mut self) -> RegDelaySelOAdq12W<'_, PsramIntfDelayCtrl7Spec> {
        RegDelaySelOAdq12W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Delay select for output ADQ pin 11"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq11(&mut self) -> RegDelaySelOAdq11W<'_, PsramIntfDelayCtrl7Spec> {
        RegDelaySelOAdq11W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for output ADQ pin 10"]
    #[inline(always)]
    pub fn reg_delay_sel_o_adq10(&mut self) -> RegDelaySelOAdq10W<'_, PsramIntfDelayCtrl7Spec> {
        RegDelaySelOAdq10W::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register 7 (ADQ13-10 Out)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrl7Spec;
impl crate::RegisterSpec for PsramIntfDelayCtrl7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrl7::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrl7Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrl7::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrl7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRL7 to value 0"]
impl crate::Resettable for PsramIntfDelayCtrl7Spec {}
