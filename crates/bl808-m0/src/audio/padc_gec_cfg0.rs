#[doc = "Register `padc_gec_cfg0` reader"]
pub type R = crate::R<PadcGecCfg0Spec>;
#[doc = "Field `padc_gec_g0` reader - "]
pub type PadcGecG0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gec_g0(&self) -> PadcGecG0R {
        PadcGecG0R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Gain Error Correction Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_gec_cfg0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcGecCfg0Spec;
impl crate::RegisterSpec for PadcGecCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_gec_cfg0::R`](R) reader structure"]
impl crate::Readable for PadcGecCfg0Spec {}
#[doc = "`reset()` method sets padc_gec_cfg0 to value 0x0080_0000"]
impl crate::Resettable for PadcGecCfg0Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
