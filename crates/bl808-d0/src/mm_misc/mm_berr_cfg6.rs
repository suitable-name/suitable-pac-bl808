#[doc = "Register `MM_BERR_CFG6` reader"]
pub type R = crate::R<MmBerrCfg6Spec>;
#[doc = "Field `sts_mmcpu_berr_addr` reader - Captured 32-bit address of the transaction that caused the MM CPU bus error."]
pub type StsMmcpuBerrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captured 32-bit address of the transaction that caused the MM CPU bus error."]
    #[inline(always)]
    pub fn sts_mmcpu_berr_addr(&self) -> StsMmcpuBerrAddrR {
        StsMmcpuBerrAddrR::new(self.bits)
    }
}
#[doc = "Multimedia MM CPU Bus Error Address Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmBerrCfg6Spec;
impl crate::RegisterSpec for MmBerrCfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_berr_cfg6::R`](R) reader structure"]
impl crate::Readable for MmBerrCfg6Spec {}
#[doc = "`reset()` method sets MM_BERR_CFG6 to value 0"]
impl crate::Resettable for MmBerrCfg6Spec {}
