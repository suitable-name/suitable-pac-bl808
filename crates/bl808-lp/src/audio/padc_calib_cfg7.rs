#[doc = "Register `padc_calib_cfg7` reader"]
pub type R = crate::R<PadcCalibCfg7Spec>;
#[doc = "Register `padc_calib_cfg7` writer"]
pub type W = crate::W<PadcCalibCfg7Spec>;
impl W {}
#[doc = "PADC Calibration Config 7: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_calib_cfg7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcCalibCfg7Spec;
impl crate::RegisterSpec for PadcCalibCfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_calib_cfg7::R`](R) reader structure"]
impl crate::Readable for PadcCalibCfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`padc_calib_cfg7::W`](W) writer structure"]
impl crate::Writable for PadcCalibCfg7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets padc_calib_cfg7 to value 0"]
impl crate::Resettable for PadcCalibCfg7Spec {}
