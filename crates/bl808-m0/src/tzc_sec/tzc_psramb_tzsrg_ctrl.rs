#[doc = "Register `TZC_PSRAMB_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcPsrambTzsrgCtrlSpec>;
#[doc = "Register `TZC_PSRAMB_TZSRG_CTRL` writer"]
pub type W = crate::W<TzcPsrambTzsrgCtrlSpec>;
#[doc = "Field `tzc_psramb_tzsrg_r0_id_en` reader - Region 0 TZID permission map"]
pub type TzcPsrambTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psramb_tzsrg_r0_id_en` writer - Region 0 TZID permission map"]
pub type TzcPsrambTzsrgR0IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_psramb_tzsrg_r1_id_en` reader - Region 1 TZID permission map"]
pub type TzcPsrambTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psramb_tzsrg_r1_id_en` writer - Region 1 TZID permission map"]
pub type TzcPsrambTzsrgR1IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_psramb_tzsrg_r2_id_en` reader - Region 2 TZID permission map"]
pub type TzcPsrambTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psramb_tzsrg_r2_id_en` writer - Region 2 TZID permission map"]
pub type TzcPsrambTzsrgR2IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_psramb_tzsrg_r0_en` reader - Region 0 enable"]
pub type TzcPsrambTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r0_en` writer - Region 0 enable"]
pub type TzcPsrambTzsrgR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_psramb_tzsrg_r1_en` reader - Region 1 enable"]
pub type TzcPsrambTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r1_en` writer - Region 1 enable"]
pub type TzcPsrambTzsrgR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_psramb_tzsrg_r2_en` reader - Region 2 enable"]
pub type TzcPsrambTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r2_en` writer - Region 2 enable"]
pub type TzcPsrambTzsrgR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_psramb_tzsrg_r0_lock` reader - Region 0 lock (Write-Once)"]
pub type TzcPsrambTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r0_lock` writer - Region 0 lock (Write-Once)"]
pub type TzcPsrambTzsrgR0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_psramb_tzsrg_r1_lock` reader - Region 1 lock (Write-Once)"]
pub type TzcPsrambTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r1_lock` writer - Region 1 lock (Write-Once)"]
pub type TzcPsrambTzsrgR1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_psramb_tzsrg_r2_lock` reader - Region 2 lock (Write-Once)"]
pub type TzcPsrambTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_psramb_tzsrg_r2_lock` writer - Region 2 lock (Write-Once)"]
pub type TzcPsrambTzsrgR2LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_id_en(&self) -> TzcPsrambTzsrgR0IdEnR {
        TzcPsrambTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_id_en(&self) -> TzcPsrambTzsrgR1IdEnR {
        TzcPsrambTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_id_en(&self) -> TzcPsrambTzsrgR2IdEnR {
        TzcPsrambTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_en(&self) -> TzcPsrambTzsrgR0EnR {
        TzcPsrambTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_en(&self) -> TzcPsrambTzsrgR1EnR {
        TzcPsrambTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_en(&self) -> TzcPsrambTzsrgR2EnR {
        TzcPsrambTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_lock(&self) -> TzcPsrambTzsrgR0LockR {
        TzcPsrambTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_lock(&self) -> TzcPsrambTzsrgR1LockR {
        TzcPsrambTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_lock(&self) -> TzcPsrambTzsrgR2LockR {
        TzcPsrambTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_id_en(
        &mut self,
    ) -> TzcPsrambTzsrgR0IdEnW<'_, TzcPsrambTzsrgCtrlSpec> {
        TzcPsrambTzsrgR0IdEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_id_en(
        &mut self,
    ) -> TzcPsrambTzsrgR1IdEnW<'_, TzcPsrambTzsrgCtrlSpec> {
        TzcPsrambTzsrgR1IdEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_id_en(
        &mut self,
    ) -> TzcPsrambTzsrgR2IdEnW<'_, TzcPsrambTzsrgCtrlSpec> {
        TzcPsrambTzsrgR2IdEnW::new(self, 4)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_en(&mut self) -> TzcPsrambTzsrgR0EnW<'_, TzcPsrambTzsrgCtrlSpec> {
        TzcPsrambTzsrgR0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_en(&mut self) -> TzcPsrambTzsrgR1EnW<'_, TzcPsrambTzsrgCtrlSpec> {
        TzcPsrambTzsrgR1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_en(&mut self) -> TzcPsrambTzsrgR2EnW<'_, TzcPsrambTzsrgCtrlSpec> {
        TzcPsrambTzsrgR2EnW::new(self, 18)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r0_lock(
        &mut self,
    ) -> TzcPsrambTzsrgR0LockW<'_, TzcPsrambTzsrgCtrlSpec> {
        TzcPsrambTzsrgR0LockW::new(self, 24)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r1_lock(
        &mut self,
    ) -> TzcPsrambTzsrgR1LockW<'_, TzcPsrambTzsrgCtrlSpec> {
        TzcPsrambTzsrgR1LockW::new(self, 25)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psramb_tzsrg_r2_lock(
        &mut self,
    ) -> TzcPsrambTzsrgR2LockW<'_, TzcPsrambTzsrgCtrlSpec> {
        TzcPsrambTzsrgR2LockW::new(self, 26)
    }
}
#[doc = "PSRAM-B Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psramb_tzsrg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsrambTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcPsrambTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psramb_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcPsrambTzsrgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_psramb_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcPsrambTzsrgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PSRAMB_TZSRG_CTRL to value 0x3f"]
impl crate::Resettable for TzcPsrambTzsrgCtrlSpec {
    const RESET_VALUE: u32 = 0x3f;
}
