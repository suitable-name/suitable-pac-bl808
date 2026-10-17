#[doc = "Register `padc_status` reader"]
pub type R = crate::R<PadcStatusSpec>;
#[doc = "Register `padc_status` writer"]
pub type W = crate::W<PadcStatusSpec>;
#[doc = "Field `padc_soft_rst` reader - "]
pub type PadcSoftRstR = crate::BitReader;
#[doc = "Field `padc_soft_rst` writer - "]
pub type PadcSoftRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_filter_rdy` reader - "]
pub type PadcFilterRdyR = crate::BitReader;
#[doc = "Field `padc_calib_busy` reader - "]
pub type PadcCalibBusyR = crate::BitReader;
#[doc = "Field `padc_int` reader - ADC Interrupt Status"]
pub type PadcIntR = crate::BitReader;
#[doc = "Field `padc_int_clr` writer - Clear ADC Interrupt"]
pub type PadcIntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn padc_soft_rst(&self) -> PadcSoftRstR {
        PadcSoftRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn padc_filter_rdy(&self) -> PadcFilterRdyR {
        PadcFilterRdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn padc_calib_busy(&self) -> PadcCalibBusyR {
        PadcCalibBusyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC Interrupt Status"]
    #[inline(always)]
    pub fn padc_int(&self) -> PadcIntR {
        PadcIntR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn padc_soft_rst(&mut self) -> PadcSoftRstW<'_, PadcStatusSpec> {
        PadcSoftRstW::new(self, 2)
    }
    #[doc = "Bit 29 - Clear ADC Interrupt"]
    #[inline(always)]
    pub fn padc_int_clr(&mut self) -> PadcIntClrW<'_, PadcStatusSpec> {
        PadcIntClrW::new(self, 29)
    }
}
#[doc = "Passthrough ADC Status: Calibration and interrupt flags\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcStatusSpec;
impl crate::RegisterSpec for PadcStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_status::R`](R) reader structure"]
impl crate::Readable for PadcStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`padc_status::W`](W) writer structure"]
impl crate::Writable for PadcStatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x2000_0000;
}
#[doc = "`reset()` method sets padc_status to value 0"]
impl crate::Resettable for PadcStatusSpec {}
