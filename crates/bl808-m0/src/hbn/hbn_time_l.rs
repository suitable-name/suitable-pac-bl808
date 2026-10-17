#[doc = "Register `HBN_TIME_L` reader"]
pub type R = crate::R<HbnTimeLSpec>;
#[doc = "Register `HBN_TIME_L` writer"]
pub type W = crate::W<HbnTimeLSpec>;
#[doc = "Field `hbn_time_l` reader - "]
pub type HbnTimeLR = crate::FieldReader<u32>;
#[doc = "Field `hbn_time_l` writer - "]
pub type HbnTimeLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_time_l(&self) -> HbnTimeLR {
        HbnTimeLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_time_l(&mut self) -> HbnTimeLW<'_, HbnTimeLSpec> {
        HbnTimeLW::new(self, 0)
    }
}
#[doc = "Hibernate Timer Low\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_time_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_time_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnTimeLSpec;
impl crate::RegisterSpec for HbnTimeLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_time_l::R`](R) reader structure"]
impl crate::Readable for HbnTimeLSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_time_l::W`](W) writer structure"]
impl crate::Writable for HbnTimeLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_TIME_L to value 0"]
impl crate::Resettable for HbnTimeLSpec {}
