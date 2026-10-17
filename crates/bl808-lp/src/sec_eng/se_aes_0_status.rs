#[doc = "Register `SE_AES_0_STATUS` reader"]
pub type R = crate::R<SeAes0StatusSpec>;
#[doc = "Field `se_aes_0_status` reader - "]
pub type SeAes0StatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_status(&self) -> SeAes0StatusR {
        SeAes0StatusR::new(self.bits)
    }
}
#[doc = "AES Engine Status\n\nYou can [`read`](crate::Reg::read) this register and get [`se_aes_0_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeAes0StatusSpec;
impl crate::RegisterSpec for SeAes0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_aes_0_status::R`](R) reader structure"]
impl crate::Readable for SeAes0StatusSpec {}
#[doc = "`reset()` method sets SE_AES_0_STATUS to value 0x4100"]
impl crate::Resettable for SeAes0StatusSpec {
    const RESET_VALUE: u32 = 0x4100;
}
