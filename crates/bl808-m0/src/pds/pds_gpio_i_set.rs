#[doc = "Register `PDS_GPIO_I_SET` reader"]
pub type R = crate::R<PdsGpioISetSpec>;
#[doc = "Register `PDS_GPIO_I_SET` writer"]
pub type W = crate::W<PdsGpioISetSpec>;
#[doc = "Field `cr_pds_gpio_ie_set` reader - "]
pub type CrPdsGpioIeSetR = crate::FieldReader;
#[doc = "Field `cr_pds_gpio_ie_set` writer - "]
pub type CrPdsGpioIeSetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cr_pds_gpio_pd_set` reader - "]
pub type CrPdsGpioPdSetR = crate::FieldReader;
#[doc = "Field `cr_pds_gpio_pd_set` writer - "]
pub type CrPdsGpioPdSetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cr_pds_gpio_pu_set` reader - "]
pub type CrPdsGpioPuSetR = crate::FieldReader;
#[doc = "Field `cr_pds_gpio_pu_set` writer - "]
pub type CrPdsGpioPuSetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cr_pds_gpio_ie_set(&self) -> CrPdsGpioIeSetR {
        CrPdsGpioIeSetR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn cr_pds_gpio_pd_set(&self) -> CrPdsGpioPdSetR {
        CrPdsGpioPdSetR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn cr_pds_gpio_pu_set(&self) -> CrPdsGpioPuSetR {
        CrPdsGpioPuSetR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cr_pds_gpio_ie_set(&mut self) -> CrPdsGpioIeSetW<'_, PdsGpioISetSpec> {
        CrPdsGpioIeSetW::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn cr_pds_gpio_pd_set(&mut self) -> CrPdsGpioPdSetW<'_, PdsGpioISetSpec> {
        CrPdsGpioPdSetW::new(self, 3)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn cr_pds_gpio_pu_set(&mut self) -> CrPdsGpioPuSetW<'_, PdsGpioISetSpec> {
        CrPdsGpioPuSetW::new(self, 6)
    }
}
#[doc = "PDS GPIO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_gpio_i_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_gpio_i_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsGpioISetSpec;
impl crate::RegisterSpec for PdsGpioISetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_gpio_i_set::R`](R) reader structure"]
impl crate::Readable for PdsGpioISetSpec {}
#[doc = "`write(|w| ..)` method takes [`pds_gpio_i_set::W`](W) writer structure"]
impl crate::Writable for PdsGpioISetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_GPIO_I_SET to value 0"]
impl crate::Resettable for PdsGpioISetSpec {}
