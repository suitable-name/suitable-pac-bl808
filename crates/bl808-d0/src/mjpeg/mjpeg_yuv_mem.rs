#[doc = "Register `MJPEG_YUV_MEM` reader"]
pub type R = crate::R<MjpegYuvMemSpec>;
#[doc = "Register `MJPEG_YUV_MEM` writer"]
pub type W = crate::W<MjpegYuvMemSpec>;
#[doc = "Field `reg_yy_mem_hblk` reader - Horizontal block count for Y memory."]
pub type RegYyMemHblkR = crate::FieldReader<u16>;
#[doc = "Field `reg_yy_mem_hblk` writer - Horizontal block count for Y memory."]
pub type RegYyMemHblkW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `reg_uv_mem_hblk` reader - Horizontal block count for UV memory."]
pub type RegUvMemHblkR = crate::FieldReader<u16>;
#[doc = "Field `reg_uv_mem_hblk` writer - Horizontal block count for UV memory."]
pub type RegUvMemHblkW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Horizontal block count for Y memory."]
    #[inline(always)]
    pub fn reg_yy_mem_hblk(&self) -> RegYyMemHblkR {
        RegYyMemHblkR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Horizontal block count for UV memory."]
    #[inline(always)]
    pub fn reg_uv_mem_hblk(&self) -> RegUvMemHblkR {
        RegUvMemHblkR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal block count for Y memory."]
    #[inline(always)]
    pub fn reg_yy_mem_hblk(&mut self) -> RegYyMemHblkW<'_, MjpegYuvMemSpec> {
        RegYyMemHblkW::new(self, 0)
    }
    #[doc = "Bits 16:28 - Horizontal block count for UV memory."]
    #[inline(always)]
    pub fn reg_uv_mem_hblk(&mut self) -> RegUvMemHblkW<'_, MjpegYuvMemSpec> {
        RegUvMemHblkW::new(self, 16)
    }
}
#[doc = "YUV Memory Block Count Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_yuv_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_yuv_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegYuvMemSpec;
impl crate::RegisterSpec for MjpegYuvMemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_yuv_mem::R`](R) reader structure"]
impl crate::Readable for MjpegYuvMemSpec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_yuv_mem::W`](W) writer structure"]
impl crate::Writable for MjpegYuvMemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_YUV_MEM to value 0x0002_0002"]
impl crate::Resettable for MjpegYuvMemSpec {
    const RESET_VALUE: u32 = 0x0002_0002;
}
