#[doc = "Register `b1` reader"]
pub type R = crate::R<B1Spec>;
#[doc = "Register `b1` writer"]
pub type W = crate::W<B1Spec>;
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
    pub fn val(&mut self) -> ValW<'_, B1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "PEQ Coefficient B1 (Numerator z-1)\n\nYou can [`read`](crate::Reg::read) this register and get [`b1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B1Spec;
impl crate::RegisterSpec for B1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b1::R`](R) reader structure"]
impl crate::Readable for B1Spec {}
#[doc = "`write(|w| ..)` method takes [`b1::W`](W) writer structure"]
impl crate::Writable for B1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets b1 to value 0"]
impl crate::Resettable for B1Spec {}
