#[doc = "Register `TZC_ROM_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcRomTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_rom_tzsrg_adr_mask` reader - Address mask bits"]
pub type TzcRomTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom_tzsrg_adr_mask_lock` reader - Address mask lock"]
pub type TzcRomTzsrgAdrMaskLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Address mask bits"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_adr_mask(&self) -> TzcRomTzsrgAdrMaskR {
        TzcRomTzsrgAdrMaskR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Address mask lock"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_adr_mask_lock(&self) -> TzcRomTzsrgAdrMaskLockR {
        TzcRomTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "ROM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_adr_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcRomTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcRomTzsrgAdrMaskSpec {}
#[doc = "`reset()` method sets TZC_ROM_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcRomTzsrgAdrMaskSpec {}
