#[doc = "Register `IntErrClr` writer"]
pub type W = crate::W<IntErrClrSpec>;
#[doc = "Field `IntErrClr` writer - Clear error interrupt for channels 0-7"]
pub type IntErrClrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Clear error interrupt for channels 0-7"]
    #[inline(always)]
    pub fn int_err_clr(&mut self) -> IntErrClrW<'_, IntErrClrSpec> {
        IntErrClrW::new(self, 0)
    }
}
#[doc = "Interrupt error clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_err_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntErrClrSpec;
impl crate::RegisterSpec for IntErrClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_err_clr::W`](W) writer structure"]
impl crate::Writable for IntErrClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IntErrClr to value 0"]
impl crate::Resettable for IntErrClrSpec {}
