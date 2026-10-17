#[doc = "Register `RTC_RST_CTRL2` reader"]
pub type R = crate::R<RtcRstCtrl2Spec>;
#[doc = "Register `RTC_RST_CTRL2` writer"]
pub type W = crate::W<RtcRstCtrl2Spec>;
#[doc = "Field `rtc_resv` reader - "]
pub type RtcResvR = crate::FieldReader;
#[doc = "Field `rtc_resv` writer - "]
pub type RtcResvW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_en_hw_pu_rc32k` reader - "]
pub type RegEnHwPuRc32kR = crate::BitReader;
#[doc = "Field `reg_en_hw_pu_rc32k` writer - "]
pub type RegEnHwPuRc32kW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_resv(&self) -> RtcResvR {
        RtcResvR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_en_hw_pu_rc32k(&self) -> RegEnHwPuRc32kR {
        RegEnHwPuRc32kR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_resv(&mut self) -> RtcResvW<'_, RtcRstCtrl2Spec> {
        RtcResvW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_en_hw_pu_rc32k(&mut self) -> RegEnHwPuRc32kW<'_, RtcRstCtrl2Spec> {
        RegEnHwPuRc32kW::new(self, 8)
    }
}
#[doc = "RTC Reset Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_rst_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_rst_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcRstCtrl2Spec;
impl crate::RegisterSpec for RtcRstCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_rst_ctrl2::R`](R) reader structure"]
impl crate::Readable for RtcRstCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`rtc_rst_ctrl2::W`](W) writer structure"]
impl crate::Writable for RtcRstCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_RST_CTRL2 to value 0"]
impl crate::Resettable for RtcRstCtrl2Spec {}
