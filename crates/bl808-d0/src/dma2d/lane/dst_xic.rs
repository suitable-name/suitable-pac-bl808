#[doc = "Register `DST_XIC` reader"]
pub type R = crate::R<DstXicSpec>;
#[doc = "Register `DST_XIC` writer"]
pub type W = crate::W<DstXicSpec>;
#[doc = "Field `DST_X_INCR` reader - "]
pub type DstXIncrR = crate::FieldReader<u32>;
#[doc = "Field `DST_X_INCR` writer - "]
pub type DstXIncrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_x_incr(&self) -> DstXIncrR {
        DstXIncrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dst_x_incr(&mut self) -> DstXIncrW<'_, DstXicSpec> {
        DstXIncrW::new(self, 0)
    }
}
#[doc = "Destination X Stride (Increment)\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_xic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_xic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstXicSpec;
impl crate::RegisterSpec for DstXicSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_xic::R`](R) reader structure"]
impl crate::Readable for DstXicSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_xic::W`](W) writer structure"]
impl crate::Writable for DstXicSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DST_XIC to value 0"]
impl crate::Resettable for DstXicSpec {}
