#[doc = "Register `TSR3` reader"]
pub type R = crate::R<Tsr3Spec>;
#[doc = "Field `tsr3_0` reader - "]
pub type Tsr3_0R = crate::BitReader;
#[doc = "Field `tsr3_1` reader - "]
pub type Tsr3_1R = crate::BitReader;
#[doc = "Field `tsr3_2` reader - "]
pub type Tsr3_2R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tsr3_0(&self) -> Tsr3_0R {
        Tsr3_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tsr3_1(&self) -> Tsr3_1R {
        Tsr3_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tsr3_2(&self) -> Tsr3_2R {
        Tsr3_2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Timer 3 Match Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tsr3Spec;
impl crate::RegisterSpec for Tsr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsr3::R`](R) reader structure"]
impl crate::Readable for Tsr3Spec {}
#[doc = "`reset()` method sets TSR3 to value 0"]
impl crate::Resettable for Tsr3Spec {}
