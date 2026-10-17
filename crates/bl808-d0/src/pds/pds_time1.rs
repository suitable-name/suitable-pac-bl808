#[doc = "Register `PDS_TIME1` reader"]
pub type R = crate::R<PdsTime1Spec>;
#[doc = "Register `PDS_TIME1` writer"]
pub type W = crate::W<PdsTime1Spec>;
#[doc = "Field `cr_sleep_duration` reader - "]
pub type CrSleepDurationR = crate::FieldReader<u32>;
#[doc = "Field `cr_sleep_duration` writer - "]
pub type CrSleepDurationW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_sleep_duration(&self) -> CrSleepDurationR {
        CrSleepDurationR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_sleep_duration(&mut self) -> CrSleepDurationW<'_, PdsTime1Spec> {
        CrSleepDurationW::new(self, 0)
    }
}
#[doc = "PDS Sleep Duration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pds_time1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pds_time1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdsTime1Spec;
impl crate::RegisterSpec for PdsTime1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_time1::R`](R) reader structure"]
impl crate::Readable for PdsTime1Spec {}
#[doc = "`write(|w| ..)` method takes [`pds_time1::W`](W) writer structure"]
impl crate::Writable for PdsTime1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDS_TIME1 to value 0x0ca8"]
impl crate::Resettable for PdsTime1Spec {
    const RESET_VALUE: u32 = 0x0ca8;
}
