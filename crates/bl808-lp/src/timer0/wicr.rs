#[doc = "Register `WICR` writer"]
pub type W = crate::W<WicrSpec>;
#[doc = "Field `wiclr` writer - "]
pub type WiclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wiclr(&mut self) -> WiclrW<'_, WicrSpec> {
        WiclrW::new(self, 0)
    }
}
#[doc = "WDT Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WicrSpec;
impl crate::RegisterSpec for WicrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wicr::W`](W) writer structure"]
impl crate::Writable for WicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WICR to value 0"]
impl crate::Resettable for WicrSpec {}
