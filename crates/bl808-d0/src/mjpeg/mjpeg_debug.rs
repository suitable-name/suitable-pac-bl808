#[doc = "Register `MJPEG_DEBUG` reader"]
pub type R = crate::R<MjpegDebugSpec>;
#[doc = "Register `MJPEG_DEBUG` writer"]
pub type W = crate::W<MjpegDebugSpec>;
#[doc = "Field `reg_mjpeg_dbg_en` reader - "]
pub type RegMjpegDbgEnR = crate::BitReader;
#[doc = "Field `reg_mjpeg_dbg_en` writer - "]
pub type RegMjpegDbgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mjpeg_dbg_sel` reader - "]
pub type RegMjpegDbgSelR = crate::FieldReader;
#[doc = "Field `reg_mjpeg_dbg_sel` writer - "]
pub type RegMjpegDbgSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_id_latch_hblk` reader - "]
pub type RegIdLatchHblkR = crate::FieldReader;
#[doc = "Field `reg_id_latch_hblk` writer - "]
pub type RegIdLatchHblkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mjpeg_dbg_en(&self) -> RegMjpegDbgEnR {
        RegMjpegDbgEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_mjpeg_dbg_sel(&self) -> RegMjpegDbgSelR {
        RegMjpegDbgSelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_id_latch_hblk(&self) -> RegIdLatchHblkR {
        RegIdLatchHblkR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mjpeg_dbg_en(&mut self) -> RegMjpegDbgEnW<'_, MjpegDebugSpec> {
        RegMjpegDbgEnW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_mjpeg_dbg_sel(&mut self) -> RegMjpegDbgSelW<'_, MjpegDebugSpec> {
        RegMjpegDbgSelW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_id_latch_hblk(&mut self) -> RegIdLatchHblkW<'_, MjpegDebugSpec> {
        RegIdLatchHblkW::new(self, 8)
    }
}
#[doc = "MJPEG Debug Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mjpeg_debug::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mjpeg_debug::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MjpegDebugSpec;
impl crate::RegisterSpec for MjpegDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mjpeg_debug::R`](R) reader structure"]
impl crate::Readable for MjpegDebugSpec {}
#[doc = "`write(|w| ..)` method takes [`mjpeg_debug::W`](W) writer structure"]
impl crate::Writable for MjpegDebugSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MJPEG_DEBUG to value 0x0100"]
impl crate::Resettable for MjpegDebugSpec {
    const RESET_VALUE: u32 = 0x0100;
}
