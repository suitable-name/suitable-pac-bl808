#[doc = "Register `TCCR` reader"]
pub type R = crate::R<TccrSpec>;
#[doc = "Register `TCCR` writer"]
pub type W = crate::W<TccrSpec>;
#[doc = "Field `cs_2` reader - Timer 2 clock source selection"]
pub type Cs2R = crate::FieldReader;
#[doc = "Field `cs_2` writer - Timer 2 clock source selection"]
pub type Cs2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cs_3` reader - Timer 3 clock source selection"]
pub type Cs3R = crate::FieldReader;
#[doc = "Field `cs_3` writer - Timer 3 clock source selection"]
pub type Cs3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cs_wdt` reader - Watchdog clock source selection"]
pub type CsWdtR = crate::FieldReader;
#[doc = "Field `cs_wdt` writer - Watchdog clock source selection"]
pub type CsWdtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ID` reader - Timer ID (Fixed 0xA5)"]
pub type IdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Timer 2 clock source selection"]
    #[inline(always)]
    pub fn cs_2(&self) -> Cs2R {
        Cs2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Timer 3 clock source selection"]
    #[inline(always)]
    pub fn cs_3(&self) -> Cs3R {
        Cs3R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Watchdog clock source selection"]
    #[inline(always)]
    pub fn cs_wdt(&self) -> CsWdtR {
        CsWdtR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Timer ID (Fixed 0xA5)"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer 2 clock source selection"]
    #[inline(always)]
    pub fn cs_2(&mut self) -> Cs2W<'_, TccrSpec> {
        Cs2W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Timer 3 clock source selection"]
    #[inline(always)]
    pub fn cs_3(&mut self) -> Cs3W<'_, TccrSpec> {
        Cs3W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Watchdog clock source selection"]
    #[inline(always)]
    pub fn cs_wdt(&mut self) -> CsWdtW<'_, TccrSpec> {
        CsWdtW::new(self, 8)
    }
}
#[doc = "Timer Clock Source Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TccrSpec;
impl crate::RegisterSpec for TccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tccr::R`](R) reader structure"]
impl crate::Readable for TccrSpec {}
#[doc = "`write(|w| ..)` method takes [`tccr::W`](W) writer structure"]
impl crate::Writable for TccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCCR to value 0xa500_0155"]
impl crate::Resettable for TccrSpec {
    const RESET_VALUE: u32 = 0xa500_0155;
}
