#[doc = "Register `INT_STS` reader"]
pub type R = crate::R<IntStsSpec>;
#[doc = "Field `pwm_ch0l_int` reader - "]
pub type PwmCh0lIntR = crate::BitReader;
#[doc = "Field `pwm_ch0h_int` reader - "]
pub type PwmCh0hIntR = crate::BitReader;
#[doc = "Field `pwm_ch1l_int` reader - "]
pub type PwmCh1lIntR = crate::BitReader;
#[doc = "Field `pwm_ch1h_int` reader - "]
pub type PwmCh1hIntR = crate::BitReader;
#[doc = "Field `pwm_ch2l_int` reader - "]
pub type PwmCh2lIntR = crate::BitReader;
#[doc = "Field `pwm_ch2h_int` reader - "]
pub type PwmCh2hIntR = crate::BitReader;
#[doc = "Field `pwm_ch3l_int` reader - "]
pub type PwmCh3lIntR = crate::BitReader;
#[doc = "Field `pwm_ch3h_int` reader - "]
pub type PwmCh3hIntR = crate::BitReader;
#[doc = "Field `pwm_prde_int` reader - "]
pub type PwmPrdeIntR = crate::BitReader;
#[doc = "Field `pwm_brk_int` reader - "]
pub type PwmBrkIntR = crate::BitReader;
#[doc = "Field `pwm_rept_int` reader - "]
pub type PwmReptIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwm_ch0l_int(&self) -> PwmCh0lIntR {
        PwmCh0lIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pwm_ch0h_int(&self) -> PwmCh0hIntR {
        PwmCh0hIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pwm_ch1l_int(&self) -> PwmCh1lIntR {
        PwmCh1lIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pwm_ch1h_int(&self) -> PwmCh1hIntR {
        PwmCh1hIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pwm_ch2l_int(&self) -> PwmCh2lIntR {
        PwmCh2lIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pwm_ch2h_int(&self) -> PwmCh2hIntR {
        PwmCh2hIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwm_ch3l_int(&self) -> PwmCh3lIntR {
        PwmCh3lIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwm_ch3h_int(&self) -> PwmCh3hIntR {
        PwmCh3hIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pwm_prde_int(&self) -> PwmPrdeIntR {
        PwmPrdeIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwm_brk_int(&self) -> PwmBrkIntR {
        PwmBrkIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pwm_rept_int(&self) -> PwmReptIntR {
        PwmReptIntR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "PWM Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStsSpec;
impl crate::RegisterSpec for IntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_sts::R`](R) reader structure"]
impl crate::Readable for IntStsSpec {}
#[doc = "`reset()` method sets INT_STS to value 0"]
impl crate::Resettable for IntStsSpec {}
