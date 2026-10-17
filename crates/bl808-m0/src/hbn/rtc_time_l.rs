#[doc = "Register `RTC_TIME_L` reader"]
pub type R = crate::R<RtcTimeLSpec>;
#[doc = "Field `rtc_time_latch_l` reader - "]
pub type RtcTimeLatchLR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_time_latch_l(&self) -> RtcTimeLatchLR {
        RtcTimeLatchLR::new(self.bits)
    }
}
#[doc = "RTC Latch Low\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_time_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTimeLSpec;
impl crate::RegisterSpec for RtcTimeLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_time_l::R`](R) reader structure"]
impl crate::Readable for RtcTimeLSpec {}
#[doc = "`reset()` method sets RTC_TIME_L to value 0"]
impl crate::Resettable for RtcTimeLSpec {}
