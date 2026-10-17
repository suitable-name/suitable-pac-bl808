#[doc = "Register `DVP2AXI_CONFIGUE` reader"]
pub type R = crate::R<Dvp2axiConfigueSpec>;
#[doc = "Register `DVP2AXI_CONFIGUE` writer"]
pub type W = crate::W<Dvp2axiConfigueSpec>;
#[doc = "Field `reg_dvp_enable` reader - Module Enable"]
pub type RegDvpEnableR = crate::BitReader;
#[doc = "Field `reg_dvp_enable` writer - Module Enable"]
pub type RegDvpEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sw_mode` reader - DVP2BUS SW manual mode"]
pub type RegSwModeR = crate::BitReader;
#[doc = "Field `reg_sw_mode` writer - DVP2BUS SW manual mode"]
pub type RegSwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_fram_vld_pol` reader - Frame Valid Polarity. 0: Active Low, 1: Active High"]
pub type RegFramVldPolR = crate::BitReader;
#[doc = "Field `reg_fram_vld_pol` writer - Frame Valid Polarity. 0: Active Low, 1: Active High"]
pub type RegFramVldPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_line_vld_pol` reader - Line Valid Polarity. 0: Active Low, 1: Active High"]
pub type RegLineVldPolR = crate::BitReader;
#[doc = "Field `reg_line_vld_pol` writer - Line Valid Polarity. 0: Active Low, 1: Active High"]
pub type RegLineVldPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_xlen` reader - Burst Length: 0:Single, 1:INCR4, 2:INCR8, 3:INCR16, 5:INCR32, 6:INCR64"]
pub type RegXlenR = crate::FieldReader;
#[doc = "Field `reg_xlen` writer - Burst Length: 0:Single, 1:INCR4, 2:INCR8, 3:INCR16, 5:INCR32, 6:INCR64"]
pub type RegXlenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_dvp_mode` reader - DVP Mode: 0:Vsync/Hsync, 1:Vsync|Hsync, 2:Vsync, 3:Hsync"]
pub type RegDvpModeR = crate::FieldReader;
#[doc = "Field `reg_dvp_mode` writer - DVP Mode: 0:Vsync/Hsync, 1:Vsync|Hsync, 2:Vsync, 3:Hsync"]
pub type RegDvpModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_hw_mode_fwrap` reader - Frame wrap in HW mode"]
pub type RegHwModeFwrapR = crate::BitReader;
#[doc = "Field `reg_hw_mode_fwrap` writer - Frame wrap in HW mode"]
pub type RegHwModeFwrapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_drop_en` reader - Enable frame drop"]
pub type RegDropEnR = crate::BitReader;
#[doc = "Field `reg_drop_en` writer - Enable frame drop"]
pub type RegDropEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_drop_even` reader - Drop even frames"]
pub type RegDropEvenR = crate::BitReader;
#[doc = "Field `reg_drop_even` writer - Drop even frames"]
pub type RegDropEvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_qos_sw_mode` reader - QOS software mode"]
pub type RegQosSwModeR = crate::BitReader;
#[doc = "Field `reg_qos_sw_mode` writer - QOS software mode"]
pub type RegQosSwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_qos_sw` reader - QOS software value"]
pub type RegQosSwR = crate::BitReader;
#[doc = "Field `reg_qos_sw` writer - QOS software value"]
pub type RegQosSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dvp_data_mode` reader - Data Mode: 0:16-bit, 1:24-bit, 2:24-bit comp 16, 3:24-bit exp 32, 4:8-bit"]
pub type RegDvpDataModeR = crate::FieldReader;
#[doc = "Field `reg_dvp_data_mode` writer - Data Mode: 0:16-bit, 1:24-bit, 2:24-bit comp 16, 3:24-bit exp 32, 4:8-bit"]
pub type RegDvpDataModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_dvp_data_bsel` reader - Byte select for 8-bit mode"]
pub type RegDvpDataBselR = crate::BitReader;
#[doc = "Field `reg_dvp_data_bsel` writer - Byte select for 8-bit mode"]
pub type RegDvpDataBselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dvp_pix_clk_cg` reader - Pixel clock gate enable"]
pub type RegDvpPixClkCgR = crate::BitReader;
#[doc = "Field `reg_dvp_pix_clk_cg` writer - Pixel clock gate enable"]
pub type RegDvpPixClkCgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_v_subsample_en` reader - Vertical subsample enable"]
pub type RegVSubsampleEnR = crate::BitReader;
#[doc = "Field `reg_v_subsample_en` writer - Vertical subsample enable"]
pub type RegVSubsampleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_v_subsample_pol` reader - Vertical subsample polarity"]
pub type RegVSubsamplePolR = crate::BitReader;
#[doc = "Field `reg_v_subsample_pol` writer - Vertical subsample polarity"]
pub type RegVSubsamplePolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dvp_wait_cycle` reader - Wait cycles in FSM"]
pub type RegDvpWaitCycleR = crate::FieldReader;
#[doc = "Field `reg_dvp_wait_cycle` writer - Wait cycles in FSM"]
pub type RegDvpWaitCycleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn reg_dvp_enable(&self) -> RegDvpEnableR {
        RegDvpEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DVP2BUS SW manual mode"]
    #[inline(always)]
    pub fn reg_sw_mode(&self) -> RegSwModeR {
        RegSwModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame Valid Polarity. 0: Active Low, 1: Active High"]
    #[inline(always)]
    pub fn reg_fram_vld_pol(&self) -> RegFramVldPolR {
        RegFramVldPolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line Valid Polarity. 0: Active Low, 1: Active High"]
    #[inline(always)]
    pub fn reg_line_vld_pol(&self) -> RegLineVldPolR {
        RegLineVldPolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Burst Length: 0:Single, 1:INCR4, 2:INCR8, 3:INCR16, 5:INCR32, 6:INCR64"]
    #[inline(always)]
    pub fn reg_xlen(&self) -> RegXlenR {
        RegXlenR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - DVP Mode: 0:Vsync/Hsync, 1:Vsync|Hsync, 2:Vsync, 3:Hsync"]
    #[inline(always)]
    pub fn reg_dvp_mode(&self) -> RegDvpModeR {
        RegDvpModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Frame wrap in HW mode"]
    #[inline(always)]
    pub fn reg_hw_mode_fwrap(&self) -> RegHwModeFwrapR {
        RegHwModeFwrapR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable frame drop"]
    #[inline(always)]
    pub fn reg_drop_en(&self) -> RegDropEnR {
        RegDropEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Drop even frames"]
    #[inline(always)]
    pub fn reg_drop_even(&self) -> RegDropEvenR {
        RegDropEvenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - QOS software mode"]
    #[inline(always)]
    pub fn reg_qos_sw_mode(&self) -> RegQosSwModeR {
        RegQosSwModeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - QOS software value"]
    #[inline(always)]
    pub fn reg_qos_sw(&self) -> RegQosSwR {
        RegQosSwR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Data Mode: 0:16-bit, 1:24-bit, 2:24-bit comp 16, 3:24-bit exp 32, 4:8-bit"]
    #[inline(always)]
    pub fn reg_dvp_data_mode(&self) -> RegDvpDataModeR {
        RegDvpDataModeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Byte select for 8-bit mode"]
    #[inline(always)]
    pub fn reg_dvp_data_bsel(&self) -> RegDvpDataBselR {
        RegDvpDataBselR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pixel clock gate enable"]
    #[inline(always)]
    pub fn reg_dvp_pix_clk_cg(&self) -> RegDvpPixClkCgR {
        RegDvpPixClkCgR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Vertical subsample enable"]
    #[inline(always)]
    pub fn reg_v_subsample_en(&self) -> RegVSubsampleEnR {
        RegVSubsampleEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Vertical subsample polarity"]
    #[inline(always)]
    pub fn reg_v_subsample_pol(&self) -> RegVSubsamplePolR {
        RegVSubsamplePolR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Wait cycles in FSM"]
    #[inline(always)]
    pub fn reg_dvp_wait_cycle(&self) -> RegDvpWaitCycleR {
        RegDvpWaitCycleR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn reg_dvp_enable(&mut self) -> RegDvpEnableW<'_, Dvp2axiConfigueSpec> {
        RegDvpEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - DVP2BUS SW manual mode"]
    #[inline(always)]
    pub fn reg_sw_mode(&mut self) -> RegSwModeW<'_, Dvp2axiConfigueSpec> {
        RegSwModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Frame Valid Polarity. 0: Active Low, 1: Active High"]
    #[inline(always)]
    pub fn reg_fram_vld_pol(&mut self) -> RegFramVldPolW<'_, Dvp2axiConfigueSpec> {
        RegFramVldPolW::new(self, 2)
    }
    #[doc = "Bit 3 - Line Valid Polarity. 0: Active Low, 1: Active High"]
    #[inline(always)]
    pub fn reg_line_vld_pol(&mut self) -> RegLineVldPolW<'_, Dvp2axiConfigueSpec> {
        RegLineVldPolW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Burst Length: 0:Single, 1:INCR4, 2:INCR8, 3:INCR16, 5:INCR32, 6:INCR64"]
    #[inline(always)]
    pub fn reg_xlen(&mut self) -> RegXlenW<'_, Dvp2axiConfigueSpec> {
        RegXlenW::new(self, 4)
    }
    #[doc = "Bits 8:10 - DVP Mode: 0:Vsync/Hsync, 1:Vsync|Hsync, 2:Vsync, 3:Hsync"]
    #[inline(always)]
    pub fn reg_dvp_mode(&mut self) -> RegDvpModeW<'_, Dvp2axiConfigueSpec> {
        RegDvpModeW::new(self, 8)
    }
    #[doc = "Bit 11 - Frame wrap in HW mode"]
    #[inline(always)]
    pub fn reg_hw_mode_fwrap(&mut self) -> RegHwModeFwrapW<'_, Dvp2axiConfigueSpec> {
        RegHwModeFwrapW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable frame drop"]
    #[inline(always)]
    pub fn reg_drop_en(&mut self) -> RegDropEnW<'_, Dvp2axiConfigueSpec> {
        RegDropEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Drop even frames"]
    #[inline(always)]
    pub fn reg_drop_even(&mut self) -> RegDropEvenW<'_, Dvp2axiConfigueSpec> {
        RegDropEvenW::new(self, 13)
    }
    #[doc = "Bit 14 - QOS software mode"]
    #[inline(always)]
    pub fn reg_qos_sw_mode(&mut self) -> RegQosSwModeW<'_, Dvp2axiConfigueSpec> {
        RegQosSwModeW::new(self, 14)
    }
    #[doc = "Bit 15 - QOS software value"]
    #[inline(always)]
    pub fn reg_qos_sw(&mut self) -> RegQosSwW<'_, Dvp2axiConfigueSpec> {
        RegQosSwW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Data Mode: 0:16-bit, 1:24-bit, 2:24-bit comp 16, 3:24-bit exp 32, 4:8-bit"]
    #[inline(always)]
    pub fn reg_dvp_data_mode(&mut self) -> RegDvpDataModeW<'_, Dvp2axiConfigueSpec> {
        RegDvpDataModeW::new(self, 16)
    }
    #[doc = "Bit 19 - Byte select for 8-bit mode"]
    #[inline(always)]
    pub fn reg_dvp_data_bsel(&mut self) -> RegDvpDataBselW<'_, Dvp2axiConfigueSpec> {
        RegDvpDataBselW::new(self, 19)
    }
    #[doc = "Bit 20 - Pixel clock gate enable"]
    #[inline(always)]
    pub fn reg_dvp_pix_clk_cg(&mut self) -> RegDvpPixClkCgW<'_, Dvp2axiConfigueSpec> {
        RegDvpPixClkCgW::new(self, 20)
    }
    #[doc = "Bit 22 - Vertical subsample enable"]
    #[inline(always)]
    pub fn reg_v_subsample_en(&mut self) -> RegVSubsampleEnW<'_, Dvp2axiConfigueSpec> {
        RegVSubsampleEnW::new(self, 22)
    }
    #[doc = "Bit 23 - Vertical subsample polarity"]
    #[inline(always)]
    pub fn reg_v_subsample_pol(&mut self) -> RegVSubsamplePolW<'_, Dvp2axiConfigueSpec> {
        RegVSubsamplePolW::new(self, 23)
    }
    #[doc = "Bits 24:31 - Wait cycles in FSM"]
    #[inline(always)]
    pub fn reg_dvp_wait_cycle(&mut self) -> RegDvpWaitCycleW<'_, Dvp2axiConfigueSpec> {
        RegDvpWaitCycleW::new(self, 24)
    }
}
#[doc = "DVP to AXI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_configue::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_configue::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvp2axiConfigueSpec;
impl crate::RegisterSpec for Dvp2axiConfigueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp2axi_configue::R`](R) reader structure"]
impl crate::Readable for Dvp2axiConfigueSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp2axi_configue::W`](W) writer structure"]
impl crate::Writable for Dvp2axiConfigueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP2AXI_CONFIGUE to value 0x4000_083c"]
impl crate::Resettable for Dvp2axiConfigueSpec {
    const RESET_VALUE: u32 = 0x4000_083c;
}
