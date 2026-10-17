#[doc = "Register `TZC_PSRAMB_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcPsrambTzsrgAdrMaskSpec>;
#[doc = "Register `TZC_PSRAMB_TZSRG_ADR_MASK` writer"]
pub type W = crate::W<TzcPsrambTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_psramb_tzsrg_adr_mask` reader - Address mask bits \\[15:0\\]"]
pub type TzcPsrambTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_adr_mask` writer - Address mask bits \\[15:0\\]"]
pub type TzcPsrambTzsrgAdrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_psramb_tzsrg_adr_mask_lock` reader - Address mask lock (Write-Once)"]
pub type TzcPsrambTzsrgAdrMaskLockR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_adr_mask_lock` writer - Address mask lock (Write-Once)"]
pub type TzcPsrambTzsrgAdrMaskLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Address mask bits \\[15:0\\]"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_adr_mask(&self) -> TzcPsrambTzsrgAdrMaskR {
        TzcPsrambTzsrgAdrMaskR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_adr_mask_lock(&self) -> TzcPsrambTzsrgAdrMaskLockR {
        TzcPsrambTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Address mask bits \\[15:0\\]"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_adr_mask(
        &mut self,
    ) -> TzcPsrambTzsrgAdrMaskW<'_, TzcPsrambTzsrgAdrMaskSpec> {
        TzcPsrambTzsrgAdrMaskW::new(self, 0)
    }
    #[doc = "Bit 16 - Address mask lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_adr_mask_lock(
        &mut self,
    ) -> TzcPsrambTzsrgAdrMaskLockW<'_, TzcPsrambTzsrgAdrMaskSpec> {
        TzcPsrambTzsrgAdrMaskLockW::new(self, 16)
    }
}
#[doc = "PSRAM-B Security Region Address Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_adr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_adr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcPsrambTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgAdrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_psramb_tzsrg_adr_mask::W`](W) writer structure"]
impl crate::Writable for TzcPsrambTzsrgAdrMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcPsrambTzsrgAdrMaskSpec {}
