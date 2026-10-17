#[doc = "Register `padc_oec_cfg3` reader"]
pub type R = crate::R<PadcOecCfg3Spec>;
#[doc = "Field `padc_oec_g3` reader - "]
pub type PadcOecG3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_oec_g3(&self) -> PadcOecG3R {
        PadcOecG3R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Offset Error Correction Config 3\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcOecCfg3Spec;
impl crate::RegisterSpec for PadcOecCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_oec_cfg3::R`](R) reader structure"]
impl crate::Readable for PadcOecCfg3Spec {}
#[doc = "`reset()` method sets padc_oec_cfg3 to value 0"]
impl crate::Resettable for PadcOecCfg3Spec {}
