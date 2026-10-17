#[doc = "Register `PDS_GPIO_PD_SET` reader"]
pub type R = crate::R<PdsGpioPdSetSpec>;
#[doc = "Register `PDS_GPIO_PD_SET` writer"]
pub type W = crate::W<PdsGpioPdSetSpec>;
#[doc = "Field `cr_pds_gpio_set_int_mask` reader - "]
pub type CrPdsGpioSetIntMaskR = crate::FieldReader<u32>;
#[doc = "Field `cr_pds_gpio_set_int_mask` writer - "]
pub type CrPdsGpioSetIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_pds_gpio_set_int_mask(&self) -> CrPdsGpioSetIntMaskR {
        CrPdsGpioSetIntMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_pds_gpio_set_int_mask(&mut self) -> CrPdsGpioSetIntMaskW<'_, PdsGpioPdSetSpec> {
        CrPdsGpioSetIntMaskW::new(self, 0)
    }
}
#[doc = "PDS GPIO Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_gpio_pd_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_gpio_pd_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsGpioPdSetSpec;
impl crate::RegisterSpec for PdsGpioPdSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_gpio_pd_set::R`](R) reader structure"]
impl crate::Readable for PdsGpioPdSetSpec {}
#[doc = "`write(|w| ..)` method takes [`pds_gpio_pd_set::W`](W) writer structure"]
impl crate::Writable for PdsGpioPdSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_GPIO_PD_SET to value 0xffff_ffff"]
impl crate::Resettable for PdsGpioPdSetSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
