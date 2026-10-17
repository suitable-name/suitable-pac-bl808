#[doc = "Register `AXI2DVP_PREFETCH` reader"]
pub type R = crate::R<Axi2dvpPrefetchSpec>;
#[doc = "Register `AXI2DVP_PREFETCH` writer"]
pub type W = crate::W<Axi2dvpPrefetchSpec>;
#[doc = "Field `cr_prefetch_v` reader - Vertical line count for prefetch mechanism"]
pub type CrPrefetchVR = crate::FieldReader<u16>;
#[doc = "Field `cr_prefetch_v` writer - Vertical line count for prefetch mechanism"]
pub type CrPrefetchVW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Vertical line count for prefetch mechanism"]
    #[inline(always)]
    pub fn cr_prefetch_v(&self) -> CrPrefetchVR {
        CrPrefetchVR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Vertical line count for prefetch mechanism"]
    #[inline(always)]
    pub fn cr_prefetch_v(&mut self) -> CrPrefetchVW<'_, Axi2dvpPrefetchSpec> {
        CrPrefetchVW::new(self, 0)
    }
}
#[doc = "AXI to DVP Vertical Prefetch Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_prefetch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_prefetch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Axi2dvpPrefetchSpec;
impl crate::RegisterSpec for Axi2dvpPrefetchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi2dvp_prefetch::R`](R) reader structure"]
impl crate::Readable for Axi2dvpPrefetchSpec {}
#[doc = "`write(|w| ..)` method takes [`axi2dvp_prefetch::W`](W) writer structure"]
impl crate::Writable for Axi2dvpPrefetchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI2DVP_PREFETCH to value 0x28"]
impl crate::Resettable for Axi2dvpPrefetchSpec {
    const RESET_VALUE: u32 = 0x28;
}
