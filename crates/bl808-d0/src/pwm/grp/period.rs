#[doc = "Register `PERIOD` reader"]
pub type R = crate::R<PeriodSpec>;
#[doc = "Register `PERIOD` writer"]
pub type W = crate::W<PeriodSpec>;
#[doc = "Field `pwm_period` reader - PWM counter period value"]
pub type PwmPeriodR = crate::FieldReader<u16>;
#[doc = "Field `pwm_period` writer - PWM counter period value"]
pub type PwmPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `pwm_int_period_cnt` reader - Interrupt period counter value"]
pub type PwmIntPeriodCntR = crate::FieldReader<u16>;
#[doc = "Field `pwm_int_period_cnt` writer - Interrupt period counter value"]
pub type PwmIntPeriodCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PWM counter period value"]
    #[inline(always)]
    pub fn pwm_period(&self) -> PwmPeriodR {
        PwmPeriodR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Interrupt period counter value"]
    #[inline(always)]
    pub fn pwm_int_period_cnt(&self) -> PwmIntPeriodCntR {
        PwmIntPeriodCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PWM counter period value"]
    #[inline(always)]
    pub fn pwm_period(&mut self) -> PwmPeriodW<'_, PeriodSpec> {
        PwmPeriodW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Interrupt period counter value"]
    #[inline(always)]
    pub fn pwm_int_period_cnt(&mut self) -> PwmIntPeriodCntW<'_, PeriodSpec> {
        PwmIntPeriodCntW::new(self, 16)
    }
}
#[doc = "PWM Period and Interrupt Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriodSpec;
impl crate::RegisterSpec for PeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`period::R`](R) reader structure"]
impl crate::Readable for PeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`period::W`](W) writer structure"]
impl crate::Writable for PeriodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIOD to value 0"]
impl crate::Resettable for PeriodSpec {}
