#[doc = "Register `padc_oec_cfg4` reader"]
pub type R = crate::R<PadcOecCfg4Spec>;
#[doc = "Field `padc_oec_g4` reader - "]
pub type PadcOecG4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_oec_g4(&self) -> PadcOecG4R {
        PadcOecG4R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Offset Error Correction Config 4\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcOecCfg4Spec;
impl crate::RegisterSpec for PadcOecCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_oec_cfg4::R`](R) reader structure"]
impl crate::Readable for PadcOecCfg4Spec {}
#[doc = "`reset()` method sets padc_oec_cfg4 to value 0"]
impl crate::Resettable for PadcOecCfg4Spec {}
