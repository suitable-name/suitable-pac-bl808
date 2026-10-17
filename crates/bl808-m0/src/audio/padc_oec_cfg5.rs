#[doc = "Register `padc_oec_cfg5` reader"]
pub type R = crate::R<PadcOecCfg5Spec>;
#[doc = "Field `padc_oec_g5` reader - "]
pub type PadcOecG5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_oec_g5(&self) -> PadcOecG5R {
        PadcOecG5R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Offset Error Correction Config 5\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcOecCfg5Spec;
impl crate::RegisterSpec for PadcOecCfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_oec_cfg5::R`](R) reader structure"]
impl crate::Readable for PadcOecCfg5Spec {}
#[doc = "`reset()` method sets padc_oec_cfg5 to value 0"]
impl crate::Resettable for PadcOecCfg5Spec {}
