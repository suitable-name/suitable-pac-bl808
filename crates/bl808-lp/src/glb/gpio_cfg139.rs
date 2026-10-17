#[doc = "Register `GPIO_CFG139` writer"]
pub type W = crate::W<GpioCfg139Spec>;
#[doc = "Field `gpio_set_reg2_1` writer - "]
pub type GpioSetReg2_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_set_reg2_1(&mut self) -> GpioSetReg2_1W<'_, GpioCfg139Spec> {
        GpioSetReg2_1W::new(self, 0)
    }
}
#[doc = "Alternative GPIO Atomic Set (Pins 32-63)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg139::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfg139Spec;
impl crate::RegisterSpec for GpioCfg139Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_cfg139::W`](W) writer structure"]
impl crate::Writable for GpioCfg139Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets GPIO_CFG139 to value 0"]
impl crate::Resettable for GpioCfg139Spec {}
