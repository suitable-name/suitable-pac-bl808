#[doc = "Register `BMX_CFG5` reader"]
pub type R = crate::R<BmxCfg5Spec>;
#[doc = "Field `sts_bmx_berr_addr` reader - "]
pub type StsBmxBerrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_bmx_berr_addr(&self) -> StsBmxBerrAddrR {
        StsBmxBerrAddrR::new(self.bits)
    }
}
#[doc = "BMX Bus Matrix Config 5 (BMX Bus Error Address)\n\nYou can [`read`](crate::Reg::read) this register and get [`bmx_cfg5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmxCfg5Spec;
impl crate::RegisterSpec for BmxCfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg5::R`](R) reader structure"]
impl crate::Readable for BmxCfg5Spec {}
#[doc = "`reset()` method sets BMX_CFG5 to value 0"]
impl crate::Resettable for BmxCfg5Spec {}
