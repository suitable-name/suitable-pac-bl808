#[doc = "Register `MJPEG_YY_FRAME_ADDR` reader"]
pub type R = crate::R<MjpegYyFrameAddrSpec>;
#[doc = "Register `MJPEG_YY_FRAME_ADDR` writer"]
pub type W = crate::W<MjpegYyFrameAddrSpec>;
#[doc = "Field `reg_yy_addr_start` reader - Start address for Y (Luminance) input data."]
pub type RegYyAddrStartR = crate::FieldReader<u32>;
#[doc = "Field `reg_yy_addr_start` writer - Start address for Y (Luminance) input data."]
pub type RegYyAddrStartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address for Y (Luminance) input data."]
    #[inline(always)]
    pub fn reg_yy_addr_start(&self) -> RegYyAddrStartR {
        RegYyAddrStartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address for Y (Luminance) input data."]
    #[inline(always)]
    pub fn reg_yy_addr_start(&mut self) -> RegYyAddrStartW<'_, MjpegYyFrameAddrSpec> {
        RegYyAddrStartW::new(self, 0)
    }
}
#[doc = "Y Component Input Frame Buffer Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_yy_frame_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_yy_frame_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegYyFrameAddrSpec;
impl crate::RegisterSpec for MjpegYyFrameAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_yy_frame_addr::R`](R) reader structure"]
impl crate::Readable for MjpegYyFrameAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_yy_frame_addr::W`](W) writer structure"]
impl crate::Writable for MjpegYyFrameAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_YY_FRAME_ADDR to value 0x8000_0000"]
impl crate::Resettable for MjpegYyFrameAddrSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
