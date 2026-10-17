#[doc = "Register `DPI_H_TIMING_0` reader"]
pub type R = crate::R<DpiHTiming0Spec>;
#[doc = "Register `DPI_H_TIMING_0` writer"]
pub type W = crate::W<DpiHTiming0Spec>;
#[doc = "Field `reg_dpi_hsw` reader - Horizontal Sync Width (pixels). Range 1-255 PCLK cycles."]
pub type RegDpiHswR = crate::FieldReader<u16>;
#[doc = "Field `reg_dpi_hsw` writer - Horizontal Sync Width (pixels). Range 1-255 PCLK cycles."]
pub type RegDpiHswW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `reg_dpi_hbp` reader - Horizontal Back Porch (pixels)."]
pub type RegDpiHbpR = crate::FieldReader<u16>;
#[doc = "Field `reg_dpi_hbp` writer - Horizontal Back Porch (pixels)."]
pub type RegDpiHbpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal Sync Width (pixels). Range 1-255 PCLK cycles."]
    #[inline(always)]
    pub fn reg_dpi_hsw(&self) -> RegDpiHswR {
        RegDpiHswR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Horizontal Back Porch (pixels)."]
    #[inline(always)]
    pub fn reg_dpi_hbp(&self) -> RegDpiHbpR {
        RegDpiHbpR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal Sync Width (pixels). Range 1-255 PCLK cycles."]
    #[inline(always)]
    pub fn reg_dpi_hsw(&mut self) -> RegDpiHswW<'_, DpiHTiming0Spec> {
        RegDpiHswW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Horizontal Back Porch (pixels)."]
    #[inline(always)]
    pub fn reg_dpi_hbp(&mut self) -> RegDpiHbpW<'_, DpiHTiming0Spec> {
        RegDpiHbpW::new(self, 16)
    }
}
#[doc = "Horizontal Sync and Back Porch Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_h_timing_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_h_timing_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiHTiming0Spec;
impl crate::RegisterSpec for DpiHTiming0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_h_timing_0::R`](R) reader structure"]
impl crate::Readable for DpiHTiming0Spec {}
#[doc = "`write(|w| ..)` method takes [`dpi_h_timing_0::W`](W) writer structure"]
impl crate::Writable for DpiHTiming0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_H_TIMING_0 to value 0"]
impl crate::Resettable for DpiHTiming0Spec {}
