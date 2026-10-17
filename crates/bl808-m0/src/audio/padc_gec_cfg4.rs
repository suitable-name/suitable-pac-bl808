#[doc = "Register `padc_gec_cfg4` reader"]
pub type R = crate::R<PadcGecCfg4Spec>;
#[doc = "Field `padc_gec_g4` reader - "]
pub type PadcGecG4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gec_g4(&self) -> PadcGecG4R {
        PadcGecG4R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Gain Error Correction Config 4\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcGecCfg4Spec;
impl crate::RegisterSpec for PadcGecCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_gec_cfg4::R`](R) reader structure"]
impl crate::Readable for PadcGecCfg4Spec {}
#[doc = "`reset()` method sets padc_gec_cfg4 to value 0x0080_0000"]
impl crate::Resettable for PadcGecCfg4Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
