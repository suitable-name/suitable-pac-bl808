#[doc = "Register `padc_oec_cfg2` reader"]
pub type R = crate::R<PadcOecCfg2Spec>;
#[doc = "Field `padc_oec_g2` reader - "]
pub type PadcOecG2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_oec_g2(&self) -> PadcOecG2R {
        PadcOecG2R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Offset Error Correction Config 2\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcOecCfg2Spec;
impl crate::RegisterSpec for PadcOecCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_oec_cfg2::R`](R) reader structure"]
impl crate::Readable for PadcOecCfg2Spec {}
#[doc = "`reset()` method sets padc_oec_cfg2 to value 0"]
impl crate::Resettable for PadcOecCfg2Spec {}
