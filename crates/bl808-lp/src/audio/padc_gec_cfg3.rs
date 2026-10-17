#[doc = "Register `padc_gec_cfg3` reader"]
pub type R = crate::R<PadcGecCfg3Spec>;
#[doc = "Field `padc_gec_g3` reader - "]
pub type PadcGecG3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gec_g3(&self) -> PadcGecG3R {
        PadcGecG3R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Gain Error Correction Config 3\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcGecCfg3Spec;
impl crate::RegisterSpec for PadcGecCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_gec_cfg3::R`](R) reader structure"]
impl crate::Readable for PadcGecCfg3Spec {}
#[doc = "`reset()` method sets padc_gec_cfg3 to value 0x0080_0000"]
impl crate::Resettable for PadcGecCfg3Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
