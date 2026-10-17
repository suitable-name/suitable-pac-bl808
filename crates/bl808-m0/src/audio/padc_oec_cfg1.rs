#[doc = "Register `padc_oec_cfg1` reader"]
pub type R = crate::R<PadcOecCfg1Spec>;
#[doc = "Field `padc_oec_g1` reader - "]
pub type PadcOecG1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_oec_g1(&self) -> PadcOecG1R {
        PadcOecG1R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Offset Error Correction Config 1\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcOecCfg1Spec;
impl crate::RegisterSpec for PadcOecCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_oec_cfg1::R`](R) reader structure"]
impl crate::Readable for PadcOecCfg1Spec {}
#[doc = "`reset()` method sets padc_oec_cfg1 to value 0"]
impl crate::Resettable for PadcOecCfg1Spec {}
