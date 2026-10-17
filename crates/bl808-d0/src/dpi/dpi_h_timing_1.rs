#[doc = "Register `DPI_H_TIMING_1` reader"]
pub type R = crate::R<DpiHTiming1Spec>;
#[doc = "Register `DPI_H_TIMING_1` writer"]
pub type W = crate::W<DpiHTiming1Spec>;
#[doc = "Field `reg_dpi_hact` reader - Horizontal Active Width (pixels). Range 176-800 pixels."]
pub type RegDpiHactR = crate::FieldReader<u16>;
#[doc = "Field `reg_dpi_hact` writer - Horizontal Active Width (pixels). Range 176-800 pixels."]
pub type RegDpiHactW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `reg_dpi_hfp` reader - Horizontal Front Porch (pixels)."]
pub type RegDpiHfpR = crate::FieldReader<u16>;
#[doc = "Field `reg_dpi_hfp` writer - Horizontal Front Porch (pixels)."]
pub type RegDpiHfpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal Active Width (pixels). Range 176-800 pixels."]
    #[inline(always)]
    pub fn reg_dpi_hact(&self) -> RegDpiHactR {
        RegDpiHactR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Horizontal Front Porch (pixels)."]
    #[inline(always)]
    pub fn reg_dpi_hfp(&self) -> RegDpiHfpR {
        RegDpiHfpR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal Active Width (pixels). Range 176-800 pixels."]
    #[inline(always)]
    pub fn reg_dpi_hact(&mut self) -> RegDpiHactW<'_, DpiHTiming1Spec> {
        RegDpiHactW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Horizontal Front Porch (pixels)."]
    #[inline(always)]
    pub fn reg_dpi_hfp(&mut self) -> RegDpiHfpW<'_, DpiHTiming1Spec> {
        RegDpiHfpW::new(self, 16)
    }
}
#[doc = "Horizontal Active and Front Porch Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_h_timing_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_h_timing_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiHTiming1Spec;
impl crate::RegisterSpec for DpiHTiming1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_h_timing_1::R`](R) reader structure"]
impl crate::Readable for DpiHTiming1Spec {}
#[doc = "`write(|w| ..)` method takes [`dpi_h_timing_1::W`](W) writer structure"]
impl crate::Writable for DpiHTiming1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPI_H_TIMING_1 to value 0"]
impl crate::Resettable for DpiHTiming1Spec {}
