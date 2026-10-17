#[doc = "Register `GPIO_CFG129` reader"]
pub type R = crate::R<GpioCfg129Spec>;
#[doc = "Field `gpio_in_reg2_1` reader - "]
pub type GpioInReg2_1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_in_reg2_1(&self) -> GpioInReg2_1R {
        GpioInReg2_1R::new(self.bits)
    }
}
#[doc = "Alternative GPIO Input Readback (Pins 32-63)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg129::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfg129Spec;
impl crate::RegisterSpec for GpioCfg129Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfg129::R`](R) reader structure"]
impl crate::Readable for GpioCfg129Spec {}
#[doc = "`reset()` method sets GPIO_CFG129 to value 0"]
impl crate::Resettable for GpioCfg129Spec {}
