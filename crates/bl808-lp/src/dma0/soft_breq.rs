#[doc = "Register `SoftBReq` reader"]
pub type R = crate::R<SoftBreqSpec>;
#[doc = "Register `SoftBReq` writer"]
pub type W = crate::W<SoftBreqSpec>;
#[doc = "Field `SoftBReq` reader - "]
pub type SoftBreqR = crate::FieldReader<u32>;
#[doc = "Field `SoftBReq` writer - "]
pub type SoftBreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_breq(&self) -> SoftBreqR {
        SoftBreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_breq(&mut self) -> SoftBreqW<'_, SoftBreqSpec> {
        SoftBreqW::new(self, 0)
    }
}
#[doc = "Software Burst Request\n\nYou can [`read`](crate::Reg::read) this register and get [`soft_breq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_breq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftBreqSpec;
impl crate::RegisterSpec for SoftBreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_breq::R`](R) reader structure"]
impl crate::Readable for SoftBreqSpec {}
#[doc = "`write(|w| ..)` method takes [`soft_breq::W`](W) writer structure"]
impl crate::Writable for SoftBreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SoftBReq to value 0"]
impl crate::Resettable for SoftBreqSpec {}
