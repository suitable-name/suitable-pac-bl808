#[doc = "Register `GPIO_CFG128` reader"]
pub type R = crate::R<GpioCfg128Spec>;
#[doc = "Field `gpio_in_reg2_0` reader - "]
pub type GpioInReg2_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_in_reg2_0(&self) -> GpioInReg2_0R {
        GpioInReg2_0R::new(self.bits)
    }
}
#[doc = "Alternative GPIO Input Readback (Pins 0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg128::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfg128Spec;
impl crate::RegisterSpec for GpioCfg128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfg128::R`](R) reader structure"]
impl crate::Readable for GpioCfg128Spec {}
#[doc = "`reset()` method sets GPIO_CFG128 to value 0"]
impl crate::Resettable for GpioCfg128Spec {}
