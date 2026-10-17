#[doc = "Register `PSRAM_INTF_DELAY_CTRL5` reader"]
pub type R = crate::R<PsramIntfDelayCtrl5Spec>;
#[doc = "Register `PSRAM_INTF_DELAY_CTRL5` writer"]
pub type W = crate::W<PsramIntfDelayCtrl5Spec>;
#[doc = "Field `reg_delay_sel_i_dqs0` reader - Delay select for input DQS pin 0"]
pub type RegDelaySelIDqs0R = crate::FieldReader<u16>;
#[doc = "Field `reg_delay_sel_i_dqs0` writer - Delay select for input DQS pin 0"]
pub type RegDelaySelIDqs0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_delay_sel_i_adq7` reader - Delay select for input ADQ pin 7"]
pub type RegDelaySelIAdq7R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq7` writer - Delay select for input ADQ pin 7"]
pub type RegDelaySelIAdq7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_delay_sel_i_adq6` reader - Delay select for input ADQ pin 6"]
pub type RegDelaySelIAdq6R = crate::FieldReader;
#[doc = "Field `reg_delay_sel_i_adq6` writer - Delay select for input ADQ pin 6"]
pub type RegDelaySelIAdq6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Delay select for input DQS pin 0"]
    #[inline(always)]
    pub fn reg_delay_sel_i_dqs0(&self) -> RegDelaySelIDqs0R {
        RegDelaySelIDqs0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Delay select for input ADQ pin 7"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq7(&self) -> RegDelaySelIAdq7R {
        RegDelaySelIAdq7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay select for input ADQ pin 6"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq6(&self) -> RegDelaySelIAdq6R {
        RegDelaySelIAdq6R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Delay select for input DQS pin 0"]
    #[inline(always)]
    pub fn reg_delay_sel_i_dqs0(&mut self) -> RegDelaySelIDqs0W<'_, PsramIntfDelayCtrl5Spec> {
        RegDelaySelIDqs0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Delay select for input ADQ pin 7"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq7(&mut self) -> RegDelaySelIAdq7W<'_, PsramIntfDelayCtrl5Spec> {
        RegDelaySelIAdq7W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay select for input ADQ pin 6"]
    #[inline(always)]
    pub fn reg_delay_sel_i_adq6(&mut self) -> RegDelaySelIAdq6W<'_, PsramIntfDelayCtrl5Spec> {
        RegDelaySelIAdq6W::new(self, 24)
    }
}
#[doc = "PSRAM Interface Fine Delay Control Register 5 (DQS0 In, ADQ7-6 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_intf_delay_ctrl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_intf_delay_ctrl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramIntfDelayCtrl5Spec;
impl crate::RegisterSpec for PsramIntfDelayCtrl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_intf_delay_ctrl5::R`](R) reader structure"]
impl crate::Readable for PsramIntfDelayCtrl5Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_intf_delay_ctrl5::W`](W) writer structure"]
impl crate::Writable for PsramIntfDelayCtrl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_INTF_DELAY_CTRL5 to value 0"]
impl crate::Resettable for PsramIntfDelayCtrl5Spec {}
