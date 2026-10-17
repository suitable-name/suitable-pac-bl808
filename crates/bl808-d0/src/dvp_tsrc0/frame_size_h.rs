#[doc = "Register `FRAME_SIZE_H` reader"]
pub type R = crate::R<FrameSizeHSpec>;
#[doc = "Register `FRAME_SIZE_H` writer"]
pub type W = crate::W<FrameSizeHSpec>;
#[doc = "Field `cr_total_h` reader - Total horizontal pixels/cycles per line"]
pub type CrTotalHR = crate::FieldReader<u16>;
#[doc = "Field `cr_total_h` writer - Total horizontal pixels/cycles per line"]
pub type CrTotalHW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `cr_blank_h` reader - Horizontal blanking period"]
pub type CrBlankHR = crate::FieldReader<u16>;
#[doc = "Field `cr_blank_h` writer - Horizontal blanking period"]
pub type CrBlankHW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Total horizontal pixels/cycles per line"]
    #[inline(always)]
    pub fn cr_total_h(&self) -> CrTotalHR {
        CrTotalHR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Horizontal blanking period"]
    #[inline(always)]
    pub fn cr_blank_h(&self) -> CrBlankHR {
        CrBlankHR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Total horizontal pixels/cycles per line"]
    #[inline(always)]
    pub fn cr_total_h(&mut self) -> CrTotalHW<'_, FrameSizeHSpec> {
        CrTotalHW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Horizontal blanking period"]
    #[inline(always)]
    pub fn cr_blank_h(&mut self) -> CrBlankHW<'_, FrameSizeHSpec> {
        CrBlankHW::new(self, 16)
    }
}
#[doc = "Horizontal Frame Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_size_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_size_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameSizeHSpec;
impl crate::RegisterSpec for FrameSizeHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_size_h::R`](R) reader structure"]
impl crate::Readable for FrameSizeHSpec {}
#[doc = "`write(|w| ..)` method takes [`frame_size_h::W`](W) writer structure"]
impl crate::Writable for FrameSizeHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAME_SIZE_H to value 0x1170_0897"]
impl crate::Resettable for FrameSizeHSpec {
    const RESET_VALUE: u32 = 0x1170_0897;
}
