#[doc = "Register `DMA_IntTCClear` writer"]
pub type W = crate::W<DmaIntTcclearSpec>;
#[doc = "Field `IntTCClear` writer - Clear terminal count interrupt for channels 0-7."]
pub type IntTcclearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Clear terminal count interrupt for channels 0-7."]
    #[inline(always)]
    pub fn int_tcclear(&mut self) -> IntTcclearW<'_, DmaIntTcclearSpec> {
        IntTcclearW::new(self, 0)
    }
}
#[doc = "Terminal count request clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_tcclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaIntTcclearSpec;
impl crate::RegisterSpec for DmaIntTcclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_int_tcclear::W`](W) writer structure"]
impl crate::Writable for DmaIntTcclearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets DMA_IntTCClear to value 0"]
impl crate::Resettable for DmaIntTcclearSpec {}
