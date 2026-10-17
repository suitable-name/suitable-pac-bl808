#[doc = "Register `TZC_I2C_CTRL` reader"]
pub type R = crate::R<TzcI2cCtrlSpec>;
#[doc = "I2C TrustZone Control (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_i2c_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcI2cCtrlSpec;
impl crate::RegisterSpec for TzcI2cCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_i2c_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcI2cCtrlSpec {}
#[doc = "`reset()` method sets TZC_I2C_CTRL to value 0"]
impl crate::Resettable for TzcI2cCtrlSpec {}
