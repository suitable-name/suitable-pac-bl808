#[doc = "Register `DPI_V_TIMING_0` reader"]
pub type R = crate::R<DpiVTiming0Spec>;
#[doc = "Register `DPI_V_TIMING_0` writer"]
pub type W = crate::W<DpiVTiming0Spec>;
#[doc = "Field `reg_dpi_vsw` reader - Vertical Sync Width (lines). Range 1-255 lines."]
pub type RegDpiVswR = crate::FieldReader<u16>;
#[doc = "Field `reg_dpi_vsw` writer - Vertical Sync Width (lines). Range 1-255 lines."]
pub type RegDpiVswW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `reg_dpi_vbp` reader - Vertical Back Porch (lines)."]
pub type RegDpiVbpR = crate::FieldReader<u16>;
#[doc = "Field `reg_dpi_vbp` writer - Vertical Back Porch (lines)."]
pub type RegDpiVbpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Vertical Sync Width (lines). Range 1-255 lines."]
    #[inline(always)]
    pub fn reg_dpi_vsw(&self) -> RegDpiVswR {
        RegDpiVswR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Vertical Back Porch (lines)."]
    #[inline(always)]
    pub fn reg_dpi_vbp(&self) -> RegDpiVbpR {
        RegDpiVbpR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Vertical Sync Width (lines). Range 1-255 lines."]
    #[inline(always)]
    pub fn reg_dpi_vsw(&mut self) -> RegDpiVswW<'_, DpiVTiming0Spec> {
        RegDpiVswW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Vertical Back Porch (lines)."]
    #[inline(always)]
    pub fn reg_dpi_vbp(&mut self) -> RegDpiVbpW<'_, DpiVTiming0Spec> {
        RegDpiVbpW::new(self, 16)
    }
}
#[doc = "Vertical Sync and Back Porch Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_v_timing_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_v_timing_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiVTiming0Spec;
impl crate::RegisterSpec for DpiVTiming0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_v_timing_0::R`](R) reader structure"]
impl crate::Readable for DpiVTiming0Spec {}
#[doc = "`write(|w| ..)` method takes [`dpi_v_timing_0::W`](W) writer structure"]
impl crate::Writable for DpiVTiming0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_V_TIMING_0 to value 0"]
impl crate::Resettable for DpiVTiming0Spec {}
