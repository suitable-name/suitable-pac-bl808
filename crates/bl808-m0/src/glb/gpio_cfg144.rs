#[doc = "Register `GPIO_CFG144` writer"]
pub type W = crate::W<GpioCfg144Spec>;
#[doc = "Field `gpio_tx_data` writer - "]
pub type GpioTxDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_tx_data(&mut self) -> GpioTxDataW<'_, GpioCfg144Spec> {
        GpioTxDataW::new(self, 0)
    }
}
#[doc = "GPIO DMA FIFO Write Data\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_cfg144::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioCfg144Spec;
impl crate::RegisterSpec for GpioCfg144Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_cfg144::W`](W) writer structure"]
impl crate::Writable for GpioCfg144Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_CFG144 to value 0"]
impl crate::Resettable for GpioCfg144Spec {}
