#[doc = "Register `MCU_E907_RTC` reader"]
pub type R = crate::R<McuE907RtcSpec>;
#[doc = "Register `MCU_E907_RTC` writer"]
pub type W = crate::W<McuE907RtcSpec>;
#[doc = "Field `reg_mcu_rtc_div` reader - E907 RTC clock divider (10 bits)"]
pub type RegMcuRtcDivR = crate::FieldReader<u16>;
#[doc = "Field `reg_mcu_rtc_div` writer - E907 RTC clock divider (10 bits)"]
pub type RegMcuRtcDivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `reg_mcu_rtc_rst` reader - E907 RTC software reset"]
pub type RegMcuRtcRstR = crate::BitReader;
#[doc = "Field `reg_mcu_rtc_rst` writer - E907 RTC software reset"]
pub type RegMcuRtcRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_rtc_en` reader - E907 RTC clock enable"]
pub type RegMcuRtcEnR = crate::BitReader;
#[doc = "Field `reg_mcu_rtc_en` writer - E907 RTC clock enable"]
pub type RegMcuRtcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - E907 RTC clock divider (10 bits)"]
    #[inline(always)]
    pub fn reg_mcu_rtc_div(&self) -> RegMcuRtcDivR {
        RegMcuRtcDivR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - E907 RTC software reset"]
    #[inline(always)]
    pub fn reg_mcu_rtc_rst(&self) -> RegMcuRtcRstR {
        RegMcuRtcRstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - E907 RTC clock enable"]
    #[inline(always)]
    pub fn reg_mcu_rtc_en(&self) -> RegMcuRtcEnR {
        RegMcuRtcEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - E907 RTC clock divider (10 bits)"]
    #[inline(always)]
    pub fn reg_mcu_rtc_div(&mut self) -> RegMcuRtcDivW<'_, McuE907RtcSpec> {
        RegMcuRtcDivW::new(self, 0)
    }
    #[doc = "Bit 30 - E907 RTC software reset"]
    #[inline(always)]
    pub fn reg_mcu_rtc_rst(&mut self) -> RegMcuRtcRstW<'_, McuE907RtcSpec> {
        RegMcuRtcRstW::new(self, 30)
    }
    #[doc = "Bit 31 - E907 RTC clock enable"]
    #[inline(always)]
    pub fn reg_mcu_rtc_en(&mut self) -> RegMcuRtcEnW<'_, McuE907RtcSpec> {
        RegMcuRtcEnW::new(self, 31)
    }
}
#[doc = "E907 (MCU1) Real-Time Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu_e907_rtc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcu_e907_rtc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McuE907RtcSpec;
impl crate::RegisterSpec for McuE907RtcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu_e907_rtc::R`](R) reader structure"]
impl crate::Readable for McuE907RtcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcu_e907_rtc::W`](W) writer structure"]
impl crate::Writable for McuE907RtcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCU_E907_RTC to value 0x8000_000a"]
impl crate::Resettable for McuE907RtcSpec {
    const RESET_VALUE: u32 = 0x8000_000a;
}
