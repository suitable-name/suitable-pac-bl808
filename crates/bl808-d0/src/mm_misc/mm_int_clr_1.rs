#[doc = "Register `MM_INT_CLR_1` writer"]
pub type W = crate::W<MmIntClr1Spec>;
#[doc = "Field `mm_int_clr1` writer - Write 1 to clear corresponding MM interrupt bits 32-63."]
pub type MmIntClr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write 1 to clear corresponding MM interrupt bits 32-63."]
    #[inline(always)]
    pub fn mm_int_clr1(&mut self) -> MmIntClr1W<'_, MmIntClr1Spec> {
        MmIntClr1W::new(self, 0)
    }
}
#[doc = "Multimedia Interrupt Clear Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_int_clr_1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmIntClr1Spec;
impl crate::RegisterSpec for MmIntClr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mm_int_clr_1::W`](W) writer structure"]
impl crate::Writable for MmIntClr1Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets MM_INT_CLR_1 to value 0"]
impl crate::Resettable for MmIntClr1Spec {}
