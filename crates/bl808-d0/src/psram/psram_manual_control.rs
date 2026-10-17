#[doc = "Register `PSRAM_MANUAL_CONTROL` reader"]
pub type R = crate::R<PsramManualControlSpec>;
#[doc = "Register `PSRAM_MANUAL_CONTROL` writer"]
pub type W = crate::W<PsramManualControlSpec>;
#[doc = "Field `reg_wc_sw` reader - Write command software value"]
pub type RegWcSwR = crate::FieldReader;
#[doc = "Field `reg_wc_sw` writer - Write command software value"]
pub type RegWcSwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `reg_wc_sw_en` reader - Write command software enable"]
pub type RegWcSwEnR = crate::BitReader;
#[doc = "Field `reg_wc_sw_en` writer - Write command software enable"]
pub type RegWcSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_state_hold_tick` reader - Hold clock ticks during state hold"]
pub type RegStateHoldTickR = crate::BitReader;
#[doc = "Field `reg_state_hold_tick` writer - Hold clock ticks during state hold"]
pub type RegStateHoldTickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dqs_latch_inv` reader - DQS latch invert enable"]
pub type RegDqsLatchInvR = crate::BitReader;
#[doc = "Field `reg_dqs_latch_inv` writer - DQS latch invert enable"]
pub type RegDqsLatchInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_wb_bl2_mask` reader - Winbond burst length 2 mask"]
pub type RegWbBl2MaskR = crate::BitReader;
#[doc = "Field `reg_wb_bl2_mask` writer - Winbond burst length 2 mask"]
pub type RegWbBl2MaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_force_ceb_low` reader - Force CEB (Chip Enable) low"]
pub type RegForceCebLowR = crate::BitReader;
#[doc = "Field `reg_force_ceb_low` writer - Force CEB (Chip Enable) low"]
pub type RegForceCebLowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_force_ceb_high` reader - Force CEB (Chip Enable) high"]
pub type RegForceCebHighR = crate::BitReader;
#[doc = "Field `reg_force_ceb_high` writer - Force CEB (Chip Enable) high"]
pub type RegForceCebHighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_psram_resetb` reader - PSRAM reset (Active Low)"]
pub type RegPsramResetbR = crate::BitReader;
#[doc = "Field `reg_psram_resetb` writer - PSRAM reset (Active Low)"]
pub type RegPsramResetbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ck_edge_nali` reader - Clock edge non-alignment control"]
pub type RegCkEdgeNaliR = crate::BitReader;
#[doc = "Field `reg_ck_edge_nali` writer - Clock edge non-alignment control"]
pub type RegCkEdgeNaliW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_config_read` reader - Read configuration status word"]
pub type StsConfigReadR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:6 - Write command software value"]
    #[inline(always)]
    pub fn reg_wc_sw(&self) -> RegWcSwR {
        RegWcSwR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Write command software enable"]
    #[inline(always)]
    pub fn reg_wc_sw_en(&self) -> RegWcSwEnR {
        RegWcSwEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Hold clock ticks during state hold"]
    #[inline(always)]
    pub fn reg_state_hold_tick(&self) -> RegStateHoldTickR {
        RegStateHoldTickR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DQS latch invert enable"]
    #[inline(always)]
    pub fn reg_dqs_latch_inv(&self) -> RegDqsLatchInvR {
        RegDqsLatchInvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Winbond burst length 2 mask"]
    #[inline(always)]
    pub fn reg_wb_bl2_mask(&self) -> RegWbBl2MaskR {
        RegWbBl2MaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Force CEB (Chip Enable) low"]
    #[inline(always)]
    pub fn reg_force_ceb_low(&self) -> RegForceCebLowR {
        RegForceCebLowR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Force CEB (Chip Enable) high"]
    #[inline(always)]
    pub fn reg_force_ceb_high(&self) -> RegForceCebHighR {
        RegForceCebHighR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PSRAM reset (Active Low)"]
    #[inline(always)]
    pub fn reg_psram_resetb(&self) -> RegPsramResetbR {
        RegPsramResetbR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock edge non-alignment control"]
    #[inline(always)]
    pub fn reg_ck_edge_nali(&self) -> RegCkEdgeNaliR {
        RegCkEdgeNaliR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Read configuration status word"]
    #[inline(always)]
    pub fn sts_config_read(&self) -> StsConfigReadR {
        StsConfigReadR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Write command software value"]
    #[inline(always)]
    pub fn reg_wc_sw(&mut self) -> RegWcSwW<'_, PsramManualControlSpec> {
        RegWcSwW::new(self, 0)
    }
    #[doc = "Bit 8 - Write command software enable"]
    #[inline(always)]
    pub fn reg_wc_sw_en(&mut self) -> RegWcSwEnW<'_, PsramManualControlSpec> {
        RegWcSwEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Hold clock ticks during state hold"]
    #[inline(always)]
    pub fn reg_state_hold_tick(&mut self) -> RegStateHoldTickW<'_, PsramManualControlSpec> {
        RegStateHoldTickW::new(self, 9)
    }
    #[doc = "Bit 10 - DQS latch invert enable"]
    #[inline(always)]
    pub fn reg_dqs_latch_inv(&mut self) -> RegDqsLatchInvW<'_, PsramManualControlSpec> {
        RegDqsLatchInvW::new(self, 10)
    }
    #[doc = "Bit 11 - Winbond burst length 2 mask"]
    #[inline(always)]
    pub fn reg_wb_bl2_mask(&mut self) -> RegWbBl2MaskW<'_, PsramManualControlSpec> {
        RegWbBl2MaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Force CEB (Chip Enable) low"]
    #[inline(always)]
    pub fn reg_force_ceb_low(&mut self) -> RegForceCebLowW<'_, PsramManualControlSpec> {
        RegForceCebLowW::new(self, 12)
    }
    #[doc = "Bit 13 - Force CEB (Chip Enable) high"]
    #[inline(always)]
    pub fn reg_force_ceb_high(&mut self) -> RegForceCebHighW<'_, PsramManualControlSpec> {
        RegForceCebHighW::new(self, 13)
    }
    #[doc = "Bit 14 - PSRAM reset (Active Low)"]
    #[inline(always)]
    pub fn reg_psram_resetb(&mut self) -> RegPsramResetbW<'_, PsramManualControlSpec> {
        RegPsramResetbW::new(self, 14)
    }
    #[doc = "Bit 15 - Clock edge non-alignment control"]
    #[inline(always)]
    pub fn reg_ck_edge_nali(&mut self) -> RegCkEdgeNaliW<'_, PsramManualControlSpec> {
        RegCkEdgeNaliW::new(self, 15)
    }
}
#[doc = "PSRAM Manual Signal Control\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_manual_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_manual_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramManualControlSpec;
impl crate::RegisterSpec for PsramManualControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_manual_control::R`](R) reader structure"]
impl crate::Readable for PsramManualControlSpec {}
#[doc = "`write(|w| ..)` method takes [`psram_manual_control::W`](W) writer structure"]
impl crate::Writable for PsramManualControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_MANUAL_CONTROL to value 0x4800"]
impl crate::Resettable for PsramManualControlSpec {
    const RESET_VALUE: u32 = 0x4800;
}
