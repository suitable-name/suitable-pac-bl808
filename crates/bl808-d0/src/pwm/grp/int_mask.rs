#[doc = "Register `INT_MASK` reader"]
pub type R = crate::R<IntMaskSpec>;
#[doc = "Register `INT_MASK` writer"]
pub type W = crate::W<IntMaskSpec>;
#[doc = "Field `cr_pwm_ch0l_mask` reader - "]
pub type CrPwmCh0lMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_ch0l_mask` writer - "]
pub type CrPwmCh0lMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch0h_mask` reader - "]
pub type CrPwmCh0hMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_ch0h_mask` writer - "]
pub type CrPwmCh0hMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch1l_mask` reader - "]
pub type CrPwmCh1lMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_ch1l_mask` writer - "]
pub type CrPwmCh1lMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch1h_mask` reader - "]
pub type CrPwmCh1hMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_ch1h_mask` writer - "]
pub type CrPwmCh1hMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch2l_mask` reader - "]
pub type CrPwmCh2lMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_ch2l_mask` writer - "]
pub type CrPwmCh2lMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch2h_mask` reader - "]
pub type CrPwmCh2hMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_ch2h_mask` writer - "]
pub type CrPwmCh2hMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch3l_mask` reader - "]
pub type CrPwmCh3lMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_ch3l_mask` writer - "]
pub type CrPwmCh3lMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_ch3h_mask` reader - "]
pub type CrPwmCh3hMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_ch3h_mask` writer - "]
pub type CrPwmCh3hMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_prde_mask` reader - "]
pub type CrPwmPrdeMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_prde_mask` writer - "]
pub type CrPwmPrdeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_brk_mask` reader - "]
pub type CrPwmBrkMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_brk_mask` writer - "]
pub type CrPwmBrkMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pwm_rept_mask` reader - "]
pub type CrPwmReptMaskR = crate::BitReader;
#[doc = "Field `cr_pwm_rept_mask` writer - "]
pub type CrPwmReptMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pwm_ch0l_mask(&self) -> CrPwmCh0lMaskR {
        CrPwmCh0lMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pwm_ch0h_mask(&self) -> CrPwmCh0hMaskR {
        CrPwmCh0hMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pwm_ch1l_mask(&self) -> CrPwmCh1lMaskR {
        CrPwmCh1lMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pwm_ch1h_mask(&self) -> CrPwmCh1hMaskR {
        CrPwmCh1hMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pwm_ch2l_mask(&self) -> CrPwmCh2lMaskR {
        CrPwmCh2lMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pwm_ch2h_mask(&self) -> CrPwmCh2hMaskR {
        CrPwmCh2hMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pwm_ch3l_mask(&self) -> CrPwmCh3lMaskR {
        CrPwmCh3lMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pwm_ch3h_mask(&self) -> CrPwmCh3hMaskR {
        CrPwmCh3hMaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pwm_prde_mask(&self) -> CrPwmPrdeMaskR {
        CrPwmPrdeMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pwm_brk_mask(&self) -> CrPwmBrkMaskR {
        CrPwmBrkMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pwm_rept_mask(&self) -> CrPwmReptMaskR {
        CrPwmReptMaskR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pwm_ch0l_mask(&mut self) -> CrPwmCh0lMaskW<'_, IntMaskSpec> {
        CrPwmCh0lMaskW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pwm_ch0h_mask(&mut self) -> CrPwmCh0hMaskW<'_, IntMaskSpec> {
        CrPwmCh0hMaskW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pwm_ch1l_mask(&mut self) -> CrPwmCh1lMaskW<'_, IntMaskSpec> {
        CrPwmCh1lMaskW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pwm_ch1h_mask(&mut self) -> CrPwmCh1hMaskW<'_, IntMaskSpec> {
        CrPwmCh1hMaskW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pwm_ch2l_mask(&mut self) -> CrPwmCh2lMaskW<'_, IntMaskSpec> {
        CrPwmCh2lMaskW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pwm_ch2h_mask(&mut self) -> CrPwmCh2hMaskW<'_, IntMaskSpec> {
        CrPwmCh2hMaskW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pwm_ch3l_mask(&mut self) -> CrPwmCh3lMaskW<'_, IntMaskSpec> {
        CrPwmCh3lMaskW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pwm_ch3h_mask(&mut self) -> CrPwmCh3hMaskW<'_, IntMaskSpec> {
        CrPwmCh3hMaskW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pwm_prde_mask(&mut self) -> CrPwmPrdeMaskW<'_, IntMaskSpec> {
        CrPwmPrdeMaskW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pwm_brk_mask(&mut self) -> CrPwmBrkMaskW<'_, IntMaskSpec> {
        CrPwmBrkMaskW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pwm_rept_mask(&mut self) -> CrPwmReptMaskW<'_, IntMaskSpec> {
        CrPwmReptMaskW::new(self, 10)
    }
}
#[doc = "PWM Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMaskSpec;
impl crate::RegisterSpec for IntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_mask::R`](R) reader structure"]
impl crate::Readable for IntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_mask::W`](W) writer structure"]
impl crate::Writable for IntMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_MASK to value 0x07ff"]
impl crate::Resettable for IntMaskSpec {
    const RESET_VALUE: u32 = 0x07ff;
}
