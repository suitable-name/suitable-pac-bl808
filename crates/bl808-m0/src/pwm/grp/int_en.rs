#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<IntEnSpec>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<IntEnSpec>;
#[doc = "Field `cr_pwm_ch0l_en` reader - "]
pub type CrPwmCh0lEnR = crate::BitReader;
#[doc = "Field `cr_pwm_ch0l_en` writer - "]
pub type CrPwmCh0lEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch0h_en` reader - "]
pub type CrPwmCh0hEnR = crate::BitReader;
#[doc = "Field `cr_pwm_ch0h_en` writer - "]
pub type CrPwmCh0hEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch1l_en` reader - "]
pub type CrPwmCh1lEnR = crate::BitReader;
#[doc = "Field `cr_pwm_ch1l_en` writer - "]
pub type CrPwmCh1lEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch1h_en` reader - "]
pub type CrPwmCh1hEnR = crate::BitReader;
#[doc = "Field `cr_pwm_ch1h_en` writer - "]
pub type CrPwmCh1hEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch2l_en` reader - "]
pub type CrPwmCh2lEnR = crate::BitReader;
#[doc = "Field `cr_pwm_ch2l_en` writer - "]
pub type CrPwmCh2lEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch2h_en` reader - "]
pub type CrPwmCh2hEnR = crate::BitReader;
#[doc = "Field `cr_pwm_ch2h_en` writer - "]
pub type CrPwmCh2hEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch3l_en` reader - "]
pub type CrPwmCh3lEnR = crate::BitReader;
#[doc = "Field `cr_pwm_ch3l_en` writer - "]
pub type CrPwmCh3lEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch3h_en` reader - "]
pub type CrPwmCh3hEnR = crate::BitReader;
#[doc = "Field `cr_pwm_ch3h_en` writer - "]
pub type CrPwmCh3hEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_prde_en` reader - "]
pub type CrPwmPrdeEnR = crate::BitReader;
#[doc = "Field `cr_pwm_prde_en` writer - "]
pub type CrPwmPrdeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_brk_en` reader - "]
pub type CrPwmBrkEnR = crate::BitReader;
#[doc = "Field `cr_pwm_brk_en` writer - "]
pub type CrPwmBrkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_rept_en` reader - "]
pub type CrPwmReptEnR = crate::BitReader;
#[doc = "Field `cr_pwm_rept_en` writer - "]
pub type CrPwmReptEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pwm_ch0l_en(&self) -> CrPwmCh0lEnR {
        CrPwmCh0lEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pwm_ch0h_en(&self) -> CrPwmCh0hEnR {
        CrPwmCh0hEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pwm_ch1l_en(&self) -> CrPwmCh1lEnR {
        CrPwmCh1lEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pwm_ch1h_en(&self) -> CrPwmCh1hEnR {
        CrPwmCh1hEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pwm_ch2l_en(&self) -> CrPwmCh2lEnR {
        CrPwmCh2lEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pwm_ch2h_en(&self) -> CrPwmCh2hEnR {
        CrPwmCh2hEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pwm_ch3l_en(&self) -> CrPwmCh3lEnR {
        CrPwmCh3lEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pwm_ch3h_en(&self) -> CrPwmCh3hEnR {
        CrPwmCh3hEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pwm_prde_en(&self) -> CrPwmPrdeEnR {
        CrPwmPrdeEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pwm_brk_en(&self) -> CrPwmBrkEnR {
        CrPwmBrkEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pwm_rept_en(&self) -> CrPwmReptEnR {
        CrPwmReptEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pwm_ch0l_en(&mut self) -> CrPwmCh0lEnW<'_, IntEnSpec> {
        CrPwmCh0lEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pwm_ch0h_en(&mut self) -> CrPwmCh0hEnW<'_, IntEnSpec> {
        CrPwmCh0hEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pwm_ch1l_en(&mut self) -> CrPwmCh1lEnW<'_, IntEnSpec> {
        CrPwmCh1lEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pwm_ch1h_en(&mut self) -> CrPwmCh1hEnW<'_, IntEnSpec> {
        CrPwmCh1hEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pwm_ch2l_en(&mut self) -> CrPwmCh2lEnW<'_, IntEnSpec> {
        CrPwmCh2lEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pwm_ch2h_en(&mut self) -> CrPwmCh2hEnW<'_, IntEnSpec> {
        CrPwmCh2hEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pwm_ch3l_en(&mut self) -> CrPwmCh3lEnW<'_, IntEnSpec> {
        CrPwmCh3lEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pwm_ch3h_en(&mut self) -> CrPwmCh3hEnW<'_, IntEnSpec> {
        CrPwmCh3hEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pwm_prde_en(&mut self) -> CrPwmPrdeEnW<'_, IntEnSpec> {
        CrPwmPrdeEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pwm_brk_en(&mut self) -> CrPwmBrkEnW<'_, IntEnSpec> {
        CrPwmBrkEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pwm_rept_en(&mut self) -> CrPwmReptEnW<'_, IntEnSpec> {
        CrPwmReptEnW::new(self, 10)
    }
}
#[doc = "PWM Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnSpec;
impl crate::RegisterSpec for IntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for IntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_EN to value 0x05ff"]
impl crate::Resettable for IntEnSpec {
    const RESET_VALUE: u32 = 0x05ff;
}
