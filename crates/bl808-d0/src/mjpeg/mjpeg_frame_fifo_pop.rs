#[doc = "Register `MJPEG_FRAME_FIFO_POP` writer"]
pub type W = crate::W<MjpegFrameFifoPopSpec>;
#[doc = "Field `rfifo_pop` writer - Read FIFO pop command (pulsed)."]
pub type RfifoPopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_w_swap_clr` writer - Write swap index clear (pulsed)."]
pub type RegWSwapClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_normal_clr` writer - Clear normal interrupt."]
pub type RegIntNormalClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_cam_clr` writer - Clear Camera/DVP interrupt."]
pub type RegIntCamClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_mem_clr` writer - Clear memory overrun interrupt."]
pub type RegIntMemClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_frame_clr` writer - Clear frame complete interrupt."]
pub type RegIntFrameClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_idle_clr` writer - Clear idle state interrupt."]
pub type RegIntIdleClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_swap_clr` writer - Clear frame swap interrupt."]
pub type RegIntSwapClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Read FIFO pop command (pulsed)."]
    #[inline(always)]
    pub fn rfifo_pop(&mut self) -> RfifoPopW<'_, MjpegFrameFifoPopSpec> {
        RfifoPopW::new(self, 0)
    }
    #[doc = "Bit 1 - Write swap index clear (pulsed)."]
    #[inline(always)]
    pub fn reg_w_swap_clr(&mut self) -> RegWSwapClrW<'_, MjpegFrameFifoPopSpec> {
        RegWSwapClrW::new(self, 1)
    }
    #[doc = "Bit 8 - Clear normal interrupt."]
    #[inline(always)]
    pub fn reg_int_normal_clr(&mut self) -> RegIntNormalClrW<'_, MjpegFrameFifoPopSpec> {
        RegIntNormalClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear Camera/DVP interrupt."]
    #[inline(always)]
    pub fn reg_int_cam_clr(&mut self) -> RegIntCamClrW<'_, MjpegFrameFifoPopSpec> {
        RegIntCamClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear memory overrun interrupt."]
    #[inline(always)]
    pub fn reg_int_mem_clr(&mut self) -> RegIntMemClrW<'_, MjpegFrameFifoPopSpec> {
        RegIntMemClrW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear frame complete interrupt."]
    #[inline(always)]
    pub fn reg_int_frame_clr(&mut self) -> RegIntFrameClrW<'_, MjpegFrameFifoPopSpec> {
        RegIntFrameClrW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear idle state interrupt."]
    #[inline(always)]
    pub fn reg_int_idle_clr(&mut self) -> RegIntIdleClrW<'_, MjpegFrameFifoPopSpec> {
        RegIntIdleClrW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear frame swap interrupt."]
    #[inline(always)]
    pub fn reg_int_swap_clr(&mut self) -> RegIntSwapClrW<'_, MjpegFrameFifoPopSpec> {
        RegIntSwapClrW::new(self, 13)
    }
}
#[doc = "Interrupt Clear and FIFO Pop Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_frame_fifo_pop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegFrameFifoPopSpec;
impl crate::RegisterSpec for MjpegFrameFifoPopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mjpeg_frame_fifo_pop::W`](W) writer structure"]
impl crate::Writable for MjpegFrameFifoPopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_FRAME_FIFO_POP to value 0"]
impl crate::Resettable for MjpegFrameFifoPopSpec {}
