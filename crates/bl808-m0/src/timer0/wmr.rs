#[doc = "Register `WMR` reader"]
pub type R = crate::R<WmrSpec>;
#[doc = "Register `WMR` writer"]
pub type W = crate::W<WmrSpec>;
#[doc = "Field `wmr` reader - Watchdog match counter limit"]
pub type WmrR = crate::FieldReader<u16>;
#[doc = "Field `wmr` writer - Watchdog match counter limit"]
pub type WmrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `wdt_align` reader - "]
pub type WdtAlignR = crate::BitReader;
#[doc = "Field `wdt_align` writer - "]
pub type WdtAlignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Watchdog match counter limit"]
    #[inline(always)]
    pub fn wmr(&self) -> WmrR {
        WmrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wdt_align(&self) -> WdtAlignR {
        WdtAlignR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watchdog match counter limit"]
    #[inline(always)]
    pub fn wmr(&mut self) -> WmrW<'_, WmrSpec> {
        WmrW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wdt_align(&mut self) -> WdtAlignW<'_, WmrSpec> {
        WdtAlignW::new(self, 16)
    }
}
#[doc = "Watchdog Match Value\n\nYou can [`read`](crate::Reg::read) this register and get [`wmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WmrSpec;
impl crate::RegisterSpec for WmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wmr::R`](R) reader structure"]
impl crate::Readable for WmrSpec {}
#[doc = "`write(|w| ..)` method takes [`wmr::W`](W) writer structure"]
impl crate::Writable for WmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WMR to value 0xffff"]
impl crate::Resettable for WmrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
