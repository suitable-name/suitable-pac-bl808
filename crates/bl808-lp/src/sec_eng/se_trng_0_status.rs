#[doc = "Register `SE_TRNG_0_STATUS` reader"]
pub type R = crate::R<SeTrng0StatusSpec>;
#[doc = "Field `se_trng_0_status` reader - "]
pub type SeTrng0StatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_status(&self) -> SeTrng0StatusR {
        SeTrng0StatusR::new(self.bits)
    }
}
#[doc = "TRNG Engine Status\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0StatusSpec;
impl crate::RegisterSpec for SeTrng0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_status::R`](R) reader structure"]
impl crate::Readable for SeTrng0StatusSpec {}
#[doc = "`reset()` method sets SE_TRNG_0_STATUS to value 0x0010_0020"]
impl crate::Resettable for SeTrng0StatusSpec {
    const RESET_VALUE: u32 = 0x0010_0020;
}
