#[doc = "Register `TZC_PSRAMB_TZSRG_R1` reader"]
pub type R = crate::R<TzcPsrambTzsrgR1Spec>;
#[doc = "Register `TZC_PSRAMB_TZSRG_R1` writer"]
pub type W = crate::W<TzcPsrambTzsrgR1Spec>;
#[doc = "Field `tzc_psramb_tzsrg_r1_end` reader - End address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_r1_end` writer - End address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR1EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_psramb_tzsrg_r1_start` reader - Start address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR1StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_r1_start` writer - Start address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR1StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_end(&self) -> TzcPsrambTzsrgR1EndR {
        TzcPsrambTzsrgR1EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_start(&self) -> TzcPsrambTzsrgR1StartR {
        TzcPsrambTzsrgR1StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_end(&mut self) -> TzcPsrambTzsrgR1EndW<'_, TzcPsrambTzsrgR1Spec> {
        TzcPsrambTzsrgR1EndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_start(
        &mut self,
    ) -> TzcPsrambTzsrgR1StartW<'_, TzcPsrambTzsrgR1Spec> {
        TzcPsrambTzsrgR1StartW::new(self, 16)
    }
}
#[doc = "PSRAM-B Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgR1Spec;
impl crate::RegisterSpec for TzcPsrambTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgR1Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_psramb_tzsrg_r1::W`](W) writer structure"]
impl crate::Writable for TzcPsrambTzsrgR1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_R1 to value 0xffff"]
impl crate::Resettable for TzcPsrambTzsrgR1Spec {
    const RESET_VALUE: u32 = 0xffff;
}
