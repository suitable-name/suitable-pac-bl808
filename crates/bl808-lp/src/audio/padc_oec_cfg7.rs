#[doc = "Register `padc_oec_cfg7` reader"]
pub type R = crate::R<PadcOecCfg7Spec>;
#[doc = "Field `padc_oec_g7` reader - "]
pub type PadcOecG7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_oec_g7(&self) -> PadcOecG7R {
        PadcOecG7R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Offset Error Correction Config 7\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcOecCfg7Spec;
impl crate::RegisterSpec for PadcOecCfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_oec_cfg7::R`](R) reader structure"]
impl crate::Readable for PadcOecCfg7Spec {}
#[doc = "`reset()` method sets padc_oec_cfg7 to value 0"]
impl crate::Resettable for PadcOecCfg7Spec {}
