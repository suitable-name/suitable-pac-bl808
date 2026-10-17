#[doc = "Register `HBN_TIME_H` reader"]
pub type R = crate::R<HbnTimeHSpec>;
#[doc = "Register `HBN_TIME_H` writer"]
pub type W = crate::W<HbnTimeHSpec>;
#[doc = "Field `hbn_time_h` reader - "]
pub type HbnTimeHR = crate::FieldReader;
#[doc = "Field `hbn_time_h` writer - "]
pub type HbnTimeHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hbn_time_h(&self) -> HbnTimeHR {
        HbnTimeHR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hbn_time_h(&mut self) -> HbnTimeHW<'_, HbnTimeHSpec> {
        HbnTimeHW::new(self, 0)
    }
}
#[doc = "Hibernate Timer High\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_time_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_time_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnTimeHSpec;
impl crate::RegisterSpec for HbnTimeHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_time_h::R`](R) reader structure"]
impl crate::Readable for HbnTimeHSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_time_h::W`](W) writer structure"]
impl crate::Writable for HbnTimeHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_TIME_H to value 0"]
impl crate::Resettable for HbnTimeHSpec {}
