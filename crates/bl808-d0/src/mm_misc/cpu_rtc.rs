#[doc = "Register `CPU_RTC` reader"]
pub type R = crate::R<CpuRtcSpec>;
#[doc = "Register `CPU_RTC` writer"]
pub type W = crate::W<CpuRtcSpec>;
#[doc = "Field `c906_rtc_div` reader - C906 Real-Time Clock divider factor."]
pub type C906RtcDivR = crate::FieldReader<u16>;
#[doc = "Field `c906_rtc_div` writer - C906 Real-Time Clock divider factor."]
pub type C906RtcDivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `c906_rtc_rst` reader - C906 RTC software reset."]
pub type C906RtcRstR = crate::BitReader;
#[doc = "Field `c906_rtc_rst` writer - C906 RTC software reset."]
pub type C906RtcRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `c906_rtc_en` reader - C906 RTC clock enable."]
pub type C906RtcEnR = crate::BitReader;
#[doc = "Field `c906_rtc_en` writer - C906 RTC clock enable."]
pub type C906RtcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - C906 Real-Time Clock divider factor."]
    #[inline(always)]
    pub fn c906_rtc_div(&self) -> C906RtcDivR {
        C906RtcDivR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - C906 RTC software reset."]
    #[inline(always)]
    pub fn c906_rtc_rst(&self) -> C906RtcRstR {
        C906RtcRstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - C906 RTC clock enable."]
    #[inline(always)]
    pub fn c906_rtc_en(&self) -> C906RtcEnR {
        C906RtcEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - C906 Real-Time Clock divider factor."]
    #[inline(always)]
    pub fn c906_rtc_div(&mut self) -> C906RtcDivW<'_, CpuRtcSpec> {
        C906RtcDivW::new(self, 0)
    }
    #[doc = "Bit 30 - C906 RTC software reset."]
    #[inline(always)]
    pub fn c906_rtc_rst(&mut self) -> C906RtcRstW<'_, CpuRtcSpec> {
        C906RtcRstW::new(self, 30)
    }
    #[doc = "Bit 31 - C906 RTC clock enable."]
    #[inline(always)]
    pub fn c906_rtc_en(&mut self) -> C906RtcEnW<'_, CpuRtcSpec> {
        C906RtcEnW::new(self, 31)
    }
}
#[doc = "D0 Core (C906) RTC Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_rtc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_rtc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuRtcSpec;
impl crate::RegisterSpec for CpuRtcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_rtc::R`](R) reader structure"]
impl crate::Readable for CpuRtcSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_rtc::W`](W) writer structure"]
impl crate::Writable for CpuRtcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_RTC to value 0"]
impl crate::Resettable for CpuRtcSpec {}
