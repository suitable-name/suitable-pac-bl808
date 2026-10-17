#[doc = "Register `FRAME_SIZE_V` reader"]
pub type R = crate::R<FrameSizeVSpec>;
#[doc = "Register `FRAME_SIZE_V` writer"]
pub type W = crate::W<FrameSizeVSpec>;
#[doc = "Field `cr_total_v` reader - Total vertical lines per frame"]
pub type CrTotalVR = crate::FieldReader<u16>;
#[doc = "Field `cr_total_v` writer - Total vertical lines per frame"]
pub type CrTotalVW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `cr_blank_v` reader - Vertical blanking period"]
pub type CrBlankVR = crate::FieldReader<u16>;
#[doc = "Field `cr_blank_v` writer - Vertical blanking period"]
pub type CrBlankVW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Total vertical lines per frame"]
    #[inline(always)]
    pub fn cr_total_v(&self) -> CrTotalVR {
        CrTotalVR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Vertical blanking period"]
    #[inline(always)]
    pub fn cr_blank_v(&self) -> CrBlankVR {
        CrBlankVR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Total vertical lines per frame"]
    #[inline(always)]
    pub fn cr_total_v(&mut self) -> CrTotalVW<'_, FrameSizeVSpec> {
        CrTotalVW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Vertical blanking period"]
    #[inline(always)]
    pub fn cr_blank_v(&mut self) -> CrBlankVW<'_, FrameSizeVSpec> {
        CrBlankVW::new(self, 16)
    }
}
#[doc = "Vertical Frame Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_size_v::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_size_v::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameSizeVSpec;
impl crate::RegisterSpec for FrameSizeVSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_size_v::R`](R) reader structure"]
impl crate::Readable for FrameSizeVSpec {}
#[doc = "`write(|w| ..)` method takes [`frame_size_v::W`](W) writer structure"]
impl crate::Writable for FrameSizeVSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAME_SIZE_V to value 0x002c_0464"]
impl crate::Resettable for FrameSizeVSpec {
    const RESET_VALUE: u32 = 0x002c_0464;
}
