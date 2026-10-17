#[doc = "Register `padc_gec_cfg7` reader"]
pub type R = crate::R<PadcGecCfg7Spec>;
#[doc = "Field `padc_gec_g7` reader - "]
pub type PadcGecG7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gec_g7(&self) -> PadcGecG7R {
        PadcGecG7R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Gain Error Correction Config 7\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcGecCfg7Spec;
impl crate::RegisterSpec for PadcGecCfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_gec_cfg7::R`](R) reader structure"]
impl crate::Readable for PadcGecCfg7Spec {}
#[doc = "`reset()` method sets padc_gec_cfg7 to value 0x0080_0000"]
impl crate::Resettable for PadcGecCfg7Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
