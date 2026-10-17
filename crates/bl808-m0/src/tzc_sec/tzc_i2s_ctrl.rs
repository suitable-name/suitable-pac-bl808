#[doc = "Register `TZC_I2S_CTRL` reader"]
pub type R = crate::R<TzcI2sCtrlSpec>;
#[doc = "I2S TrustZone Control (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_i2s_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcI2sCtrlSpec;
impl crate::RegisterSpec for TzcI2sCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_i2s_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcI2sCtrlSpec {}
#[doc = "`reset()` method sets TZC_I2S_CTRL to value 0"]
impl crate::Resettable for TzcI2sCtrlSpec {}
