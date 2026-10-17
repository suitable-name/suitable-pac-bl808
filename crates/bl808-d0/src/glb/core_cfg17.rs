#[doc = "Register `CORE_CFG17` reader"]
pub type R = crate::R<CoreCfg17Spec>;
#[doc = "Field `np_int_sta1` reader - "]
pub type NpIntSta1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn np_int_sta1(&self) -> NpIntSta1R {
        NpIntSta1R::new(self.bits)
    }
}
#[doc = "New Peripheral Interrupt Status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_cfg17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreCfg17Spec;
impl crate::RegisterSpec for CoreCfg17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_cfg17::R`](R) reader structure"]
impl crate::Readable for CoreCfg17Spec {}
#[doc = "`reset()` method sets CORE_CFG17 to value 0"]
impl crate::Resettable for CoreCfg17Spec {}
