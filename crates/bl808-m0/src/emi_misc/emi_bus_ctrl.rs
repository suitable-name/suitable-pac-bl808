#[doc = "Register `EMI_BUS_CTRL` reader"]
pub type R = crate::R<EmiBusCtrlSpec>;
#[doc = "Register `EMI_BUS_CTRL` writer"]
pub type W = crate::W<EmiBusCtrlSpec>;
#[doc = "Field `rg_emi_pclk_force_on` reader - "]
pub type RgEmiPclkForceOnR = crate::FieldReader<u16>;
#[doc = "Field `rg_emi_pclk_force_on` writer - "]
pub type RgEmiPclkForceOnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rg_emi_pclk_force_on(&self) -> RgEmiPclkForceOnR {
        RgEmiPclkForceOnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rg_emi_pclk_force_on(&mut self) -> RgEmiPclkForceOnW<'_, EmiBusCtrlSpec> {
        RgEmiPclkForceOnW::new(self, 0)
    }
}
#[doc = "EMI Bus Control\n\nYou can [`read`](crate::Reg::read) this register and get [`emi_bus_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emi_bus_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmiBusCtrlSpec;
impl crate::RegisterSpec for EmiBusCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emi_bus_ctrl::R`](R) reader structure"]
impl crate::Readable for EmiBusCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`emi_bus_ctrl::W`](W) writer structure"]
impl crate::Writable for EmiBusCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMI_BUS_CTRL to value 0xffff"]
impl crate::Resettable for EmiBusCtrlSpec {
    const RESET_VALUE: u32 = 0xffff;
}
