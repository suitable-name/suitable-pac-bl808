#[doc = "Register `GPIO_LAT2` reader"]
pub type R = crate::R<GpioLat2Spec>;
#[doc = "Field `gpio_lat2` reader - "]
pub type GpioLat2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_lat2(&self) -> GpioLat2R {
        GpioLat2R::new(self.bits)
    }
}
#[doc = "GPIO Negative Edge Latch Value\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_lat2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioLat2Spec;
impl crate::RegisterSpec for GpioLat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_lat2::R`](R) reader structure"]
impl crate::Readable for GpioLat2Spec {}
#[doc = "`reset()` method sets GPIO_LAT2 to value 0"]
impl crate::Resettable for GpioLat2Spec {}
