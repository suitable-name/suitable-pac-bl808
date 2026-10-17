#[doc = "Register `DPI_INT_STS` reader"]
pub type R = crate::R<DpiIntStsSpec>;
#[doc = "Register `DPI_INT_STS` writer"]
pub type W = crate::W<DpiIntStsSpec>;
#[doc = "Field `dpi_v_done_int` reader - Vertical Frame Done Interrupt status"]
pub type DpiVDoneIntR = crate::BitReader;
#[doc = "Field `dpi_fifo_err_int` reader - FIFO Overflow/Underflow Interrupt status"]
pub type DpiFifoErrIntR = crate::BitReader;
#[doc = "Field `reg_dpi_v_done_mask` reader - Mask for V_DONE interrupt (1=masked)"]
pub type RegDpiVDoneMaskR = crate::BitReader;
#[doc = "Field `reg_dpi_v_done_mask` writer - Mask for V_DONE interrupt (1=masked)"]
pub type RegDpiVDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dpi_fifo_err_mask` reader - Mask for FIFO_ERR interrupt (1=masked)"]
pub type RegDpiFifoErrMaskR = crate::BitReader;
#[doc = "Field `reg_dpi_fifo_err_mask` writer - Mask for FIFO_ERR interrupt (1=masked)"]
pub type RegDpiFifoErrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dpi_v_done_clr` writer - Clear for V_DONE interrupt"]
pub type RegDpiVDoneClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `reg_dpi_fifo_err_clr` writer - Clear for FIFO_ERR interrupt"]
pub type RegDpiFifoErrClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Vertical Frame Done Interrupt status"]
    #[inline(always)]
    pub fn dpi_v_done_int(&self) -> DpiVDoneIntR {
        DpiVDoneIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Overflow/Underflow Interrupt status"]
    #[inline(always)]
    pub fn dpi_fifo_err_int(&self) -> DpiFifoErrIntR {
        DpiFifoErrIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask for V_DONE interrupt (1=masked)"]
    #[inline(always)]
    pub fn reg_dpi_v_done_mask(&self) -> RegDpiVDoneMaskR {
        RegDpiVDoneMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask for FIFO_ERR interrupt (1=masked)"]
    #[inline(always)]
    pub fn reg_dpi_fifo_err_mask(&self) -> RegDpiFifoErrMaskR {
        RegDpiFifoErrMaskR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Mask for V_DONE interrupt (1=masked)"]
    #[inline(always)]
    pub fn reg_dpi_v_done_mask(&mut self) -> RegDpiVDoneMaskW<'_, DpiIntStsSpec> {
        RegDpiVDoneMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask for FIFO_ERR interrupt (1=masked)"]
    #[inline(always)]
    pub fn reg_dpi_fifo_err_mask(&mut self) -> RegDpiFifoErrMaskW<'_, DpiIntStsSpec> {
        RegDpiFifoErrMaskW::new(self, 9)
    }
    #[doc = "Bit 16 - Clear for V_DONE interrupt"]
    #[inline(always)]
    pub fn reg_dpi_v_done_clr(&mut self) -> RegDpiVDoneClrW<'_, DpiIntStsSpec> {
        RegDpiVDoneClrW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear for FIFO_ERR interrupt"]
    #[inline(always)]
    pub fn reg_dpi_fifo_err_clr(&mut self) -> RegDpiFifoErrClrW<'_, DpiIntStsSpec> {
        RegDpiFifoErrClrW::new(self, 17)
    }
}
#[doc = "DPI Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpi_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiIntStsSpec;
impl crate::RegisterSpec for DpiIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_int_sts::R`](R) reader structure"]
impl crate::Readable for DpiIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi_int_sts::W`](W) writer structure"]
impl crate::Writable for DpiIntStsSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0003_0000;
}
#[doc = "`reset()` method sets DPI_INT_STS to value 0"]
impl crate::Resettable for DpiIntStsSpec {}
