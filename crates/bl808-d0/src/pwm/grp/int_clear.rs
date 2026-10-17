#[doc = "Register `INT_CLEAR` reader"]
pub type R = crate::R<IntClearSpec>;
#[doc = "Register `INT_CLEAR` writer"]
pub type W = crate::W<IntClearSpec>;
#[doc = "Field `cr_pwm_ch0l_clr` reader - "]
pub type CrPwmCh0lClrR = crate::BitReader;
#[doc = "Field `cr_pwm_ch0l_clr` writer - "]
pub type CrPwmCh0lClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_pwm_ch0h_clr` reader - "]
pub type CrPwmCh0hClrR = crate::BitReader;
#[doc = "Field `cr_pwm_ch0h_clr` writer - "]
pub type CrPwmCh0hClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_pwm_ch1l_clr` reader - "]
pub type CrPwmCh1lClrR = crate::BitReader;
#[doc = "Field `cr_pwm_ch1l_clr` writer - "]
pub type CrPwmCh1lClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_pwm_ch1h_clr` reader - "]
pub type CrPwmCh1hClrR = crate::BitReader;
#[doc = "Field `cr_pwm_ch1h_clr` writer - "]
pub type CrPwmCh1hClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_pwm_ch2l_clr` reader - "]
pub type CrPwmCh2lClrR = crate::BitReader;
#[doc = "Field `cr_pwm_ch2l_clr` writer - "]
pub type CrPwmCh2lClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_pwm_ch2h_clr` reader - "]
pub type CrPwmCh2hClrR = crate::BitReader;
#[doc = "Field `cr_pwm_ch2h_clr` writer - "]
pub type CrPwmCh2hClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_pwm_ch3l_clr` reader - "]
pub type CrPwmCh3lClrR = crate::BitReader;
#[doc = "Field `cr_pwm_ch3l_clr` writer - "]
pub type CrPwmCh3lClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_pwm_ch3h_clr` reader - "]
pub type CrPwmCh3hClrR = crate::BitReader;
#[doc = "Field `cr_pwm_ch3h_clr` writer - "]
pub type CrPwmCh3hClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_pwm_prde_clr` reader - "]
pub type CrPwmPrdeClrR = crate::BitReader;
#[doc = "Field `cr_pwm_prde_clr` writer - "]
pub type CrPwmPrdeClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_pwm_brk_clr` reader - "]
pub type CrPwmBrkClrR = crate::BitReader;
#[doc = "Field `cr_pwm_brk_clr` writer - "]
pub type CrPwmBrkClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_pwm_rept_clr` reader - "]
pub type CrPwmReptClrR = crate::BitReader;
#[doc = "Field `cr_pwm_rept_clr` writer - "]
pub type CrPwmReptClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pwm_ch0l_clr(&self) -> CrPwmCh0lClrR {
        CrPwmCh0lClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pwm_ch0h_clr(&self) -> CrPwmCh0hClrR {
        CrPwmCh0hClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pwm_ch1l_clr(&self) -> CrPwmCh1lClrR {
        CrPwmCh1lClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pwm_ch1h_clr(&self) -> CrPwmCh1hClrR {
        CrPwmCh1hClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pwm_ch2l_clr(&self) -> CrPwmCh2lClrR {
        CrPwmCh2lClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pwm_ch2h_clr(&self) -> CrPwmCh2hClrR {
        CrPwmCh2hClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pwm_ch3l_clr(&self) -> CrPwmCh3lClrR {
        CrPwmCh3lClrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pwm_ch3h_clr(&self) -> CrPwmCh3hClrR {
        CrPwmCh3hClrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pwm_prde_clr(&self) -> CrPwmPrdeClrR {
        CrPwmPrdeClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pwm_brk_clr(&self) -> CrPwmBrkClrR {
        CrPwmBrkClrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pwm_rept_clr(&self) -> CrPwmReptClrR {
        CrPwmReptClrR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pwm_ch0l_clr(&mut self) -> CrPwmCh0lClrW<'_, IntClearSpec> {
        CrPwmCh0lClrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pwm_ch0h_clr(&mut self) -> CrPwmCh0hClrW<'_, IntClearSpec> {
        CrPwmCh0hClrW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pwm_ch1l_clr(&mut self) -> CrPwmCh1lClrW<'_, IntClearSpec> {
        CrPwmCh1lClrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pwm_ch1h_clr(&mut self) -> CrPwmCh1hClrW<'_, IntClearSpec> {
        CrPwmCh1hClrW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pwm_ch2l_clr(&mut self) -> CrPwmCh2lClrW<'_, IntClearSpec> {
        CrPwmCh2lClrW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pwm_ch2h_clr(&mut self) -> CrPwmCh2hClrW<'_, IntClearSpec> {
        CrPwmCh2hClrW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pwm_ch3l_clr(&mut self) -> CrPwmCh3lClrW<'_, IntClearSpec> {
        CrPwmCh3lClrW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pwm_ch3h_clr(&mut self) -> CrPwmCh3hClrW<'_, IntClearSpec> {
        CrPwmCh3hClrW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pwm_prde_clr(&mut self) -> CrPwmPrdeClrW<'_, IntClearSpec> {
        CrPwmPrdeClrW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pwm_brk_clr(&mut self) -> CrPwmBrkClrW<'_, IntClearSpec> {
        CrPwmBrkClrW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pwm_rept_clr(&mut self) -> CrPwmReptClrW<'_, IntClearSpec> {
        CrPwmReptClrW::new(self, 10)
    }
}
#[doc = "PWM Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClearSpec;
impl crate::RegisterSpec for IntClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_clear::R`](R) reader structure"]
impl crate::Readable for IntClearSpec {}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for IntClearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07ff;
}
#[doc = "`reset()` method sets INT_CLEAR to value 0"]
impl crate::Resettable for IntClearSpec {}
