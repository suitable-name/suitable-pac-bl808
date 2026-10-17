#[doc = "Register `CORE_CFG24` reader"]
pub type R = crate::R<CoreCfg24Spec>;
#[doc = "Field `sts_e902_int_bus_0` reader - "]
pub type StsE902IntBus0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_e902_int_bus_0(&self) -> StsE902IntBus0R {
        StsE902IntBus0R::new(self.bits)
    }
}
#[doc = "E902 Interrupt Bus Status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreCfg24Spec;
impl crate::RegisterSpec for CoreCfg24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_cfg24::R`](R) reader structure"]
impl crate::Readable for CoreCfg24Spec {}
#[doc = "`reset()` method sets CORE_CFG24 to value 0"]
impl crate::Resettable for CoreCfg24Spec {}
