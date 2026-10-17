#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL5` reader"]
pub type R = crate::R<PsramRoughDelayCtrl5Spec>;
#[doc = "Register `PSRAM_ROUGH_DELAY_CTRL5` writer"]
pub type W = crate::W<PsramRoughDelayCtrl5Spec>;
#[doc = "Field `reg_rough_sel_i_dqs0` reader - Rough delay select for input DQS pin 0"]
pub type RegRoughSelIDqs0R = crate::FieldReader<u16>;
#[doc = "Field `reg_rough_sel_i_dqs0` writer - Rough delay select for input DQS pin 0"]
pub type RegRoughSelIDqs0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_rough_sel_i_adq7` reader - Rough delay select for input ADQ pin 7"]
pub type RegRoughSelIAdq7R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq7` writer - Rough delay select for input ADQ pin 7"]
pub type RegRoughSelIAdq7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_rough_sel_i_adq6` reader - Rough delay select for input ADQ pin 6"]
pub type RegRoughSelIAdq6R = crate::FieldReader;
#[doc = "Field `reg_rough_sel_i_adq6` writer - Rough delay select for input ADQ pin 6"]
pub type RegRoughSelIAdq6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Rough delay select for input DQS pin 0"]
    #[inline(always)]
    pub fn reg_rough_sel_i_dqs0(&self) -> RegRoughSelIDqs0R {
        RegRoughSelIDqs0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Rough delay select for input ADQ pin 7"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq7(&self) -> RegRoughSelIAdq7R {
        RegRoughSelIAdq7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Rough delay select for input ADQ pin 6"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq6(&self) -> RegRoughSelIAdq6R {
        RegRoughSelIAdq6R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Rough delay select for input DQS pin 0"]
    #[inline(always)]
    pub fn reg_rough_sel_i_dqs0(&mut self) -> RegRoughSelIDqs0W<'_, PsramRoughDelayCtrl5Spec> {
        RegRoughSelIDqs0W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Rough delay select for input ADQ pin 7"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq7(&mut self) -> RegRoughSelIAdq7W<'_, PsramRoughDelayCtrl5Spec> {
        RegRoughSelIAdq7W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Rough delay select for input ADQ pin 6"]
    #[inline(always)]
    pub fn reg_rough_sel_i_adq6(&mut self) -> RegRoughSelIAdq6W<'_, PsramRoughDelayCtrl5Spec> {
        RegRoughSelIAdq6W::new(self, 24)
    }
}
#[doc = "PSRAM Rough Delay Control Register 5 (DQS0 In, ADQ7-6 In)\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_rough_delay_ctrl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_rough_delay_ctrl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramRoughDelayCtrl5Spec;
impl crate::RegisterSpec for PsramRoughDelayCtrl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_rough_delay_ctrl5::R`](R) reader structure"]
impl crate::Readable for PsramRoughDelayCtrl5Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_rough_delay_ctrl5::W`](W) writer structure"]
impl crate::Writable for PsramRoughDelayCtrl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_ROUGH_DELAY_CTRL5 to value 0"]
impl crate::Resettable for PsramRoughDelayCtrl5Spec {}
