#[doc = "Register `TZC_PSRAMB_TZSRG_R0` reader"]
pub type R = crate::R<TzcPsrambTzsrgR0Spec>;
#[doc = "Register `TZC_PSRAMB_TZSRG_R0` writer"]
pub type W = crate::W<TzcPsrambTzsrgR0Spec>;
#[doc = "Field `tzc_psramb_tzsrg_r0_end` reader - End address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_r0_end` writer - End address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR0EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_psramb_tzsrg_r0_start` reader - Start address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR0StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_r0_start` writer - Start address boundary (16-bit offset)"]
pub type TzcPsrambTzsrgR0StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_end(&self) -> TzcPsrambTzsrgR0EndR {
        TzcPsrambTzsrgR0EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_start(&self) -> TzcPsrambTzsrgR0StartR {
        TzcPsrambTzsrgR0StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_end(&mut self) -> TzcPsrambTzsrgR0EndW<'_, TzcPsrambTzsrgR0Spec> {
        TzcPsrambTzsrgR0EndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_start(
        &mut self,
    ) -> TzcPsrambTzsrgR0StartW<'_, TzcPsrambTzsrgR0Spec> {
        TzcPsrambTzsrgR0StartW::new(self, 16)
    }
}
#[doc = "PSRAM-B Security Region 0 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_r0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgR0Spec;
impl crate::RegisterSpec for TzcPsrambTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgR0Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_psramb_tzsrg_r0::W`](W) writer structure"]
impl crate::Writable for TzcPsrambTzsrgR0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_R0 to value 0xffff"]
impl crate::Resettable for TzcPsrambTzsrgR0Spec {
    const RESET_VALUE: u32 = 0xffff;
}
