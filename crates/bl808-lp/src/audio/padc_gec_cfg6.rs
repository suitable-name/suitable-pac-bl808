#[doc = "Register `padc_gec_cfg6` reader"]
pub type R = crate::R<PadcGecCfg6Spec>;
#[doc = "Field `padc_gec_g6` reader - "]
pub type PadcGecG6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gec_g6(&self) -> PadcGecG6R {
        PadcGecG6R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Gain Error Correction Config 6\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcGecCfg6Spec;
impl crate::RegisterSpec for PadcGecCfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_gec_cfg6::R`](R) reader structure"]
impl crate::Readable for PadcGecCfg6Spec {}
#[doc = "`reset()` method sets padc_gec_cfg6 to value 0x0080_0000"]
impl crate::Resettable for PadcGecCfg6Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
