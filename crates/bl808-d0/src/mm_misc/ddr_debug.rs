#[doc = "Register `DDR_DEBUG` reader"]
pub type R = crate::R<DdrDebugSpec>;
#[doc = "Field `ddr_calib_done` reader - DDR PHY calibration complete status."]
pub type DdrCalibDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DDR PHY calibration complete status."]
    #[inline(always)]
    pub fn ddr_calib_done(&self) -> DdrCalibDoneR {
        DdrCalibDoneR::new((self.bits & 1) != 0)
    }
}
#[doc = "DDR Calibration Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr_debug::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDebugSpec;
impl crate::RegisterSpec for DdrDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_debug::R`](R) reader structure"]
impl crate::Readable for DdrDebugSpec {}
#[doc = "`reset()` method sets DDR_DEBUG to value 0"]
impl crate::Resettable for DdrDebugSpec {}
