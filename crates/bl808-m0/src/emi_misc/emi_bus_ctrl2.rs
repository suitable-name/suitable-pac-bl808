#[doc = "Register `EMI_BUS_CTRL2` reader"]
pub type R = crate::R<EmiBusCtrl2Spec>;
#[doc = "Register `EMI_BUS_CTRL2` writer"]
pub type W = crate::W<EmiBusCtrl2Spec>;
#[doc = "Field `reg_sf_harb_mode` reader - "]
pub type RegSfHarbModeR = crate::BitReader;
#[doc = "Field `reg_sf_harb_mode` writer - "]
pub type RegSfHarbModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_sf_harb_mode(&self) -> RegSfHarbModeR {
        RegSfHarbModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_sf_harb_mode(&mut self) -> RegSfHarbModeW<'_, EmiBusCtrl2Spec> {
        RegSfHarbModeW::new(self, 0)
    }
}
#[doc = "EMI Bus Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`emi_bus_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emi_bus_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmiBusCtrl2Spec;
impl crate::RegisterSpec for EmiBusCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emi_bus_ctrl2::R`](R) reader structure"]
impl crate::Readable for EmiBusCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`emi_bus_ctrl2::W`](W) writer structure"]
impl crate::Writable for EmiBusCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMI_BUS_CTRL2 to value 0"]
impl crate::Resettable for EmiBusCtrl2Spec {}
