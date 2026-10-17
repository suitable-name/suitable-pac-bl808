#[doc = "Register `a2` reader"]
pub type R = crate::R<A2Spec>;
#[doc = "Register `a2` writer"]
pub type W = crate::W<A2Spec>;
#[doc = "Field `val` reader - "]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `val` writer - "]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, A2Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "PEQ Coefficient A2 (Denominator z-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`a2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A2Spec;
impl crate::RegisterSpec for A2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a2::R`](R) reader structure"]
impl crate::Readable for A2Spec {}
#[doc = "`write(|w| ..)` method takes [`a2::W`](W) writer structure"]
impl crate::Writable for A2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets a2 to value 0"]
impl crate::Resettable for A2Spec {}
