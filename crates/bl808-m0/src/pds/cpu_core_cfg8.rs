#[doc = "Register `CPU_CORE_CFG8` reader"]
pub type R = crate::R<CpuCoreCfg8Spec>;
#[doc = "Register `CPU_CORE_CFG8` writer"]
pub type W = crate::W<CpuCoreCfg8Spec>;
#[doc = "Field `e902_rtc_div` reader - "]
pub type E902RtcDivR = crate::FieldReader<u16>;
#[doc = "Field `e902_rtc_div` writer - "]
pub type E902RtcDivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `e902_rtc_rst` reader - "]
pub type E902RtcRstR = crate::BitReader;
#[doc = "Field `e902_rtc_rst` writer - "]
pub type E902RtcRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `e902_rtc_en` reader - "]
pub type E902RtcEnR = crate::BitReader;
#[doc = "Field `e902_rtc_en` writer - "]
pub type E902RtcEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn e902_rtc_div(&self) -> E902RtcDivR {
        E902RtcDivR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn e902_rtc_rst(&self) -> E902RtcRstR {
        E902RtcRstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn e902_rtc_en(&self) -> E902RtcEnR {
        E902RtcEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn e902_rtc_div(&mut self) -> E902RtcDivW<'_, CpuCoreCfg8Spec> {
        E902RtcDivW::new(self, 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn e902_rtc_rst(&mut self) -> E902RtcRstW<'_, CpuCoreCfg8Spec> {
        E902RtcRstW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn e902_rtc_en(&mut self) -> E902RtcEnW<'_, CpuCoreCfg8Spec> {
        E902RtcEnW::new(self, 31)
    }
}
#[doc = "E902 (PICO) RTC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCoreCfg8Spec;
impl crate::RegisterSpec for CpuCoreCfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg8::R`](R) reader structure"]
impl crate::Readable for CpuCoreCfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_core_cfg8::W`](W) writer structure"]
impl crate::Writable for CpuCoreCfg8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CORE_CFG8 to value 0x0a"]
impl crate::Resettable for CpuCoreCfg8Spec {
    const RESET_VALUE: u32 = 0x0a;
}
