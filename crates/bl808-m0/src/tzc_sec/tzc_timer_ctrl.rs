#[doc = "Register `TZC_TIMER_CTRL` reader"]
pub type R = crate::R<TzcTimerCtrlSpec>;
#[doc = "TIMER TrustZone Control (Reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_timer_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcTimerCtrlSpec;
impl crate::RegisterSpec for TzcTimerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_timer_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcTimerCtrlSpec {}
#[doc = "`reset()` method sets TZC_TIMER_CTRL to value 0"]
impl crate::Resettable for TzcTimerCtrlSpec {}
