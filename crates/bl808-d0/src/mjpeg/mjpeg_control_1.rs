#[doc = "Register `MJPEG_CONTROL_1` reader"]
pub type R = crate::R<MjpegControl1Spec>;
#[doc = "Register `MJPEG_CONTROL_1` writer"]
pub type W = crate::W<MjpegControl1Spec>;
#[doc = "Field `reg_mjpeg_enable` reader - MJPEG module enable."]
pub type RegMjpegEnableR = crate::BitReader;
#[doc = "Field `reg_mjpeg_enable` writer - MJPEG module enable."]
pub type RegMjpegEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mjpeg_bit_order` reader - Output JPEG bitstream order."]
pub type RegMjpegBitOrderR = crate::BitReader;
#[doc = "Field `reg_mjpeg_bit_order` writer - Output JPEG bitstream order."]
pub type RegMjpegBitOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_order_u_even` reader - UV component order selection for even pixels."]
pub type RegOrderUEvenR = crate::BitReader;
#[doc = "Field `reg_order_u_even` writer - UV component order selection for even pixels."]
pub type RegOrderUEvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_last_hf_wblk_dmy` reader - Dummy bit for last horizontal fetch block."]
pub type RegLastHfWblkDmyR = crate::BitReader;
#[doc = "Field `reg_last_hf_wblk_dmy` writer - Dummy bit for last horizontal fetch block."]
pub type RegLastHfWblkDmyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_last_hf_hblk_dmy` reader - Dummy bit for last vertical fetch block."]
pub type RegLastHfHblkDmyR = crate::BitReader;
#[doc = "Field `reg_last_hf_hblk_dmy` writer - Dummy bit for last vertical fetch block."]
pub type RegLastHfHblkDmyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_reflect_dmy` reader - Dummy bit for reflection logic."]
pub type RegReflectDmyR = crate::BitReader;
#[doc = "Field `reg_reflect_dmy` writer - Dummy bit for reflection logic."]
pub type RegReflectDmyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_read_fwrap` reader - Enable frame wrap for input reading."]
pub type RegReadFwrapR = crate::BitReader;
#[doc = "Field `reg_read_fwrap` writer - Enable frame wrap for input reading."]
pub type RegReadFwrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_w_xlen` reader - AXI write burst length selection."]
pub type RegWXlenR = crate::FieldReader;
#[doc = "Field `reg_w_xlen` writer - AXI write burst length selection."]
pub type RegWXlenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_yuv_mode` reader - YUV format mode selection."]
pub type RegYuvModeR = crate::FieldReader;
#[doc = "Field `reg_yuv_mode` writer - YUV format mode selection."]
pub type RegYuvModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_mjpeg_hw_frame` reader - Hardware frame buffer index (0-5)."]
pub type RegMjpegHwFrameR = crate::FieldReader;
#[doc = "Field `reg_mjpeg_hw_frame` writer - Hardware frame buffer index (0-5)."]
pub type RegMjpegHwFrameW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - MJPEG module enable."]
    #[inline(always)]
    pub fn reg_mjpeg_enable(&self) -> RegMjpegEnableR {
        RegMjpegEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output JPEG bitstream order."]
    #[inline(always)]
    pub fn reg_mjpeg_bit_order(&self) -> RegMjpegBitOrderR {
        RegMjpegBitOrderR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UV component order selection for even pixels."]
    #[inline(always)]
    pub fn reg_order_u_even(&self) -> RegOrderUEvenR {
        RegOrderUEvenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Dummy bit for last horizontal fetch block."]
    #[inline(always)]
    pub fn reg_last_hf_wblk_dmy(&self) -> RegLastHfWblkDmyR {
        RegLastHfWblkDmyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dummy bit for last vertical fetch block."]
    #[inline(always)]
    pub fn reg_last_hf_hblk_dmy(&self) -> RegLastHfHblkDmyR {
        RegLastHfHblkDmyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Dummy bit for reflection logic."]
    #[inline(always)]
    pub fn reg_reflect_dmy(&self) -> RegReflectDmyR {
        RegReflectDmyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable frame wrap for input reading."]
    #[inline(always)]
    pub fn reg_read_fwrap(&self) -> RegReadFwrapR {
        RegReadFwrapR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - AXI write burst length selection."]
    #[inline(always)]
    pub fn reg_w_xlen(&self) -> RegWXlenR {
        RegWXlenR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - YUV format mode selection."]
    #[inline(always)]
    pub fn reg_yuv_mode(&self) -> RegYuvModeR {
        RegYuvModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Hardware frame buffer index (0-5)."]
    #[inline(always)]
    pub fn reg_mjpeg_hw_frame(&self) -> RegMjpegHwFrameR {
        RegMjpegHwFrameR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MJPEG module enable."]
    #[inline(always)]
    pub fn reg_mjpeg_enable(&mut self) -> RegMjpegEnableW<'_, MjpegControl1Spec> {
        RegMjpegEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Output JPEG bitstream order."]
    #[inline(always)]
    pub fn reg_mjpeg_bit_order(&mut self) -> RegMjpegBitOrderW<'_, MjpegControl1Spec> {
        RegMjpegBitOrderW::new(self, 1)
    }
    #[doc = "Bit 2 - UV component order selection for even pixels."]
    #[inline(always)]
    pub fn reg_order_u_even(&mut self) -> RegOrderUEvenW<'_, MjpegControl1Spec> {
        RegOrderUEvenW::new(self, 2)
    }
    #[doc = "Bit 4 - Dummy bit for last horizontal fetch block."]
    #[inline(always)]
    pub fn reg_last_hf_wblk_dmy(&mut self) -> RegLastHfWblkDmyW<'_, MjpegControl1Spec> {
        RegLastHfWblkDmyW::new(self, 4)
    }
    #[doc = "Bit 5 - Dummy bit for last vertical fetch block."]
    #[inline(always)]
    pub fn reg_last_hf_hblk_dmy(&mut self) -> RegLastHfHblkDmyW<'_, MjpegControl1Spec> {
        RegLastHfHblkDmyW::new(self, 5)
    }
    #[doc = "Bit 6 - Dummy bit for reflection logic."]
    #[inline(always)]
    pub fn reg_reflect_dmy(&mut self) -> RegReflectDmyW<'_, MjpegControl1Spec> {
        RegReflectDmyW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable frame wrap for input reading."]
    #[inline(always)]
    pub fn reg_read_fwrap(&mut self) -> RegReadFwrapW<'_, MjpegControl1Spec> {
        RegReadFwrapW::new(self, 7)
    }
    #[doc = "Bits 8:10 - AXI write burst length selection."]
    #[inline(always)]
    pub fn reg_w_xlen(&mut self) -> RegWXlenW<'_, MjpegControl1Spec> {
        RegWXlenW::new(self, 8)
    }
    #[doc = "Bits 12:13 - YUV format mode selection."]
    #[inline(always)]
    pub fn reg_yuv_mode(&mut self) -> RegYuvModeW<'_, MjpegControl1Spec> {
        RegYuvModeW::new(self, 12)
    }
    #[doc = "Bits 24:29 - Hardware frame buffer index (0-5)."]
    #[inline(always)]
    pub fn reg_mjpeg_hw_frame(&mut self) -> RegMjpegHwFrameW<'_, MjpegControl1Spec> {
        RegMjpegHwFrameW::new(self, 24)
    }
}
#[doc = "MJPEG Control Register 1: Enable, Bit Order, and Burst Length\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_control_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_control_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegControl1Spec;
impl crate::RegisterSpec for MjpegControl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_control_1::R`](R) reader structure"]
impl crate::Readable for MjpegControl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_control_1::W`](W) writer structure"]
impl crate::Writable for MjpegControl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_CONTROL_1 to value 0x0386"]
impl crate::Resettable for MjpegControl1Spec {
    const RESET_VALUE: u32 = 0x0386;
}
