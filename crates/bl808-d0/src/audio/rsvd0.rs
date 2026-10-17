#[doc = "Register `rsvd0` reader"]
pub type R = crate::R<Rsvd0Spec>;
#[doc = "Register `rsvd0` writer"]
pub type W = crate::W<Rsvd0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Reserved: Do not modify\n\nYou can [`read`](crate::Reg::read) this register and get [`rsvd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsvd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsvd0Spec;
impl crate::RegisterSpec for Rsvd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsvd0::R`](R) reader structure"]
impl crate::Readable for Rsvd0Spec {}
#[doc = "`write(|w| ..)` method takes [`rsvd0::W`](W) writer structure"]
impl crate::Writable for Rsvd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets rsvd0 to value 0"]
impl crate::Resettable for Rsvd0Spec {}
