#[doc = "Register `SoftLSReq` reader"]
pub type R = crate::R<SoftLsreqSpec>;
#[doc = "Register `SoftLSReq` writer"]
pub type W = crate::W<SoftLsreqSpec>;
#[doc = "Field `SoftLSReq` reader - "]
pub type SoftLsreqR = crate::FieldReader<u32>;
#[doc = "Field `SoftLSReq` writer - "]
pub type SoftLsreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lsreq(&self) -> SoftLsreqR {
        SoftLsreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lsreq(&mut self) -> SoftLsreqW<'_, SoftLsreqSpec> {
        SoftLsreqW::new(self, 0)
    }
}
#[doc = "Software Last Single Request\n\nYou can [`read`](crate::Reg::read) this register and get [`soft_lsreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soft_lsreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftLsreqSpec;
impl crate::RegisterSpec for SoftLsreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_lsreq::R`](R) reader structure"]
impl crate::Readable for SoftLsreqSpec {}
#[doc = "`write(|w| ..)` method takes [`soft_lsreq::W`](W) writer structure"]
impl crate::Writable for SoftLsreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SoftLSReq to value 0"]
impl crate::Resettable for SoftLsreqSpec {}
