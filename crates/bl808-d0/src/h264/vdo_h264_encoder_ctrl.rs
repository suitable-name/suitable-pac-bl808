#[doc = "Register `VDO_H264_ENCODER_CTRL` reader"]
pub type R = crate::R<VdoH264EncoderCtrlSpec>;
#[doc = "Register `VDO_H264_ENCODER_CTRL` writer"]
pub type W = crate::W<VdoH264EncoderCtrlSpec>;
#[doc = "Field `cfg_enc_ctrl_mby_idx` reader - Current MB row index (debug)"]
pub type CfgEncCtrlMbyIdxR = crate::FieldReader;
#[doc = "Field `cfg_core_clk_en` reader - Core clock enable"]
pub type CfgCoreClkEnR = crate::BitReader;
#[doc = "Field `cfg_core_clk_en` writer - Core clock enable"]
pub type CfgCoreClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_zstuf_dis` reader - Disable zero-stuffing"]
pub type CfgZstufDisR = crate::BitReader;
#[doc = "Field `cfg_zstuf_dis` writer - Disable zero-stuffing"]
pub type CfgZstufDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_qr_upd` reader - Main-stream QP update trigger"]
pub type CfgQrUpdR = crate::BitReader;
#[doc = "Field `cfg_qr_upd` writer - Main-stream QP update trigger"]
pub type CfgQrUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_s_qr_upd` reader - Secondary-stream QP update trigger"]
pub type CfgSQrUpdR = crate::BitReader;
#[doc = "Field `cfg_s_qr_upd` writer - Secondary-stream QP update trigger"]
pub type CfgSQrUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_mem_clk_en` reader - Memory clock enable"]
pub type CfgMemClkEnR = crate::BitReader;
#[doc = "Field `cfg_mem_clk_en` writer - Memory clock enable"]
pub type CfgMemClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_rc_gop_upd` reader - Main-stream rate-control GOP update trigger"]
pub type CfgRcGopUpdR = crate::BitReader;
#[doc = "Field `cfg_rc_gop_upd` writer - Main-stream rate-control GOP update trigger"]
pub type CfgRcGopUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_s_rc_gop_upd` reader - Secondary-stream rate-control GOP update trigger"]
pub type CfgSRcGopUpdR = crate::BitReader;
#[doc = "Field `cfg_s_rc_gop_upd` writer - Secondary-stream rate-control GOP update trigger"]
pub type CfgSRcGopUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_duo_bs_en` reader - Dual bitstream (main+secondary) enable"]
pub type CfgDuoBsEnR = crate::BitReader;
#[doc = "Field `cfg_duo_bs_en` writer - Dual bitstream (main+secondary) enable"]
pub type CfgDuoBsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_buf_clr` reader - Main-stream buffer clear"]
pub type CfgBufClrR = crate::BitReader;
#[doc = "Field `cfg_buf_clr` writer - Main-stream buffer clear"]
pub type CfgBufClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_s_buf_clr` reader - Secondary-stream buffer clear"]
pub type CfgSBufClrR = crate::BitReader;
#[doc = "Field `cfg_s_buf_clr` writer - Secondary-stream buffer clear"]
pub type CfgSBufClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_vdonr_en` reader - Main-stream 3D-NR enable"]
pub type CfgVdonrEnR = crate::BitReader;
#[doc = "Field `cfg_vdonr_en` writer - Main-stream 3D-NR enable"]
pub type CfgVdonrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_s_vdonr_en` reader - Secondary-stream 3D-NR enable"]
pub type CfgSVdonrEnR = crate::BitReader;
#[doc = "Field `cfg_s_vdonr_en` writer - Secondary-stream 3D-NR enable"]
pub type CfgSVdonrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_bs_status_init` reader - Main-stream bitstream status init"]
pub type CfgBsStatusInitR = crate::BitReader;
#[doc = "Field `cfg_bs_status_init` writer - Main-stream bitstream status init"]
pub type CfgBsStatusInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_s_bs_status_init` reader - Secondary-stream bitstream status init"]
pub type CfgSBsStatusInitR = crate::BitReader;
#[doc = "Field `cfg_s_bs_status_init` writer - Secondary-stream bitstream status init"]
pub type CfgSBsStatusInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_src_line_clr` reader - Main-stream source line clear"]
pub type CfgSrcLineClrR = crate::BitReader;
#[doc = "Field `cfg_src_line_clr` writer - Main-stream source line clear"]
pub type CfgSrcLineClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_s_src_line_clr` reader - Secondary-stream source line clear"]
pub type CfgSSrcLineClrR = crate::BitReader;
#[doc = "Field `cfg_s_src_line_clr` writer - Secondary-stream source line clear"]
pub type CfgSSrcLineClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_ctrl_mode` reader - Controller mode (0=DVP, 1=SW, 2=auto)"]
pub type CfgCtrlModeR = crate::FieldReader;
#[doc = "Field `cfg_ctrl_mode` writer - Controller mode (0=DVP, 1=SW, 2=auto)"]
pub type CfgCtrlModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cfg_cref_en` reader - Main-stream cross-reference enable"]
pub type CfgCrefEnR = crate::BitReader;
#[doc = "Field `cfg_cref_en` writer - Main-stream cross-reference enable"]
pub type CfgCrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_s_cref_en` reader - Secondary-stream cross-reference enable"]
pub type CfgSCrefEnR = crate::BitReader;
#[doc = "Field `cfg_s_cref_en` writer - Secondary-stream cross-reference enable"]
pub type CfgSCrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_frame_start` reader - Main-stream frame start trigger (SW mode)"]
pub type CfgFrameStartR = crate::BitReader;
#[doc = "Field `cfg_frame_start` writer - Main-stream frame start trigger (SW mode)"]
pub type CfgFrameStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_enc_seq_en` reader - Main-stream encoder sequence enable"]
pub type CfgEncSeqEnR = crate::BitReader;
#[doc = "Field `cfg_enc_seq_en` writer - Main-stream encoder sequence enable"]
pub type CfgEncSeqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_s_frame_start` reader - Secondary-stream frame start trigger (SW mode)"]
pub type CfgSFrameStartR = crate::BitReader;
#[doc = "Field `cfg_s_frame_start` writer - Secondary-stream frame start trigger (SW mode)"]
pub type CfgSFrameStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_s_enc_seq_en` reader - Secondary-stream encoder sequence enable"]
pub type CfgSEncSeqEnR = crate::BitReader;
#[doc = "Field `cfg_s_enc_seq_en` writer - Secondary-stream encoder sequence enable"]
pub type CfgSEncSeqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Current MB row index (debug)"]
    #[inline(always)]
    pub fn cfg_enc_ctrl_mby_idx(&self) -> CfgEncCtrlMbyIdxR {
        CfgEncCtrlMbyIdxR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Core clock enable"]
    #[inline(always)]
    pub fn cfg_core_clk_en(&self) -> CfgCoreClkEnR {
        CfgCoreClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable zero-stuffing"]
    #[inline(always)]
    pub fn cfg_zstuf_dis(&self) -> CfgZstufDisR {
        CfgZstufDisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Main-stream QP update trigger"]
    #[inline(always)]
    pub fn cfg_qr_upd(&self) -> CfgQrUpdR {
        CfgQrUpdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Secondary-stream QP update trigger"]
    #[inline(always)]
    pub fn cfg_s_qr_upd(&self) -> CfgSQrUpdR {
        CfgSQrUpdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Memory clock enable"]
    #[inline(always)]
    pub fn cfg_mem_clk_en(&self) -> CfgMemClkEnR {
        CfgMemClkEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Main-stream rate-control GOP update trigger"]
    #[inline(always)]
    pub fn cfg_rc_gop_upd(&self) -> CfgRcGopUpdR {
        CfgRcGopUpdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Secondary-stream rate-control GOP update trigger"]
    #[inline(always)]
    pub fn cfg_s_rc_gop_upd(&self) -> CfgSRcGopUpdR {
        CfgSRcGopUpdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Dual bitstream (main+secondary) enable"]
    #[inline(always)]
    pub fn cfg_duo_bs_en(&self) -> CfgDuoBsEnR {
        CfgDuoBsEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Main-stream buffer clear"]
    #[inline(always)]
    pub fn cfg_buf_clr(&self) -> CfgBufClrR {
        CfgBufClrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Secondary-stream buffer clear"]
    #[inline(always)]
    pub fn cfg_s_buf_clr(&self) -> CfgSBufClrR {
        CfgSBufClrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Main-stream 3D-NR enable"]
    #[inline(always)]
    pub fn cfg_vdonr_en(&self) -> CfgVdonrEnR {
        CfgVdonrEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Secondary-stream 3D-NR enable"]
    #[inline(always)]
    pub fn cfg_s_vdonr_en(&self) -> CfgSVdonrEnR {
        CfgSVdonrEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Main-stream bitstream status init"]
    #[inline(always)]
    pub fn cfg_bs_status_init(&self) -> CfgBsStatusInitR {
        CfgBsStatusInitR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Secondary-stream bitstream status init"]
    #[inline(always)]
    pub fn cfg_s_bs_status_init(&self) -> CfgSBsStatusInitR {
        CfgSBsStatusInitR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Main-stream source line clear"]
    #[inline(always)]
    pub fn cfg_src_line_clr(&self) -> CfgSrcLineClrR {
        CfgSrcLineClrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Secondary-stream source line clear"]
    #[inline(always)]
    pub fn cfg_s_src_line_clr(&self) -> CfgSSrcLineClrR {
        CfgSSrcLineClrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Controller mode (0=DVP, 1=SW, 2=auto)"]
    #[inline(always)]
    pub fn cfg_ctrl_mode(&self) -> CfgCtrlModeR {
        CfgCtrlModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Main-stream cross-reference enable"]
    #[inline(always)]
    pub fn cfg_cref_en(&self) -> CfgCrefEnR {
        CfgCrefEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Secondary-stream cross-reference enable"]
    #[inline(always)]
    pub fn cfg_s_cref_en(&self) -> CfgSCrefEnR {
        CfgSCrefEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Main-stream frame start trigger (SW mode)"]
    #[inline(always)]
    pub fn cfg_frame_start(&self) -> CfgFrameStartR {
        CfgFrameStartR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Main-stream encoder sequence enable"]
    #[inline(always)]
    pub fn cfg_enc_seq_en(&self) -> CfgEncSeqEnR {
        CfgEncSeqEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Secondary-stream frame start trigger (SW mode)"]
    #[inline(always)]
    pub fn cfg_s_frame_start(&self) -> CfgSFrameStartR {
        CfgSFrameStartR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Secondary-stream encoder sequence enable"]
    #[inline(always)]
    pub fn cfg_s_enc_seq_en(&self) -> CfgSEncSeqEnR {
        CfgSEncSeqEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Core clock enable"]
    #[inline(always)]
    pub fn cfg_core_clk_en(&mut self) -> CfgCoreClkEnW<'_, VdoH264EncoderCtrlSpec> {
        CfgCoreClkEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable zero-stuffing"]
    #[inline(always)]
    pub fn cfg_zstuf_dis(&mut self) -> CfgZstufDisW<'_, VdoH264EncoderCtrlSpec> {
        CfgZstufDisW::new(self, 9)
    }
    #[doc = "Bit 10 - Main-stream QP update trigger"]
    #[inline(always)]
    pub fn cfg_qr_upd(&mut self) -> CfgQrUpdW<'_, VdoH264EncoderCtrlSpec> {
        CfgQrUpdW::new(self, 10)
    }
    #[doc = "Bit 11 - Secondary-stream QP update trigger"]
    #[inline(always)]
    pub fn cfg_s_qr_upd(&mut self) -> CfgSQrUpdW<'_, VdoH264EncoderCtrlSpec> {
        CfgSQrUpdW::new(self, 11)
    }
    #[doc = "Bit 12 - Memory clock enable"]
    #[inline(always)]
    pub fn cfg_mem_clk_en(&mut self) -> CfgMemClkEnW<'_, VdoH264EncoderCtrlSpec> {
        CfgMemClkEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Main-stream rate-control GOP update trigger"]
    #[inline(always)]
    pub fn cfg_rc_gop_upd(&mut self) -> CfgRcGopUpdW<'_, VdoH264EncoderCtrlSpec> {
        CfgRcGopUpdW::new(self, 13)
    }
    #[doc = "Bit 14 - Secondary-stream rate-control GOP update trigger"]
    #[inline(always)]
    pub fn cfg_s_rc_gop_upd(&mut self) -> CfgSRcGopUpdW<'_, VdoH264EncoderCtrlSpec> {
        CfgSRcGopUpdW::new(self, 14)
    }
    #[doc = "Bit 15 - Dual bitstream (main+secondary) enable"]
    #[inline(always)]
    pub fn cfg_duo_bs_en(&mut self) -> CfgDuoBsEnW<'_, VdoH264EncoderCtrlSpec> {
        CfgDuoBsEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Main-stream buffer clear"]
    #[inline(always)]
    pub fn cfg_buf_clr(&mut self) -> CfgBufClrW<'_, VdoH264EncoderCtrlSpec> {
        CfgBufClrW::new(self, 16)
    }
    #[doc = "Bit 17 - Secondary-stream buffer clear"]
    #[inline(always)]
    pub fn cfg_s_buf_clr(&mut self) -> CfgSBufClrW<'_, VdoH264EncoderCtrlSpec> {
        CfgSBufClrW::new(self, 17)
    }
    #[doc = "Bit 18 - Main-stream 3D-NR enable"]
    #[inline(always)]
    pub fn cfg_vdonr_en(&mut self) -> CfgVdonrEnW<'_, VdoH264EncoderCtrlSpec> {
        CfgVdonrEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Secondary-stream 3D-NR enable"]
    #[inline(always)]
    pub fn cfg_s_vdonr_en(&mut self) -> CfgSVdonrEnW<'_, VdoH264EncoderCtrlSpec> {
        CfgSVdonrEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Main-stream bitstream status init"]
    #[inline(always)]
    pub fn cfg_bs_status_init(&mut self) -> CfgBsStatusInitW<'_, VdoH264EncoderCtrlSpec> {
        CfgBsStatusInitW::new(self, 20)
    }
    #[doc = "Bit 21 - Secondary-stream bitstream status init"]
    #[inline(always)]
    pub fn cfg_s_bs_status_init(&mut self) -> CfgSBsStatusInitW<'_, VdoH264EncoderCtrlSpec> {
        CfgSBsStatusInitW::new(self, 21)
    }
    #[doc = "Bit 22 - Main-stream source line clear"]
    #[inline(always)]
    pub fn cfg_src_line_clr(&mut self) -> CfgSrcLineClrW<'_, VdoH264EncoderCtrlSpec> {
        CfgSrcLineClrW::new(self, 22)
    }
    #[doc = "Bit 23 - Secondary-stream source line clear"]
    #[inline(always)]
    pub fn cfg_s_src_line_clr(&mut self) -> CfgSSrcLineClrW<'_, VdoH264EncoderCtrlSpec> {
        CfgSSrcLineClrW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Controller mode (0=DVP, 1=SW, 2=auto)"]
    #[inline(always)]
    pub fn cfg_ctrl_mode(&mut self) -> CfgCtrlModeW<'_, VdoH264EncoderCtrlSpec> {
        CfgCtrlModeW::new(self, 24)
    }
    #[doc = "Bit 26 - Main-stream cross-reference enable"]
    #[inline(always)]
    pub fn cfg_cref_en(&mut self) -> CfgCrefEnW<'_, VdoH264EncoderCtrlSpec> {
        CfgCrefEnW::new(self, 26)
    }
    #[doc = "Bit 27 - Secondary-stream cross-reference enable"]
    #[inline(always)]
    pub fn cfg_s_cref_en(&mut self) -> CfgSCrefEnW<'_, VdoH264EncoderCtrlSpec> {
        CfgSCrefEnW::new(self, 27)
    }
    #[doc = "Bit 28 - Main-stream frame start trigger (SW mode)"]
    #[inline(always)]
    pub fn cfg_frame_start(&mut self) -> CfgFrameStartW<'_, VdoH264EncoderCtrlSpec> {
        CfgFrameStartW::new(self, 28)
    }
    #[doc = "Bit 29 - Main-stream encoder sequence enable"]
    #[inline(always)]
    pub fn cfg_enc_seq_en(&mut self) -> CfgEncSeqEnW<'_, VdoH264EncoderCtrlSpec> {
        CfgEncSeqEnW::new(self, 29)
    }
    #[doc = "Bit 30 - Secondary-stream frame start trigger (SW mode)"]
    #[inline(always)]
    pub fn cfg_s_frame_start(&mut self) -> CfgSFrameStartW<'_, VdoH264EncoderCtrlSpec> {
        CfgSFrameStartW::new(self, 30)
    }
    #[doc = "Bit 31 - Secondary-stream encoder sequence enable"]
    #[inline(always)]
    pub fn cfg_s_enc_seq_en(&mut self) -> CfgSEncSeqEnW<'_, VdoH264EncoderCtrlSpec> {
        CfgSEncSeqEnW::new(self, 31)
    }
}
#[doc = "H264 encoder control register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_encoder_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_encoder_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264EncoderCtrlSpec;
impl crate::RegisterSpec for VdoH264EncoderCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_encoder_ctrl::R`](R) reader structure"]
impl crate::Readable for VdoH264EncoderCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_encoder_ctrl::W`](W) writer structure"]
impl crate::Writable for VdoH264EncoderCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_ENCODER_CTRL to value 0"]
impl crate::Resettable for VdoH264EncoderCtrlSpec {}
