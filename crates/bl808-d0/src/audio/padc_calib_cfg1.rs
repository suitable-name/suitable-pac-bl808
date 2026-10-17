#[doc = "Register `padc_calib_cfg1` reader"]
pub type R = crate::R<PadcCalibCfg1Spec>;
#[doc = "Field `padc_oec` reader - "]
pub type PadcOecR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_oec(&self) -> PadcOecR {
        PadcOecR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PADC Calibration Config 1: Offset Error Correction result\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcCalibCfg1Spec;
impl crate::RegisterSpec for PadcCalibCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_calib_cfg1::R`](R) reader structure"]
impl crate::Readable for PadcCalibCfg1Spec {}
#[doc = "`reset()` method sets padc_calib_cfg1 to value 0"]
impl crate::Resettable for PadcCalibCfg1Spec {}
