#[doc = "Register `SE_SHA_0_STATUS` reader"]
pub type R = crate::R<SeSha0StatusSpec>;
#[doc = "Field `se_sha_0_status` reader - "]
pub type SeSha0StatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_status(&self) -> SeSha0StatusR {
        SeSha0StatusR::new(self.bits)
    }
}
#[doc = "SHA Engine Status\n\nYou can [`read`](crate::Reg::read) this register and get [`se_sha_0_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeSha0StatusSpec;
impl crate::RegisterSpec for SeSha0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_sha_0_status::R`](R) reader structure"]
impl crate::Readable for SeSha0StatusSpec {}
#[doc = "`reset()` method sets SE_SHA_0_STATUS to value 0x41"]
impl crate::Resettable for SeSha0StatusSpec {
    const RESET_VALUE: u32 = 0x41;
}
