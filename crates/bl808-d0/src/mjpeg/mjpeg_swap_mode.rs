#[doc = "Register `MJPEG_SWAP_MODE` reader"]
pub type R = crate::R<MjpegSwapModeSpec>;
#[doc = "Register `MJPEG_SWAP_MODE` writer"]
pub type W = crate::W<MjpegSwapModeSpec>;
#[doc = "Field `reg_w_swap_mode` reader - Enable hardware output buffer swap mode."]
pub type RegWSwapModeR = crate::BitReader;
#[doc = "Field `reg_w_swap_mode` writer - Enable hardware output buffer swap mode."]
pub type RegWSwapModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_swap0_full` reader - Swap Buffer 0 full status."]
pub type StsSwap0FullR = crate::BitReader;
#[doc = "Field `sts_swap1_full` reader - Swap Buffer 1 full status."]
pub type StsSwap1FullR = crate::BitReader;
#[doc = "Field `sts_read_swap_idx` reader - Index of the buffer currently being read."]
pub type StsReadSwapIdxR = crate::BitReader;
#[doc = "Field `sts_swap_fstart` reader - Frame swap start status."]
pub type StsSwapFstartR = crate::BitReader;
#[doc = "Field `sts_swap_fend` reader - Frame swap end status."]
pub type StsSwapFendR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable hardware output buffer swap mode."]
    #[inline(always)]
    pub fn reg_w_swap_mode(&self) -> RegWSwapModeR {
        RegWSwapModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Swap Buffer 0 full status."]
    #[inline(always)]
    pub fn sts_swap0_full(&self) -> StsSwap0FullR {
        StsSwap0FullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Swap Buffer 1 full status."]
    #[inline(always)]
    pub fn sts_swap1_full(&self) -> StsSwap1FullR {
        StsSwap1FullR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Index of the buffer currently being read."]
    #[inline(always)]
    pub fn sts_read_swap_idx(&self) -> StsReadSwapIdxR {
        StsReadSwapIdxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Frame swap start status."]
    #[inline(always)]
    pub fn sts_swap_fstart(&self) -> StsSwapFstartR {
        StsSwapFstartR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Frame swap end status."]
    #[inline(always)]
    pub fn sts_swap_fend(&self) -> StsSwapFendR {
        StsSwapFendR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable hardware output buffer swap mode."]
    #[inline(always)]
    pub fn reg_w_swap_mode(&mut self) -> RegWSwapModeW<'_, MjpegSwapModeSpec> {
        RegWSwapModeW::new(self, 0)
    }
}
#[doc = "Buffer Swap Configuration and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_swap_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_swap_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegSwapModeSpec;
impl crate::RegisterSpec for MjpegSwapModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_swap_mode::R`](R) reader structure"]
impl crate::Readable for MjpegSwapModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_swap_mode::W`](W) writer structure"]
impl crate::Writable for MjpegSwapModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_SWAP_MODE to value 0"]
impl crate::Resettable for MjpegSwapModeSpec {}
