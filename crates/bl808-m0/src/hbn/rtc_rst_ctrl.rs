#[doc = "Register `RTC_RST_CTRL` reader"]
pub type R = crate::R<RtcRstCtrlSpec>;
#[doc = "Register `RTC_RST_CTRL` writer"]
pub type W = crate::W<RtcRstCtrlSpec>;
#[doc = "Field `rtc_rst_wait_cnt_rtc` reader - "]
pub type RtcRstWaitCntRtcR = crate::FieldReader<u16>;
#[doc = "Field `rtc_rst_wait_cnt_rtc` writer - "]
pub type RtcRstWaitCntRtcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rtc_rst_refdiv_rtc` reader - "]
pub type RtcRstRefdivRtcR = crate::FieldReader;
#[doc = "Field `rtc_rst_refdiv_rtc` writer - "]
pub type RtcRstRefdivRtcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rtc_rst_ctrl_misc` reader - "]
pub type RtcRstCtrlMiscR = crate::FieldReader<u16>;
#[doc = "Field `rtc_rst_ctrl_misc` writer - "]
pub type RtcRstCtrlMiscW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rtc_rst_wait_cnt_rtc(&self) -> RtcRstWaitCntRtcR {
        RtcRstWaitCntRtcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rtc_rst_refdiv_rtc(&self) -> RtcRstRefdivRtcR {
        RtcRstRefdivRtcR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn rtc_rst_ctrl_misc(&self) -> RtcRstCtrlMiscR {
        RtcRstCtrlMiscR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rtc_rst_wait_cnt_rtc(&mut self) -> RtcRstWaitCntRtcW<'_, RtcRstCtrlSpec> {
        RtcRstWaitCntRtcW::new(self, 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rtc_rst_refdiv_rtc(&mut self) -> RtcRstRefdivRtcW<'_, RtcRstCtrlSpec> {
        RtcRstRefdivRtcW::new(self, 16)
    }
    #[doc = "Bits 19:31"]
    #[inline(always)]
    pub fn rtc_rst_ctrl_misc(&mut self) -> RtcRstCtrlMiscW<'_, RtcRstCtrlSpec> {
        RtcRstCtrlMiscW::new(self, 19)
    }
}
#[doc = "RTC Reset Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcRstCtrlSpec;
impl crate::RegisterSpec for RtcRstCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for RtcRstCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for RtcRstCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_RST_CTRL to value 0x5094_3c00"]
impl crate::Resettable for RtcRstCtrlSpec {
    const RESET_VALUE: u32 = 0x5094_3c00;
}
