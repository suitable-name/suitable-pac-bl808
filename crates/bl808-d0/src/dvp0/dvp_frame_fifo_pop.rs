#[doc = "Register `DVP_FRAME_FIFO_POP` writer"]
pub type W = crate::W<DvpFrameFifoPopSpec>;
#[doc = "Field `rfifo_pop` writer - Read FIFO pop"]
pub type RfifoPopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_normal_clr` writer - Clear normal interrupt"]
pub type RegIntNormalClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_mem_clr` writer - Clear memory interrupt"]
pub type RegIntMemClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_frame_clr` writer - Clear frame interrupt"]
pub type RegIntFrameClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_fifo_clr` writer - Clear FIFO interrupt"]
pub type RegIntFifoClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_hcnt_clr` writer - Clear HSYNC count interrupt"]
pub type RegIntHcntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_vcnt_clr` writer - Clear VSYNC count interrupt"]
pub type RegIntVcntClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Read FIFO pop"]
    #[inline(always)]
    pub fn rfifo_pop(&mut self) -> RfifoPopW<'_, DvpFrameFifoPopSpec> {
        RfifoPopW::new(self, 0)
    }
    #[doc = "Bit 4 - Clear normal interrupt"]
    #[inline(always)]
    pub fn reg_int_normal_clr(&mut self) -> RegIntNormalClrW<'_, DvpFrameFifoPopSpec> {
        RegIntNormalClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear memory interrupt"]
    #[inline(always)]
    pub fn reg_int_mem_clr(&mut self) -> RegIntMemClrW<'_, DvpFrameFifoPopSpec> {
        RegIntMemClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear frame interrupt"]
    #[inline(always)]
    pub fn reg_int_frame_clr(&mut self) -> RegIntFrameClrW<'_, DvpFrameFifoPopSpec> {
        RegIntFrameClrW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear FIFO interrupt"]
    #[inline(always)]
    pub fn reg_int_fifo_clr(&mut self) -> RegIntFifoClrW<'_, DvpFrameFifoPopSpec> {
        RegIntFifoClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear HSYNC count interrupt"]
    #[inline(always)]
    pub fn reg_int_hcnt_clr(&mut self) -> RegIntHcntClrW<'_, DvpFrameFifoPopSpec> {
        RegIntHcntClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear VSYNC count interrupt"]
    #[inline(always)]
    pub fn reg_int_vcnt_clr(&mut self) -> RegIntVcntClrW<'_, DvpFrameFifoPopSpec> {
        RegIntVcntClrW::new(self, 9)
    }
}
#[doc = "FIFO Pop and Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp_frame_fifo_pop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvpFrameFifoPopSpec;
impl crate::RegisterSpec for DvpFrameFifoPopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dvp_frame_fifo_pop::W`](W) writer structure"]
impl crate::Writable for DvpFrameFifoPopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP_FRAME_FIFO_POP to value 0"]
impl crate::Resettable for DvpFrameFifoPopSpec {}
