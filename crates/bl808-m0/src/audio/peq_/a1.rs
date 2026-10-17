#[doc = "Register `a1` reader"]
pub type R = crate::R<A1Spec>;
#[doc = "Register `a1` writer"]
pub type W = crate::W<A1Spec>;
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
    pub fn val(&mut self) -> ValW<'_, A1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "PEQ Coefficient A1 (Denominator z-1)\n\nYou can [`read`](crate::Reg::read) this register and get [`a1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A1Spec;
impl crate::RegisterSpec for A1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a1::R`](R) reader structure"]
impl crate::Readable for A1Spec {}
#[doc = "`write(|w| ..)` method takes [`a1::W`](W) writer structure"]
impl crate::Writable for A1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets a1 to value 0"]
impl crate::Resettable for A1Spec {}
