#[doc = "Register `BMX_CFG6` reader"]
pub type R = crate::R<BmxCfg6Spec>;
#[doc = "Field `sts_mcu_berr_addr` reader - "]
pub type StsMcuBerrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_mcu_berr_addr(&self) -> StsMcuBerrAddrR {
        StsMcuBerrAddrR::new(self.bits)
    }
}
#[doc = "BMX Bus Matrix Config 6 (MCU Bus Error Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxCfg6Spec;
impl crate::RegisterSpec for BmxCfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg6::R`](R) reader structure"]
impl crate::Readable for BmxCfg6Spec {}
#[doc = "`reset()` method sets BMX_CFG6 to value 0"]
impl crate::Resettable for BmxCfg6Spec {}
