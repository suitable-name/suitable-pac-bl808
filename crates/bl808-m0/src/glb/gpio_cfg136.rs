#[doc = "Register `GPIO_CFG136` reader"]
pub type R = crate::R<GpioCfg136Spec>;
#[doc = "Register `GPIO_CFG136` writer"]
pub type W = crate::W<GpioCfg136Spec>;
#[doc = "Field `gpio_out_reg2_0` reader - "]
pub type GpioOutReg2_0R = crate::FieldReader<u32>;
#[doc = "Field `gpio_out_reg2_0` writer - "]
pub type GpioOutReg2_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_out_reg2_0(&self) -> GpioOutReg2_0R {
        GpioOutReg2_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_out_reg2_0(&mut self) -> GpioOutReg2_0W<'_, GpioCfg136Spec> {
        GpioOutReg2_0W::new(self, 0)
    }
}
#[doc = "Alternative GPIO Output (Pins 0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg136::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg136::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfg136Spec;
impl crate::RegisterSpec for GpioCfg136Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfg136::R`](R) reader structure"]
impl crate::Readable for GpioCfg136Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfg136::W`](W) writer structure"]
impl crate::Writable for GpioCfg136Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFG136 to value 0"]
impl crate::Resettable for GpioCfg136Spec {}
