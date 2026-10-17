#[doc = "Register `TZC_WRAM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcWramTzsrgCtrlSpec>;
#[doc = "Register `TZC_WRAM_TZSRG_CTRL` writer"]
pub type W = crate::W<TzcWramTzsrgCtrlSpec>;
#[doc = "Field `tzc_wram_tzsrg_r0_id_en` reader - Region 0 TZID permission map"]
pub type TzcWramTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_wram_tzsrg_r0_id_en` writer - Region 0 TZID permission map"]
pub type TzcWramTzsrgR0IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_wram_tzsrg_r1_id_en` reader - Region 1 TZID permission map"]
pub type TzcWramTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_wram_tzsrg_r1_id_en` writer - Region 1 TZID permission map"]
pub type TzcWramTzsrgR1IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_wram_tzsrg_r2_id_en` reader - Region 2 TZID permission map"]
pub type TzcWramTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_wram_tzsrg_r2_id_en` writer - Region 2 TZID permission map"]
pub type TzcWramTzsrgR2IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_wram_tzsrg_rx_id_en` reader - Region X (reserved) TZID permission map"]
pub type TzcWramTzsrgRxIdEnR = crate::FieldReader;
#[doc = "Field `tzc_wram_tzsrg_rx_id_en` writer - Region X (reserved) TZID permission map"]
pub type TzcWramTzsrgRxIdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_wram_tzsrg_r0_en` reader - Region 0 enable"]
pub type TzcWramTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r0_en` writer - Region 0 enable"]
pub type TzcWramTzsrgR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wram_tzsrg_r1_en` reader - Region 1 enable"]
pub type TzcWramTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r1_en` writer - Region 1 enable"]
pub type TzcWramTzsrgR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wram_tzsrg_r2_en` reader - Region 2 enable"]
pub type TzcWramTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r2_en` writer - Region 2 enable"]
pub type TzcWramTzsrgR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wram_tzsrg_rx_en` reader - Region X (reserved) enable"]
pub type TzcWramTzsrgRxEnR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_rx_en` writer - Region X (reserved) enable"]
pub type TzcWramTzsrgRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wram_tzsrg_r0_lock` reader - Region 0 lock (Write-Once)"]
pub type TzcWramTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r0_lock` writer - Region 0 lock (Write-Once)"]
pub type TzcWramTzsrgR0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wram_tzsrg_r1_lock` reader - Region 1 lock (Write-Once)"]
pub type TzcWramTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r1_lock` writer - Region 1 lock (Write-Once)"]
pub type TzcWramTzsrgR1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wram_tzsrg_r2_lock` reader - Region 2 lock (Write-Once)"]
pub type TzcWramTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_r2_lock` writer - Region 2 lock (Write-Once)"]
pub type TzcWramTzsrgR2LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wram_tzsrg_rx_lock` reader - Region X (reserved) lock (Write-Once)"]
pub type TzcWramTzsrgRxLockR = crate::BitReader;
#[doc = "Field `tzc_wram_tzsrg_rx_lock` writer - Region X (reserved) lock (Write-Once)"]
pub type TzcWramTzsrgRxLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_id_en(&self) -> TzcWramTzsrgR0IdEnR {
        TzcWramTzsrgR0IdEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_id_en(&self) -> TzcWramTzsrgR1IdEnR {
        TzcWramTzsrgR1IdEnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_id_en(&self) -> TzcWramTzsrgR2IdEnR {
        TzcWramTzsrgR2IdEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region X (reserved) TZID permission map"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_rx_id_en(&self) -> TzcWramTzsrgRxIdEnR {
        TzcWramTzsrgRxIdEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_en(&self) -> TzcWramTzsrgR0EnR {
        TzcWramTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_en(&self) -> TzcWramTzsrgR1EnR {
        TzcWramTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_en(&self) -> TzcWramTzsrgR2EnR {
        TzcWramTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Region X (reserved) enable"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_rx_en(&self) -> TzcWramTzsrgRxEnR {
        TzcWramTzsrgRxEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_lock(&self) -> TzcWramTzsrgR0LockR {
        TzcWramTzsrgR0LockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_lock(&self) -> TzcWramTzsrgR1LockR {
        TzcWramTzsrgR1LockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_lock(&self) -> TzcWramTzsrgR2LockR {
        TzcWramTzsrgR2LockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Region X (reserved) lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_rx_lock(&self) -> TzcWramTzsrgRxLockR {
        TzcWramTzsrgRxLockR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_id_en(&mut self) -> TzcWramTzsrgR0IdEnW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgR0IdEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_id_en(&mut self) -> TzcWramTzsrgR1IdEnW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgR1IdEnW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_id_en(&mut self) -> TzcWramTzsrgR2IdEnW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgR2IdEnW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Region X (reserved) TZID permission map"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_rx_id_en(&mut self) -> TzcWramTzsrgRxIdEnW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgRxIdEnW::new(self, 12)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_en(&mut self) -> TzcWramTzsrgR0EnW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgR0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_en(&mut self) -> TzcWramTzsrgR1EnW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgR1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_en(&mut self) -> TzcWramTzsrgR2EnW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgR2EnW::new(self, 18)
    }
    #[doc = "Bit 19 - Region X (reserved) enable"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_rx_en(&mut self) -> TzcWramTzsrgRxEnW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgRxEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r0_lock(&mut self) -> TzcWramTzsrgR0LockW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgR0LockW::new(self, 20)
    }
    #[doc = "Bit 21 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r1_lock(&mut self) -> TzcWramTzsrgR1LockW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgR1LockW::new(self, 21)
    }
    #[doc = "Bit 22 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_r2_lock(&mut self) -> TzcWramTzsrgR2LockW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgR2LockW::new(self, 22)
    }
    #[doc = "Bit 23 - Region X (reserved) lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_wram_tzsrg_rx_lock(&mut self) -> TzcWramTzsrgRxLockW<'_, TzcWramTzsrgCtrlSpec> {
        TzcWramTzsrgRxLockW::new(self, 23)
    }
}
#[doc = "WRAM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_wram_tzsrg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcWramTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcWramTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_wram_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcWramTzsrgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_wram_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcWramTzsrgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_WRAM_TZSRG_CTRL to value 0xffff"]
impl crate::Resettable for TzcWramTzsrgCtrlSpec {
    const RESET_VALUE: u32 = 0xffff;
}
