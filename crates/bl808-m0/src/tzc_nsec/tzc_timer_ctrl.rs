#[doc = "Register `TZC_TIMER_CTRL` reader"]
pub type R = crate::R<TzcTimerCtrlSpec>;
#[doc = "Register `TZC_TIMER_CTRL` writer"]
pub type W = crate::W<TzcTimerCtrlSpec>;
impl W {}
#[doc = "TIMER Control (Reserved/Placeholder for TZSID)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_timer_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_timer_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcTimerCtrlSpec;
impl crate::RegisterSpec for TzcTimerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_timer_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcTimerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_timer_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcTimerCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_TIMER_CTRL to value 0"]
impl crate::Resettable for TzcTimerCtrlSpec {}
