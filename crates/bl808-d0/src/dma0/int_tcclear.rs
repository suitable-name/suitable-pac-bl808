#[doc = "Register `IntTCClear` writer"]
pub type W = crate::W<IntTcclearSpec>;
#[doc = "Field `IntTCClear` writer - Clear terminal count interrupt for channels 0-7"]
pub type IntTcclearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Clear terminal count interrupt for channels 0-7"]
    #[inline(always)]
    pub fn int_tcclear(&mut self) -> IntTcclearW<'_, IntTcclearSpec> {
        IntTcclearW::new(self, 0)
    }
}
#[doc = "Terminal count request clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_tcclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntTcclearSpec;
impl crate::RegisterSpec for IntTcclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_tcclear::W`](W) writer structure"]
impl crate::Writable for IntTcclearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IntTCClear to value 0"]
impl crate::Resettable for IntTcclearSpec {}
