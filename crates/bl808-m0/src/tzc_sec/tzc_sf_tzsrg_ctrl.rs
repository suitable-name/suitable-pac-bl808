#[doc = "Register `TZC_SF_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcSfTzsrgCtrlSpec>;
#[doc = "Register `TZC_SF_TZSRG_CTRL` writer"]
pub type W = crate::W<TzcSfTzsrgCtrlSpec>;
#[doc = "Field `tzc_sf_tzsrg_r0_id_en` reader - Region 0 TZID permission map"]
pub type TzcSfTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r0_id_en` writer - Region 0 TZID permission map"]
pub type TzcSfTzsrgR0IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_sf_tzsrg_r1_id_en` reader - Region 1 TZID permission map"]
pub type TzcSfTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r1_id_en` writer - Region 1 TZID permission map"]
pub type TzcSfTzsrgR1IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_sf_tzsrg_r2_id_en` reader - Region 2 TZID permission map"]
pub type TzcSfTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r2_id_en` writer - Region 2 TZID permission map"]
pub type TzcSfTzsrgR2IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_sf_tzsrg_r3_id_en` reader - Region 3 TZID permission map"]
pub type TzcSfTzsrgR3IdEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_r3_id_en` writer - Region 3 TZID permission map"]
pub type TzcSfTzsrgR3IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_sf_tzsrg_rx_id_en` reader - Region X (reserved) TZID permission map"]
pub type TzcSfTzsrgRxIdEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_tzsrg_rx_id_en` writer - Region X (reserved) TZID permission map"]
pub type TzcSfTzsrgRxIdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_sf_tzsrg_r0_en` reader - Region 0 enable"]
pub type TzcSfTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r0_en` writer - Region 0 enable"]
pub type TzcSfTzsrgR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r1_en` reader - Region 1 enable"]
pub type TzcSfTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r1_en` writer - Region 1 enable"]
pub type TzcSfTzsrgR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r2_en` reader - Region 2 enable"]
pub type TzcSfTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r2_en` writer - Region 2 enable"]
pub type TzcSfTzsrgR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r3_en` reader - Region 3 enable"]
pub type TzcSfTzsrgR3EnR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r3_en` writer - Region 3 enable"]
pub type TzcSfTzsrgR3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_rx_en` reader - Region X (reserved) enable"]
pub type TzcSfTzsrgRxEnR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_rx_en` writer - Region X (reserved) enable"]
pub type TzcSfTzsrgRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r0_lock` reader - Region 0 lock (Write-Once)"]
pub type TzcSfTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r0_lock` writer - Region 0 lock (Write-Once)"]
pub type TzcSfTzsrgR0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r1_lock` reader - Region 1 lock (Write-Once)"]
pub type TzcSfTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r1_lock` writer - Region 1 lock (Write-Once)"]
pub type TzcSfTzsrgR1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r2_lock` reader - Region 2 lock (Write-Once)"]
pub type TzcSfTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r2_lock` writer - Region 2 lock (Write-Once)"]
pub type TzcSfTzsrgR2LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_r3_lock` reader - Region 3 lock (Write-Once)"]
pub type TzcSfTzsrgR3LockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_r3_lock` writer - Region 3 lock (Write-Once)"]
pub type TzcSfTzsrgR3LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsrg_rx_lock` reader - Region X (reserved) lock (Write-Once)"]
pub type TzcSfTzsrgRxLockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_rx_lock` writer - Region X (reserved) lock (Write-Once)"]
pub type TzcSfTzsrgRxLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_id_en(&self) -> TzcSfTzsrgR0IdEnR {
        TzcSfTzsrgR0IdEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_id_en(&self) -> TzcSfTzsrgR1IdEnR {
        TzcSfTzsrgR1IdEnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_id_en(&self) -> TzcSfTzsrgR2IdEnR {
        TzcSfTzsrgR2IdEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region 3 TZID permission map"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_id_en(&self) -> TzcSfTzsrgR3IdEnR {
        TzcSfTzsrgR3IdEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Region X (reserved) TZID permission map"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_id_en(&self) -> TzcSfTzsrgRxIdEnR {
        TzcSfTzsrgRxIdEnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_en(&self) -> TzcSfTzsrgR0EnR {
        TzcSfTzsrgR0EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_en(&self) -> TzcSfTzsrgR1EnR {
        TzcSfTzsrgR1EnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_en(&self) -> TzcSfTzsrgR2EnR {
        TzcSfTzsrgR2EnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Region 3 enable"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_en(&self) -> TzcSfTzsrgR3EnR {
        TzcSfTzsrgR3EnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Region X (reserved) enable"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_en(&self) -> TzcSfTzsrgRxEnR {
        TzcSfTzsrgRxEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_lock(&self) -> TzcSfTzsrgR0LockR {
        TzcSfTzsrgR0LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_lock(&self) -> TzcSfTzsrgR1LockR {
        TzcSfTzsrgR1LockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_lock(&self) -> TzcSfTzsrgR2LockR {
        TzcSfTzsrgR2LockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Region 3 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_lock(&self) -> TzcSfTzsrgR3LockR {
        TzcSfTzsrgR3LockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Region X (reserved) lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_lock(&self) -> TzcSfTzsrgRxLockR {
        TzcSfTzsrgRxLockR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_id_en(&mut self) -> TzcSfTzsrgR0IdEnW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR0IdEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_id_en(&mut self) -> TzcSfTzsrgR1IdEnW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR1IdEnW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_id_en(&mut self) -> TzcSfTzsrgR2IdEnW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR2IdEnW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Region 3 TZID permission map"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_id_en(&mut self) -> TzcSfTzsrgR3IdEnW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR3IdEnW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Region X (reserved) TZID permission map"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_id_en(&mut self) -> TzcSfTzsrgRxIdEnW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgRxIdEnW::new(self, 16)
    }
    #[doc = "Bit 20 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_en(&mut self) -> TzcSfTzsrgR0EnW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR0EnW::new(self, 20)
    }
    #[doc = "Bit 21 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_en(&mut self) -> TzcSfTzsrgR1EnW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR1EnW::new(self, 21)
    }
    #[doc = "Bit 22 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_en(&mut self) -> TzcSfTzsrgR2EnW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR2EnW::new(self, 22)
    }
    #[doc = "Bit 23 - Region 3 enable"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_en(&mut self) -> TzcSfTzsrgR3EnW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR3EnW::new(self, 23)
    }
    #[doc = "Bit 24 - Region X (reserved) enable"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_en(&mut self) -> TzcSfTzsrgRxEnW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgRxEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r0_lock(&mut self) -> TzcSfTzsrgR0LockW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR0LockW::new(self, 25)
    }
    #[doc = "Bit 26 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_lock(&mut self) -> TzcSfTzsrgR1LockW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR1LockW::new(self, 26)
    }
    #[doc = "Bit 27 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_lock(&mut self) -> TzcSfTzsrgR2LockW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR2LockW::new(self, 27)
    }
    #[doc = "Bit 28 - Region 3 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_lock(&mut self) -> TzcSfTzsrgR3LockW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgR3LockW::new(self, 28)
    }
    #[doc = "Bit 29 - Region X (reserved) lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_rx_lock(&mut self) -> TzcSfTzsrgRxLockW<'_, TzcSfTzsrgCtrlSpec> {
        TzcSfTzsrgRxLockW::new(self, 29)
    }
}
#[doc = "Serial Flash (XIP) Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcSfTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_sf_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcSfTzsrgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_SF_TZSRG_CTRL to value 0x000f_ffff"]
impl crate::Resettable for TzcSfTzsrgCtrlSpec {
    const RESET_VALUE: u32 = 0x000f_ffff;
}
