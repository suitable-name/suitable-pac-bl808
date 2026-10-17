#[doc = "Register `MJPEG_YUV_MEM_SW` reader"]
pub type R = crate::R<MjpegYuvMemSwSpec>;
#[doc = "Register `MJPEG_YUV_MEM_SW` writer"]
pub type W = crate::W<MjpegYuvMemSwSpec>;
#[doc = "Field `reg_sw_kick_hblk` reader - Number of horizontal blocks to process when in SW kick mode."]
pub type RegSwKickHblkR = crate::FieldReader<u16>;
#[doc = "Field `reg_sw_kick_hblk` writer - Number of horizontal blocks to process when in SW kick mode."]
pub type RegSwKickHblkW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Number of horizontal blocks to process when in SW kick mode."]
    #[inline(always)]
    pub fn reg_sw_kick_hblk(&self) -> RegSwKickHblkR {
        RegSwKickHblkR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Number of horizontal blocks to process when in SW kick mode."]
    #[inline(always)]
    pub fn reg_sw_kick_hblk(&mut self) -> RegSwKickHblkW<'_, MjpegYuvMemSwSpec> {
        RegSwKickHblkW::new(self, 0)
    }
}
#[doc = "Software Kick Row Count\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_yuv_mem_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_yuv_mem_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegYuvMemSwSpec;
impl crate::RegisterSpec for MjpegYuvMemSwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_yuv_mem_sw::R`](R) reader structure"]
impl crate::Readable for MjpegYuvMemSwSpec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_yuv_mem_sw::W`](W) writer structure"]
impl crate::Writable for MjpegYuvMemSwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_YUV_MEM_SW to value 0x02"]
impl crate::Resettable for MjpegYuvMemSwSpec {
    const RESET_VALUE: u32 = 0x02;
}
