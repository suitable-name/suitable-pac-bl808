#[doc = "Register `CORE_CFG25` reader"]
pub type R = crate::R<CoreCfg25Spec>;
#[doc = "Field `sts_e902_int_bus_1` reader - "]
pub type StsE902IntBus1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_e902_int_bus_1(&self) -> StsE902IntBus1R {
        StsE902IntBus1R::new(self.bits)
    }
}
#[doc = "E902 Interrupt Bus Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg25::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreCfg25Spec;
impl crate::RegisterSpec for CoreCfg25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_cfg25::R`](R) reader structure"]
impl crate::Readable for CoreCfg25Spec {}
#[doc = "`reset()` method sets CORE_CFG25 to value 0"]
impl crate::Resettable for CoreCfg25Spec {}
