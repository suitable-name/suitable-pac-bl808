#[doc = "Register `MJPEG_CONTROL_2` reader"]
pub type R = crate::R<MjpegControl2Spec>;
#[doc = "Register `MJPEG_CONTROL_2` writer"]
pub type W = crate::W<MjpegControl2Spec>;
#[doc = "Field `reg_sw_frame` reader - Software frame buffer index."]
pub type RegSwFrameR = crate::FieldReader;
#[doc = "Field `reg_sw_frame` writer - Software frame buffer index."]
pub type RegSwFrameW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `reg_sw_kick` writer - Software kick/start signal (pulse once)."]
pub type RegSwKickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sw_kick_mode` reader - Software kick mode enable."]
pub type RegSwKickModeR = crate::BitReader;
#[doc = "Field `reg_sw_kick_mode` writer - Software kick mode enable."]
pub type RegSwKickModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mjpeg_sw_mode` reader - MJPEG software mode enable."]
pub type RegMjpegSwModeR = crate::BitReader;
#[doc = "Field `reg_mjpeg_sw_mode` writer - MJPEG software mode enable."]
pub type RegMjpegSwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mjpeg_sw_run` reader - MJPEG software run/start."]
pub type RegMjpegSwRunR = crate::BitReader;
#[doc = "Field `reg_mjpeg_sw_run` writer - MJPEG software run/start."]
pub type RegMjpegSwRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_yy_dvp2axi_sel` reader - Select DVP2AXI source lane for Y component."]
pub type RegYyDvp2axiSelR = crate::FieldReader;
#[doc = "Field `reg_yy_dvp2axi_sel` writer - Select DVP2AXI source lane for Y component."]
pub type RegYyDvp2axiSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_uv_dvp2axi_sel` reader - Select DVP2AXI source lane for UV component."]
pub type RegUvDvp2axiSelR = crate::FieldReader;
#[doc = "Field `reg_uv_dvp2axi_sel` writer - Select DVP2AXI source lane for UV component."]
pub type RegUvDvp2axiSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_mjpeg_wait_cycle` reader - Wait cycles for FSM synchronization."]
pub type RegMjpegWaitCycleR = crate::FieldReader<u16>;
#[doc = "Field `reg_mjpeg_wait_cycle` writer - Wait cycles for FSM synchronization."]
pub type RegMjpegWaitCycleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - Software frame buffer index."]
    #[inline(always)]
    pub fn reg_sw_frame(&self) -> RegSwFrameR {
        RegSwFrameR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Software kick mode enable."]
    #[inline(always)]
    pub fn reg_sw_kick_mode(&self) -> RegSwKickModeR {
        RegSwKickModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MJPEG software mode enable."]
    #[inline(always)]
    pub fn reg_mjpeg_sw_mode(&self) -> RegMjpegSwModeR {
        RegMjpegSwModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MJPEG software run/start."]
    #[inline(always)]
    pub fn reg_mjpeg_sw_run(&self) -> RegMjpegSwRunR {
        RegMjpegSwRunR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Select DVP2AXI source lane for Y component."]
    #[inline(always)]
    pub fn reg_yy_dvp2axi_sel(&self) -> RegYyDvp2axiSelR {
        RegYyDvp2axiSelR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Select DVP2AXI source lane for UV component."]
    #[inline(always)]
    pub fn reg_uv_dvp2axi_sel(&self) -> RegUvDvp2axiSelR {
        RegUvDvp2axiSelR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Wait cycles for FSM synchronization."]
    #[inline(always)]
    pub fn reg_mjpeg_wait_cycle(&self) -> RegMjpegWaitCycleR {
        RegMjpegWaitCycleR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - Software frame buffer index."]
    #[inline(always)]
    pub fn reg_sw_frame(&mut self) -> RegSwFrameW<'_, MjpegControl2Spec> {
        RegSwFrameW::new(self, 0)
    }
    #[doc = "Bit 6 - Software kick/start signal (pulse once)."]
    #[inline(always)]
    pub fn reg_sw_kick(&mut self) -> RegSwKickW<'_, MjpegControl2Spec> {
        RegSwKickW::new(self, 6)
    }
    #[doc = "Bit 7 - Software kick mode enable."]
    #[inline(always)]
    pub fn reg_sw_kick_mode(&mut self) -> RegSwKickModeW<'_, MjpegControl2Spec> {
        RegSwKickModeW::new(self, 7)
    }
    #[doc = "Bit 8 - MJPEG software mode enable."]
    #[inline(always)]
    pub fn reg_mjpeg_sw_mode(&mut self) -> RegMjpegSwModeW<'_, MjpegControl2Spec> {
        RegMjpegSwModeW::new(self, 8)
    }
    #[doc = "Bit 9 - MJPEG software run/start."]
    #[inline(always)]
    pub fn reg_mjpeg_sw_run(&mut self) -> RegMjpegSwRunW<'_, MjpegControl2Spec> {
        RegMjpegSwRunW::new(self, 9)
    }
    #[doc = "Bits 10:12 - Select DVP2AXI source lane for Y component."]
    #[inline(always)]
    pub fn reg_yy_dvp2axi_sel(&mut self) -> RegYyDvp2axiSelW<'_, MjpegControl2Spec> {
        RegYyDvp2axiSelW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Select DVP2AXI source lane for UV component."]
    #[inline(always)]
    pub fn reg_uv_dvp2axi_sel(&mut self) -> RegUvDvp2axiSelW<'_, MjpegControl2Spec> {
        RegUvDvp2axiSelW::new(self, 13)
    }
    #[doc = "Bits 16:31 - Wait cycles for FSM synchronization."]
    #[inline(always)]
    pub fn reg_mjpeg_wait_cycle(&mut self) -> RegMjpegWaitCycleW<'_, MjpegControl2Spec> {
        RegMjpegWaitCycleW::new(self, 16)
    }
}
#[doc = "MJPEG Control Register 2: SW Mode, Kick, and DVP Select\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_control_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_control_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegControl2Spec;
impl crate::RegisterSpec for MjpegControl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_control_2::R`](R) reader structure"]
impl crate::Readable for MjpegControl2Spec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_control_2::W`](W) writer structure"]
impl crate::Writable for MjpegControl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_CONTROL_2 to value 0x0100_2000"]
impl crate::Resettable for MjpegControl2Spec {
    const RESET_VALUE: u32 = 0x0100_2000;
}
