#[doc = "Register `RTC_TIME_H` reader"]
pub type R = crate::R<RtcTimeHSpec>;
#[doc = "Register `RTC_TIME_H` writer"]
pub type W = crate::W<RtcTimeHSpec>;
#[doc = "Field `rtc_time_latch_h` reader - "]
pub type RtcTimeLatchHR = crate::FieldReader;
#[doc = "Field `rtc_time_latch` writer - Write 1 to trigger latching of the 64-bit RTC counter"]
pub type RtcTimeLatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_time_latch_h(&self) -> RtcTimeLatchHR {
        RtcTimeLatchHR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Write 1 to trigger latching of the 64-bit RTC counter"]
    #[inline(always)]
    pub fn rtc_time_latch(&mut self) -> RtcTimeLatchW<'_, RtcTimeHSpec> {
        RtcTimeLatchW::new(self, 31)
    }
}
#[doc = "RTC Latch High and Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_time_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_time_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTimeHSpec;
impl crate::RegisterSpec for RtcTimeHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_time_h::R`](R) reader structure"]
impl crate::Readable for RtcTimeHSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_time_h::W`](W) writer structure"]
impl crate::Writable for RtcTimeHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_TIME_H to value 0"]
impl crate::Resettable for RtcTimeHSpec {}
