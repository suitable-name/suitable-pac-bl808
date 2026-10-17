#[doc = "Register `padc_calib_cfg2` reader"]
pub type R = crate::R<PadcCalibCfg2Spec>;
#[doc = "Field `padc_gec` reader - "]
pub type PadcGecR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gec(&self) -> PadcGecR {
        PadcGecR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Calibration Config 2: Gain Error Correction result\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcCalibCfg2Spec;
impl crate::RegisterSpec for PadcCalibCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_calib_cfg2::R`](R) reader structure"]
impl crate::Readable for PadcCalibCfg2Spec {}
#[doc = "`reset()` method sets padc_calib_cfg2 to value 0x0080_0000"]
impl crate::Resettable for PadcCalibCfg2Spec {
    const RESET_VALUE: u32 = 0x0080_0000;
}
