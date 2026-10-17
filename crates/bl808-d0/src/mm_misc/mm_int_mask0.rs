#[doc = "Register `MM_INT_MASK0` reader"]
pub type R = crate::R<MmIntMask0Spec>;
#[doc = "Register `MM_INT_MASK0` writer"]
pub type W = crate::W<MmIntMask0Spec>;
#[doc = "Field `mm_int_mask0` reader - Interrupt mask for MM sources 0-31. 0: Masked, 1: Enabled."]
pub type MmIntMask0R = crate::FieldReader<u32>;
#[doc = "Field `mm_int_mask0` writer - Interrupt mask for MM sources 0-31. 0: Masked, 1: Enabled."]
pub type MmIntMask0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt mask for MM sources 0-31. 0: Masked, 1: Enabled."]
    #[inline(always)]
    pub fn mm_int_mask0(&self) -> MmIntMask0R {
        MmIntMask0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt mask for MM sources 0-31. 0: Masked, 1: Enabled."]
    #[inline(always)]
    pub fn mm_int_mask0(&mut self) -> MmIntMask0W<'_, MmIntMask0Spec> {
        MmIntMask0W::new(self, 0)
    }
}
#[doc = "Multimedia Interrupt Mask Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_int_mask0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_int_mask0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmIntMask0Spec;
impl crate::RegisterSpec for MmIntMask0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_int_mask0::R`](R) reader structure"]
impl crate::Readable for MmIntMask0Spec {}
#[doc = "`write(|w| ..)` method takes [`mm_int_mask0::W`](W) writer structure"]
impl crate::Writable for MmIntMask0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_INT_MASK0 to value 0"]
impl crate::Resettable for MmIntMask0Spec {}
