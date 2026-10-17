#[doc = "Register `TZC_L2SRAM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcL2sramTzsrgCtrlSpec>;
#[doc = "Register `TZC_L2SRAM_TZSRG_CTRL` writer"]
pub type W = crate::W<TzcL2sramTzsrgCtrlSpec>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_id_en` reader - Region 0 TZID permission map"]
pub type TzcL2sramTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_l2sram_tzsrg_r0_id_en` writer - Region 0 TZID permission map"]
pub type TzcL2sramTzsrgR0IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_id_en` reader - Region 1 TZID permission map"]
pub type TzcL2sramTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_l2sram_tzsrg_r1_id_en` writer - Region 1 TZID permission map"]
pub type TzcL2sramTzsrgR1IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_id_en` reader - Region 2 TZID permission map"]
pub type TzcL2sramTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_l2sram_tzsrg_r2_id_en` writer - Region 2 TZID permission map"]
pub type TzcL2sramTzsrgR2IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_en` reader - Region 0 enable"]
pub type TzcL2sramTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r0_en` writer - Region 0 enable"]
pub type TzcL2sramTzsrgR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_en` reader - Region 1 enable"]
pub type TzcL2sramTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r1_en` writer - Region 1 enable"]
pub type TzcL2sramTzsrgR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_en` reader - Region 2 enable"]
pub type TzcL2sramTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r2_en` writer - Region 2 enable"]
pub type TzcL2sramTzsrgR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_l2sram_tzsrg_r0_lock` reader - Region 0 lock (Write-Once)"]
pub type TzcL2sramTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r0_lock` writer - Region 0 lock (Write-Once)"]
pub type TzcL2sramTzsrgR0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_l2sram_tzsrg_r1_lock` reader - Region 1 lock (Write-Once)"]
pub type TzcL2sramTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r1_lock` writer - Region 1 lock (Write-Once)"]
pub type TzcL2sramTzsrgR1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_l2sram_tzsrg_r2_lock` reader - Region 2 lock (Write-Once)"]
pub type TzcL2sramTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_l2sram_tzsrg_r2_lock` writer - Region 2 lock (Write-Once)"]
pub type TzcL2sramTzsrgR2LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_id_en(&self) -> TzcL2sramTzsrgR0IdEnR {
        TzcL2sramTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_id_en(&self) -> TzcL2sramTzsrgR1IdEnR {
        TzcL2sramTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_id_en(&self) -> TzcL2sramTzsrgR2IdEnR {
        TzcL2sramTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_en(&self) -> TzcL2sramTzsrgR0EnR {
        TzcL2sramTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_en(&self) -> TzcL2sramTzsrgR1EnR {
        TzcL2sramTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_en(&self) -> TzcL2sramTzsrgR2EnR {
        TzcL2sramTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_lock(&self) -> TzcL2sramTzsrgR0LockR {
        TzcL2sramTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_lock(&self) -> TzcL2sramTzsrgR1LockR {
        TzcL2sramTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_lock(&self) -> TzcL2sramTzsrgR2LockR {
        TzcL2sramTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_id_en(
        &mut self,
    ) -> TzcL2sramTzsrgR0IdEnW<'_, TzcL2sramTzsrgCtrlSpec> {
        TzcL2sramTzsrgR0IdEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_id_en(
        &mut self,
    ) -> TzcL2sramTzsrgR1IdEnW<'_, TzcL2sramTzsrgCtrlSpec> {
        TzcL2sramTzsrgR1IdEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_id_en(
        &mut self,
    ) -> TzcL2sramTzsrgR2IdEnW<'_, TzcL2sramTzsrgCtrlSpec> {
        TzcL2sramTzsrgR2IdEnW::new(self, 4)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_en(&mut self) -> TzcL2sramTzsrgR0EnW<'_, TzcL2sramTzsrgCtrlSpec> {
        TzcL2sramTzsrgR0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_en(&mut self) -> TzcL2sramTzsrgR1EnW<'_, TzcL2sramTzsrgCtrlSpec> {
        TzcL2sramTzsrgR1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_en(&mut self) -> TzcL2sramTzsrgR2EnW<'_, TzcL2sramTzsrgCtrlSpec> {
        TzcL2sramTzsrgR2EnW::new(self, 18)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r0_lock(
        &mut self,
    ) -> TzcL2sramTzsrgR0LockW<'_, TzcL2sramTzsrgCtrlSpec> {
        TzcL2sramTzsrgR0LockW::new(self, 24)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r1_lock(
        &mut self,
    ) -> TzcL2sramTzsrgR1LockW<'_, TzcL2sramTzsrgCtrlSpec> {
        TzcL2sramTzsrgR1LockW::new(self, 25)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_l2sram_tzsrg_r2_lock(
        &mut self,
    ) -> TzcL2sramTzsrgR2LockW<'_, TzcL2sramTzsrgCtrlSpec> {
        TzcL2sramTzsrgR2LockW::new(self, 26)
    }
}
#[doc = "L2SRAM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_l2sram_tzsrg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_l2sram_tzsrg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcL2sramTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcL2sramTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_l2sram_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcL2sramTzsrgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_l2sram_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcL2sramTzsrgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_L2SRAM_TZSRG_CTRL to value 0x3f"]
impl crate::Resettable for TzcL2sramTzsrgCtrlSpec {
    const RESET_VALUE: u32 = 0x3f;
}
