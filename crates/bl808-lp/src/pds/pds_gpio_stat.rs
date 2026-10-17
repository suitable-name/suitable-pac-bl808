#[doc = "Register `PDS_GPIO_STAT` reader"]
pub type R = crate::R<PdsGpioStatSpec>;
#[doc = "Register `PDS_GPIO_STAT` writer"]
pub type W = crate::W<PdsGpioStatSpec>;
#[doc = "Field `pds_gpio_int_stat` reader - "]
pub type PdsGpioIntStatR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pds_gpio_int_stat(&self) -> PdsGpioIntStatR {
        PdsGpioIntStatR::new(self.bits)
    }
}
impl W {}
#[doc = "PDS GPIO Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_gpio_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_gpio_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsGpioStatSpec;
impl crate::RegisterSpec for PdsGpioStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_gpio_stat::R`](R) reader structure"]
impl crate::Readable for PdsGpioStatSpec {}
#[doc = "`write(|w| ..)` method takes [`pds_gpio_stat::W`](W) writer structure"]
impl crate::Writable for PdsGpioStatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_GPIO_STAT to value 0"]
impl crate::Resettable for PdsGpioStatSpec {}
