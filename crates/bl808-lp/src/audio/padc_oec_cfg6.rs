#[doc = "Register `padc_oec_cfg6` reader"]
pub type R = crate::R<PadcOecCfg6Spec>;
#[doc = "Field `padc_oec_g6` reader - "]
pub type PadcOecG6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_oec_g6(&self) -> PadcOecG6R {
        PadcOecG6R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Offset Error Correction Config 6\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcOecCfg6Spec;
impl crate::RegisterSpec for PadcOecCfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_oec_cfg6::R`](R) reader structure"]
impl crate::Readable for PadcOecCfg6Spec {}
#[doc = "`reset()` method sets padc_oec_cfg6 to value 0"]
impl crate::Resettable for PadcOecCfg6Spec {}
