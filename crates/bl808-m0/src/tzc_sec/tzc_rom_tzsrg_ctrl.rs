#[doc = "Register `TZC_ROM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcRomTzsrgCtrlSpec>;
#[doc = "Register `TZC_ROM_TZSRG_CTRL` writer"]
pub type W = crate::W<TzcRomTzsrgCtrlSpec>;
#[doc = "Field `tzc_rom_tzsrg_r0_id_en` reader - Region 0 permission settings"]
pub type TzcRomTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_rom_tzsrg_r0_id_en` writer - Region 0 permission settings"]
pub type TzcRomTzsrgR0IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_rom_tzsrg_r1_id_en` reader - Region 1 permission settings"]
pub type TzcRomTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_rom_tzsrg_r1_id_en` writer - Region 1 permission settings"]
pub type TzcRomTzsrgR1IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_rom_tzsrg_r2_id_en` reader - Region 2 permission settings"]
pub type TzcRomTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_rom_tzsrg_r2_id_en` writer - Region 2 permission settings"]
pub type TzcRomTzsrgR2IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_rom_tzsrg_r0_en` reader - Region 0 enable"]
pub type TzcRomTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r0_en` writer - Region 0 enable"]
pub type TzcRomTzsrgR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_r1_en` reader - Region 1 enable"]
pub type TzcRomTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r1_en` writer - Region 1 enable"]
pub type TzcRomTzsrgR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_r2_en` reader - Region 2 enable"]
pub type TzcRomTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r2_en` writer - Region 2 enable"]
pub type TzcRomTzsrgR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_r0_lock` reader - Region 0 lock (Write-Once)"]
pub type TzcRomTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r0_lock` writer - Region 0 lock (Write-Once)"]
pub type TzcRomTzsrgR0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_r1_lock` reader - Region 1 lock (Write-Once)"]
pub type TzcRomTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r1_lock` writer - Region 1 lock (Write-Once)"]
pub type TzcRomTzsrgR1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_rom_tzsrg_r2_lock` reader - Region 2 lock (Write-Once)"]
pub type TzcRomTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_rom_tzsrg_r2_lock` writer - Region 2 lock (Write-Once)"]
pub type TzcRomTzsrgR2LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sboot_done` reader - Secure boot completion flag/control"]
pub type TzcSbootDoneR = crate::FieldReader;
#[doc = "Field `tzc_sboot_done` writer - Secure boot completion flag/control"]
pub type TzcSbootDoneW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Region 0 permission settings"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_id_en(&self) -> TzcRomTzsrgR0IdEnR {
        TzcRomTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Region 1 permission settings"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_id_en(&self) -> TzcRomTzsrgR1IdEnR {
        TzcRomTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Region 2 permission settings"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_id_en(&self) -> TzcRomTzsrgR2IdEnR {
        TzcRomTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_en(&self) -> TzcRomTzsrgR0EnR {
        TzcRomTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_en(&self) -> TzcRomTzsrgR1EnR {
        TzcRomTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_en(&self) -> TzcRomTzsrgR2EnR {
        TzcRomTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_lock(&self) -> TzcRomTzsrgR0LockR {
        TzcRomTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_lock(&self) -> TzcRomTzsrgR1LockR {
        TzcRomTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_lock(&self) -> TzcRomTzsrgR2LockR {
        TzcRomTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Secure boot completion flag/control"]
    #[inline(always)]
    pub fn tzc_sboot_done(&self) -> TzcSbootDoneR {
        TzcSbootDoneR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Region 0 permission settings"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_id_en(&mut self) -> TzcRomTzsrgR0IdEnW<'_, TzcRomTzsrgCtrlSpec> {
        TzcRomTzsrgR0IdEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Region 1 permission settings"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_id_en(&mut self) -> TzcRomTzsrgR1IdEnW<'_, TzcRomTzsrgCtrlSpec> {
        TzcRomTzsrgR1IdEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Region 2 permission settings"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_id_en(&mut self) -> TzcRomTzsrgR2IdEnW<'_, TzcRomTzsrgCtrlSpec> {
        TzcRomTzsrgR2IdEnW::new(self, 4)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_en(&mut self) -> TzcRomTzsrgR0EnW<'_, TzcRomTzsrgCtrlSpec> {
        TzcRomTzsrgR0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_en(&mut self) -> TzcRomTzsrgR1EnW<'_, TzcRomTzsrgCtrlSpec> {
        TzcRomTzsrgR1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_en(&mut self) -> TzcRomTzsrgR2EnW<'_, TzcRomTzsrgCtrlSpec> {
        TzcRomTzsrgR2EnW::new(self, 18)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r0_lock(&mut self) -> TzcRomTzsrgR0LockW<'_, TzcRomTzsrgCtrlSpec> {
        TzcRomTzsrgR0LockW::new(self, 24)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r1_lock(&mut self) -> TzcRomTzsrgR1LockW<'_, TzcRomTzsrgCtrlSpec> {
        TzcRomTzsrgR1LockW::new(self, 25)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_rom_tzsrg_r2_lock(&mut self) -> TzcRomTzsrgR2LockW<'_, TzcRomTzsrgCtrlSpec> {
        TzcRomTzsrgR2LockW::new(self, 26)
    }
    #[doc = "Bits 28:31 - Secure boot completion flag/control"]
    #[inline(always)]
    pub fn tzc_sboot_done(&mut self) -> TzcSbootDoneW<'_, TzcRomTzsrgCtrlSpec> {
        TzcSbootDoneW::new(self, 28)
    }
}
#[doc = "ROM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_rom_tzsrg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcRomTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcRomTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_rom_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcRomTzsrgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_rom_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcRomTzsrgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_ROM_TZSRG_CTRL to value 0x3f"]
impl crate::Resettable for TzcRomTzsrgCtrlSpec {
    const RESET_VALUE: u32 = 0x3f;
}
