#[doc = "Register `EF_DAT_1_RSVD_1` reader"]
pub type R = crate::R<EfDat1Rsvd1Spec>;
#[doc = "Register `EF_DAT_1_RSVD_1` writer"]
pub type W = crate::W<EfDat1Rsvd1Spec>;
#[doc = "Field `ef_dat_1_rsvd_1` reader - "]
pub type EfDat1Rsvd1R = crate::FieldReader<u32>;
#[doc = "Field `ef_dat_1_rsvd_1` writer - "]
pub type EfDat1Rsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dat_1_rsvd_1(&self) -> EfDat1Rsvd1R {
        EfDat1Rsvd1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_dat_1_rsvd_1(&mut self) -> EfDat1Rsvd1W<'_, EfDat1Rsvd1Spec> {
        EfDat1Rsvd1W::new(self, 0)
    }
}
#[doc = "eFuse Data 1 Reserved Word 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_dat_1_rsvd_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_dat_1_rsvd_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfDat1Rsvd1Spec;
impl crate::RegisterSpec for EfDat1Rsvd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_dat_1_rsvd_1::R`](R) reader structure"]
impl crate::Readable for EfDat1Rsvd1Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_dat_1_rsvd_1::W`](W) writer structure"]
impl crate::Writable for EfDat1Rsvd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_DAT_1_RSVD_1 to value 0"]
impl crate::Resettable for EfDat1Rsvd1Spec {}
