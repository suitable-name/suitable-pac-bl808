#[doc = "Register `SE_TRNG_0_DOUT_%s` reader"]
pub type R = crate::R<SeTrng0Dout_Spec>;
#[doc = "Field `se_trng_0_dout` reader - "]
pub type SeTrng0DoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_dout(&self) -> SeTrng0DoutR {
        SeTrng0DoutR::new(self.bits)
    }
}
#[doc = "TRNG Output Data %s\n\nYou can [`read`](crate::Reg::read) this register and get [`se_trng_0_dout_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeTrng0Dout_Spec;
impl crate::RegisterSpec for SeTrng0Dout_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_trng_0_dout_::R`](R) reader structure"]
impl crate::Readable for SeTrng0Dout_Spec {}
#[doc = "`reset()` method sets SE_TRNG_0_DOUT_%s to value 0"]
impl crate::Resettable for SeTrng0Dout_Spec {}
