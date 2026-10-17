#[doc = "Register `TZC_XRAM_TZSRG_CTRL` reader"]
pub type R = crate::R<TzcXramTzsrgCtrlSpec>;
#[doc = "Register `TZC_XRAM_TZSRG_CTRL` writer"]
pub type W = crate::W<TzcXramTzsrgCtrlSpec>;
#[doc = "Field `tzc_xram_tzsrg_r0_id_en` reader - Region 0 TZID permission map"]
pub type TzcXramTzsrgR0IdEnR = crate::FieldReader;
#[doc = "Field `tzc_xram_tzsrg_r0_id_en` writer - Region 0 TZID permission map"]
pub type TzcXramTzsrgR0IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_xram_tzsrg_r1_id_en` reader - Region 1 TZID permission map"]
pub type TzcXramTzsrgR1IdEnR = crate::FieldReader;
#[doc = "Field `tzc_xram_tzsrg_r1_id_en` writer - Region 1 TZID permission map"]
pub type TzcXramTzsrgR1IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_xram_tzsrg_r2_id_en` reader - Region 2 TZID permission map"]
pub type TzcXramTzsrgR2IdEnR = crate::FieldReader;
#[doc = "Field `tzc_xram_tzsrg_r2_id_en` writer - Region 2 TZID permission map"]
pub type TzcXramTzsrgR2IdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_xram_tzsrg_r0_en` reader - Region 0 enable"]
pub type TzcXramTzsrgR0EnR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r0_en` writer - Region 0 enable"]
pub type TzcXramTzsrgR0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_xram_tzsrg_r1_en` reader - Region 1 enable"]
pub type TzcXramTzsrgR1EnR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r1_en` writer - Region 1 enable"]
pub type TzcXramTzsrgR1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_xram_tzsrg_r2_en` reader - Region 2 enable"]
pub type TzcXramTzsrgR2EnR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r2_en` writer - Region 2 enable"]
pub type TzcXramTzsrgR2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_xram_tzsrg_r0_lock` reader - Region 0 lock (Write-Once)"]
pub type TzcXramTzsrgR0LockR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r0_lock` writer - Region 0 lock (Write-Once)"]
pub type TzcXramTzsrgR0LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_xram_tzsrg_r1_lock` reader - Region 1 lock (Write-Once)"]
pub type TzcXramTzsrgR1LockR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r1_lock` writer - Region 1 lock (Write-Once)"]
pub type TzcXramTzsrgR1LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_xram_tzsrg_r2_lock` reader - Region 2 lock (Write-Once)"]
pub type TzcXramTzsrgR2LockR = crate::BitReader;
#[doc = "Field `tzc_xram_tzsrg_r2_lock` writer - Region 2 lock (Write-Once)"]
pub type TzcXramTzsrgR2LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_id_en(&self) -> TzcXramTzsrgR0IdEnR {
        TzcXramTzsrgR0IdEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_id_en(&self) -> TzcXramTzsrgR1IdEnR {
        TzcXramTzsrgR1IdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_id_en(&self) -> TzcXramTzsrgR2IdEnR {
        TzcXramTzsrgR2IdEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_en(&self) -> TzcXramTzsrgR0EnR {
        TzcXramTzsrgR0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_en(&self) -> TzcXramTzsrgR1EnR {
        TzcXramTzsrgR1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_en(&self) -> TzcXramTzsrgR2EnR {
        TzcXramTzsrgR2EnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_lock(&self) -> TzcXramTzsrgR0LockR {
        TzcXramTzsrgR0LockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_lock(&self) -> TzcXramTzsrgR1LockR {
        TzcXramTzsrgR1LockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_lock(&self) -> TzcXramTzsrgR2LockR {
        TzcXramTzsrgR2LockR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Region 0 TZID permission map"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_id_en(&mut self) -> TzcXramTzsrgR0IdEnW<'_, TzcXramTzsrgCtrlSpec> {
        TzcXramTzsrgR0IdEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Region 1 TZID permission map"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_id_en(&mut self) -> TzcXramTzsrgR1IdEnW<'_, TzcXramTzsrgCtrlSpec> {
        TzcXramTzsrgR1IdEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Region 2 TZID permission map"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_id_en(&mut self) -> TzcXramTzsrgR2IdEnW<'_, TzcXramTzsrgCtrlSpec> {
        TzcXramTzsrgR2IdEnW::new(self, 4)
    }
    #[doc = "Bit 16 - Region 0 enable"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_en(&mut self) -> TzcXramTzsrgR0EnW<'_, TzcXramTzsrgCtrlSpec> {
        TzcXramTzsrgR0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - Region 1 enable"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_en(&mut self) -> TzcXramTzsrgR1EnW<'_, TzcXramTzsrgCtrlSpec> {
        TzcXramTzsrgR1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - Region 2 enable"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_en(&mut self) -> TzcXramTzsrgR2EnW<'_, TzcXramTzsrgCtrlSpec> {
        TzcXramTzsrgR2EnW::new(self, 18)
    }
    #[doc = "Bit 24 - Region 0 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r0_lock(&mut self) -> TzcXramTzsrgR0LockW<'_, TzcXramTzsrgCtrlSpec> {
        TzcXramTzsrgR0LockW::new(self, 24)
    }
    #[doc = "Bit 25 - Region 1 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r1_lock(&mut self) -> TzcXramTzsrgR1LockW<'_, TzcXramTzsrgCtrlSpec> {
        TzcXramTzsrgR1LockW::new(self, 25)
    }
    #[doc = "Bit 26 - Region 2 lock (Write-Once)"]
    #[inline(always)]
    pub fn tzc_xram_tzsrg_r2_lock(&mut self) -> TzcXramTzsrgR2LockW<'_, TzcXramTzsrgCtrlSpec> {
        TzcXramTzsrgR2LockW::new(self, 26)
    }
}
#[doc = "XRAM Security Region Generator Control\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_xram_tzsrg_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_xram_tzsrg_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcXramTzsrgCtrlSpec;
impl crate::RegisterSpec for TzcXramTzsrgCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_xram_tzsrg_ctrl::R`](R) reader structure"]
impl crate::Readable for TzcXramTzsrgCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_xram_tzsrg_ctrl::W`](W) writer structure"]
impl crate::Writable for TzcXramTzsrgCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_XRAM_TZSRG_CTRL to value 0x3f"]
impl crate::Resettable for TzcXramTzsrgCtrlSpec {
    const RESET_VALUE: u32 = 0x3f;
}
