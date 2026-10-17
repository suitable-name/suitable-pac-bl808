#[doc = "Register `TSR2` reader"]
pub type R = crate::R<Tsr2Spec>;
#[doc = "Field `tsr2_0` reader - "]
pub type Tsr2_0R = crate::BitReader;
#[doc = "Field `tsr2_1` reader - "]
pub type Tsr2_1R = crate::BitReader;
#[doc = "Field `tsr2_2` reader - "]
pub type Tsr2_2R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tsr2_0(&self) -> Tsr2_0R {
        Tsr2_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tsr2_1(&self) -> Tsr2_1R {
        Tsr2_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tsr2_2(&self) -> Tsr2_2R {
        Tsr2_2R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Timer 2 Match Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tsr2Spec;
impl crate::RegisterSpec for Tsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsr2::R`](R) reader structure"]
impl crate::Readable for Tsr2Spec {}
#[doc = "`reset()` method sets TSR2 to value 0"]
impl crate::Resettable for Tsr2Spec {}
