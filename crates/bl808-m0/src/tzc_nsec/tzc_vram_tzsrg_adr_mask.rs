#[doc = "Register `TZC_VRAM_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcVramTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_vram_tzsrg_adr_mask` reader - "]
pub type TzcVramTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_vram_tzsrg_adr_mask_lock` reader - "]
pub type TzcVramTzsrgAdrMaskLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_adr_mask(&self) -> TzcVramTzsrgAdrMaskR {
        TzcVramTzsrgAdrMaskR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_adr_mask_lock(&self) -> TzcVramTzsrgAdrMaskLockR {
        TzcVramTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "VRAM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_adr_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcVramTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcVramTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_vram_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcVramTzsrgAdrMaskSpec {}
#[doc = "`reset()` method sets TZC_VRAM_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcVramTzsrgAdrMaskSpec {}
