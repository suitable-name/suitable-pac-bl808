#[doc = "Register `TZC_OCRAM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcOcramTzsrgCtrlSpec>;
#[doc = "Register `TZC_OCRAM_TZSRG_CTRL` writer"]
pub type W = crate::W<TzcOcramTzsrgCtrlSpec>;
#[doc = "Field `tzc_ocram_tzsrg_r0_id_en` reader - Region 0 TZID permission map (4 bits = 4 masters)"]
pub type TzcOcramTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_ocram_tzsrg_r0_id_en` writer - Region 0 TZID permission map (4 bits = 4 masters)"]
pub type TzcOcramTzsrgR0IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_ocram_tzsrg_r1_id_en` reader - Region 1 TZID permission map"]
pub type TzcOcramTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_ocram_tzsrg_r1_id_en` writer - Region 1 TZID permission map"]
pub type TzcOcramTzsrgR1IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_ocram_tzsrg_r2_id_en` reader - Region 2 TZID permission map"]
pub type TzcOcramTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_ocram_tzsrg_r2_id_en` writer - Region 2 TZID permission map"]
pub type TzcOcramTzsrgR2IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_ocram_tzsrg_rx_id_en` reader - Region X (reserved) TZID permission map"]
pub type TzcOcramTzsrgRxIdEnR = crate::FieldReader;
#[doc = "Field `tzc_ocram_tzsrg_rx_id_en` writer - Region X (reserved) TZID permission map"]
pub type TzcOcramTzsrgRxIdEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `tzc_ocram_tzsrg_r0_en` reader - Region 0 enable"]
pub type TzcOcramTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r0_en` writer - Region 0 enable"]
pub type TzcOcramTzsrgR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_ocram_tzsrg_r1_en` reader - Region 1 enable"]
pub type TzcOcramTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r1_en` writer - Region 1 enable"]
pub type TzcOcramTzsrgR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_ocram_tzsrg_r2_en` reader - Region 2 enable"]
pub type TzcOcramTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r2_en` writer - Region 2 enable"]
pub type TzcOcramTzsrgR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_ocram_tzsrg_rx_en` reader - Region X (reserved) enable"]
pub type TzcOcramTzsrgRxEnR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_rx_en` writer - Region X (reserved) enable"]
pub type TzcOcramTzsrgRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_ocram_tzsrg_r0_lock` reader - Region 0 lock (Write-Once)"]
pub type TzcOcramTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r0_lock` writer - Region 0 lock (Write-Once)"]
pub type TzcOcramTzsrgR0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_ocram_tzsrg_r1_lock` reader - Region 1 lock (Write-Once)"]
pub type TzcOcramTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r1_lock` writer - Region 1 lock (Write-Once)"]
pub type TzcOcramTzsrgR1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_ocram_tzsrg_r2_lock` reader - Region 2 lock (Write-Once)"]
pub type TzcOcramTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_r2_lock` writer - Region 2 lock (Write-Once)"]
pub type TzcOcramTzsrgR2LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_ocram_tzsrg_rx_lock` reader - Region X (reserved) lock (Write-Once)"]
pub type TzcOcramTzsrgRxLockR = crate::BitReader;
#[doc = "Field `tzc_ocram_tzsrg_rx_lock` writer - Region X (reserved) lock (Write-Once)"]
pub type TzcOcramTzsrgRxLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Region 0 TZID permission map (4 bits = 4 masters)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_id_en(&self) -> TzcOcramTzsrgR0IdEnR {
        TzcOcramTzsrgR0IdEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_id_en(&self) -> TzcOcramTzsrgR1IdEnR {
        TzcOcramTzsrgR1IdEnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_id_en(&self) -> TzcOcramTzsrgR2IdEnR {
        TzcOcramTzsrgR2IdEnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region X (reserved) TZID permission map"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_id_en(&self) -> TzcOcramTzsrgRxIdEnR {
        TzcOcramTzsrgRxIdEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_en(&self) -> TzcOcramTzsrgR0EnR {
        TzcOcramTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_en(&self) -> TzcOcramTzsrgR1EnR {
        TzcOcramTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_en(&self) -> TzcOcramTzsrgR2EnR {
        TzcOcramTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Region X (reserved) enable"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_en(&self) -> TzcOcramTzsrgRxEnR {
        TzcOcramTzsrgRxEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_lock(&self) -> TzcOcramTzsrgR0LockR {
        TzcOcramTzsrgR0LockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_lock(&self) -> TzcOcramTzsrgR1LockR {
        TzcOcramTzsrgR1LockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_lock(&self) -> TzcOcramTzsrgR2LockR {
        TzcOcramTzsrgR2LockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Region X (reserved) lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_lock(&self) -> TzcOcramTzsrgRxLockR {
        TzcOcramTzsrgRxLockR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Region 0 TZID permission map (4 bits = 4 masters)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_id_en(&mut self) -> TzcOcramTzsrgR0IdEnW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgR0IdEnW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_id_en(&mut self) -> TzcOcramTzsrgR1IdEnW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgR1IdEnW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_id_en(&mut self) -> TzcOcramTzsrgR2IdEnW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgR2IdEnW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Region X (reserved) TZID permission map"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_id_en(&mut self) -> TzcOcramTzsrgRxIdEnW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgRxIdEnW::new(self, 12)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_en(&mut self) -> TzcOcramTzsrgR0EnW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgR0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_en(&mut self) -> TzcOcramTzsrgR1EnW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgR1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_en(&mut self) -> TzcOcramTzsrgR2EnW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgR2EnW::new(self, 18)
    }
    #[doc = "Bit 19 - Region X (reserved) enable"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_en(&mut self) -> TzcOcramTzsrgRxEnW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgRxEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r0_lock(&mut self) -> TzcOcramTzsrgR0LockW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgR0LockW::new(self, 20)
    }
    #[doc = "Bit 21 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r1_lock(&mut self) -> TzcOcramTzsrgR1LockW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgR1LockW::new(self, 21)
    }
    #[doc = "Bit 22 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_lock(&mut self) -> TzcOcramTzsrgR2LockW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgR2LockW::new(self, 22)
    }
    #[doc = "Bit 23 - Region X (reserved) lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_rx_lock(&mut self) -> TzcOcramTzsrgRxLockW<'_, TzcOcramTzsrgCtrlSpec> {
        TzcOcramTzsrgRxLockW::new(self, 23)
    }
}
#[doc = "OCRAM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_ocram_tzsrg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcOcramTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcOcramTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_ocram_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcOcramTzsrgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_ocram_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcOcramTzsrgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_OCRAM_TZSRG_CTRL to value 0xffff"]
impl crate::Resettable for TzcOcramTzsrgCtrlSpec {
    const RESET_VALUE: u32 = 0xffff;
}
