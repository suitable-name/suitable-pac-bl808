#[doc = "Register `padc_oec_cfg0` reader"]
pub type R = crate::R<PadcOecCfg0Spec>;
#[doc = "Field `padc_oec_g0` reader - "]
pub type PadcOecG0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_oec_g0(&self) -> PadcOecG0R {
        PadcOecG0R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Offset Error Correction Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_oec_cfg0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcOecCfg0Spec;
impl crate::RegisterSpec for PadcOecCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_oec_cfg0::R`](R) reader structure"]
impl crate::Readable for PadcOecCfg0Spec {}
#[doc = "`reset()` method sets padc_oec_cfg0 to value 0"]
impl crate::Resettable for PadcOecCfg0Spec {}
