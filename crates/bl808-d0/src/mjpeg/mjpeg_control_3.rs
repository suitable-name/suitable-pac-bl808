#[doc = "Register `MJPEG_CONTROL_3` reader"]
pub type R = crate::R<MjpegControl3Spec>;
#[doc = "Register `MJPEG_CONTROL_3` writer"]
pub type W = crate::W<MjpegControl3Spec>;
#[doc = "Field `reg_int_normal_en` reader - Normal interrupt enable."]
pub type RegIntNormalEnR = crate::BitReader;
#[doc = "Field `reg_int_normal_en` writer - Normal interrupt enable."]
pub type RegIntNormalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_cam_en` reader - Camera/DVP interrupt enable."]
pub type RegIntCamEnR = crate::BitReader;
#[doc = "Field `reg_int_cam_en` writer - Camera/DVP interrupt enable."]
pub type RegIntCamEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_mem_en` reader - Memory overrun interrupt enable."]
pub type RegIntMemEnR = crate::BitReader;
#[doc = "Field `reg_int_mem_en` writer - Memory overrun interrupt enable."]
pub type RegIntMemEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_frame_en` reader - Frame complete interrupt enable."]
pub type RegIntFrameEnR = crate::BitReader;
#[doc = "Field `reg_int_frame_en` writer - Frame complete interrupt enable."]
pub type RegIntFrameEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_normal_int` reader - Normal interrupt status."]
pub type StsNormalIntR = crate::BitReader;
#[doc = "Field `sts_cam_int` reader - Camera/DVP interrupt status."]
pub type StsCamIntR = crate::BitReader;
#[doc = "Field `sts_mem_int` reader - Memory overrun interrupt status."]
pub type StsMemIntR = crate::BitReader;
#[doc = "Field `sts_frame_int` reader - Frame complete interrupt status."]
pub type StsFrameIntR = crate::BitReader;
#[doc = "Field `mjpeg_idle` reader - MJPEG FSM idle status."]
pub type MjpegIdleR = crate::BitReader;
#[doc = "Field `mjpeg_func` reader - MJPEG FSM functional/running status."]
pub type MjpegFuncR = crate::BitReader;
#[doc = "Field `mjpeg_wait` reader - MJPEG FSM waiting status."]
pub type MjpegWaitR = crate::BitReader;
#[doc = "Field `mjpeg_flsh` reader - MJPEG FSM flushing status."]
pub type MjpegFlshR = crate::BitReader;
#[doc = "Field `mjpeg_mans` reader - MJPEG Manual Start status."]
pub type MjpegMansR = crate::BitReader;
#[doc = "Field `mjpeg_manf` reader - MJPEG Manual Finish status."]
pub type MjpegManfR = crate::BitReader;
#[doc = "Field `axi_read_idle` reader - AXI Read channel idle status."]
pub type AxiReadIdleR = crate::BitReader;
#[doc = "Field `axi_write_idle` reader - AXI Write channel idle status."]
pub type AxiWriteIdleR = crate::BitReader;
#[doc = "Field `reg_frame_cnt_trgr_int` reader - Frame count to trigger an interrupt."]
pub type RegFrameCntTrgrIntR = crate::FieldReader;
#[doc = "Field `reg_frame_cnt_trgr_int` writer - Frame count to trigger an interrupt."]
pub type RegFrameCntTrgrIntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `reg_int_idle_en` reader - Idle state interrupt enable."]
pub type RegIntIdleEnR = crate::BitReader;
#[doc = "Field `reg_int_idle_en` writer - Idle state interrupt enable."]
pub type RegIntIdleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_idle_int` reader - Idle state interrupt status."]
pub type StsIdleIntR = crate::BitReader;
#[doc = "Field `frame_valid_cnt` reader - Number of valid frames processed."]
pub type FrameValidCntR = crate::FieldReader;
#[doc = "Field `reg_int_swap_en` reader - Frame swap interrupt enable."]
pub type RegIntSwapEnR = crate::BitReader;
#[doc = "Field `reg_int_swap_en` writer - Frame swap interrupt enable."]
pub type RegIntSwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_swap_int` reader - Frame swap interrupt status."]
pub type StsSwapIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Normal interrupt enable."]
    #[inline(always)]
    pub fn reg_int_normal_en(&self) -> RegIntNormalEnR {
        RegIntNormalEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Camera/DVP interrupt enable."]
    #[inline(always)]
    pub fn reg_int_cam_en(&self) -> RegIntCamEnR {
        RegIntCamEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory overrun interrupt enable."]
    #[inline(always)]
    pub fn reg_int_mem_en(&self) -> RegIntMemEnR {
        RegIntMemEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame complete interrupt enable."]
    #[inline(always)]
    pub fn reg_int_frame_en(&self) -> RegIntFrameEnR {
        RegIntFrameEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Normal interrupt status."]
    #[inline(always)]
    pub fn sts_normal_int(&self) -> StsNormalIntR {
        StsNormalIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Camera/DVP interrupt status."]
    #[inline(always)]
    pub fn sts_cam_int(&self) -> StsCamIntR {
        StsCamIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Memory overrun interrupt status."]
    #[inline(always)]
    pub fn sts_mem_int(&self) -> StsMemIntR {
        StsMemIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame complete interrupt status."]
    #[inline(always)]
    pub fn sts_frame_int(&self) -> StsFrameIntR {
        StsFrameIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MJPEG FSM idle status."]
    #[inline(always)]
    pub fn mjpeg_idle(&self) -> MjpegIdleR {
        MjpegIdleR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MJPEG FSM functional/running status."]
    #[inline(always)]
    pub fn mjpeg_func(&self) -> MjpegFuncR {
        MjpegFuncR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MJPEG FSM waiting status."]
    #[inline(always)]
    pub fn mjpeg_wait(&self) -> MjpegWaitR {
        MjpegWaitR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MJPEG FSM flushing status."]
    #[inline(always)]
    pub fn mjpeg_flsh(&self) -> MjpegFlshR {
        MjpegFlshR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MJPEG Manual Start status."]
    #[inline(always)]
    pub fn mjpeg_mans(&self) -> MjpegMansR {
        MjpegMansR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MJPEG Manual Finish status."]
    #[inline(always)]
    pub fn mjpeg_manf(&self) -> MjpegManfR {
        MjpegManfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AXI Read channel idle status."]
    #[inline(always)]
    pub fn axi_read_idle(&self) -> AxiReadIdleR {
        AxiReadIdleR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AXI Write channel idle status."]
    #[inline(always)]
    pub fn axi_write_idle(&self) -> AxiWriteIdleR {
        AxiWriteIdleR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Frame count to trigger an interrupt."]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&self) -> RegFrameCntTrgrIntR {
        RegFrameCntTrgrIntR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Idle state interrupt enable."]
    #[inline(always)]
    pub fn reg_int_idle_en(&self) -> RegIntIdleEnR {
        RegIntIdleEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Idle state interrupt status."]
    #[inline(always)]
    pub fn sts_idle_int(&self) -> StsIdleIntR {
        StsIdleIntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Number of valid frames processed."]
    #[inline(always)]
    pub fn frame_valid_cnt(&self) -> FrameValidCntR {
        FrameValidCntR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Frame swap interrupt enable."]
    #[inline(always)]
    pub fn reg_int_swap_en(&self) -> RegIntSwapEnR {
        RegIntSwapEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Frame swap interrupt status."]
    #[inline(always)]
    pub fn sts_swap_int(&self) -> StsSwapIntR {
        StsSwapIntR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Normal interrupt enable."]
    #[inline(always)]
    pub fn reg_int_normal_en(&mut self) -> RegIntNormalEnW<'_, MjpegControl3Spec> {
        RegIntNormalEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Camera/DVP interrupt enable."]
    #[inline(always)]
    pub fn reg_int_cam_en(&mut self) -> RegIntCamEnW<'_, MjpegControl3Spec> {
        RegIntCamEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Memory overrun interrupt enable."]
    #[inline(always)]
    pub fn reg_int_mem_en(&mut self) -> RegIntMemEnW<'_, MjpegControl3Spec> {
        RegIntMemEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame complete interrupt enable."]
    #[inline(always)]
    pub fn reg_int_frame_en(&mut self) -> RegIntFrameEnW<'_, MjpegControl3Spec> {
        RegIntFrameEnW::new(self, 3)
    }
    #[doc = "Bits 16:20 - Frame count to trigger an interrupt."]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&mut self) -> RegFrameCntTrgrIntW<'_, MjpegControl3Spec> {
        RegFrameCntTrgrIntW::new(self, 16)
    }
    #[doc = "Bit 21 - Idle state interrupt enable."]
    #[inline(always)]
    pub fn reg_int_idle_en(&mut self) -> RegIntIdleEnW<'_, MjpegControl3Spec> {
        RegIntIdleEnW::new(self, 21)
    }
    #[doc = "Bit 29 - Frame swap interrupt enable."]
    #[inline(always)]
    pub fn reg_int_swap_en(&mut self) -> RegIntSwapEnW<'_, MjpegControl3Spec> {
        RegIntSwapEnW::new(self, 29)
    }
}
#[doc = "MJPEG Interrupt, Status, and Frame Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_control_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_control_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegControl3Spec;
impl crate::RegisterSpec for MjpegControl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_control_3::R`](R) reader structure"]
impl crate::Readable for MjpegControl3Spec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_control_3::W`](W) writer structure"]
impl crate::Writable for MjpegControl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_CONTROL_3 to value 0x0103"]
impl crate::Resettable for MjpegControl3Spec {
    const RESET_VALUE: u32 = 0x0103;
}
