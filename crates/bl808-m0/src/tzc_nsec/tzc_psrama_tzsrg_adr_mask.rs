#[doc = "Register `TZC_PSRAMA_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcPsramaTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_psrama_tzsrg_adr_mask` reader - "]
pub type TzcPsramaTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_psrama_tzsrg_adr_mask_lock` reader - "]
pub type TzcPsramaTzsrgAdrMaskLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_adr_mask(&self) -> TzcPsramaTzsrgAdrMaskR {
        TzcPsramaTzsrgAdrMaskR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_adr_mask_lock(&self) -> TzcPsramaTzsrgAdrMaskLockR {
        TzcPsramaTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "PSRAM-A TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_adr_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsramaTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcPsramaTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psrama_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcPsramaTzsrgAdrMaskSpec {}
#[doc = "`reset()` method sets TZC_PSRAMA_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcPsramaTzsrgAdrMaskSpec {}
