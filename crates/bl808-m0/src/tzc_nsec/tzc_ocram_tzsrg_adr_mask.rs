#[doc = "Register `TZC_OCRAM_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcOcramTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_ocram_tzsrg_adr_mask` reader - "]
pub type TzcOcramTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_ocram_tzsrg_adr_mask_lock` reader - "]
pub type TzcOcramTzsrgAdrMaskLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_adr_mask(&self) -> TzcOcramTzsrgAdrMaskR {
        TzcOcramTzsrgAdrMaskR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_adr_mask_lock(&self) -> TzcOcramTzsrgAdrMaskLockR {
        TzcOcramTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "OCRAM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_adr_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcOcramTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcOcramTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_ocram_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcOcramTzsrgAdrMaskSpec {}
#[doc = "`reset()` method sets TZC_OCRAM_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcOcramTzsrgAdrMaskSpec {}
