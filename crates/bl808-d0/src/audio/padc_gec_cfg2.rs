#[doc = "Register `padc_gec_cfg2` reader"]
pub type R = crate::R<PadcGecCfg2Spec>;
#[doc = "Field `padc_gec_g2` reader - "]
pub type PadcGecG2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gec_g2(&self) -> PadcGecG2R {
        PadcGecG2R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Gain Error Correction Config 2\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcGecCfg2Spec;
impl crate::RegisterSpec for PadcGecCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_gec_cfg2::R`](R) reader structure"]
impl crate::Readable for PadcGecCfg2Spec {}
#[doc = "`reset()` method sets padc_gec_cfg2 to value 0x0080_0000"]
impl crate::Resettable for PadcGecCfg2Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
