#[doc = "Register `JPEG_FRAME_ADDR` reader"]
pub type R = crate::R<JpegFrameAddrSpec>;
#[doc = "Register `JPEG_FRAME_ADDR` writer"]
pub type W = crate::W<JpegFrameAddrSpec>;
#[doc = "Field `reg_w_addr_start` reader - Start address for compressed JPEG output stream."]
pub type RegWAddrStartR = crate::FieldReader<u32>;
#[doc = "Field `reg_w_addr_start` writer - Start address for compressed JPEG output stream."]
pub type RegWAddrStartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address for compressed JPEG output stream."]
    #[inline(always)]
    pub fn reg_w_addr_start(&self) -> RegWAddrStartR {
        RegWAddrStartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address for compressed JPEG output stream."]
    #[inline(always)]
    pub fn reg_w_addr_start(&mut self) -> RegWAddrStartW<'_, JpegFrameAddrSpec> {
        RegWAddrStartW::new(self, 0)
    }
}
#[doc = "Output JPEG Frame Buffer Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`jpeg_frame_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpeg_frame_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JpegFrameAddrSpec;
impl crate::RegisterSpec for JpegFrameAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jpeg_frame_addr::R`](R) reader structure"]
impl crate::Readable for JpegFrameAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`jpeg_frame_addr::W`](W) writer structure"]
impl crate::Writable for JpegFrameAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JPEG_FRAME_ADDR to value 0x8040_0000"]
impl crate::Resettable for JpegFrameAddrSpec {
    const RESET_VALUE: u32 = 0x8040_0000;
}
