#[doc = "Register `TZC_PSRAMA_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcPsramaTzsrgCtrlSpec>;
#[doc = "Register `TZC_PSRAMA_TZSRG_CTRL` writer"]
pub type W = crate::W<TzcPsramaTzsrgCtrlSpec>;
#[doc = "Field `tzc_psrama_tzsrg_r0_id_en` reader - Region 0 TZID permission map"]
pub type TzcPsramaTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psrama_tzsrg_r0_id_en` writer - Region 0 TZID permission map"]
pub type TzcPsramaTzsrgR0IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_psrama_tzsrg_r1_id_en` reader - Region 1 TZID permission map"]
pub type TzcPsramaTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psrama_tzsrg_r1_id_en` writer - Region 1 TZID permission map"]
pub type TzcPsramaTzsrgR1IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_psrama_tzsrg_r2_id_en` reader - Region 2 TZID permission map"]
pub type TzcPsramaTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_psrama_tzsrg_r2_id_en` writer - Region 2 TZID permission map"]
pub type TzcPsramaTzsrgR2IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_psrama_tzsrg_r0_en` reader - Region 0 enable"]
pub type TzcPsramaTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r0_en` writer - Region 0 enable"]
pub type TzcPsramaTzsrgR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_psrama_tzsrg_r1_en` reader - Region 1 enable"]
pub type TzcPsramaTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r1_en` writer - Region 1 enable"]
pub type TzcPsramaTzsrgR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_psrama_tzsrg_r2_en` reader - Region 2 enable"]
pub type TzcPsramaTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r2_en` writer - Region 2 enable"]
pub type TzcPsramaTzsrgR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_psrama_tzsrg_r0_lock` reader - Region 0 lock (Write-Once)"]
pub type TzcPsramaTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r0_lock` writer - Region 0 lock (Write-Once)"]
pub type TzcPsramaTzsrgR0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_psrama_tzsrg_r1_lock` reader - Region 1 lock (Write-Once)"]
pub type TzcPsramaTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r1_lock` writer - Region 1 lock (Write-Once)"]
pub type TzcPsramaTzsrgR1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_psrama_tzsrg_r2_lock` reader - Region 2 lock (Write-Once)"]
pub type TzcPsramaTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_psrama_tzsrg_r2_lock` writer - Region 2 lock (Write-Once)"]
pub type TzcPsramaTzsrgR2LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r0_id_en(&self) -> TzcPsramaTzsrgR0IdEnR {
        TzcPsramaTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_id_en(&self) -> TzcPsramaTzsrgR1IdEnR {
        TzcPsramaTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_id_en(&self) -> TzcPsramaTzsrgR2IdEnR {
        TzcPsramaTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r0_en(&self) -> TzcPsramaTzsrgR0EnR {
        TzcPsramaTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_en(&self) -> TzcPsramaTzsrgR1EnR {
        TzcPsramaTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_en(&self) -> TzcPsramaTzsrgR2EnR {
        TzcPsramaTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r0_lock(&self) -> TzcPsramaTzsrgR0LockR {
        TzcPsramaTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_lock(&self) -> TzcPsramaTzsrgR1LockR {
        TzcPsramaTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_lock(&self) -> TzcPsramaTzsrgR2LockR {
        TzcPsramaTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r0_id_en(
        &mut self,
    ) -> TzcPsramaTzsrgR0IdEnW<'_, TzcPsramaTzsrgCtrlSpec> {
        TzcPsramaTzsrgR0IdEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_id_en(
        &mut self,
    ) -> TzcPsramaTzsrgR1IdEnW<'_, TzcPsramaTzsrgCtrlSpec> {
        TzcPsramaTzsrgR1IdEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_id_en(
        &mut self,
    ) -> TzcPsramaTzsrgR2IdEnW<'_, TzcPsramaTzsrgCtrlSpec> {
        TzcPsramaTzsrgR2IdEnW::new(self, 4)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r0_en(&mut self) -> TzcPsramaTzsrgR0EnW<'_, TzcPsramaTzsrgCtrlSpec> {
        TzcPsramaTzsrgR0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_en(&mut self) -> TzcPsramaTzsrgR1EnW<'_, TzcPsramaTzsrgCtrlSpec> {
        TzcPsramaTzsrgR1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_en(&mut self) -> TzcPsramaTzsrgR2EnW<'_, TzcPsramaTzsrgCtrlSpec> {
        TzcPsramaTzsrgR2EnW::new(self, 18)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r0_lock(
        &mut self,
    ) -> TzcPsramaTzsrgR0LockW<'_, TzcPsramaTzsrgCtrlSpec> {
        TzcPsramaTzsrgR0LockW::new(self, 24)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r1_lock(
        &mut self,
    ) -> TzcPsramaTzsrgR1LockW<'_, TzcPsramaTzsrgCtrlSpec> {
        TzcPsramaTzsrgR1LockW::new(self, 25)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_psrama_tzsrg_r2_lock(
        &mut self,
    ) -> TzcPsramaTzsrgR2LockW<'_, TzcPsramaTzsrgCtrlSpec> {
        TzcPsramaTzsrgR2LockW::new(self, 26)
    }
}
#[doc = "PSRAM-A Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_psrama_tzsrg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_psrama_tzsrg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcPsramaTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcPsramaTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_psrama_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcPsramaTzsrgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_psrama_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcPsramaTzsrgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_PSRAMA_TZSRG_CTRL to value 0x3f"]
impl crate::Resettable for TzcPsramaTzsrgCtrlSpec {
    const RESET_VALUE: u32 = 0x3f;
}
