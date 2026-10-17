#[doc = "Register `DVP2AXI_FRAME_PERIOD` reader"]
pub type R = crate::R<Dvp2axiFramePeriodSpec>;
#[doc = "Register `DVP2AXI_FRAME_PERIOD` writer"]
pub type W = crate::W<Dvp2axiFramePeriodSpec>;
#[doc = "Field `reg_frame_period` reader - "]
pub type RegFramePeriodR = crate::FieldReader;
#[doc = "Field `reg_frame_period` writer - "]
pub type RegFramePeriodW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_frame_period(&self) -> RegFramePeriodR {
        RegFramePeriodR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_frame_period(&mut self) -> RegFramePeriodW<'_, Dvp2axiFramePeriodSpec> {
        RegFramePeriodW::new(self, 0)
    }
}
#[doc = "Frame Period\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_frame_period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_frame_period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvp2axiFramePeriodSpec;
impl crate::RegisterSpec for Dvp2axiFramePeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp2axi_frame_period::R`](R) reader structure"]
impl crate::Readable for Dvp2axiFramePeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp2axi_frame_period::W`](W) writer structure"]
impl crate::Writable for Dvp2axiFramePeriodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP2AXI_FRAME_PERIOD to value 0"]
impl crate::Resettable for Dvp2axiFramePeriodSpec {}
