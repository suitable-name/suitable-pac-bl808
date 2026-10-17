#[doc = "Register `SRC_YIC` reader"]
pub type R = crate::R<SrcYicSpec>;
#[doc = "Register `SRC_YIC` writer"]
pub type W = crate::W<SrcYicSpec>;
#[doc = "Field `SRC_Y_INCR` reader - "]
pub type SrcYIncrR = crate::FieldReader<u32>;
#[doc = "Field `SRC_Y_INCR` writer - "]
pub type SrcYIncrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_y_incr(&self) -> SrcYIncrR {
        SrcYIncrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_y_incr(&mut self) -> SrcYIncrW<'_, SrcYicSpec> {
        SrcYIncrW::new(self, 0)
    }
}
#[doc = "Source Y Stride (Increment)\n\nYou can [`read`](crate::Reg::read) this register and get [`src_yic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_yic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcYicSpec;
impl crate::RegisterSpec for SrcYicSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_yic::R`](R) reader structure"]
impl crate::Readable for SrcYicSpec {}
#[doc = "`write(|w| ..)` method takes [`src_yic::W`](W) writer structure"]
impl crate::Writable for SrcYicSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_YIC to value 0"]
impl crate::Resettable for SrcYicSpec {}
