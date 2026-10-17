#[doc = "Register `SF_AHB2SIF_STATUS` reader"]
pub type R = crate::R<SfAhb2sifStatusSpec>;
#[doc = "Field `sf_ahb2sif_status` reader - "]
pub type SfAhb2sifStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_ahb2sif_status(&self) -> SfAhb2sifStatusR {
        SfAhb2sifStatusR::new(self.bits)
    }
}
#[doc = "AHB to Serial Interface Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ahb2sif_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfAhb2sifStatusSpec;
impl crate::RegisterSpec for SfAhb2sifStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ahb2sif_status::R`](R) reader structure"]
impl crate::Readable for SfAhb2sifStatusSpec {}
#[doc = "`reset()` method sets SF_AHB2SIF_STATUS to value 0"]
impl crate::Resettable for SfAhb2sifStatusSpec {}
