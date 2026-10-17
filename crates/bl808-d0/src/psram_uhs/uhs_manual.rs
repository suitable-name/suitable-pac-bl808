#[doc = "Register `UHS_MANUAL` reader"]
pub type R = crate::R<UhsManualSpec>;
#[doc = "Register `UHS_MANUAL` writer"]
pub type W = crate::W<UhsManualSpec>;
#[doc = "Field `reg_force_ceb_low` reader - Force CEB low"]
pub type RegForceCebLowR = crate::BitReader;
#[doc = "Field `reg_force_ceb_low` writer - Force CEB low"]
pub type RegForceCebLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_force_ceb_high` reader - Force CEB high"]
pub type RegForceCebHighR = crate::BitReader;
#[doc = "Field `reg_force_ceb_high` writer - Force CEB high"]
pub type RegForceCebHighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_psram_resetb` reader - Manual PSRAM Reset (Active Low)"]
pub type RegPsramResetbR = crate::BitReader;
#[doc = "Field `reg_psram_resetb` writer - Manual PSRAM Reset (Active Low)"]
pub type RegPsramResetbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_x16_mode` reader - Enable UHS X16 mode"]
pub type RegX16ModeR = crate::BitReader;
#[doc = "Field `reg_x16_mode` writer - Enable UHS X16 mode"]
pub type RegX16ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_wrap2incr_en` reader - Wrap to Incremental burst conversion enable"]
pub type RegWrap2incrEnR = crate::BitReader;
#[doc = "Field `reg_wrap2incr_en` writer - Wrap to Incremental burst conversion enable"]
pub type RegWrap2incrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pck_s_div` reader - Clock divider for S-phase"]
pub type RegPckSDivR = crate::FieldReader;
#[doc = "Field `reg_pck_s_div` writer - Clock divider for S-phase"]
pub type RegPckSDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_pck_t_div` reader - Clock divider for T-phase"]
pub type RegPckTDivR = crate::FieldReader;
#[doc = "Field `reg_pck_t_div` writer - Clock divider for T-phase"]
pub type RegPckTDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Force CEB low"]
    #[inline(always)]
    pub fn reg_force_ceb_low(&self) -> RegForceCebLowR {
        RegForceCebLowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force CEB high"]
    #[inline(always)]
    pub fn reg_force_ceb_high(&self) -> RegForceCebHighR {
        RegForceCebHighR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Manual PSRAM Reset (Active Low)"]
    #[inline(always)]
    pub fn reg_psram_resetb(&self) -> RegPsramResetbR {
        RegPsramResetbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable UHS X16 mode"]
    #[inline(always)]
    pub fn reg_x16_mode(&self) -> RegX16ModeR {
        RegX16ModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wrap to Incremental burst conversion enable"]
    #[inline(always)]
    pub fn reg_wrap2incr_en(&self) -> RegWrap2incrEnR {
        RegWrap2incrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Clock divider for S-phase"]
    #[inline(always)]
    pub fn reg_pck_s_div(&self) -> RegPckSDivR {
        RegPckSDivR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:31 - Clock divider for T-phase"]
    #[inline(always)]
    pub fn reg_pck_t_div(&self) -> RegPckTDivR {
        RegPckTDivR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force CEB low"]
    #[inline(always)]
    pub fn reg_force_ceb_low(&mut self) -> RegForceCebLowW<'_, UhsManualSpec> {
        RegForceCebLowW::new(self, 0)
    }
    #[doc = "Bit 1 - Force CEB high"]
    #[inline(always)]
    pub fn reg_force_ceb_high(&mut self) -> RegForceCebHighW<'_, UhsManualSpec> {
        RegForceCebHighW::new(self, 1)
    }
    #[doc = "Bit 2 - Manual PSRAM Reset (Active Low)"]
    #[inline(always)]
    pub fn reg_psram_resetb(&mut self) -> RegPsramResetbW<'_, UhsManualSpec> {
        RegPsramResetbW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable UHS X16 mode"]
    #[inline(always)]
    pub fn reg_x16_mode(&mut self) -> RegX16ModeW<'_, UhsManualSpec> {
        RegX16ModeW::new(self, 3)
    }
    #[doc = "Bit 4 - Wrap to Incremental burst conversion enable"]
    #[inline(always)]
    pub fn reg_wrap2incr_en(&mut self) -> RegWrap2incrEnW<'_, UhsManualSpec> {
        RegWrap2incrEnW::new(self, 4)
    }
    #[doc = "Bits 16:18 - Clock divider for S-phase"]
    #[inline(always)]
    pub fn reg_pck_s_div(&mut self) -> RegPckSDivW<'_, UhsManualSpec> {
        RegPckSDivW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clock divider for T-phase"]
    #[inline(always)]
    pub fn reg_pck_t_div(&mut self) -> RegPckTDivW<'_, UhsManualSpec> {
        RegPckTDivW::new(self, 24)
    }
}
#[doc = "UHS Manual Control and Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_manual::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_manual::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsManualSpec;
impl crate::RegisterSpec for UhsManualSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_manual::R`](R) reader structure"]
impl crate::Readable for UhsManualSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_manual::W`](W) writer structure"]
impl crate::Writable for UhsManualSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_MANUAL to value 0x4000_001c"]
impl crate::Resettable for UhsManualSpec {
    const RESET_VALUE: u32 = 0x4000_001c;
}
