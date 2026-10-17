#[doc = "Register `MM_BERR_CFG2` reader"]
pub type R = crate::R<MmBerrCfg2Spec>;
#[doc = "Field `sts_berr_src` reader - Source ID of the master that caused the main bus error."]
pub type StsBerrSrcR = crate::FieldReader;
#[doc = "Field `sts_berr_id` reader - Transaction ID of the main bus error."]
pub type StsBerrIdR = crate::FieldReader;
#[doc = "Field `sts_codec_berr_src` reader - Source ID of the master that caused the CODEC bus error."]
pub type StsCodecBerrSrcR = crate::FieldReader;
#[doc = "Field `sts_codec_berr_id` reader - Transaction ID of the CODEC bus error."]
pub type StsCodecBerrIdR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Source ID of the master that caused the main bus error."]
    #[inline(always)]
    pub fn sts_berr_src(&self) -> StsBerrSrcR {
        StsBerrSrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Transaction ID of the main bus error."]
    #[inline(always)]
    pub fn sts_berr_id(&self) -> StsBerrIdR {
        StsBerrIdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Source ID of the master that caused the CODEC bus error."]
    #[inline(always)]
    pub fn sts_codec_berr_src(&self) -> StsCodecBerrSrcR {
        StsCodecBerrSrcR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - Transaction ID of the CODEC bus error."]
    #[inline(always)]
    pub fn sts_codec_berr_id(&self) -> StsCodecBerrIdR {
        StsCodecBerrIdR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Multimedia Bus Error Source and ID Register (Main and CODEC)\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmBerrCfg2Spec;
impl crate::RegisterSpec for MmBerrCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_berr_cfg2::R`](R) reader structure"]
impl crate::Readable for MmBerrCfg2Spec {}
#[doc = "`reset()` method sets MM_BERR_CFG2 to value 0"]
impl crate::Resettable for MmBerrCfg2Spec {}
