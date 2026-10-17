#[doc = "Register `TZC_I2S_CTRL` reader"]
pub type R = crate::R<TzcI2sCtrlSpec>;
#[doc = "Register `TZC_I2S_CTRL` writer"]
pub type W = crate::W<TzcI2sCtrlSpec>;
impl W {}
#[doc = "I2S Control (Reserved/Placeholder for TZSID)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_i2s_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_i2s_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcI2sCtrlSpec;
impl crate::RegisterSpec for TzcI2sCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_i2s_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcI2sCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_i2s_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcI2sCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_I2S_CTRL to value 0"]
impl crate::Resettable for TzcI2sCtrlSpec {}
