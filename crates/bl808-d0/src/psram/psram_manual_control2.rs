#[doc = "Register `PSRAM_MANUAL_CONTROL2` reader"]
pub type R = crate::R<PsramManualControl2Spec>;
#[doc = "Register `PSRAM_MANUAL_CONTROL2` writer"]
pub type W = crate::W<PsramManualControl2Spec>;
#[doc = "Field `reg_hold_cycle_sw` reader - Hold cycle software value"]
pub type RegHoldCycleSwR = crate::FieldReader;
#[doc = "Field `reg_hold_cycle_sw` writer - Hold cycle software value"]
pub type RegHoldCycleSwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `reg_hc_sw_en` reader - Hold cycle software enable"]
pub type RegHcSwEnR = crate::BitReader;
#[doc = "Field `reg_hc_sw_en` writer - Hold cycle software enable"]
pub type RegHcSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dqs_rel_val` reader - DQS relative value"]
pub type RegDqsRelValR = crate::FieldReader;
#[doc = "Field `reg_dqs_rel_val` writer - DQS relative value"]
pub type RegDqsRelValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `reg_pwrap_sw_sht_b` reader - PSRAM wrap software shift bit"]
pub type RegPwrapSwShtBR = crate::FieldReader;
#[doc = "Field `reg_pwrap_sw_sht_b` writer - PSRAM wrap software shift bit"]
pub type RegPwrapSwShtBW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_pwrap_sw_en` reader - PSRAM wrap software enable"]
pub type RegPwrapSwEnR = crate::BitReader;
#[doc = "Field `reg_pwrap_sw_en` writer - PSRAM wrap software enable"]
pub type RegPwrapSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_addr_mask` reader - Address mask bits"]
pub type RegAddrMaskR = crate::FieldReader;
#[doc = "Field `reg_addr_mask` writer - Address mask bits"]
pub type RegAddrMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - Hold cycle software value"]
    #[inline(always)]
    pub fn reg_hold_cycle_sw(&self) -> RegHoldCycleSwR {
        RegHoldCycleSwR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Hold cycle software enable"]
    #[inline(always)]
    pub fn reg_hc_sw_en(&self) -> RegHcSwEnR {
        RegHcSwEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - DQS relative value"]
    #[inline(always)]
    pub fn reg_dqs_rel_val(&self) -> RegDqsRelValR {
        RegDqsRelValR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - PSRAM wrap software shift bit"]
    #[inline(always)]
    pub fn reg_pwrap_sw_sht_b(&self) -> RegPwrapSwShtBR {
        RegPwrapSwShtBR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - PSRAM wrap software enable"]
    #[inline(always)]
    pub fn reg_pwrap_sw_en(&self) -> RegPwrapSwEnR {
        RegPwrapSwEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address mask bits"]
    #[inline(always)]
    pub fn reg_addr_mask(&self) -> RegAddrMaskR {
        RegAddrMaskR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Hold cycle software value"]
    #[inline(always)]
    pub fn reg_hold_cycle_sw(&mut self) -> RegHoldCycleSwW<'_, PsramManualControl2Spec> {
        RegHoldCycleSwW::new(self, 0)
    }
    #[doc = "Bit 7 - Hold cycle software enable"]
    #[inline(always)]
    pub fn reg_hc_sw_en(&mut self) -> RegHcSwEnW<'_, PsramManualControl2Spec> {
        RegHcSwEnW::new(self, 7)
    }
    #[doc = "Bits 8:14 - DQS relative value"]
    #[inline(always)]
    pub fn reg_dqs_rel_val(&mut self) -> RegDqsRelValW<'_, PsramManualControl2Spec> {
        RegDqsRelValW::new(self, 8)
    }
    #[doc = "Bits 16:19 - PSRAM wrap software shift bit"]
    #[inline(always)]
    pub fn reg_pwrap_sw_sht_b(&mut self) -> RegPwrapSwShtBW<'_, PsramManualControl2Spec> {
        RegPwrapSwShtBW::new(self, 16)
    }
    #[doc = "Bit 23 - PSRAM wrap software enable"]
    #[inline(always)]
    pub fn reg_pwrap_sw_en(&mut self) -> RegPwrapSwEnW<'_, PsramManualControl2Spec> {
        RegPwrapSwEnW::new(self, 23)
    }
    #[doc = "Bits 24:31 - Address mask bits"]
    #[inline(always)]
    pub fn reg_addr_mask(&mut self) -> RegAddrMaskW<'_, PsramManualControl2Spec> {
        RegAddrMaskW::new(self, 24)
    }
}
#[doc = "PSRAM Manual Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_manual_control2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_manual_control2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramManualControl2Spec;
impl crate::RegisterSpec for PsramManualControl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_manual_control2::R`](R) reader structure"]
impl crate::Readable for PsramManualControl2Spec {}
#[doc = "`write(|w| ..)` method takes [`psram_manual_control2::W`](W) writer structure"]
impl crate::Writable for PsramManualControl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_MANUAL_CONTROL2 to value 0x1f08_2008"]
impl crate::Resettable for PsramManualControl2Spec {
    const RESET_VALUE: u32 = 0x1f08_2008;
}
