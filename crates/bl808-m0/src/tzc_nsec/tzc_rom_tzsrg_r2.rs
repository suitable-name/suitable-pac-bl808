#[doc = "Register `TZC_ROM_TZSRG_R2` reader"]
pub type R = crate::R<TzcRomTzsrgR2Spec>;
#[doc = "Field `tzc_rom_tzsrg_r2_end` reader - Region 2 end address"]
pub type TzcRomTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom_tzsrg_r2_start` reader - Region 2 start address"]
pub type TzcRomTzsrgR2StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Region 2 end address"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_end(&self) -> TzcRomTzsrgR2EndR {
        TzcRomTzsrgR2EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Region 2 start address"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_start(&self) -> TzcRomTzsrgR2StartR {
        TzcRomTzsrgR2StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "ROM TZSRG Region 2 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomTzsrgR2Spec;
impl crate::RegisterSpec for TzcRomTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcRomTzsrgR2Spec {}
#[doc = "`reset()` method sets TZC_ROM_TZSRG_R2 to value 0"]
impl crate::Resettable for TzcRomTzsrgR2Spec {}
