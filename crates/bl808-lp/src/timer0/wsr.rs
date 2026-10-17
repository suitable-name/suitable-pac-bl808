#[doc = "Register `WSR` reader"]
pub type R = crate::R<WsrSpec>;
#[doc = "Register `WSR` writer"]
pub type W = crate::W<WsrSpec>;
#[doc = "Field `wts` writer - Watchdog timeout status"]
pub type WtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Watchdog timeout status"]
    #[inline(always)]
    pub fn wts(&mut self) -> WtsW<'_, WsrSpec> {
        WtsW::new(self, 0)
    }
}
#[doc = "Watchdog Reset Status\n\nYou can [`read`](crate::Reg::read) this register and get [`wsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WsrSpec;
impl crate::RegisterSpec for WsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wsr::R`](R) reader structure"]
impl crate::Readable for WsrSpec {}
#[doc = "`write(|w| ..)` method takes [`wsr::W`](W) writer structure"]
impl crate::Writable for WsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WSR to value 0"]
impl crate::Resettable for WsrSpec {}
