#[doc = "Register `CLOCK_DIVIDER` reader"]
pub type R = crate::R<ClockDividerSpec>;
#[doc = "Register `CLOCK_DIVIDER` writer"]
pub type W = crate::W<ClockDividerSpec>;
#[doc = "Field `cd` reader - "]
pub type CdR = crate::FieldReader;
#[doc = "Field `cd` writer - "]
pub type CdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `clockoff` reader - "]
pub type ClockoffR = crate::BitReader;
#[doc = "Field `clockoff` writer - "]
pub type ClockoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `readonly` reader - "]
pub type ReadonlyR = crate::BitReader;
#[doc = "Field `rxinten` reader - "]
pub type RxintenR = crate::BitReader;
#[doc = "Field `rxinten` writer - "]
pub type RxintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cbp` reader - "]
pub type CbpR = crate::BitReader;
#[doc = "Field `cbp` writer - "]
pub type CbpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mode` reader - "]
pub type ModeR = crate::BitReader;
#[doc = "Field `mode` writer - "]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cd(&self) -> CdR {
        CdR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clockoff(&self) -> ClockoffR {
        ClockoffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn readonly(&self) -> ReadonlyR {
        ReadonlyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rxinten(&self) -> RxintenR {
        RxintenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cbp(&self) -> CbpR {
        CbpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cd(&mut self) -> CdW<'_, ClockDividerSpec> {
        CdW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clockoff(&mut self) -> ClockoffW<'_, ClockDividerSpec> {
        ClockoffW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rxinten(&mut self) -> RxintenW<'_, ClockDividerSpec> {
        RxintenW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cbp(&mut self) -> CbpW<'_, ClockDividerSpec> {
        CbpW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ClockDividerSpec> {
        ModeW::new(self, 7)
    }
}
#[doc = "Clock Divider and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_divider::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_divider::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockDividerSpec;
impl crate::RegisterSpec for ClockDividerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_divider::R`](R) reader structure"]
impl crate::Readable for ClockDividerSpec {}
#[doc = "`write(|w| ..)` method takes [`clock_divider::W`](W) writer structure"]
impl crate::Writable for ClockDividerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK_DIVIDER to value 0"]
impl crate::Resettable for ClockDividerSpec {}
