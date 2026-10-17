#[doc = "Register `MM_BERR_CFG7` reader"]
pub type R = crate::R<MmBerrCfg7Spec>;
#[doc = "Field `sts_mminfra_berr_addr` reader - Captured 32-bit address of the transaction that caused the infrastructure bus error."]
pub type StsMminfraBerrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captured 32-bit address of the transaction that caused the infrastructure bus error."]
    #[inline(always)]
    pub fn sts_mminfra_berr_addr(&self) -> StsMminfraBerrAddrR {
        StsMminfraBerrAddrR::new(self.bits)
    }
}
#[doc = "Multimedia Infrastructure Bus Error Address Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmBerrCfg7Spec;
impl crate::RegisterSpec for MmBerrCfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_berr_cfg7::R`](R) reader structure"]
impl crate::Readable for MmBerrCfg7Spec {}
#[doc = "`reset()` method sets MM_BERR_CFG7 to value 0"]
impl crate::Resettable for MmBerrCfg7Spec {}
