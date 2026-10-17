#[doc = "Register `padc_gec_cfg5` reader"]
pub type R = crate::R<PadcGecCfg5Spec>;
#[doc = "Field `padc_gec_g5` reader - "]
pub type PadcGecG5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gec_g5(&self) -> PadcGecG5R {
        PadcGecG5R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Gain Error Correction Config 5\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcGecCfg5Spec;
impl crate::RegisterSpec for PadcGecCfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_gec_cfg5::R`](R) reader structure"]
impl crate::Readable for PadcGecCfg5Spec {}
#[doc = "`reset()` method sets padc_gec_cfg5 to value 0x0080_0000"]
impl crate::Resettable for PadcGecCfg5Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
