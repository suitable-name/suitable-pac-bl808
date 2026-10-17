#[doc = "Register `TZC_ROM_TZSRG_R0` reader"]
pub type R = crate::R<TzcRomTzsrgR0Spec>;
#[doc = "Field `tzc_rom_tzsrg_r0_end` reader - Region 0 end address"]
pub type TzcRomTzsrgR0EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_rom_tzsrg_r0_start` reader - Region 0 start address"]
pub type TzcRomTzsrgR0StartR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Region 0 end address"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_end(&self) -> TzcRomTzsrgR0EndR {
        TzcRomTzsrgR0EndR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Region 0 start address"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_start(&self) -> TzcRomTzsrgR0StartR {
        TzcRomTzsrgR0StartR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "ROM TZSRG Region 0 Start/End\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_r0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomTzsrgR0Spec;
impl crate::RegisterSpec for TzcRomTzsrgR0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_r0::R`](R) reader structure"]
impl crate::Readable for TzcRomTzsrgR0Spec {}
#[doc = "`reset()` method sets TZC_ROM_TZSRG_R0 to value 0"]
impl crate::Resettable for TzcRomTzsrgR0Spec {}
