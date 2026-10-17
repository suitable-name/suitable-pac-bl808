#[doc = "Register `TZC_XRAM_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcXramTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_xram_tzsrg_adr_mask` reader - "]
pub type TzcXramTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_xram_tzsrg_adr_mask_lock` reader - "]
pub type TzcXramTzsrgAdrMaskLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_adr_mask(&self) -> TzcXramTzsrgAdrMaskR {
        TzcXramTzsrgAdrMaskR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_adr_mask_lock(&self) -> TzcXramTzsrgAdrMaskLockR {
        TzcXramTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "XRAM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_adr_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcXramTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcXramTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_xram_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcXramTzsrgAdrMaskSpec {}
#[doc = "`reset()` method sets TZC_XRAM_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcXramTzsrgAdrMaskSpec {}
