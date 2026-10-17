#[doc = "Register `coex_interrupt` reader"]
pub type R = crate::R<CoexInterruptSpec>;
#[doc = "Register `coex_interrupt` writer"]
pub type W = crate::W<CoexInterruptSpec>;
#[doc = "Field `sts_coex_int` reader - Coexistence state change interrupt status"]
pub type StsCoexIntR = crate::BitReader;
#[doc = "Field `coex_int_en` reader - Interrupt enable mask"]
pub type CoexIntEnR = crate::BitReader;
#[doc = "Field `coex_int_en` writer - Interrupt enable mask"]
pub type CoexIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `coex_int_clr` writer - Write 1 to clear interrupt"]
pub type CoexIntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Coexistence state change interrupt status"]
    #[inline(always)]
    pub fn sts_coex_int(&self) -> StsCoexIntR {
        StsCoexIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable mask"]
    #[inline(always)]
    pub fn coex_int_en(&self) -> CoexIntEnR {
        CoexIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Interrupt enable mask"]
    #[inline(always)]
    pub fn coex_int_en(&mut self) -> CoexIntEnW<'_, CoexInterruptSpec> {
        CoexIntEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn coex_int_clr(&mut self) -> CoexIntClrW<'_, CoexInterruptSpec> {
        CoexIntClrW::new(self, 16)
    }
}
#[doc = "Coexistence Interrupt Status/Control\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoexInterruptSpec;
impl crate::RegisterSpec for CoexInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coex_interrupt::R`](R) reader structure"]
impl crate::Readable for CoexInterruptSpec {}
#[doc = "`write(|w| ..)` method takes [`coex_interrupt::W`](W) writer structure"]
impl crate::Writable for CoexInterruptSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0000;
}
#[doc = "`reset()` method sets coex_interrupt to value 0"]
impl crate::Resettable for CoexInterruptSpec {}
