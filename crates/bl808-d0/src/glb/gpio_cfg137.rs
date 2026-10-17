#[doc = "Register `GPIO_CFG137` reader"]
pub type R = crate::R<GpioCfg137Spec>;
#[doc = "Register `GPIO_CFG137` writer"]
pub type W = crate::W<GpioCfg137Spec>;
#[doc = "Field `gpio_out_reg2_1` reader - "]
pub type GpioOutReg2_1R = crate::FieldReader<u32>;
#[doc = "Field `gpio_out_reg2_1` writer - "]
pub type GpioOutReg2_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_out_reg2_1(&self) -> GpioOutReg2_1R {
        GpioOutReg2_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_out_reg2_1(&mut self) -> GpioOutReg2_1W<'_, GpioCfg137Spec> {
        GpioOutReg2_1W::new(self, 0)
    }
}
#[doc = "Alternative GPIO Output (Pins 32-63)\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_cfg137::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg137::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfg137Spec;
impl crate::RegisterSpec for GpioCfg137Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_cfg137::R`](R) reader structure"]
impl crate::Readable for GpioCfg137Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_cfg137::W`](W) writer structure"]
impl crate::Writable for GpioCfg137Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFG137 to value 0"]
impl crate::Resettable for GpioCfg137Spec {}
