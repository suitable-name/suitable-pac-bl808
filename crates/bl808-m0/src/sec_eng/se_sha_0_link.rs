#[doc = "Register `SE_SHA_0_LINK` reader"]
pub type R = crate::R<SeSha0LinkSpec>;
#[doc = "Register `SE_SHA_0_LINK` writer"]
pub type W = crate::W<SeSha0LinkSpec>;
#[doc = "Field `se_sha_0_lca` reader - "]
pub type SeSha0LcaR = crate::FieldReader<u32>;
#[doc = "Field `se_sha_0_lca` writer - "]
pub type SeSha0LcaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_lca(&self) -> SeSha0LcaR {
        SeSha0LcaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_lca(&mut self) -> SeSha0LcaW<'_, SeSha0LinkSpec> {
        SeSha0LcaW::new(self, 0)
    }
}
#[doc = "SHA Link List Address\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_link::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_sha_0_link::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0LinkSpec;
impl crate::RegisterSpec for SeSha0LinkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_link::R`](R) reader structure"]
impl crate::Readable for SeSha0LinkSpec {}
#[doc = "`write(|w| ..)` method takes [`se_sha_0_link::W`](W) writer structure"]
impl crate::Writable for SeSha0LinkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_SHA_0_LINK to value 0"]
impl crate::Resettable for SeSha0LinkSpec {}
