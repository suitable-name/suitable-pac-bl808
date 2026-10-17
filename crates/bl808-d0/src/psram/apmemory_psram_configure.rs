#[doc = "Register `APMEMORY_PSRAM_CONFIGURE` reader"]
pub type R = crate::R<ApmemoryPsramConfigureSpec>;
#[doc = "Register `APMEMORY_PSRAM_CONFIGURE` writer"]
pub type W = crate::W<ApmemoryPsramConfigureSpec>;
#[doc = "Field `reg_ap_burst_length` reader - APMemory burst length"]
pub type RegApBurstLengthR = crate::FieldReader;
#[doc = "Field `reg_ap_burst_length` writer - APMemory burst length"]
pub type RegApBurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_ap_burst_type` reader - APMemory burst type"]
pub type RegApBurstTypeR = crate::BitReader;
#[doc = "Field `reg_ap_burst_type` writer - APMemory burst type"]
pub type RegApBurstTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ap_rbx` reader - APMemory read-burst-exit"]
pub type RegApRbxR = crate::BitReader;
#[doc = "Field `reg_ap_rbx` writer - APMemory read-burst-exit"]
pub type RegApRbxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ap_dpd` reader - APMemory Deep Power Down enable"]
pub type RegApDpdR = crate::BitReader;
#[doc = "Field `reg_ap_dpd` writer - APMemory Deep Power Down enable"]
pub type RegApDpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ap_sleep` reader - APMemory sleep mode enable"]
pub type RegApSleepR = crate::BitReader;
#[doc = "Field `reg_ap_sleep` writer - APMemory sleep mode enable"]
pub type RegApSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ap_pasr` reader - APMemory Partial Array Self Refresh"]
pub type RegApPasrR = crate::FieldReader;
#[doc = "Field `reg_ap_pasr` writer - APMemory Partial Array Self Refresh"]
pub type RegApPasrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_ap_w_latency_code` reader - APMemory write latency code"]
pub type RegApWLatencyCodeR = crate::FieldReader;
#[doc = "Field `reg_ap_w_latency_code` writer - APMemory write latency code"]
pub type RegApWLatencyCodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_ap_drive_st` reader - APMemory drive strength"]
pub type RegApDriveStR = crate::FieldReader;
#[doc = "Field `reg_ap_drive_st` writer - APMemory drive strength"]
pub type RegApDriveStW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_ap_rf` reader - APMemory refresh frequency"]
pub type RegApRfR = crate::FieldReader;
#[doc = "Field `reg_ap_rf` writer - APMemory refresh frequency"]
pub type RegApRfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_ap_r_latency_code` reader - APMemory read latency code"]
pub type RegApRLatencyCodeR = crate::FieldReader;
#[doc = "Field `reg_ap_r_latency_code` writer - APMemory read latency code"]
pub type RegApRLatencyCodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_ap_r_latency_type` reader - APMemory read latency type"]
pub type RegApRLatencyTypeR = crate::BitReader;
#[doc = "Field `reg_ap_r_latency_type` writer - APMemory read latency type"]
pub type RegApRLatencyTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ap_linear_dis` reader - APMemory linear burst disable"]
pub type RegApLinearDisR = crate::BitReader;
#[doc = "Field `reg_ap_linear_dis` writer - APMemory linear burst disable"]
pub type RegApLinearDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_glb_reset_pulse` writer - Global reset pulse (Write 1)"]
pub type RegGlbResetPulseW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - APMemory burst length"]
    #[inline(always)]
    pub fn reg_ap_burst_length(&self) -> RegApBurstLengthR {
        RegApBurstLengthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - APMemory burst type"]
    #[inline(always)]
    pub fn reg_ap_burst_type(&self) -> RegApBurstTypeR {
        RegApBurstTypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APMemory read-burst-exit"]
    #[inline(always)]
    pub fn reg_ap_rbx(&self) -> RegApRbxR {
        RegApRbxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - APMemory Deep Power Down enable"]
    #[inline(always)]
    pub fn reg_ap_dpd(&self) -> RegApDpdR {
        RegApDpdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - APMemory sleep mode enable"]
    #[inline(always)]
    pub fn reg_ap_sleep(&self) -> RegApSleepR {
        RegApSleepR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - APMemory Partial Array Self Refresh"]
    #[inline(always)]
    pub fn reg_ap_pasr(&self) -> RegApPasrR {
        RegApPasrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - APMemory write latency code"]
    #[inline(always)]
    pub fn reg_ap_w_latency_code(&self) -> RegApWLatencyCodeR {
        RegApWLatencyCodeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17 - APMemory drive strength"]
    #[inline(always)]
    pub fn reg_ap_drive_st(&self) -> RegApDriveStR {
        RegApDriveStR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - APMemory refresh frequency"]
    #[inline(always)]
    pub fn reg_ap_rf(&self) -> RegApRfR {
        RegApRfR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - APMemory read latency code"]
    #[inline(always)]
    pub fn reg_ap_r_latency_code(&self) -> RegApRLatencyCodeR {
        RegApRLatencyCodeR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - APMemory read latency type"]
    #[inline(always)]
    pub fn reg_ap_r_latency_type(&self) -> RegApRLatencyTypeR {
        RegApRLatencyTypeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - APMemory linear burst disable"]
    #[inline(always)]
    pub fn reg_ap_linear_dis(&self) -> RegApLinearDisR {
        RegApLinearDisR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - APMemory burst length"]
    #[inline(always)]
    pub fn reg_ap_burst_length(&mut self) -> RegApBurstLengthW<'_, ApmemoryPsramConfigureSpec> {
        RegApBurstLengthW::new(self, 0)
    }
    #[doc = "Bit 4 - APMemory burst type"]
    #[inline(always)]
    pub fn reg_ap_burst_type(&mut self) -> RegApBurstTypeW<'_, ApmemoryPsramConfigureSpec> {
        RegApBurstTypeW::new(self, 4)
    }
    #[doc = "Bit 5 - APMemory read-burst-exit"]
    #[inline(always)]
    pub fn reg_ap_rbx(&mut self) -> RegApRbxW<'_, ApmemoryPsramConfigureSpec> {
        RegApRbxW::new(self, 5)
    }
    #[doc = "Bit 6 - APMemory Deep Power Down enable"]
    #[inline(always)]
    pub fn reg_ap_dpd(&mut self) -> RegApDpdW<'_, ApmemoryPsramConfigureSpec> {
        RegApDpdW::new(self, 6)
    }
    #[doc = "Bit 7 - APMemory sleep mode enable"]
    #[inline(always)]
    pub fn reg_ap_sleep(&mut self) -> RegApSleepW<'_, ApmemoryPsramConfigureSpec> {
        RegApSleepW::new(self, 7)
    }
    #[doc = "Bits 8:10 - APMemory Partial Array Self Refresh"]
    #[inline(always)]
    pub fn reg_ap_pasr(&mut self) -> RegApPasrW<'_, ApmemoryPsramConfigureSpec> {
        RegApPasrW::new(self, 8)
    }
    #[doc = "Bits 12:14 - APMemory write latency code"]
    #[inline(always)]
    pub fn reg_ap_w_latency_code(&mut self) -> RegApWLatencyCodeW<'_, ApmemoryPsramConfigureSpec> {
        RegApWLatencyCodeW::new(self, 12)
    }
    #[doc = "Bits 16:17 - APMemory drive strength"]
    #[inline(always)]
    pub fn reg_ap_drive_st(&mut self) -> RegApDriveStW<'_, ApmemoryPsramConfigureSpec> {
        RegApDriveStW::new(self, 16)
    }
    #[doc = "Bits 18:19 - APMemory refresh frequency"]
    #[inline(always)]
    pub fn reg_ap_rf(&mut self) -> RegApRfW<'_, ApmemoryPsramConfigureSpec> {
        RegApRfW::new(self, 18)
    }
    #[doc = "Bits 20:22 - APMemory read latency code"]
    #[inline(always)]
    pub fn reg_ap_r_latency_code(&mut self) -> RegApRLatencyCodeW<'_, ApmemoryPsramConfigureSpec> {
        RegApRLatencyCodeW::new(self, 20)
    }
    #[doc = "Bit 24 - APMemory read latency type"]
    #[inline(always)]
    pub fn reg_ap_r_latency_type(&mut self) -> RegApRLatencyTypeW<'_, ApmemoryPsramConfigureSpec> {
        RegApRLatencyTypeW::new(self, 24)
    }
    #[doc = "Bit 25 - APMemory linear burst disable"]
    #[inline(always)]
    pub fn reg_ap_linear_dis(&mut self) -> RegApLinearDisW<'_, ApmemoryPsramConfigureSpec> {
        RegApLinearDisW::new(self, 25)
    }
    #[doc = "Bit 28 - Global reset pulse (Write 1)"]
    #[inline(always)]
    pub fn reg_glb_reset_pulse(&mut self) -> RegGlbResetPulseW<'_, ApmemoryPsramConfigureSpec> {
        RegGlbResetPulseW::new(self, 28)
    }
}
#[doc = "APMemory Specific Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`apmemory_psram_configure::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apmemory_psram_configure::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApmemoryPsramConfigureSpec;
impl crate::RegisterSpec for ApmemoryPsramConfigureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apmemory_psram_configure::R`](R) reader structure"]
impl crate::Readable for ApmemoryPsramConfigureSpec {}
#[doc = "`write(|w| ..)` method takes [`apmemory_psram_configure::W`](W) writer structure"]
impl crate::Writable for ApmemoryPsramConfigureSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1000_0000;
}
#[doc = "`reset()` method sets APMEMORY_PSRAM_CONFIGURE to value 0x0021_2011"]
impl crate::Resettable for ApmemoryPsramConfigureSpec {
    const RESET_VALUE: u32 = 0x0021_2011;
}
