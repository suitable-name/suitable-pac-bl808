#[doc = "Register `SE_SHA_0_HASH_L_%s` reader"]
pub type R = crate::R<SeSha0HashL_Spec>;
#[doc = "Field `se_sha_0_hash_l` reader - "]
pub type SeSha0HashLR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_l(&self) -> SeSha0HashLR {
        SeSha0HashLR::new(self.bits)
    }
}
#[doc = "SHA Hash Result Low \\[255:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_hash_l_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0HashL_Spec;
impl crate::RegisterSpec for SeSha0HashL_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_hash_l_::R`](R) reader structure"]
impl crate::Readable for SeSha0HashL_Spec {}
#[doc = "`reset()` method sets SE_SHA_0_HASH_L_%s to value 0"]
impl crate::Resettable for SeSha0HashL_Spec {}
