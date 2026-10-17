#[doc = "Register `TZC_PSRAMB_TZSRG_R2` reader"]
pub type R = crate::R<TzcPsrambTzsrgR2Spec>;
#[doc = "Register `TZC_PSRAMB_TZSRG_R2` writer"]
pub type W = crate::W<TzcPsrambTzsrgR2Spec>;
#[doc = "Field `tzc_psramb_tzsrg_r2_end` reader - End address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_r2_end` writer - End address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR2EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_psramb_tzsrg_r2_start` reader - Start address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR2StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_r2_start` writer - Start address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR2StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_end(&self) -> TzcPsrambTzsrgR2EndR {
        TzcPsrambTzsrgR2EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_start(&self) -> TzcPsrambTzsrgR2StartR {
        TzcPsrambTzsrgR2StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_end(&mut self) -> TzcPsrambTzsrgR2EndW<'_, TzcPsrambTzsrgR2Spec> {
        TzcPsrambTzsrgR2EndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_start(
        &mut self,
    ) -> TzcPsrambTzsrgR2StartW<'_, TzcPsrambTzsrgR2Spec> {
        TzcPsrambTzsrgR2StartW::new(self, 16)
    }
}
#[doc = "PSRAM-B Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgR2Spec;
impl crate::RegisterSpec for TzcPsrambTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgR2Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_psramb_tzsrg_r2::W`](W) writer structure"]
impl crate::Writable for TzcPsrambTzsrgR2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_R2 to value 0xffff"]
impl crate::Resettable for TzcPsrambTzsrgR2Spec {
    const RESET_VALUE: u32 = 0xffff;
}
