#[doc = "Register `b2` reader"]
pub type R = crate::R<B2Spec>;
#[doc = "Register `b2` writer"]
pub type W = crate::W<B2Spec>;
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
    pub fn val(&mut self) -> ValW<'_, B2Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "PEQ Coefficient B2 (Numerator z-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`b2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B2Spec;
impl crate::RegisterSpec for B2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b2::R`](R) reader structure"]
impl crate::Readable for B2Spec {}
#[doc = "`write(|w| ..)` method takes [`b2::W`](W) writer structure"]
impl crate::Writable for B2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets b2 to value 0"]
impl crate::Resettable for B2Spec {}
