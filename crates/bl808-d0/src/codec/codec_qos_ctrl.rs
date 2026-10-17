#[doc = "Register `CODEC_QOS_CTRL` reader"]
pub type R = crate::R<CodecQosCtrlSpec>;
#[doc = "Register `CODEC_QOS_CTRL` writer"]
pub type W = crate::W<CodecQosCtrlSpec>;
#[doc = "Field `REG_JENC_AWQOS` reader - QoS for MJPEG Encoder AXI write"]
pub type RegJencAwqosR = crate::BitReader;
#[doc = "Field `REG_JENC_AWQOS` writer - QoS for MJPEG Encoder AXI write"]
pub type RegJencAwqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_JENC_ARQOS` reader - QoS for MJPEG Encoder AXI read"]
pub type RegJencArqosR = crate::BitReader;
#[doc = "Field `REG_JENC_ARQOS` writer - QoS for MJPEG Encoder AXI read"]
pub type RegJencArqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_JDEC_AWQOS` reader - QoS for MJPEG Decoder AXI write"]
pub type RegJdecAwqosR = crate::BitReader;
#[doc = "Field `REG_JDEC_AWQOS` writer - QoS for MJPEG Decoder AXI write"]
pub type RegJdecAwqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_JDEC_ARQOS` reader - QoS for MJPEG Decoder AXI read"]
pub type RegJdecArqosR = crate::BitReader;
#[doc = "Field `REG_JDEC_ARQOS` writer - QoS for MJPEG Decoder AXI read"]
pub type RegJdecArqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_VDO0_AWQOS` reader - QoS for Video/H264 Port 0 AXI write"]
pub type RegVdo0AwqosR = crate::BitReader;
#[doc = "Field `REG_VDO0_AWQOS` writer - QoS for Video/H264 Port 0 AXI write"]
pub type RegVdo0AwqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_VDO0_ARQOS` reader - QoS for Video/H264 Port 0 AXI read"]
pub type RegVdo0ArqosR = crate::BitReader;
#[doc = "Field `REG_VDO0_ARQOS` writer - QoS for Video/H264 Port 0 AXI read"]
pub type RegVdo0ArqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_VDO1_AWQOS` reader - QoS for Video/H264 Port 1 AXI write"]
pub type RegVdo1AwqosR = crate::BitReader;
#[doc = "Field `REG_VDO1_AWQOS` writer - QoS for Video/H264 Port 1 AXI write"]
pub type RegVdo1AwqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_VDO1_ARQOS` reader - QoS for Video/H264 Port 1 AXI read"]
pub type RegVdo1ArqosR = crate::BitReader;
#[doc = "Field `REG_VDO1_ARQOS` writer - QoS for Video/H264 Port 1 AXI read"]
pub type RegVdo1ArqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REF_AWQOS` reader - QoS for Reference Frame buffer AXI write"]
pub type RegRefAwqosR = crate::BitReader;
#[doc = "Field `REG_REF_AWQOS` writer - QoS for Reference Frame buffer AXI write"]
pub type RegRefAwqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REF_ARQOS` reader - QoS for Reference Frame buffer AXI read"]
pub type RegRefArqosR = crate::BitReader;
#[doc = "Field `REG_REF_ARQOS` writer - QoS for Reference Frame buffer AXI read"]
pub type RegRefArqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_CNN_AWQOS` reader - QoS for NPU/CNN AXI write"]
pub type RegCnnAwqosR = crate::BitReader;
#[doc = "Field `REG_CNN_AWQOS` writer - QoS for NPU/CNN AXI write"]
pub type RegCnnAwqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_CNN_ARQOS` reader - QoS for NPU/CNN AXI read"]
pub type RegCnnArqosR = crate::BitReader;
#[doc = "Field `REG_CNN_ARQOS` writer - QoS for NPU/CNN AXI read"]
pub type RegCnnArqosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - QoS for MJPEG Encoder AXI write"]
    #[inline(always)]
    pub fn reg_jenc_awqos(&self) -> RegJencAwqosR {
        RegJencAwqosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QoS for MJPEG Encoder AXI read"]
    #[inline(always)]
    pub fn reg_jenc_arqos(&self) -> RegJencArqosR {
        RegJencArqosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - QoS for MJPEG Decoder AXI write"]
    #[inline(always)]
    pub fn reg_jdec_awqos(&self) -> RegJdecAwqosR {
        RegJdecAwqosR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - QoS for MJPEG Decoder AXI read"]
    #[inline(always)]
    pub fn reg_jdec_arqos(&self) -> RegJdecArqosR {
        RegJdecArqosR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - QoS for Video/H264 Port 0 AXI write"]
    #[inline(always)]
    pub fn reg_vdo0_awqos(&self) -> RegVdo0AwqosR {
        RegVdo0AwqosR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - QoS for Video/H264 Port 0 AXI read"]
    #[inline(always)]
    pub fn reg_vdo0_arqos(&self) -> RegVdo0ArqosR {
        RegVdo0ArqosR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - QoS for Video/H264 Port 1 AXI write"]
    #[inline(always)]
    pub fn reg_vdo1_awqos(&self) -> RegVdo1AwqosR {
        RegVdo1AwqosR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - QoS for Video/H264 Port 1 AXI read"]
    #[inline(always)]
    pub fn reg_vdo1_arqos(&self) -> RegVdo1ArqosR {
        RegVdo1ArqosR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - QoS for Reference Frame buffer AXI write"]
    #[inline(always)]
    pub fn reg_ref_awqos(&self) -> RegRefAwqosR {
        RegRefAwqosR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - QoS for Reference Frame buffer AXI read"]
    #[inline(always)]
    pub fn reg_ref_arqos(&self) -> RegRefArqosR {
        RegRefArqosR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - QoS for NPU/CNN AXI write"]
    #[inline(always)]
    pub fn reg_cnn_awqos(&self) -> RegCnnAwqosR {
        RegCnnAwqosR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - QoS for NPU/CNN AXI read"]
    #[inline(always)]
    pub fn reg_cnn_arqos(&self) -> RegCnnArqosR {
        RegCnnArqosR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QoS for MJPEG Encoder AXI write"]
    #[inline(always)]
    pub fn reg_jenc_awqos(&mut self) -> RegJencAwqosW<'_, CodecQosCtrlSpec> {
        RegJencAwqosW::new(self, 0)
    }
    #[doc = "Bit 1 - QoS for MJPEG Encoder AXI read"]
    #[inline(always)]
    pub fn reg_jenc_arqos(&mut self) -> RegJencArqosW<'_, CodecQosCtrlSpec> {
        RegJencArqosW::new(self, 1)
    }
    #[doc = "Bit 2 - QoS for MJPEG Decoder AXI write"]
    #[inline(always)]
    pub fn reg_jdec_awqos(&mut self) -> RegJdecAwqosW<'_, CodecQosCtrlSpec> {
        RegJdecAwqosW::new(self, 2)
    }
    #[doc = "Bit 3 - QoS for MJPEG Decoder AXI read"]
    #[inline(always)]
    pub fn reg_jdec_arqos(&mut self) -> RegJdecArqosW<'_, CodecQosCtrlSpec> {
        RegJdecArqosW::new(self, 3)
    }
    #[doc = "Bit 4 - QoS for Video/H264 Port 0 AXI write"]
    #[inline(always)]
    pub fn reg_vdo0_awqos(&mut self) -> RegVdo0AwqosW<'_, CodecQosCtrlSpec> {
        RegVdo0AwqosW::new(self, 4)
    }
    #[doc = "Bit 5 - QoS for Video/H264 Port 0 AXI read"]
    #[inline(always)]
    pub fn reg_vdo0_arqos(&mut self) -> RegVdo0ArqosW<'_, CodecQosCtrlSpec> {
        RegVdo0ArqosW::new(self, 5)
    }
    #[doc = "Bit 6 - QoS for Video/H264 Port 1 AXI write"]
    #[inline(always)]
    pub fn reg_vdo1_awqos(&mut self) -> RegVdo1AwqosW<'_, CodecQosCtrlSpec> {
        RegVdo1AwqosW::new(self, 6)
    }
    #[doc = "Bit 7 - QoS for Video/H264 Port 1 AXI read"]
    #[inline(always)]
    pub fn reg_vdo1_arqos(&mut self) -> RegVdo1ArqosW<'_, CodecQosCtrlSpec> {
        RegVdo1ArqosW::new(self, 7)
    }
    #[doc = "Bit 8 - QoS for Reference Frame buffer AXI write"]
    #[inline(always)]
    pub fn reg_ref_awqos(&mut self) -> RegRefAwqosW<'_, CodecQosCtrlSpec> {
        RegRefAwqosW::new(self, 8)
    }
    #[doc = "Bit 9 - QoS for Reference Frame buffer AXI read"]
    #[inline(always)]
    pub fn reg_ref_arqos(&mut self) -> RegRefArqosW<'_, CodecQosCtrlSpec> {
        RegRefArqosW::new(self, 9)
    }
    #[doc = "Bit 10 - QoS for NPU/CNN AXI write"]
    #[inline(always)]
    pub fn reg_cnn_awqos(&mut self) -> RegCnnAwqosW<'_, CodecQosCtrlSpec> {
        RegCnnAwqosW::new(self, 10)
    }
    #[doc = "Bit 11 - QoS for NPU/CNN AXI read"]
    #[inline(always)]
    pub fn reg_cnn_arqos(&mut self) -> RegCnnArqosW<'_, CodecQosCtrlSpec> {
        RegCnnArqosW::new(self, 11)
    }
}
#[doc = "Codec Quality of Service Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_qos_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_qos_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecQosCtrlSpec;
impl crate::RegisterSpec for CodecQosCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_qos_ctrl::R`](R) reader structure"]
impl crate::Readable for CodecQosCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`codec_qos_ctrl::W`](W) writer structure"]
impl crate::Writable for CodecQosCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CODEC_QOS_CTRL to value 0"]
impl crate::Resettable for CodecQosCtrlSpec {}
