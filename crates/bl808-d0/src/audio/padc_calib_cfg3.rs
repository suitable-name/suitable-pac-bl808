#[doc = "Register `padc_calib_cfg3` reader"]
pub type R = crate::R<PadcCalibCfg3Spec>;
#[doc = "Register `padc_calib_cfg3` writer"]
pub type W = crate::W<PadcCalibCfg3Spec>;
impl W {}
#[doc = "PADC Calibration Config 3: Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_calib_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcCalibCfg3Spec;
impl crate::RegisterSpec for PadcCalibCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_calib_cfg3::R`](R) reader structure"]
impl crate::Readable for PadcCalibCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`padc_calib_cfg3::W`](W) writer structure"]
impl crate::Writable for PadcCalibCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets padc_calib_cfg3 to value 0"]
impl crate::Resettable for PadcCalibCfg3Spec {}
