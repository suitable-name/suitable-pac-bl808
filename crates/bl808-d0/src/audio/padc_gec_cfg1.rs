#[doc = "Register `padc_gec_cfg1` reader"]
pub type R = crate::R<PadcGecCfg1Spec>;
#[doc = "Field `padc_gec_g1` reader - "]
pub type PadcGecG1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gec_g1(&self) -> PadcGecG1R {
        PadcGecG1R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Gain Error Correction Config 1\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcGecCfg1Spec;
impl crate::RegisterSpec for PadcGecCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_gec_cfg1::R`](R) reader structure"]
impl crate::Readable for PadcGecCfg1Spec {}
#[doc = "`reset()` method sets padc_gec_cfg1 to value 0x0080_0000"]
impl crate::Resettable for PadcGecCfg1Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
