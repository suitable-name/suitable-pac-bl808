#[doc = "Register `DEAD_TIME` reader"]
pub type R = crate::R<DeadTimeSpec>;
#[doc = "Register `DEAD_TIME` writer"]
pub type W = crate::W<DeadTimeSpec>;
#[doc = "Field `pwm_ch0_dtg` reader - CH0 dead-time generator value"]
pub type PwmCh0DtgR = crate::FieldReader;
#[doc = "Field `pwm_ch0_dtg` writer - CH0 dead-time generator value"]
pub type PwmCh0DtgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pwm_ch1_dtg` reader - "]
pub type PwmCh1DtgR = crate::FieldReader;
#[doc = "Field `pwm_ch1_dtg` writer - "]
pub type PwmCh1DtgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pwm_ch2_dtg` reader - "]
pub type PwmCh2DtgR = crate::FieldReader;
#[doc = "Field `pwm_ch2_dtg` writer - "]
pub type PwmCh2DtgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pwm_ch3_dtg` reader - "]
pub type PwmCh3DtgR = crate::FieldReader;
#[doc = "Field `pwm_ch3_dtg` writer - "]
pub type PwmCh3DtgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - CH0 dead-time generator value"]
    #[inline(always)]
    pub fn pwm_ch0_dtg(&self) -> PwmCh0DtgR {
        PwmCh0DtgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pwm_ch1_dtg(&self) -> PwmCh1DtgR {
        PwmCh1DtgR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pwm_ch2_dtg(&self) -> PwmCh2DtgR {
        PwmCh2DtgR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pwm_ch3_dtg(&self) -> PwmCh3DtgR {
        PwmCh3DtgR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CH0 dead-time generator value"]
    #[inline(always)]
    pub fn pwm_ch0_dtg(&mut self) -> PwmCh0DtgW<'_, DeadTimeSpec> {
        PwmCh0DtgW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pwm_ch1_dtg(&mut self) -> PwmCh1DtgW<'_, DeadTimeSpec> {
        PwmCh1DtgW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pwm_ch2_dtg(&mut self) -> PwmCh2DtgW<'_, DeadTimeSpec> {
        PwmCh2DtgW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pwm_ch3_dtg(&mut self) -> PwmCh3DtgW<'_, DeadTimeSpec> {
        PwmCh3DtgW::new(self, 24)
    }
}
#[doc = "PWM Dead-Time Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dead_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dead_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeadTimeSpec;
impl crate::RegisterSpec for DeadTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dead_time::R`](R) reader structure"]
impl crate::Readable for DeadTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`dead_time::W`](W) writer structure"]
impl crate::Writable for DeadTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEAD_TIME to value 0"]
impl crate::Resettable for DeadTimeSpec {}
