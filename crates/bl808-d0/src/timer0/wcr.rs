#[doc = "Register `WCR` writer"]
pub type W = crate::W<WcrSpec>;
#[doc = "Field `wcr` writer - "]
pub type WcrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wcr(&mut self) -> WcrW<'_, WcrSpec> {
        WcrW::new(self, 0)
    }
}
#[doc = "WDT Counter Reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WcrSpec;
impl crate::RegisterSpec for WcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wcr::W`](W) writer structure"]
impl crate::Writable for WcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WcrSpec {}
