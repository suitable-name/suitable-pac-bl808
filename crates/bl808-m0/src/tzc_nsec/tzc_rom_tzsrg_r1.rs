#[doc = "Register `TZC_ROM_TZSRG_R1` reader"]
pub type R = crate::R<TzcRomTzsrgR1Spec>;
#[doc = "Field `tzc_rom_tzsrg_r1_end` reader - Region 1 end address"]
pub type TzcRomTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom_tzsrg_r1_start` reader - Region 1 start address"]
pub type TzcRomTzsrgR1StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Region 1 end address"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_end(&self) -> TzcRomTzsrgR1EndR {
        TzcRomTzsrgR1EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Region 1 start address"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_start(&self) -> TzcRomTzsrgR1StartR {
        TzcRomTzsrgR1StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "ROM TZSRG Region 1 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomTzsrgR1Spec;
impl crate::RegisterSpec for TzcRomTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcRomTzsrgR1Spec {}
#[doc = "`reset()` method sets TZC_ROM_TZSRG_R1 to value 0"]
impl crate::Resettable for TzcRomTzsrgR1Spec {}
