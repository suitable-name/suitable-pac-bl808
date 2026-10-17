#[doc = "Register `MM_BERR_CFG4` reader"]
pub type R = crate::R<MmBerrCfg4Spec>;
#[doc = "Field `sts_berr_addr` reader - Captured 32-bit address of the transaction that caused the main bus error."]
pub type StsBerrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captured 32-bit address of the transaction that caused the main bus error."]
    #[inline(always)]
    pub fn sts_berr_addr(&self) -> StsBerrAddrR {
        StsBerrAddrR::new(self.bits)
    }
}
#[doc = "Multimedia Main Bus Error Address Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmBerrCfg4Spec;
impl crate::RegisterSpec for MmBerrCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_berr_cfg4::R`](R) reader structure"]
impl crate::Readable for MmBerrCfg4Spec {}
#[doc = "`reset()` method sets MM_BERR_CFG4 to value 0"]
impl crate::Resettable for MmBerrCfg4Spec {}
