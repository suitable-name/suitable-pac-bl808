#[doc = "Register `RTC_CFG0` reader"]
pub type R = crate::R<RtcCfg0Spec>;
#[doc = "Register `RTC_CFG0` writer"]
pub type W = crate::W<RtcCfg0Spec>;
#[doc = "Field `cpu_rtc_div` reader - "]
pub type CpuRtcDivR = crate::FieldReader<u32>;
#[doc = "Field `cpu_rtc_div` writer - "]
pub type CpuRtcDivW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `cpu_rtc_en` reader - "]
pub type CpuRtcEnR = crate::BitReader;
#[doc = "Field `cpu_rtc_en` writer - "]
pub type CpuRtcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_rtc_sel` reader - "]
pub type CpuRtcSelR = crate::BitReader;
#[doc = "Field `cpu_rtc_sel` writer - "]
pub type CpuRtcSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn cpu_rtc_div(&self) -> CpuRtcDivR {
        CpuRtcDivR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cpu_rtc_en(&self) -> CpuRtcEnR {
        CpuRtcEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cpu_rtc_sel(&self) -> CpuRtcSelR {
        CpuRtcSelR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn cpu_rtc_div(&mut self) -> CpuRtcDivW<'_, RtcCfg0Spec> {
        CpuRtcDivW::new(self, 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cpu_rtc_en(&mut self) -> CpuRtcEnW<'_, RtcCfg0Spec> {
        CpuRtcEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cpu_rtc_sel(&mut self) -> CpuRtcSelW<'_, RtcCfg0Spec> {
        CpuRtcSelW::new(self, 19)
    }
}
#[doc = "RTC Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCfg0Spec;
impl crate::RegisterSpec for RtcCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_cfg0::R`](R) reader structure"]
impl crate::Readable for RtcCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`rtc_cfg0::W`](W) writer structure"]
impl crate::Writable for RtcCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CFG0 to value 0"]
impl crate::Resettable for RtcCfg0Spec {}
