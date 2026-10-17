#[doc = "Register `TOUCH_DATA_HYSTERSIS` reader"]
pub type R = crate::R<TouchDataHystersisSpec>;
#[doc = "Register `TOUCH_DATA_HYSTERSIS` writer"]
pub type W = crate::W<TouchDataHystersisSpec>;
#[doc = "Field `touch_data_hys` reader - "]
pub type TouchDataHysR = crate::FieldReader<u16>;
#[doc = "Field `touch_data_hys` writer - "]
pub type TouchDataHysW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn touch_data_hys(&self) -> TouchDataHysR {
        TouchDataHysR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn touch_data_hys(&mut self) -> TouchDataHysW<'_, TouchDataHystersisSpec> {
        TouchDataHysW::new(self, 0)
    }
}
#[doc = "Touch Data Hysteresis Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_data_hystersis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_data_hystersis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchDataHystersisSpec;
impl crate::RegisterSpec for TouchDataHystersisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_data_hystersis::R`](R) reader structure"]
impl crate::Readable for TouchDataHystersisSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_data_hystersis::W`](W) writer structure"]
impl crate::Writable for TouchDataHystersisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_DATA_HYSTERSIS to value 0"]
impl crate::Resettable for TouchDataHystersisSpec {}
