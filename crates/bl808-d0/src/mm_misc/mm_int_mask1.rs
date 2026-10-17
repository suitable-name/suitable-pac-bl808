#[doc = "Register `MM_INT_MASK1` reader"]
pub type R = crate::R<MmIntMask1Spec>;
#[doc = "Register `MM_INT_MASK1` writer"]
pub type W = crate::W<MmIntMask1Spec>;
#[doc = "Field `mm_int_mask1` reader - Interrupt mask for MM sources 32-63. 0: Masked, 1: Enabled."]
pub type MmIntMask1R = crate::FieldReader<u32>;
#[doc = "Field `mm_int_mask1` writer - Interrupt mask for MM sources 32-63. 0: Masked, 1: Enabled."]
pub type MmIntMask1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt mask for MM sources 32-63. 0: Masked, 1: Enabled."]
    #[inline(always)]
    pub fn mm_int_mask1(&self) -> MmIntMask1R {
        MmIntMask1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt mask for MM sources 32-63. 0: Masked, 1: Enabled."]
    #[inline(always)]
    pub fn mm_int_mask1(&mut self) -> MmIntMask1W<'_, MmIntMask1Spec> {
        MmIntMask1W::new(self, 0)
    }
}
#[doc = "Multimedia Interrupt Mask Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_int_mask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_int_mask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmIntMask1Spec;
impl crate::RegisterSpec for MmIntMask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_int_mask1::R`](R) reader structure"]
impl crate::Readable for MmIntMask1Spec {}
#[doc = "`write(|w| ..)` method takes [`mm_int_mask1::W`](W) writer structure"]
impl crate::Writable for MmIntMask1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_INT_MASK1 to value 0"]
impl crate::Resettable for MmIntMask1Spec {}
