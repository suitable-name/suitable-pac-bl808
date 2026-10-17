#[doc = "Register `BUS_TIMING_0` reader"]
pub type R = crate::R<BusTiming0Spec>;
#[doc = "Register `BUS_TIMING_0` writer"]
pub type W = crate::W<BusTiming0Spec>;
#[doc = "Field `baud_rate_prescaler` reader - "]
pub type BaudRatePrescalerR = crate::FieldReader;
#[doc = "Field `baud_rate_prescaler` writer - "]
pub type BaudRatePrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `sync_jump_width` reader - "]
pub type SyncJumpWidthR = crate::FieldReader;
#[doc = "Field `sync_jump_width` writer - "]
pub type SyncJumpWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn baud_rate_prescaler(&self) -> BaudRatePrescalerR {
        BaudRatePrescalerR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sync_jump_width(&self) -> SyncJumpWidthR {
        SyncJumpWidthR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn baud_rate_prescaler(&mut self) -> BaudRatePrescalerW<'_, BusTiming0Spec> {
        BaudRatePrescalerW::new(self, 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sync_jump_width(&mut self) -> SyncJumpWidthW<'_, BusTiming0Spec> {
        SyncJumpWidthW::new(self, 6)
    }
}
#[doc = "Bus Timing Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_timing_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_timing_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusTiming0Spec;
impl crate::RegisterSpec for BusTiming0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_timing_0::R`](R) reader structure"]
impl crate::Readable for BusTiming0Spec {}
#[doc = "`write(|w| ..)` method takes [`bus_timing_0::W`](W) writer structure"]
impl crate::Writable for BusTiming0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_TIMING_0 to value 0"]
impl crate::Resettable for BusTiming0Spec {}
