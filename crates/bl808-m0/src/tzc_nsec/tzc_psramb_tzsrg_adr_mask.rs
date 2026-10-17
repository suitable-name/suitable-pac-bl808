#[doc = "Register `TZC_PSRAMB_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcPsrambTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_psramb_tzsrg_adr_mask` reader - "]
pub type TzcPsrambTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psramb_tzsrg_adr_mask_lock` reader - "]
pub type TzcPsrambTzsrgAdrMaskLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_adr_mask(&self) -> TzcPsrambTzsrgAdrMaskR {
        TzcPsrambTzsrgAdrMaskR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_adr_mask_lock(&self) -> TzcPsrambTzsrgAdrMaskLockR {
        TzcPsrambTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "PSRAM-B TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_adr_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcPsrambTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgAdrMaskSpec {}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcPsrambTzsrgAdrMaskSpec {}
