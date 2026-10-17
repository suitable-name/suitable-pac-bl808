#[doc = "Register `SD_CLOCK_AND_BURST_SIZE_SETUP` reader"]
pub type R = crate::R<SdClockAndBurstSizeSetupSpec>;
#[doc = "Register `SD_CLOCK_AND_BURST_SIZE_SETUP` writer"]
pub type W = crate::W<SdClockAndBurstSizeSetupSpec>;
#[doc = "Field `BRST_SIZE` reader - "]
pub type BrstSizeR = crate::FieldReader;
#[doc = "Field `BRST_SIZE` writer - "]
pub type BrstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA_SIZE` reader - "]
pub type DmaSizeR = crate::FieldReader;
#[doc = "Field `DMA_SIZE` writer - "]
pub type DmaSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRIORITY` reader - "]
pub type PriorityR = crate::BitReader;
#[doc = "Field `PRIORITY` writer - "]
pub type PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_NON_POST_WR` reader - "]
pub type AxiNonPostWrR = crate::BitReader;
#[doc = "Field `AXI_NON_POST_WR` writer - "]
pub type AxiNonPostWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_ENDIAN` reader - "]
pub type RdEndianR = crate::BitReader;
#[doc = "Field `RD_ENDIAN` writer - "]
pub type RdEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_ENDIAN` reader - "]
pub type WrEndianR = crate::BitReader;
#[doc = "Field `WR_ENDIAN` writer - "]
pub type WrEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_OSTDG` reader - "]
pub type RdOstdgR = crate::BitReader;
#[doc = "Field `RD_OSTDG` writer - "]
pub type RdOstdgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_OSTDG` reader - "]
pub type WrOstdgR = crate::BitReader;
#[doc = "Field `WR_OSTDG` writer - "]
pub type WrOstdgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn brst_size(&self) -> BrstSizeR {
        BrstSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn dma_size(&self) -> DmaSizeR {
        DmaSizeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn axi_non_post_wr(&self) -> AxiNonPostWrR {
        AxiNonPostWrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_endian(&self) -> RdEndianR {
        RdEndianR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wr_endian(&self) -> WrEndianR {
        WrEndianR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rd_ostdg(&self) -> RdOstdgR {
        RdOstdgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_ostdg(&self) -> WrOstdgR {
        WrOstdgR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn brst_size(&mut self) -> BrstSizeW<'_, SdClockAndBurstSizeSetupSpec> {
        BrstSizeW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn dma_size(&mut self) -> DmaSizeW<'_, SdClockAndBurstSizeSetupSpec> {
        DmaSizeW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn priority(&mut self) -> PriorityW<'_, SdClockAndBurstSizeSetupSpec> {
        PriorityW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn axi_non_post_wr(&mut self) -> AxiNonPostWrW<'_, SdClockAndBurstSizeSetupSpec> {
        AxiNonPostWrW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rd_endian(&mut self) -> RdEndianW<'_, SdClockAndBurstSizeSetupSpec> {
        RdEndianW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wr_endian(&mut self) -> WrEndianW<'_, SdClockAndBurstSizeSetupSpec> {
        WrEndianW::new(self, 7)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rd_ostdg(&mut self) -> RdOstdgW<'_, SdClockAndBurstSizeSetupSpec> {
        RdOstdgW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_ostdg(&mut self) -> WrOstdgW<'_, SdClockAndBurstSizeSetupSpec> {
        WrOstdgW::new(self, 15)
    }
}
#[doc = "Vendor specific burst size and endianness configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_clock_and_burst_size_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_clock_and_burst_size_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdClockAndBurstSizeSetupSpec;
impl crate::RegisterSpec for SdClockAndBurstSizeSetupSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_clock_and_burst_size_setup::R`](R) reader structure"]
impl crate::Readable for SdClockAndBurstSizeSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_clock_and_burst_size_setup::W`](W) writer structure"]
impl crate::Writable for SdClockAndBurstSizeSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_CLOCK_AND_BURST_SIZE_SETUP to value 0"]
impl crate::Resettable for SdClockAndBurstSizeSetupSpec {}
