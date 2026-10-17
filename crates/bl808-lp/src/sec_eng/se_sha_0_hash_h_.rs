#[doc = "Register `SE_SHA_0_HASH_H_%s` reader"]
pub type R = crate::R<SeSha0HashH_Spec>;
#[doc = "Field `se_sha_0_hash_h` reader - "]
pub type SeSha0HashHR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_h(&self) -> SeSha0HashHR {
        SeSha0HashHR::new(self.bits)
    }
}
#[doc = "SHA Hash Result High \\[511:256\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_h_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0HashH_Spec;
impl crate::RegisterSpec for SeSha0HashH_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_hash_h_::R`](R) reader structure"]
impl crate::Readable for SeSha0HashH_Spec {}
#[doc = "`reset()` method sets SE_SHA_0_HASH_H_%s to value 0"]
impl crate::Resettable for SeSha0HashH_Spec {}
