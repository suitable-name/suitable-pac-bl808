#[doc = "Register `TICR2` writer"]
pub type W = crate::W<Ticr2Spec>;
#[doc = "Field `tclr2_0` writer - "]
pub type Tclr2_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tclr2_1` writer - "]
pub type Tclr2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tclr2_2` writer - "]
pub type Tclr2_2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tclr2_0(&mut self) -> Tclr2_0W<'_, Ticr2Spec> {
        Tclr2_0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tclr2_1(&mut self) -> Tclr2_1W<'_, Ticr2Spec> {
        Tclr2_1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tclr2_2(&mut self) -> Tclr2_2W<'_, Ticr2Spec> {
        Tclr2_2W::new(self, 2)
    }
}
#[doc = "Timer 2 Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ticr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ticr2Spec;
impl crate::RegisterSpec for Ticr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ticr2::W`](W) writer structure"]
impl crate::Writable for Ticr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TICR2 to value 0"]
impl crate::Resettable for Ticr2Spec {}
