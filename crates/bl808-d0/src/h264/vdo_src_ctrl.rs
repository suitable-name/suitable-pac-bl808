#[doc = "Register `VDO_SRC_CTRL` reader"]
pub type R = crate::R<VdoSrcCtrlSpec>;
#[doc = "Register `VDO_SRC_CTRL` writer"]
pub type W = crate::W<VdoSrcCtrlSpec>;
#[doc = "Field `frm_buf_num` reader - Number of source frame buffers"]
pub type FrmBufNumR = crate::FieldReader;
#[doc = "Field `frm_buf_num` writer - Number of source frame buffers"]
pub type FrmBufNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `mfrm_buf_en` reader - Multiple frame buffer mode enable"]
pub type MfrmBufEnR = crate::BitReader;
#[doc = "Field `mfrm_buf_en` writer - Multiple frame buffer mode enable"]
pub type MfrmBufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `req_period` reader - Source request period"]
pub type ReqPeriodR = crate::FieldReader<u16>;
#[doc = "Field `req_period` writer - Source request period"]
pub type ReqPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `no_wait_1st_vsync` reader - Do not wait for first VSYNC before starting"]
pub type NoWait1stVsyncR = crate::BitReader;
#[doc = "Field `no_wait_1st_vsync` writer - Do not wait for first VSYNC before starting"]
pub type NoWait1stVsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_frm_buf_num` reader - Secondary stream number of source frame buffers"]
pub type SFrmBufNumR = crate::FieldReader;
#[doc = "Field `s_frm_buf_num` writer - Secondary stream number of source frame buffers"]
pub type SFrmBufNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `s_mfrm_buf_en` reader - Secondary stream multiple frame buffer mode enable"]
pub type SMfrmBufEnR = crate::BitReader;
#[doc = "Field `s_mfrm_buf_en` writer - Secondary stream multiple frame buffer mode enable"]
pub type SMfrmBufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_req_period` reader - Secondary stream source request period"]
pub type SReqPeriodR = crate::FieldReader<u16>;
#[doc = "Field `s_req_period` writer - Secondary stream source request period"]
pub type SReqPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `s_no_wait_1st_vsync` reader - Secondary stream do not wait for first VSYNC"]
pub type SNoWait1stVsyncR = crate::BitReader;
#[doc = "Field `s_no_wait_1st_vsync` writer - Secondary stream do not wait for first VSYNC"]
pub type SNoWait1stVsyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Number of source frame buffers"]
    #[inline(always)]
    pub fn frm_buf_num(&self) -> FrmBufNumR {
        FrmBufNumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Multiple frame buffer mode enable"]
    #[inline(always)]
    pub fn mfrm_buf_en(&self) -> MfrmBufEnR {
        MfrmBufEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:14 - Source request period"]
    #[inline(always)]
    pub fn req_period(&self) -> ReqPeriodR {
        ReqPeriodR::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Do not wait for first VSYNC before starting"]
    #[inline(always)]
    pub fn no_wait_1st_vsync(&self) -> NoWait1stVsyncR {
        NoWait1stVsyncR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Secondary stream number of source frame buffers"]
    #[inline(always)]
    pub fn s_frm_buf_num(&self) -> SFrmBufNumR {
        SFrmBufNumR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Secondary stream multiple frame buffer mode enable"]
    #[inline(always)]
    pub fn s_mfrm_buf_en(&self) -> SMfrmBufEnR {
        SMfrmBufEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:30 - Secondary stream source request period"]
    #[inline(always)]
    pub fn s_req_period(&self) -> SReqPeriodR {
        SReqPeriodR::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Secondary stream do not wait for first VSYNC"]
    #[inline(always)]
    pub fn s_no_wait_1st_vsync(&self) -> SNoWait1stVsyncR {
        SNoWait1stVsyncR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of source frame buffers"]
    #[inline(always)]
    pub fn frm_buf_num(&mut self) -> FrmBufNumW<'_, VdoSrcCtrlSpec> {
        FrmBufNumW::new(self, 0)
    }
    #[doc = "Bit 4 - Multiple frame buffer mode enable"]
    #[inline(always)]
    pub fn mfrm_buf_en(&mut self) -> MfrmBufEnW<'_, VdoSrcCtrlSpec> {
        MfrmBufEnW::new(self, 4)
    }
    #[doc = "Bits 5:14 - Source request period"]
    #[inline(always)]
    pub fn req_period(&mut self) -> ReqPeriodW<'_, VdoSrcCtrlSpec> {
        ReqPeriodW::new(self, 5)
    }
    #[doc = "Bit 15 - Do not wait for first VSYNC before starting"]
    #[inline(always)]
    pub fn no_wait_1st_vsync(&mut self) -> NoWait1stVsyncW<'_, VdoSrcCtrlSpec> {
        NoWait1stVsyncW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Secondary stream number of source frame buffers"]
    #[inline(always)]
    pub fn s_frm_buf_num(&mut self) -> SFrmBufNumW<'_, VdoSrcCtrlSpec> {
        SFrmBufNumW::new(self, 16)
    }
    #[doc = "Bit 20 - Secondary stream multiple frame buffer mode enable"]
    #[inline(always)]
    pub fn s_mfrm_buf_en(&mut self) -> SMfrmBufEnW<'_, VdoSrcCtrlSpec> {
        SMfrmBufEnW::new(self, 20)
    }
    #[doc = "Bits 21:30 - Secondary stream source request period"]
    #[inline(always)]
    pub fn s_req_period(&mut self) -> SReqPeriodW<'_, VdoSrcCtrlSpec> {
        SReqPeriodW::new(self, 21)
    }
    #[doc = "Bit 31 - Secondary stream do not wait for first VSYNC"]
    #[inline(always)]
    pub fn s_no_wait_1st_vsync(&mut self) -> SNoWait1stVsyncW<'_, VdoSrcCtrlSpec> {
        SNoWait1stVsyncW::new(self, 31)
    }
}
#[doc = "Video source buffer and timing control\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_src_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_src_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoSrcCtrlSpec;
impl crate::RegisterSpec for VdoSrcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_src_ctrl::R`](R) reader structure"]
impl crate::Readable for VdoSrcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_src_ctrl::W`](W) writer structure"]
impl crate::Writable for VdoSrcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_SRC_CTRL to value 0"]
impl crate::Resettable for VdoSrcCtrlSpec {}
