#[doc = "Register `MJPEG_UV_FRAME_ADDR` reader"]
pub type R = crate::R<MjpegUvFrameAddrSpec>;
#[doc = "Register `MJPEG_UV_FRAME_ADDR` writer"]
pub type W = crate::W<MjpegUvFrameAddrSpec>;
#[doc = "Field `reg_uv_addr_start` reader - Start address for UV (Chrominance) input data."]
pub type RegUvAddrStartR = crate::FieldReader<u32>;
#[doc = "Field `reg_uv_addr_start` writer - Start address for UV (Chrominance) input data."]
pub type RegUvAddrStartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address for UV (Chrominance) input data."]
    #[inline(always)]
    pub fn reg_uv_addr_start(&self) -> RegUvAddrStartR {
        RegUvAddrStartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address for UV (Chrominance) input data."]
    #[inline(always)]
    pub fn reg_uv_addr_start(&mut self) -> RegUvAddrStartW<'_, MjpegUvFrameAddrSpec> {
        RegUvAddrStartW::new(self, 0)
    }
}
#[doc = "UV Component Input Frame Buffer Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_uv_frame_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_uv_frame_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegUvFrameAddrSpec;
impl crate::RegisterSpec for MjpegUvFrameAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_uv_frame_addr::R`](R) reader structure"]
impl crate::Readable for MjpegUvFrameAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_uv_frame_addr::W`](W) writer structure"]
impl crate::Writable for MjpegUvFrameAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_UV_FRAME_ADDR to value 0x8000_0000"]
impl crate::Resettable for MjpegUvFrameAddrSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
