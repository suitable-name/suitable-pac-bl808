#[doc = "Register `FRAME_SIZE_CEA_861` reader"]
pub type R = crate::R<FrameSizeCea861Spec>;
#[doc = "Register `FRAME_SIZE_CEA_861` writer"]
pub type W = crate::W<FrameSizeCea861Spec>;
#[doc = "Field `cr_h_duration` reader - Horizontal sync pulse duration (cycles)"]
pub type CrHDurationR = crate::FieldReader;
#[doc = "Field `cr_h_duration` writer - Horizontal sync pulse duration (cycles)"]
pub type CrHDurationW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_h_placement` reader - Horizontal active area start position"]
pub type CrHPlacementR = crate::FieldReader;
#[doc = "Field `cr_h_placement` writer - Horizontal active area start position"]
pub type CrHPlacementW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_v_duration` reader - Vertical sync pulse duration (lines)"]
pub type CrVDurationR = crate::FieldReader;
#[doc = "Field `cr_v_duration` writer - Vertical sync pulse duration (lines)"]
pub type CrVDurationW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_v_placement` reader - Vertical active area start position"]
pub type CrVPlacementR = crate::FieldReader;
#[doc = "Field `cr_v_placement` writer - Vertical active area start position"]
pub type CrVPlacementW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Horizontal sync pulse duration (cycles)"]
    #[inline(always)]
    pub fn cr_h_duration(&self) -> CrHDurationR {
        CrHDurationR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal active area start position"]
    #[inline(always)]
    pub fn cr_h_placement(&self) -> CrHPlacementR {
        CrHPlacementR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Vertical sync pulse duration (lines)"]
    #[inline(always)]
    pub fn cr_v_duration(&self) -> CrVDurationR {
        CrVDurationR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vertical active area start position"]
    #[inline(always)]
    pub fn cr_v_placement(&self) -> CrVPlacementR {
        CrVPlacementR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Horizontal sync pulse duration (cycles)"]
    #[inline(always)]
    pub fn cr_h_duration(&mut self) -> CrHDurationW<'_, FrameSizeCea861Spec> {
        CrHDurationW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Horizontal active area start position"]
    #[inline(always)]
    pub fn cr_h_placement(&mut self) -> CrHPlacementW<'_, FrameSizeCea861Spec> {
        CrHPlacementW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Vertical sync pulse duration (lines)"]
    #[inline(always)]
    pub fn cr_v_duration(&mut self) -> CrVDurationW<'_, FrameSizeCea861Spec> {
        CrVDurationW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Vertical active area start position"]
    #[inline(always)]
    pub fn cr_v_placement(&mut self) -> CrVPlacementW<'_, FrameSizeCea861Spec> {
        CrVPlacementW::new(self, 24)
    }
}
#[doc = "CEA-861 Timing Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_size_cea_861::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_size_cea_861::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameSizeCea861Spec;
impl crate::RegisterSpec for FrameSizeCea861Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_size_cea_861::R`](R) reader structure"]
impl crate::Readable for FrameSizeCea861Spec {}
#[doc = "`write(|w| ..)` method takes [`frame_size_cea_861::W`](W) writer structure"]
impl crate::Writable for FrameSizeCea861Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAME_SIZE_CEA_861 to value 0x0308_5783"]
impl crate::Resettable for FrameSizeCea861Spec {
    const RESET_VALUE: u32 = 0x0308_5783;
}
