#[doc = "Register `SRC_XIC` reader"]
pub type R = crate::R<SrcXicSpec>;
#[doc = "Register `SRC_XIC` writer"]
pub type W = crate::W<SrcXicSpec>;
#[doc = "Field `SRC_X_INCR` reader - "]
pub type SrcXIncrR = crate::FieldReader<u32>;
#[doc = "Field `SRC_X_INCR` writer - "]
pub type SrcXIncrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_x_incr(&self) -> SrcXIncrR {
        SrcXIncrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn src_x_incr(&mut self) -> SrcXIncrW<'_, SrcXicSpec> {
        SrcXIncrW::new(self, 0)
    }
}
#[doc = "Source X Stride (Increment)\n\nYou can [`read`](crate::Reg::read) this register and get [`src_xic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_xic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcXicSpec;
impl crate::RegisterSpec for SrcXicSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_xic::R`](R) reader structure"]
impl crate::Readable for SrcXicSpec {}
#[doc = "`write(|w| ..)` method takes [`src_xic::W`](W) writer structure"]
impl crate::Writable for SrcXicSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_XIC to value 0"]
impl crate::Resettable for SrcXicSpec {}
