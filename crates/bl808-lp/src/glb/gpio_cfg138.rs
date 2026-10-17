#[doc = "Register `GPIO_CFG138` writer"]
pub type W = crate::W<GpioCfg138Spec>;
#[doc = "Field `gpio_set_reg2_0` writer - "]
pub type GpioSetReg2_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_set_reg2_0(&mut self) -> GpioSetReg2_0W<'_, GpioCfg138Spec> {
        GpioSetReg2_0W::new(self, 0)
    }
}
#[doc = "Alternative GPIO Atomic Set (Pins 0-31)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg138::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfg138Spec;
impl crate::RegisterSpec for GpioCfg138Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_cfg138::W`](W) writer structure"]
impl crate::Writable for GpioCfg138Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets GPIO_CFG138 to value 0"]
impl crate::Resettable for GpioCfg138Spec {}
