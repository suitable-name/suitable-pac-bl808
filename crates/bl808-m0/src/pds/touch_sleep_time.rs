#[doc = "Register `TOUCH_SLEEP_TIME` reader"]
pub type R = crate::R<TouchSleepTimeSpec>;
#[doc = "Register `TOUCH_SLEEP_TIME` writer"]
pub type W = crate::W<TouchSleepTimeSpec>;
#[doc = "Field `touch_sleep_cycle` reader - "]
pub type TouchSleepCycleR = crate::FieldReader<u32>;
#[doc = "Field `touch_sleep_cycle` writer - "]
pub type TouchSleepCycleW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn touch_sleep_cycle(&self) -> TouchSleepCycleR {
        TouchSleepCycleR::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22"]
    #[inline(always)]
    pub fn touch_sleep_cycle(&mut self) -> TouchSleepCycleW<'_, TouchSleepTimeSpec> {
        TouchSleepCycleW::new(self, 0)
    }
}
#[doc = "Touch Sleep Time Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_sleep_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_sleep_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchSleepTimeSpec;
impl crate::RegisterSpec for TouchSleepTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_sleep_time::R`](R) reader structure"]
impl crate::Readable for TouchSleepTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_sleep_time::W`](W) writer structure"]
impl crate::Writable for TouchSleepTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_SLEEP_TIME to value 0x0007_ffff"]
impl crate::Resettable for TouchSleepTimeSpec {
    const RESET_VALUE: u32 = 0x0007_ffff;
}
