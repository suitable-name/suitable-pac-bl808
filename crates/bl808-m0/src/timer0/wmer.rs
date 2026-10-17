#[doc = "Register `WMER` reader"]
pub type R = crate::R<WmerSpec>;
#[doc = "Register `WMER` writer"]
pub type W = crate::W<WmerSpec>;
#[doc = "Field `we` reader - Watchdog Enable"]
pub type WeR = crate::BitReader;
#[doc = "Field `we` writer - Watchdog Enable"]
pub type WeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrie` reader - Watchdog Reset/Interrupt: 0:Interrupt, 1:Reset"]
pub type WrieR = crate::BitReader;
#[doc = "Field `wrie` writer - Watchdog Reset/Interrupt: 0:Interrupt, 1:Reset"]
pub type WrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Watchdog Enable"]
    #[inline(always)]
    pub fn we(&self) -> WeR {
        WeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Reset/Interrupt: 0:Interrupt, 1:Reset"]
    #[inline(always)]
    pub fn wrie(&self) -> WrieR {
        WrieR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Enable"]
    #[inline(always)]
    pub fn we(&mut self) -> WeW<'_, WmerSpec> {
        WeW::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog Reset/Interrupt: 0:Interrupt, 1:Reset"]
    #[inline(always)]
    pub fn wrie(&mut self) -> WrieW<'_, WmerSpec> {
        WrieW::new(self, 1)
    }
}
#[doc = "Watchdog reset/interrupt Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wmer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wmer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WmerSpec;
impl crate::RegisterSpec for WmerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wmer::R`](R) reader structure"]
impl crate::Readable for WmerSpec {}
#[doc = "`write(|w| ..)` method takes [`wmer::W`](W) writer structure"]
impl crate::Writable for WmerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WMER to value 0"]
impl crate::Resettable for WmerSpec {}
