#[doc = "Register `TCDR` reader"]
pub type R = crate::R<TcdrSpec>;
#[doc = "Register `TCDR` writer"]
pub type W = crate::W<TcdrSpec>;
#[doc = "Field `tcdr2` reader - Timer 2 clock division factor"]
pub type Tcdr2R = crate::FieldReader;
#[doc = "Field `tcdr2` writer - Timer 2 clock division factor"]
pub type Tcdr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `tcdr3` reader - Timer 3 clock division factor"]
pub type Tcdr3R = crate::FieldReader;
#[doc = "Field `tcdr3` writer - Timer 3 clock division factor"]
pub type Tcdr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `wcdr` reader - Watchdog clock division factor"]
pub type WcdrR = crate::FieldReader;
#[doc = "Field `wcdr` writer - Watchdog clock division factor"]
pub type WcdrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - Timer 2 clock division factor"]
    #[inline(always)]
    pub fn tcdr2(&self) -> Tcdr2R {
        Tcdr2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Timer 3 clock division factor"]
    #[inline(always)]
    pub fn tcdr3(&self) -> Tcdr3R {
        Tcdr3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Watchdog clock division factor"]
    #[inline(always)]
    pub fn wcdr(&self) -> WcdrR {
        WcdrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Timer 2 clock division factor"]
    #[inline(always)]
    pub fn tcdr2(&mut self) -> Tcdr2W<'_, TcdrSpec> {
        Tcdr2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Timer 3 clock division factor"]
    #[inline(always)]
    pub fn tcdr3(&mut self) -> Tcdr3W<'_, TcdrSpec> {
        Tcdr3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Watchdog clock division factor"]
    #[inline(always)]
    pub fn wcdr(&mut self) -> WcdrW<'_, TcdrSpec> {
        WcdrW::new(self, 24)
    }
}
#[doc = "Timer and WDT Division\n\nYou can [`read`](crate::Reg::read) this register and get [`tcdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcdrSpec;
impl crate::RegisterSpec for TcdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcdr::R`](R) reader structure"]
impl crate::Readable for TcdrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcdr::W`](W) writer structure"]
impl crate::Writable for TcdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCDR to value 0"]
impl crate::Resettable for TcdrSpec {}
