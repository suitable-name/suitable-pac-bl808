#[doc = "Register `TZC_WRAM_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcWramTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_wram_tzsrg_adr_mask` reader - "]
pub type TzcWramTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_wram_tzsrg_adr_mask_lock` reader - "]
pub type TzcWramTzsrgAdrMaskLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_adr_mask(&self) -> TzcWramTzsrgAdrMaskR {
        TzcWramTzsrgAdrMaskR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_adr_mask_lock(&self) -> TzcWramTzsrgAdrMaskLockR {
        TzcWramTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "WRAM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_adr_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcWramTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcWramTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wram_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcWramTzsrgAdrMaskSpec {}
#[doc = "`reset()` method sets TZC_WRAM_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcWramTzsrgAdrMaskSpec {}
