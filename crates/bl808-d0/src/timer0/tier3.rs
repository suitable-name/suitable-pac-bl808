#[doc = "Register `TIER3` reader"]
pub type R = crate::R<Tier3Spec>;
#[doc = "Register `TIER3` writer"]
pub type W = crate::W<Tier3Spec>;
#[doc = "Field `tier3_0` reader - "]
pub type Tier3_0R = crate::BitReader;
#[doc = "Field `tier3_0` writer - "]
pub type Tier3_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tier3_1` reader - "]
pub type Tier3_1R = crate::BitReader;
#[doc = "Field `tier3_1` writer - "]
pub type Tier3_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tier3_2` reader - "]
pub type Tier3_2R = crate::BitReader;
#[doc = "Field `tier3_2` writer - "]
pub type Tier3_2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tier3_0(&self) -> Tier3_0R {
        Tier3_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tier3_1(&self) -> Tier3_1R {
        Tier3_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tier3_2(&self) -> Tier3_2R {
        Tier3_2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tier3_0(&mut self) -> Tier3_0W<'_, Tier3Spec> {
        Tier3_0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tier3_1(&mut self) -> Tier3_1W<'_, Tier3Spec> {
        Tier3_1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tier3_2(&mut self) -> Tier3_2W<'_, Tier3Spec> {
        Tier3_2W::new(self, 2)
    }
}
#[doc = "Timer 3 Match Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`tier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tier3Spec;
impl crate::RegisterSpec for Tier3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tier3::R`](R) reader structure"]
impl crate::Readable for Tier3Spec {}
#[doc = "`write(|w| ..)` method takes [`tier3::W`](W) writer structure"]
impl crate::Writable for Tier3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIER3 to value 0"]
impl crate::Resettable for Tier3Spec {}
