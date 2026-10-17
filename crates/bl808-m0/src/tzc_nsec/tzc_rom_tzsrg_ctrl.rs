#[doc = "Register `TZC_ROM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcRomTzsrgCtrlSpec>;
#[doc = "Field `tzc_rom_tzsrg_r0_id_en` reader - Region 0 ID Enable (0:Non-Secure, 1:Secure)"]
pub type TzcRomTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_rom_tzsrg_r1_id_en` reader - "]
pub type TzcRomTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_rom_tzsrg_r2_id_en` reader - "]
pub type TzcRomTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_rom_tzsrg_r0_en` reader - Region 0 Enable"]
pub type TzcRomTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r1_en` reader - Region 1 Enable"]
pub type TzcRomTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r2_en` reader - Region 2 Enable"]
pub type TzcRomTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r0_lock` reader - Region 0 Lock"]
pub type TzcRomTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r1_lock` reader - Region 1 Lock"]
pub type TzcRomTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r2_lock` reader - Region 2 Lock"]
pub type TzcRomTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_sboot_done` reader - Secure Boot Done Status"]
pub type TzcSbootDoneR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Region 0 ID Enable (0:Non-Secure, 1:Secure)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_id_en(&self) -> TzcRomTzsrgR0IdEnR {
        TzcRomTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_id_en(&self) -> TzcRomTzsrgR1IdEnR {
        TzcRomTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_id_en(&self) -> TzcRomTzsrgR2IdEnR {
        TzcRomTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - Region 0 Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_en(&self) -> TzcRomTzsrgR0EnR {
        TzcRomTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Region 1 Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_en(&self) -> TzcRomTzsrgR1EnR {
        TzcRomTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Region 2 Enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_en(&self) -> TzcRomTzsrgR2EnR {
        TzcRomTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Region 0 Lock"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_lock(&self) -> TzcRomTzsrgR0LockR {
        TzcRomTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Region 1 Lock"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_lock(&self) -> TzcRomTzsrgR1LockR {
        TzcRomTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Region 2 Lock"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_lock(&self) -> TzcRomTzsrgR2LockR {
        TzcRomTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Secure Boot Done Status"]
    #[inline(always)]
    pub fn tzc_sboot_done(&self) -> TzcSbootDoneR {
        TzcSbootDoneR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "ROM TZSRG Control Register (Region Enable/ID Enable/Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcRomTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcRomTzsrgCtrlSpec {}
#[doc = "`reset()` method sets TZC_ROM_TZSRG_CTRL to value 0"]
impl crate::Resettable for TzcRomTzsrgCtrlSpec {}
