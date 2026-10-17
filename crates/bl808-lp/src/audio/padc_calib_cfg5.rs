#[doc = "Register `padc_calib_cfg5` reader"]
pub type R = crate::R<PadcCalibCfg5Spec>;
#[doc = "Register `padc_calib_cfg5` writer"]
pub type W = crate::W<PadcCalibCfg5Spec>;
#[doc = "Field `padc_gaincal_ref` reader - "]
pub type PadcGaincalRefR = crate::FieldReader<u32>;
#[doc = "Field `padc_gaincal_ref` writer - "]
pub type PadcGaincalRefW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gaincal_ref(&self) -> PadcGaincalRefR {
        PadcGaincalRefR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn padc_gaincal_ref(&mut self) -> PadcGaincalRefW<'_, PadcCalibCfg5Spec> {
        PadcGaincalRefW::new(self, 0)
    }
}
#[doc = "PADC Calibration Config 5: Gain calibration reference\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_calib_cfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcCalibCfg5Spec;
impl crate::RegisterSpec for PadcCalibCfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_calib_cfg5::R`](R) reader structure"]
impl crate::Readable for PadcCalibCfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`padc_calib_cfg5::W`](W) writer structure"]
impl crate::Writable for PadcCalibCfg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets padc_calib_cfg5 to value 0"]
impl crate::Resettable for PadcCalibCfg5Spec {}
