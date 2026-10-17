#[doc = "Register `TILR2` reader"]
pub type R = crate::R<Tilr2Spec>;
#[doc = "Register `TILR2` writer"]
pub type W = crate::W<Tilr2Spec>;
#[doc = "Field `tilr2_0` reader - "]
pub type Tilr2_0R = crate::BitReader;
#[doc = "Field `tilr2_0` writer - "]
pub type Tilr2_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tilr2_1` reader - "]
pub type Tilr2_1R = crate::BitReader;
#[doc = "Field `tilr2_1` writer - "]
pub type Tilr2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tilr2_2` reader - "]
pub type Tilr2_2R = crate::BitReader;
#[doc = "Field `tilr2_2` writer - "]
pub type Tilr2_2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tilr2_0(&self) -> Tilr2_0R {
        Tilr2_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tilr2_1(&self) -> Tilr2_1R {
        Tilr2_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tilr2_2(&self) -> Tilr2_2R {
        Tilr2_2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tilr2_0(&mut self) -> Tilr2_0W<'_, Tilr2Spec> {
        Tilr2_0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tilr2_1(&mut self) -> Tilr2_1W<'_, Tilr2Spec> {
        Tilr2_1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tilr2_2(&mut self) -> Tilr2_2W<'_, Tilr2Spec> {
        Tilr2_2W::new(self, 2)
    }
}
#[doc = "Timer 2 Match Interrupt Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`tilr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tilr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tilr2Spec;
impl crate::RegisterSpec for Tilr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tilr2::R`](R) reader structure"]
impl crate::Readable for Tilr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tilr2::W`](W) writer structure"]
impl crate::Writable for Tilr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TILR2 to value 0"]
impl crate::Resettable for Tilr2Spec {}
