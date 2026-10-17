#[doc = "Register `SE_TRNG_0_TEST_OUT_%s` reader"]
pub type R = crate::R<SeTrng0TestOut_Spec>;
#[doc = "Field `se_trng_0_test_out` reader - "]
pub type SeTrng0TestOutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_test_out(&self) -> SeTrng0TestOutR {
        SeTrng0TestOutR::new(self.bits)
    }
}
#[doc = "TRNG Test Output %s\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_test_out_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0TestOut_Spec;
impl crate::RegisterSpec for SeTrng0TestOut_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_test_out_::R`](R) reader structure"]
impl crate::Readable for SeTrng0TestOut_Spec {}
#[doc = "`reset()` method sets SE_TRNG_0_TEST_OUT_%s to value 0"]
impl crate::Resettable for SeTrng0TestOut_Spec {}
