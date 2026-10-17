#[doc = "Register `MJPEG_Q_ENC` reader"]
pub type R = crate::R<MjpegQEncSpec>;
#[doc = "Register `MJPEG_Q_ENC` writer"]
pub type W = crate::W<MjpegQEncSpec>;
#[doc = "Field `frame_q_sram_0` reader - Quantization table SRAM status for frame 0."]
pub type FrameQSram0R = crate::BitReader;
#[doc = "Field `frame_q_sram_1` reader - Quantization table SRAM status for frame 1."]
pub type FrameQSram1R = crate::BitReader;
#[doc = "Field `frame_q_sram_2` reader - Quantization table SRAM status for frame 2."]
pub type FrameQSram2R = crate::BitReader;
#[doc = "Field `frame_q_sram_3` reader - Quantization table SRAM status for frame 3."]
pub type FrameQSram3R = crate::BitReader;
#[doc = "Field `reg_q_sram_sw` writer - Software update command for Quantization SRAM."]
pub type RegQSramSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_q_sram_enc` reader - Quantization SRAM busy/encoding status."]
pub type StsQSramEncR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Quantization table SRAM status for frame 0."]
    #[inline(always)]
    pub fn frame_q_sram_0(&self) -> FrameQSram0R {
        FrameQSram0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Quantization table SRAM status for frame 1."]
    #[inline(always)]
    pub fn frame_q_sram_1(&self) -> FrameQSram1R {
        FrameQSram1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Quantization table SRAM status for frame 2."]
    #[inline(always)]
    pub fn frame_q_sram_2(&self) -> FrameQSram2R {
        FrameQSram2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quantization table SRAM status for frame 3."]
    #[inline(always)]
    pub fn frame_q_sram_3(&self) -> FrameQSram3R {
        FrameQSram3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 25 - Quantization SRAM busy/encoding status."]
    #[inline(always)]
    pub fn sts_q_sram_enc(&self) -> StsQSramEncR {
        StsQSramEncR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Software update command for Quantization SRAM."]
    #[inline(always)]
    pub fn reg_q_sram_sw(&mut self) -> RegQSramSwW<'_, MjpegQEncSpec> {
        RegQSramSwW::new(self, 24)
    }
}
#[doc = "Quantization Table Encoding Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_q_enc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_q_enc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegQEncSpec;
impl crate::RegisterSpec for MjpegQEncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_q_enc::R`](R) reader structure"]
impl crate::Readable for MjpegQEncSpec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_q_enc::W`](W) writer structure"]
impl crate::Writable for MjpegQEncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_Q_ENC to value 0"]
impl crate::Resettable for MjpegQEncSpec {}
