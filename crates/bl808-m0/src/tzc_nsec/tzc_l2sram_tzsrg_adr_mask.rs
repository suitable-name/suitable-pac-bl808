#[doc = "Register `TZC_L2SRAM_TZSRG_ADR_MASK` reader"]
pub type R = crate::R<TzcL2sramTzsrgAdrMaskSpec>;
#[doc = "Field `tzc_l2sram_tzsrg_adr_mask` reader - "]
pub type TzcL2sramTzsrgAdrMaskR = crate::FieldReader<u16>;
#[doc = "Field `tzc_l2sram_tzsrg_adr_mask_lock` reader - "]
pub type TzcL2sramTzsrgAdrMaskLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_adr_mask(&self) -> TzcL2sramTzsrgAdrMaskR {
        TzcL2sramTzsrgAdrMaskR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_adr_mask_lock(&self) -> TzcL2sramTzsrgAdrMaskLockR {
        TzcL2sramTzsrgAdrMaskLockR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "L2SRAM TZSRG Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_adr_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcL2sramTzsrgAdrMaskSpec;
impl crate::RegisterSpec for TzcL2sramTzsrgAdrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_l2sram_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TzcL2sramTzsrgAdrMaskSpec {}
#[doc = "`reset()` method sets TZC_L2SRAM_TZSRG_ADR_MASK to value 0"]
impl crate::Resettable for TzcL2sramTzsrgAdrMaskSpec {}
