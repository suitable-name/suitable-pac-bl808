#[doc = "Register `TOUCH_RSVD` reader"]
pub type R = crate::R<TouchRsvdSpec>;
#[doc = "Register `TOUCH_RSVD` writer"]
pub type W = crate::W<TouchRsvdSpec>;
#[doc = "Field `touch_reserved` reader - "]
pub type TouchReservedR = crate::FieldReader;
#[doc = "Field `touch_reserved` writer - "]
pub type TouchReservedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn touch_reserved(&self) -> TouchReservedR {
        TouchReservedR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn touch_reserved(&mut self) -> TouchReservedW<'_, TouchRsvdSpec> {
        TouchReservedW::new(self, 0)
    }
}
#[doc = "Touch Reserved Register\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_rsvd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_rsvd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchRsvdSpec;
impl crate::RegisterSpec for TouchRsvdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_rsvd::R`](R) reader structure"]
impl crate::Readable for TouchRsvdSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_rsvd::W`](W) writer structure"]
impl crate::Writable for TouchRsvdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_RSVD to value 0"]
impl crate::Resettable for TouchRsvdSpec {}
