#[doc = "Register `BUS_TIMING_1` reader"]
pub type R = crate::R<BusTiming1Spec>;
#[doc = "Register `BUS_TIMING_1` writer"]
pub type W = crate::W<BusTiming1Spec>;
#[doc = "Field `time_segment_1` reader - "]
pub type TimeSegment1R = crate::FieldReader;
#[doc = "Field `time_segment_1` writer - "]
pub type TimeSegment1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `time_segment_2` reader - "]
pub type TimeSegment2R = crate::FieldReader;
#[doc = "Field `time_segment_2` writer - "]
pub type TimeSegment2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sampling` reader - "]
pub type SamplingR = crate::BitReader;
#[doc = "Field `sampling` writer - "]
pub type SamplingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn time_segment_1(&self) -> TimeSegment1R {
        TimeSegment1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn time_segment_2(&self) -> TimeSegment2R {
        TimeSegment2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sampling(&self) -> SamplingR {
        SamplingR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn time_segment_1(&mut self) -> TimeSegment1W<'_, BusTiming1Spec> {
        TimeSegment1W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn time_segment_2(&mut self) -> TimeSegment2W<'_, BusTiming1Spec> {
        TimeSegment2W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sampling(&mut self) -> SamplingW<'_, BusTiming1Spec> {
        SamplingW::new(self, 7)
    }
}
#[doc = "Bus Timing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_timing_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_timing_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusTiming1Spec;
impl crate::RegisterSpec for BusTiming1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_timing_1::R`](R) reader structure"]
impl crate::Readable for BusTiming1Spec {}
#[doc = "`write(|w| ..)` method takes [`bus_timing_1::W`](W) writer structure"]
impl crate::Writable for BusTiming1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_TIMING_1 to value 0"]
impl crate::Resettable for BusTiming1Spec {}
