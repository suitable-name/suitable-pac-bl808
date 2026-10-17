#[doc = "Register `SW_RESET_CODEC_SUB` reader"]
pub type R = crate::R<SwResetCodecSubSpec>;
#[doc = "Register `SW_RESET_CODEC_SUB` writer"]
pub type W = crate::W<SwResetCodecSubSpec>;
#[doc = "Field `swrst_codec_misc` reader - "]
pub type SwrstCodecMiscR = crate::BitReader;
#[doc = "Field `swrst_codec_misc` writer - "]
pub type SwrstCodecMiscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_mjpeg` reader - "]
pub type SwrstMjpegR = crate::BitReader;
#[doc = "Field `swrst_mjpeg` writer - "]
pub type SwrstMjpegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_h264` reader - "]
pub type SwrstH264R = crate::BitReader;
#[doc = "Field `swrst_h264` writer - "]
pub type SwrstH264W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_mjpeg_dec` reader - "]
pub type SwrstMjpegDecR = crate::BitReader;
#[doc = "Field `swrst_mjpeg_dec` writer - "]
pub type SwrstMjpegDecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_cnn` reader - "]
pub type SwrstCnnR = crate::BitReader;
#[doc = "Field `swrst_cnn` writer - "]
pub type SwrstCnnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swrst_vram` reader - "]
pub type SwrstVramR = crate::BitReader;
#[doc = "Field `swrst_vram` writer - "]
pub type SwrstVramW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_codec_misc(&self) -> SwrstCodecMiscR {
        SwrstCodecMiscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_mjpeg(&self) -> SwrstMjpegR {
        SwrstMjpegR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_h264(&self) -> SwrstH264R {
        SwrstH264R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_mjpeg_dec(&self) -> SwrstMjpegDecR {
        SwrstMjpegDecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_cnn(&self) -> SwrstCnnR {
        SwrstCnnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_vram(&self) -> SwrstVramR {
        SwrstVramR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_codec_misc(&mut self) -> SwrstCodecMiscW<'_, SwResetCodecSubSpec> {
        SwrstCodecMiscW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_mjpeg(&mut self) -> SwrstMjpegW<'_, SwResetCodecSubSpec> {
        SwrstMjpegW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_h264(&mut self) -> SwrstH264W<'_, SwResetCodecSubSpec> {
        SwrstH264W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_mjpeg_dec(&mut self) -> SwrstMjpegDecW<'_, SwResetCodecSubSpec> {
        SwrstMjpegDecW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_cnn(&mut self) -> SwrstCnnW<'_, SwResetCodecSubSpec> {
        SwrstCnnW::new(self, 4)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_vram(&mut self) -> SwrstVramW<'_, SwResetCodecSubSpec> {
        SwrstVramW::new(self, 16)
    }
}
#[doc = "Codec Subsystem Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_reset_codec_sub::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_reset_codec_sub::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwResetCodecSubSpec;
impl crate::RegisterSpec for SwResetCodecSubSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_reset_codec_sub::R`](R) reader structure"]
impl crate::Readable for SwResetCodecSubSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_reset_codec_sub::W`](W) writer structure"]
impl crate::Writable for SwResetCodecSubSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SW_RESET_CODEC_SUB to value 0"]
impl crate::Resettable for SwResetCodecSubSpec {}
