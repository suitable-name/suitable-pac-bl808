#[doc = "Register `TZC_VRAM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcVramTzsrgCtrlSpec>;
#[doc = "Register `TZC_VRAM_TZSRG_CTRL` writer"]
pub type W = crate::W<TzcVramTzsrgCtrlSpec>;
#[doc = "Field `tzc_vram_tzsrg_r0_id_en` reader - Region 0 TZID permission map"]
pub type TzcVramTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_vram_tzsrg_r0_id_en` writer - Region 0 TZID permission map"]
pub type TzcVramTzsrgR0IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_vram_tzsrg_r1_id_en` reader - Region 1 TZID permission map"]
pub type TzcVramTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_vram_tzsrg_r1_id_en` writer - Region 1 TZID permission map"]
pub type TzcVramTzsrgR1IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_vram_tzsrg_r2_id_en` reader - Region 2 TZID permission map"]
pub type TzcVramTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_vram_tzsrg_r2_id_en` writer - Region 2 TZID permission map"]
pub type TzcVramTzsrgR2IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_vram_tzsrg_r0_en` reader - Region 0 enable"]
pub type TzcVramTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r0_en` writer - Region 0 enable"]
pub type TzcVramTzsrgR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_vram_tzsrg_r1_en` reader - Region 1 enable"]
pub type TzcVramTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r1_en` writer - Region 1 enable"]
pub type TzcVramTzsrgR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_vram_tzsrg_r2_en` reader - Region 2 enable"]
pub type TzcVramTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r2_en` writer - Region 2 enable"]
pub type TzcVramTzsrgR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_vram_tzsrg_r0_lock` reader - Region 0 lock (Write-Once)"]
pub type TzcVramTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r0_lock` writer - Region 0 lock (Write-Once)"]
pub type TzcVramTzsrgR0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_vram_tzsrg_r1_lock` reader - Region 1 lock (Write-Once)"]
pub type TzcVramTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r1_lock` writer - Region 1 lock (Write-Once)"]
pub type TzcVramTzsrgR1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_vram_tzsrg_r2_lock` reader - Region 2 lock (Write-Once)"]
pub type TzcVramTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_vram_tzsrg_r2_lock` writer - Region 2 lock (Write-Once)"]
pub type TzcVramTzsrgR2LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_id_en(&self) -> TzcVramTzsrgR0IdEnR {
        TzcVramTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_id_en(&self) -> TzcVramTzsrgR1IdEnR {
        TzcVramTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_id_en(&self) -> TzcVramTzsrgR2IdEnR {
        TzcVramTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_en(&self) -> TzcVramTzsrgR0EnR {
        TzcVramTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_en(&self) -> TzcVramTzsrgR1EnR {
        TzcVramTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_en(&self) -> TzcVramTzsrgR2EnR {
        TzcVramTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_lock(&self) -> TzcVramTzsrgR0LockR {
        TzcVramTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_lock(&self) -> TzcVramTzsrgR1LockR {
        TzcVramTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_lock(&self) -> TzcVramTzsrgR2LockR {
        TzcVramTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_id_en(&mut self) -> TzcVramTzsrgR0IdEnW<'_, TzcVramTzsrgCtrlSpec> {
        TzcVramTzsrgR0IdEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_id_en(&mut self) -> TzcVramTzsrgR1IdEnW<'_, TzcVramTzsrgCtrlSpec> {
        TzcVramTzsrgR1IdEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_id_en(&mut self) -> TzcVramTzsrgR2IdEnW<'_, TzcVramTzsrgCtrlSpec> {
        TzcVramTzsrgR2IdEnW::new(self, 4)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_en(&mut self) -> TzcVramTzsrgR0EnW<'_, TzcVramTzsrgCtrlSpec> {
        TzcVramTzsrgR0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_en(&mut self) -> TzcVramTzsrgR1EnW<'_, TzcVramTzsrgCtrlSpec> {
        TzcVramTzsrgR1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_en(&mut self) -> TzcVramTzsrgR2EnW<'_, TzcVramTzsrgCtrlSpec> {
        TzcVramTzsrgR2EnW::new(self, 18)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r0_lock(&mut self) -> TzcVramTzsrgR0LockW<'_, TzcVramTzsrgCtrlSpec> {
        TzcVramTzsrgR0LockW::new(self, 24)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r1_lock(&mut self) -> TzcVramTzsrgR1LockW<'_, TzcVramTzsrgCtrlSpec> {
        TzcVramTzsrgR1LockW::new(self, 25)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_vram_tzsrg_r2_lock(&mut self) -> TzcVramTzsrgR2LockW<'_, TzcVramTzsrgCtrlSpec> {
        TzcVramTzsrgR2LockW::new(self, 26)
    }
}
#[doc = "VRAM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_vram_tzsrg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_vram_tzsrg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcVramTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcVramTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_vram_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcVramTzsrgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_vram_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcVramTzsrgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_VRAM_TZSRG_CTRL to value 0x3f"]
impl crate::Resettable for TzcVramTzsrgCtrlSpec {
    const RESET_VALUE: u32 = 0x3f;
}
