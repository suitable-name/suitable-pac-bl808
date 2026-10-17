#[doc = "Register `HBN_IRQ_CLR` writer"]
pub type W = crate::W<HbnIrqClrSpec>;
#[doc = "Field `irq_clr` writer - "]
pub type IrqClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_clr(&mut self) -> IrqClrW<'_, HbnIrqClrSpec> {
        IrqClrW::new(self, 0)
    }
}
#[doc = "HBN Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_irq_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnIrqClrSpec;
impl crate::RegisterSpec for HbnIrqClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hbn_irq_clr::W`](W) writer structure"]
impl crate::Writable for HbnIrqClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_IRQ_CLR to value 0"]
impl crate::Resettable for HbnIrqClrSpec {}
