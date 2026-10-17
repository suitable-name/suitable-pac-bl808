#[doc = "Register `DPI_V_TIMING_1` reader"]
pub type R = crate::R<DpiVTiming1Spec>;
#[doc = "Register `DPI_V_TIMING_1` writer"]
pub type W = crate::W<DpiVTiming1Spec>;
#[doc = "Field `reg_dpi_vact` reader - Vertical Active Height (lines). Range 208-480 lines."]
pub type RegDpiVactR = crate::FieldReader<u16>;
#[doc = "Field `reg_dpi_vact` writer - Vertical Active Height (lines). Range 208-480 lines."]
pub type RegDpiVactW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `reg_dpi_vfp` reader - Vertical Front Porch (lines)."]
pub type RegDpiVfpR = crate::FieldReader<u16>;
#[doc = "Field `reg_dpi_vfp` writer - Vertical Front Porch (lines)."]
pub type RegDpiVfpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Vertical Active Height (lines). Range 208-480 lines."]
    #[inline(always)]
    pub fn reg_dpi_vact(&self) -> RegDpiVactR {
        RegDpiVactR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Vertical Front Porch (lines)."]
    #[inline(always)]
    pub fn reg_dpi_vfp(&self) -> RegDpiVfpR {
        RegDpiVfpR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Vertical Active Height (lines). Range 208-480 lines."]
    #[inline(always)]
    pub fn reg_dpi_vact(&mut self) -> RegDpiVactW<'_, DpiVTiming1Spec> {
        RegDpiVactW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Vertical Front Porch (lines)."]
    #[inline(always)]
    pub fn reg_dpi_vfp(&mut self) -> RegDpiVfpW<'_, DpiVTiming1Spec> {
        RegDpiVfpW::new(self, 16)
    }
}
#[doc = "Vertical Active and Front Porch Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_v_timing_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_v_timing_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiVTiming1Spec;
impl crate::RegisterSpec for DpiVTiming1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_v_timing_1::R`](R) reader structure"]
impl crate::Readable for DpiVTiming1Spec {}
#[doc = "`write(|w| ..)` method takes [`dpi_v_timing_1::W`](W) writer structure"]
impl crate::Writable for DpiVTiming1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_V_TIMING_1 to value 0"]
impl crate::Resettable for DpiVTiming1Spec {}
