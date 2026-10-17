#[doc = "Register `SoftLBReq` reader"]
pub type R = crate::R<SoftLbreqSpec>;
#[doc = "Register `SoftLBReq` writer"]
pub type W = crate::W<SoftLbreqSpec>;
#[doc = "Field `SoftLBReq` reader - "]
pub type SoftLbreqR = crate::FieldReader<u32>;
#[doc = "Field `SoftLBReq` writer - "]
pub type SoftLbreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lbreq(&self) -> SoftLbreqR {
        SoftLbreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lbreq(&mut self) -> SoftLbreqW<'_, SoftLbreqSpec> {
        SoftLbreqW::new(self, 0)
    }
}
#[doc = "Software Last Burst Request\n\nYou can [`read`](crate::Reg::read) this register and get [`soft_lbreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_lbreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftLbreqSpec;
impl crate::RegisterSpec for SoftLbreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_lbreq::R`](R) reader structure"]
impl crate::Readable for SoftLbreqSpec {}
#[doc = "`write(|w| ..)` method takes [`soft_lbreq::W`](W) writer structure"]
impl crate::Writable for SoftLbreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SoftLBReq to value 0"]
impl crate::Resettable for SoftLbreqSpec {}
