#[doc = "Register `CORE_CFG16` reader"]
pub type R = crate::R<CoreCfg16Spec>;
#[doc = "Field `np_int_sta0` reader - "]
pub type NpIntSta0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn np_int_sta0(&self) -> NpIntSta0R {
        NpIntSta0R::new(self.bits)
    }
}
#[doc = "New Peripheral Interrupt Status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreCfg16Spec;
impl crate::RegisterSpec for CoreCfg16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_cfg16::R`](R) reader structure"]
impl crate::Readable for CoreCfg16Spec {}
#[doc = "`reset()` method sets CORE_CFG16 to value 0"]
impl crate::Resettable for CoreCfg16Spec {}
