#[doc = "Register `SYS_CFG1` reader"]
pub type R = crate::R<SysCfg1Spec>;
#[doc = "Register `SYS_CFG1` writer"]
pub type W = crate::W<SysCfg1Spec>;
#[doc = "Field `reg_bclk_div_act_pulse` reader - "]
pub type RegBclkDivActPulseR = crate::BitReader;
#[doc = "Field `reg_bclk_div_act_pulse` writer - "]
pub type RegBclkDivActPulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bclk_div_bypass` reader - "]
pub type RegBclkDivBypassR = crate::BitReader;
#[doc = "Field `reg_bclk_div_bypass` writer - "]
pub type RegBclkDivBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_bclk_prot_done` reader - "]
pub type StsBclkProtDoneR = crate::BitReader;
#[doc = "Field `reg_bclk_sw_done_cnt` reader - "]
pub type RegBclkSwDoneCntR = crate::FieldReader;
#[doc = "Field `reg_bclk_sw_done_cnt` writer - "]
pub type RegBclkSwDoneCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_pico_clk_div_act_pulse` reader - "]
pub type RegPicoClkDivActPulseR = crate::BitReader;
#[doc = "Field `reg_pico_clk_div_act_pulse` writer - "]
pub type RegPicoClkDivActPulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pico_clk_div_bypass` reader - "]
pub type RegPicoClkDivBypassR = crate::BitReader;
#[doc = "Field `reg_pico_clk_div_bypass` writer - "]
pub type RegPicoClkDivBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_pico_clk_prot_done` reader - "]
pub type StsPicoClkProtDoneR = crate::BitReader;
#[doc = "Field `reg_pico_clk_sw_done_cnt` reader - "]
pub type RegPicoClkSwDoneCntR = crate::FieldReader;
#[doc = "Field `reg_pico_clk_sw_done_cnt` writer - "]
pub type RegPicoClkSwDoneCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `fclk_sw_state` reader - "]
pub type FclkSwStateR = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bclk_div_act_pulse(&self) -> RegBclkDivActPulseR {
        RegBclkDivActPulseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_bclk_div_bypass(&self) -> RegBclkDivBypassR {
        RegBclkDivBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sts_bclk_prot_done(&self) -> StsBclkProtDoneR {
        StsBclkProtDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_bclk_sw_done_cnt(&self) -> RegBclkSwDoneCntR {
        RegBclkSwDoneCntR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_pico_clk_div_act_pulse(&self) -> RegPicoClkDivActPulseR {
        RegPicoClkDivActPulseR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_pico_clk_div_bypass(&self) -> RegPicoClkDivBypassR {
        RegPicoClkDivBypassR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sts_pico_clk_prot_done(&self) -> StsPicoClkProtDoneR {
        StsPicoClkProtDoneR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_pico_clk_sw_done_cnt(&self) -> RegPicoClkSwDoneCntR {
        RegPicoClkSwDoneCntR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn fclk_sw_state(&self) -> FclkSwStateR {
        FclkSwStateR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bclk_div_act_pulse(&mut self) -> RegBclkDivActPulseW<'_, SysCfg1Spec> {
        RegBclkDivActPulseW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_bclk_div_bypass(&mut self) -> RegBclkDivBypassW<'_, SysCfg1Spec> {
        RegBclkDivBypassW::new(self, 1)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_bclk_sw_done_cnt(&mut self) -> RegBclkSwDoneCntW<'_, SysCfg1Spec> {
        RegBclkSwDoneCntW::new(self, 4)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_pico_clk_div_act_pulse(&mut self) -> RegPicoClkDivActPulseW<'_, SysCfg1Spec> {
        RegPicoClkDivActPulseW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_pico_clk_div_bypass(&mut self) -> RegPicoClkDivBypassW<'_, SysCfg1Spec> {
        RegPicoClkDivBypassW::new(self, 17)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_pico_clk_sw_done_cnt(&mut self) -> RegPicoClkSwDoneCntW<'_, SysCfg1Spec> {
        RegPicoClkSwDoneCntW::new(self, 20)
    }
}
#[doc = "System Clock Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysCfg1Spec;
impl crate::RegisterSpec for SysCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_cfg1::R`](R) reader structure"]
impl crate::Readable for SysCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_cfg1::W`](W) writer structure"]
impl crate::Writable for SysCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_CFG1 to value 0"]
impl crate::Resettable for SysCfg1Spec {}
