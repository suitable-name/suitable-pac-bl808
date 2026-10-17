#[doc = "Register `DST_YIC` reader"]
pub type R = crate::R<DstYicSpec>;
#[doc = "Register `DST_YIC` writer"]
pub type W = crate::W<DstYicSpec>;
#[doc = "Field `DST_Y_INCR` reader - "]
pub type DstYIncrR = crate::FieldReader<u32>;
#[doc = "Field `DST_Y_INCR` writer - "]
pub type DstYIncrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_y_incr(&self) -> DstYIncrR {
        DstYIncrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_y_incr(&mut self) -> DstYIncrW<'_, DstYicSpec> {
        DstYIncrW::new(self, 0)
    }
}
#[doc = "Destination Y Stride (Increment)\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_yic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_yic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstYicSpec;
impl crate::RegisterSpec for DstYicSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_yic::R`](R) reader structure"]
impl crate::Readable for DstYicSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_yic::W`](W) writer structure"]
impl crate::Writable for DstYicSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DST_YIC to value 0"]
impl crate::Resettable for DstYicSpec {}
