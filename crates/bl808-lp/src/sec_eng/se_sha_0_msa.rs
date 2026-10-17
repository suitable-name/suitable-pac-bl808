#[doc = "Register `SE_SHA_0_MSA` reader"]
pub type R = crate::R<SeSha0MsaSpec>;
#[doc = "Register `SE_SHA_0_MSA` writer"]
pub type W = crate::W<SeSha0MsaSpec>;
#[doc = "Field `se_sha_0_msa` reader - "]
pub type SeSha0MsaR = crate::FieldReader<u32>;
#[doc = "Field `se_sha_0_msa` writer - "]
pub type SeSha0MsaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_msa(&self) -> SeSha0MsaR {
        SeSha0MsaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_msa(&mut self) -> SeSha0MsaW<'_, SeSha0MsaSpec> {
        SeSha0MsaW::new(self, 0)
    }
}
#[doc = "SHA Message Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_msa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_msa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0MsaSpec;
impl crate::RegisterSpec for SeSha0MsaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_msa::R`](R) reader structure"]
impl crate::Readable for SeSha0MsaSpec {}
#[doc = "`write(|w| ..)` method takes [`se_sha_0_msa::W`](W) writer structure"]
impl crate::Writable for SeSha0MsaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_SHA_0_MSA to value 0"]
impl crate::Resettable for SeSha0MsaSpec {}
