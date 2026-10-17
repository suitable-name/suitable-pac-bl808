#[doc = "Register `BMX_CFG4` reader"]
pub type R = crate::R<BmxCfg4Spec>;
#[doc = "Field `sts_bmx_berr_src` reader - "]
pub type StsBmxBerrSrcR = crate::FieldReader<u16>;
#[doc = "Field `sts_mcu_berr_src` reader - "]
pub type StsMcuBerrSrcR = crate::BitReader;
#[doc = "Field `sts_mcu_berr_id` reader - "]
pub type StsMcuBerrIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sts_bmx_berr_src(&self) -> StsBmxBerrSrcR {
        StsBmxBerrSrcR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sts_mcu_berr_src(&self) -> StsMcuBerrSrcR {
        StsMcuBerrSrcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sts_mcu_berr_id(&self) -> StsMcuBerrIdR {
        StsMcuBerrIdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "BMX Bus Matrix Config 4 (Bus Error Source)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxCfg4Spec;
impl crate::RegisterSpec for BmxCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg4::R`](R) reader structure"]
impl crate::Readable for BmxCfg4Spec {}
#[doc = "`reset()` method sets BMX_CFG4 to value 0"]
impl crate::Resettable for BmxCfg4Spec {}
