#[doc = "Register `SD_FIFO_PARAM` reader"]
pub type R = crate::R<SdFifoParamSpec>;
#[doc = "Register `SD_FIFO_PARAM` writer"]
pub type W = crate::W<SdFifoParamSpec>;
#[doc = "Field `RTC` reader - "]
pub type RtcR = crate::FieldReader;
#[doc = "Field `RTC` writer - "]
pub type RtcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WTC` reader - "]
pub type WtcR = crate::FieldReader;
#[doc = "Field `WTC` writer - "]
pub type WtcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIFO_CLK` reader - "]
pub type FifoClkR = crate::BitReader;
#[doc = "Field `FIFO_CLK` writer - "]
pub type FifoClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_CS` reader - "]
pub type FifoCsR = crate::BitReader;
#[doc = "Field `FIFO_CS` writer - "]
pub type FifoCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDWN` reader - "]
pub type PdwnR = crate::BitReader;
#[doc = "Field `PDWN` writer - "]
pub type PdwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_DAT3` reader - "]
pub type UseDat3R = crate::BitReader;
#[doc = "Field `USE_DAT3` writer - "]
pub type UseDat3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GATE_CTL` reader - "]
pub type ClkGateCtlR = crate::BitReader;
#[doc = "Field `CLK_GATE_CTL` writer - "]
pub type ClkGateCtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GATE_ON` reader - "]
pub type ClkGateOnR = crate::BitReader;
#[doc = "Field `CLK_GATE_ON` writer - "]
pub type ClkGateOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRRD_CLK_OEN` reader - "]
pub type OvrrdClkOenR = crate::BitReader;
#[doc = "Field `OVRRD_CLK_OEN` writer - "]
pub type OvrrdClkOenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CLK_ON` reader - "]
pub type ForceClkOnR = crate::BitReader;
#[doc = "Field `FORCE_CLK_ON` writer - "]
pub type ForceClkOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDFVSSM` reader - "]
pub type PdfvssmR = crate::BitReader;
#[doc = "Field `PDFVSSM` writer - "]
pub type PdfvssmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDLVMC` reader - "]
pub type PdlvmcR = crate::BitReader;
#[doc = "Field `PDLVMC` writer - "]
pub type PdlvmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_GATE_CLK_CNT` reader - "]
pub type PreGateClkCntR = crate::FieldReader;
#[doc = "Field `PRE_GATE_CLK_CNT` writer - "]
pub type PreGateClkCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn wtc(&self) -> WtcR {
        WtcR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fifo_clk(&self) -> FifoClkR {
        FifoClkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fifo_cs(&self) -> FifoCsR {
        FifoCsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pdwn(&self) -> PdwnR {
        PdwnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn use_dat3(&self) -> UseDat3R {
        UseDat3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_gate_ctl(&self) -> ClkGateCtlR {
        ClkGateCtlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_gate_on(&self) -> ClkGateOnR {
        ClkGateOnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ovrrd_clk_oen(&self) -> OvrrdClkOenR {
        OvrrdClkOenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn force_clk_on(&self) -> ForceClkOnR {
        ForceClkOnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pdfvssm(&self) -> PdfvssmR {
        PdfvssmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pdlvmc(&self) -> PdlvmcR {
        PdlvmcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pre_gate_clk_cnt(&self) -> PreGateClkCntR {
        PreGateClkCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, SdFifoParamSpec> {
        RtcW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn wtc(&mut self) -> WtcW<'_, SdFifoParamSpec> {
        WtcW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fifo_clk(&mut self) -> FifoClkW<'_, SdFifoParamSpec> {
        FifoClkW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fifo_cs(&mut self) -> FifoCsW<'_, SdFifoParamSpec> {
        FifoCsW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pdwn(&mut self) -> PdwnW<'_, SdFifoParamSpec> {
        PdwnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn use_dat3(&mut self) -> UseDat3W<'_, SdFifoParamSpec> {
        UseDat3W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clk_gate_ctl(&mut self) -> ClkGateCtlW<'_, SdFifoParamSpec> {
        ClkGateCtlW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clk_gate_on(&mut self) -> ClkGateOnW<'_, SdFifoParamSpec> {
        ClkGateOnW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ovrrd_clk_oen(&mut self) -> OvrrdClkOenW<'_, SdFifoParamSpec> {
        OvrrdClkOenW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn force_clk_on(&mut self) -> ForceClkOnW<'_, SdFifoParamSpec> {
        ForceClkOnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pdfvssm(&mut self) -> PdfvssmW<'_, SdFifoParamSpec> {
        PdfvssmW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pdlvmc(&mut self) -> PdlvmcW<'_, SdFifoParamSpec> {
        PdlvmcW::new(self, 14)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pre_gate_clk_cnt(&mut self) -> PreGateClkCntW<'_, SdFifoParamSpec> {
        PreGateClkCntW::new(self, 16)
    }
}
#[doc = "Vendor specific FIFO control and clock gating parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_fifo_param::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_fifo_param::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdFifoParamSpec;
impl crate::RegisterSpec for SdFifoParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_fifo_param::R`](R) reader structure"]
impl crate::Readable for SdFifoParamSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_fifo_param::W`](W) writer structure"]
impl crate::Writable for SdFifoParamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_FIFO_PARAM to value 0"]
impl crate::Resettable for SdFifoParamSpec {}
