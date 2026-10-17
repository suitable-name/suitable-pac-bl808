#[doc = "Register `MM_BERR_CFG5` reader"]
pub type R = crate::R<MmBerrCfg5Spec>;
#[doc = "Field `sts_codec_berr_addr` reader - Captured 32-bit address of the transaction that caused the CODEC bus error."]
pub type StsCodecBerrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captured 32-bit address of the transaction that caused the CODEC bus error."]
    #[inline(always)]
    pub fn sts_codec_berr_addr(&self) -> StsCodecBerrAddrR {
        StsCodecBerrAddrR::new(self.bits)
    }
}
#[doc = "Multimedia CODEC Bus Error Address Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmBerrCfg5Spec;
impl crate::RegisterSpec for MmBerrCfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_berr_cfg5::R`](R) reader structure"]
impl crate::Readable for MmBerrCfg5Spec {}
#[doc = "`reset()` method sets MM_BERR_CFG5 to value 0"]
impl crate::Resettable for MmBerrCfg5Spec {}
