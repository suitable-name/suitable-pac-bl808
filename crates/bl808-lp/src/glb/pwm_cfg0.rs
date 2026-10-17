#[doc = "Register `PWM_CFG0` reader"]
pub type R = crate::R<PwmCfg0Spec>;
#[doc = "Register `PWM_CFG0` writer"]
pub type W = crate::W<PwmCfg0Spec>;
#[doc = "Field `reg_pwm1_io_sel` reader - "]
pub type RegPwm1IoSelR = crate::BitReader;
#[doc = "Field `reg_pwm1_io_sel` writer - "]
pub type RegPwm1IoSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pwm2_io_sel` reader - "]
pub type RegPwm2IoSelR = crate::BitReader;
#[doc = "Field `reg_pwm2_io_sel` writer - "]
pub type RegPwm2IoSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pwm1_io_sel(&self) -> RegPwm1IoSelR {
        RegPwm1IoSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_pwm2_io_sel(&self) -> RegPwm2IoSelR {
        RegPwm2IoSelR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pwm1_io_sel(&mut self) -> RegPwm1IoSelW<'_, PwmCfg0Spec> {
        RegPwm1IoSelW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_pwm2_io_sel(&mut self) -> RegPwm2IoSelW<'_, PwmCfg0Spec> {
        RegPwm2IoSelW::new(self, 1)
    }
}
#[doc = "PWM IO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmCfg0Spec;
impl crate::RegisterSpec for PwmCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_cfg0::R`](R) reader structure"]
impl crate::Readable for PwmCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pwm_cfg0::W`](W) writer structure"]
impl crate::Writable for PwmCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM_CFG0 to value 0"]
impl crate::Resettable for PwmCfg0Spec {}
