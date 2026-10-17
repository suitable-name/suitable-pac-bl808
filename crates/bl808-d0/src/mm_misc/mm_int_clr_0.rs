#[doc = "Register `MM_INT_CLR_0` writer"]
pub type W = crate::W<MmIntClr0Spec>;
#[doc = "Field `mm_int_clr0` writer - Write 1 to clear corresponding MM interrupt bits 0-31."]
pub type MmIntClr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write 1 to clear corresponding MM interrupt bits 0-31."]
    #[inline(always)]
    pub fn mm_int_clr0(&mut self) -> MmIntClr0W<'_, MmIntClr0Spec> {
        MmIntClr0W::new(self, 0)
    }
}
#[doc = "Multimedia Interrupt Clear Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_int_clr_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmIntClr0Spec;
impl crate::RegisterSpec for MmIntClr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mm_int_clr_0::W`](W) writer structure"]
impl crate::Writable for MmIntClr0Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets MM_INT_CLR_0 to value 0"]
impl crate::Resettable for MmIntClr0Spec {}
