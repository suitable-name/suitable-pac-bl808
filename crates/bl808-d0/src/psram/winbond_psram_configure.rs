#[doc = "Register `WINBOND_PSRAM_CONFIGURE` reader"]
pub type R = crate::R<WinbondPsramConfigureSpec>;
#[doc = "Register `WINBOND_PSRAM_CONFIGURE` writer"]
pub type W = crate::W<WinbondPsramConfigureSpec>;
#[doc = "Field `reg_wb_latency` reader - Winbond latency cycles"]
pub type RegWbLatencyR = crate::FieldReader;
#[doc = "Field `reg_wb_latency` writer - Winbond latency cycles"]
pub type RegWbLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_wb_drive_st` reader - Winbond drive strength"]
pub type RegWbDriveStR = crate::FieldReader;
#[doc = "Field `reg_wb_drive_st` writer - Winbond drive strength"]
pub type RegWbDriveStW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_wb_hybrid_en` reader - Winbond hybrid enable"]
pub type RegWbHybridEnR = crate::BitReader;
#[doc = "Field `reg_wb_hybrid_en` writer - Winbond hybrid enable"]
pub type RegWbHybridEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_wb_burst_length` reader - Winbond burst length"]
pub type RegWbBurstLengthR = crate::FieldReader;
#[doc = "Field `reg_wb_burst_length` writer - Winbond burst length"]
pub type RegWbBurstLengthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_wb_fix_latency` reader - Winbond fixed latency enable"]
pub type RegWbFixLatencyR = crate::BitReader;
#[doc = "Field `reg_wb_fix_latency` writer - Winbond fixed latency enable"]
pub type RegWbFixLatencyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_wb_dpd_dis` reader - Winbond Deep Power Down disable"]
pub type RegWbDpdDisR = crate::BitReader;
#[doc = "Field `reg_wb_dpd_dis` writer - Winbond Deep Power Down disable"]
pub type RegWbDpdDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_wb_pasr` reader - Winbond Partial Array Self Refresh"]
pub type RegWbPasrR = crate::FieldReader;
#[doc = "Field `reg_wb_pasr` writer - Winbond Partial Array Self Refresh"]
pub type RegWbPasrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `reg_wb_hybrid_slp` reader - Winbond hybrid sleep enable"]
pub type RegWbHybridSlpR = crate::BitReader;
#[doc = "Field `reg_wb_hybrid_slp` writer - Winbond hybrid sleep enable"]
pub type RegWbHybridSlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_wb_linear_dis` reader - Winbond linear burst disable"]
pub type RegWbLinearDisR = crate::BitReader;
#[doc = "Field `reg_wb_linear_dis` writer - Winbond linear burst disable"]
pub type RegWbLinearDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_wb_ipd` reader - Winbond input pull-down"]
pub type RegWbIpdR = crate::BitReader;
#[doc = "Field `reg_wb_ipd` writer - Winbond input pull-down"]
pub type RegWbIpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_wb_mclk_type` reader - Winbond MCLK type"]
pub type RegWbMclkTypeR = crate::BitReader;
#[doc = "Field `reg_wb_mclk_type` writer - Winbond MCLK type"]
pub type RegWbMclkTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_wb_sw_rst` reader - Winbond software reset"]
pub type RegWbSwRstR = crate::BitReader;
#[doc = "Field `reg_wb_sw_rst` writer - Winbond software reset"]
pub type RegWbSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Winbond latency cycles"]
    #[inline(always)]
    pub fn reg_wb_latency(&self) -> RegWbLatencyR {
        RegWbLatencyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Winbond drive strength"]
    #[inline(always)]
    pub fn reg_wb_drive_st(&self) -> RegWbDriveStR {
        RegWbDriveStR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Winbond hybrid enable"]
    #[inline(always)]
    pub fn reg_wb_hybrid_en(&self) -> RegWbHybridEnR {
        RegWbHybridEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Winbond burst length"]
    #[inline(always)]
    pub fn reg_wb_burst_length(&self) -> RegWbBurstLengthR {
        RegWbBurstLengthR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Winbond fixed latency enable"]
    #[inline(always)]
    pub fn reg_wb_fix_latency(&self) -> RegWbFixLatencyR {
        RegWbFixLatencyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Winbond Deep Power Down disable"]
    #[inline(always)]
    pub fn reg_wb_dpd_dis(&self) -> RegWbDpdDisR {
        RegWbDpdDisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Winbond Partial Array Self Refresh"]
    #[inline(always)]
    pub fn reg_wb_pasr(&self) -> RegWbPasrR {
        RegWbPasrR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Winbond hybrid sleep enable"]
    #[inline(always)]
    pub fn reg_wb_hybrid_slp(&self) -> RegWbHybridSlpR {
        RegWbHybridSlpR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Winbond linear burst disable"]
    #[inline(always)]
    pub fn reg_wb_linear_dis(&self) -> RegWbLinearDisR {
        RegWbLinearDisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Winbond input pull-down"]
    #[inline(always)]
    pub fn reg_wb_ipd(&self) -> RegWbIpdR {
        RegWbIpdR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Winbond MCLK type"]
    #[inline(always)]
    pub fn reg_wb_mclk_type(&self) -> RegWbMclkTypeR {
        RegWbMclkTypeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Winbond software reset"]
    #[inline(always)]
    pub fn reg_wb_sw_rst(&self) -> RegWbSwRstR {
        RegWbSwRstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Winbond latency cycles"]
    #[inline(always)]
    pub fn reg_wb_latency(&mut self) -> RegWbLatencyW<'_, WinbondPsramConfigureSpec> {
        RegWbLatencyW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Winbond drive strength"]
    #[inline(always)]
    pub fn reg_wb_drive_st(&mut self) -> RegWbDriveStW<'_, WinbondPsramConfigureSpec> {
        RegWbDriveStW::new(self, 4)
    }
    #[doc = "Bit 7 - Winbond hybrid enable"]
    #[inline(always)]
    pub fn reg_wb_hybrid_en(&mut self) -> RegWbHybridEnW<'_, WinbondPsramConfigureSpec> {
        RegWbHybridEnW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Winbond burst length"]
    #[inline(always)]
    pub fn reg_wb_burst_length(&mut self) -> RegWbBurstLengthW<'_, WinbondPsramConfigureSpec> {
        RegWbBurstLengthW::new(self, 8)
    }
    #[doc = "Bit 12 - Winbond fixed latency enable"]
    #[inline(always)]
    pub fn reg_wb_fix_latency(&mut self) -> RegWbFixLatencyW<'_, WinbondPsramConfigureSpec> {
        RegWbFixLatencyW::new(self, 12)
    }
    #[doc = "Bit 13 - Winbond Deep Power Down disable"]
    #[inline(always)]
    pub fn reg_wb_dpd_dis(&mut self) -> RegWbDpdDisW<'_, WinbondPsramConfigureSpec> {
        RegWbDpdDisW::new(self, 13)
    }
    #[doc = "Bits 16:20 - Winbond Partial Array Self Refresh"]
    #[inline(always)]
    pub fn reg_wb_pasr(&mut self) -> RegWbPasrW<'_, WinbondPsramConfigureSpec> {
        RegWbPasrW::new(self, 16)
    }
    #[doc = "Bit 24 - Winbond hybrid sleep enable"]
    #[inline(always)]
    pub fn reg_wb_hybrid_slp(&mut self) -> RegWbHybridSlpW<'_, WinbondPsramConfigureSpec> {
        RegWbHybridSlpW::new(self, 24)
    }
    #[doc = "Bit 25 - Winbond linear burst disable"]
    #[inline(always)]
    pub fn reg_wb_linear_dis(&mut self) -> RegWbLinearDisW<'_, WinbondPsramConfigureSpec> {
        RegWbLinearDisW::new(self, 25)
    }
    #[doc = "Bit 29 - Winbond input pull-down"]
    #[inline(always)]
    pub fn reg_wb_ipd(&mut self) -> RegWbIpdW<'_, WinbondPsramConfigureSpec> {
        RegWbIpdW::new(self, 29)
    }
    #[doc = "Bit 30 - Winbond MCLK type"]
    #[inline(always)]
    pub fn reg_wb_mclk_type(&mut self) -> RegWbMclkTypeW<'_, WinbondPsramConfigureSpec> {
        RegWbMclkTypeW::new(self, 30)
    }
    #[doc = "Bit 31 - Winbond software reset"]
    #[inline(always)]
    pub fn reg_wb_sw_rst(&mut self) -> RegWbSwRstW<'_, WinbondPsramConfigureSpec> {
        RegWbSwRstW::new(self, 31)
    }
}
#[doc = "Winbond Specific Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`winbond_psram_configure::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winbond_psram_configure::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinbondPsramConfigureSpec;
impl crate::RegisterSpec for WinbondPsramConfigureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winbond_psram_configure::R`](R) reader structure"]
impl crate::Readable for WinbondPsramConfigureSpec {}
#[doc = "`write(|w| ..)` method takes [`winbond_psram_configure::W`](W) writer structure"]
impl crate::Writable for WinbondPsramConfigureSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WINBOND_PSRAM_CONFIGURE to value 0x4000_3782"]
impl crate::Resettable for WinbondPsramConfigureSpec {
    const RESET_VALUE: u32 = 0x4000_3782;
}
