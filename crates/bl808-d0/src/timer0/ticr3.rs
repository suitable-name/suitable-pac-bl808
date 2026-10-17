#[doc = "Register `TICR3` writer"]
pub type W = crate::W<Ticr3Spec>;
#[doc = "Field `tclr3_0` writer - "]
pub type Tclr3_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tclr3_1` writer - "]
pub type Tclr3_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tclr3_2` writer - "]
pub type Tclr3_2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tclr3_0(&mut self) -> Tclr3_0W<'_, Ticr3Spec> {
        Tclr3_0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tclr3_1(&mut self) -> Tclr3_1W<'_, Ticr3Spec> {
        Tclr3_1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tclr3_2(&mut self) -> Tclr3_2W<'_, Ticr3Spec> {
        Tclr3_2W::new(self, 2)
    }
}
#[doc = "Timer 3 Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ticr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ticr3Spec;
impl crate::RegisterSpec for Ticr3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ticr3::W`](W) writer structure"]
impl crate::Writable for Ticr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TICR3 to value 0"]
impl crate::Resettable for Ticr3Spec {}
