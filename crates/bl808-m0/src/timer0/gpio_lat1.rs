#[doc = "Register `GPIO_LAT1` reader"]
pub type R = crate::R<GpioLat1Spec>;
#[doc = "Field `gpio_lat1` reader - "]
pub type GpioLat1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_lat1(&self) -> GpioLat1R {
        GpioLat1R::new(self.bits)
    }
}
#[doc = "GPIO Positive Edge Latch Value\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_lat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioLat1Spec;
impl crate::RegisterSpec for GpioLat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_lat1::R`](R) reader structure"]
impl crate::Readable for GpioLat1Spec {}
#[doc = "`reset()` method sets GPIO_LAT1 to value 0"]
impl crate::Resettable for GpioLat1Spec {}
