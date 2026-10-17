#[doc = "Register `MM_BERR_CFG3` reader"]
pub type R = crate::R<MmBerrCfg3Spec>;
#[doc = "Field `sts_mmcpu_berr_src` reader - Source ID of the master that caused the MM CPU bus error."]
pub type StsMmcpuBerrSrcR = crate::BitReader;
#[doc = "Field `sts_mmcpu_berr_id` reader - Transaction ID of the MM CPU bus error."]
pub type StsMmcpuBerrIdR = crate::FieldReader;
#[doc = "Field `sts_mminfra_berr_src` reader - Source ID of the master that caused the MM infrastructure bus error."]
pub type StsMminfraBerrSrcR = crate::FieldReader;
#[doc = "Field `sts_mminfra_berr_id` reader - Transaction ID of the MM infrastructure bus error."]
pub type StsMminfraBerrIdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Source ID of the master that caused the MM CPU bus error."]
    #[inline(always)]
    pub fn sts_mmcpu_berr_src(&self) -> StsMmcpuBerrSrcR {
        StsMmcpuBerrSrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transaction ID of the MM CPU bus error."]
    #[inline(always)]
    pub fn sts_mmcpu_berr_id(&self) -> StsMmcpuBerrIdR {
        StsMmcpuBerrIdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Source ID of the master that caused the MM infrastructure bus error."]
    #[inline(always)]
    pub fn sts_mminfra_berr_src(&self) -> StsMminfraBerrSrcR {
        StsMminfraBerrSrcR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:29 - Transaction ID of the MM infrastructure bus error."]
    #[inline(always)]
    pub fn sts_mminfra_berr_id(&self) -> StsMminfraBerrIdR {
        StsMminfraBerrIdR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Multimedia Bus Error Source and ID Register (MM CPU and MM Infra)\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmBerrCfg3Spec;
impl crate::RegisterSpec for MmBerrCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_berr_cfg3::R`](R) reader structure"]
impl crate::Readable for MmBerrCfg3Spec {}
#[doc = "`reset()` method sets MM_BERR_CFG3 to value 0"]
impl crate::Resettable for MmBerrCfg3Spec {}
