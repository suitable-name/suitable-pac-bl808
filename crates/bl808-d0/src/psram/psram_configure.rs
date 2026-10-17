#[doc = "Register `PSRAM_CONFIGURE` reader"]
pub type R = crate::R<PsramConfigureSpec>;
#[doc = "Register `PSRAM_CONFIGURE` writer"]
pub type W = crate::W<PsramConfigureSpec>;
#[doc = "Field `reg_vendor_sel` reader - PSRAM Vendor: 0:Winbond, 1:APM_XCELLA, 2:APM_HYPER"]
pub type RegVendorSelR = crate::FieldReader;
#[doc = "Field `reg_vendor_sel` writer - PSRAM Vendor: 0:Winbond, 1:APM_XCELLA, 2:APM_HYPER"]
pub type RegVendorSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_ap_mr` reader - APMemory pSRAM Mode Register R/W selection (MA0-MA8)"]
pub type RegApMrR = crate::FieldReader;
#[doc = "Field `reg_ap_mr` writer - APMemory pSRAM Mode Register R/W selection (MA0-MA8)"]
pub type RegApMrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_wb_reg_sel` reader - Winbond pSRAM Register R/W selection (ID/CR)"]
pub type RegWbRegSelR = crate::FieldReader;
#[doc = "Field `reg_wb_reg_sel` writer - Winbond pSRAM Register R/W selection (ID/CR)"]
pub type RegWbRegSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_config_w_pusle` writer - Trigger configuration write pulse"]
pub type RegConfigWPusleW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `reg_config_r_pusle` writer - Trigger configuration read pulse"]
pub type RegConfigRPusleW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `sts_config_w_done` reader - Status of config write done"]
pub type StsConfigWDoneR = crate::BitReader;
#[doc = "Field `sts_config_r_done` reader - Status of config read done"]
pub type StsConfigRDoneR = crate::BitReader;
#[doc = "Field `reg_config_req` reader - Configuration request"]
pub type RegConfigReqR = crate::BitReader;
#[doc = "Field `reg_config_req` writer - Configuration request"]
pub type RegConfigReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_config_gnt` reader - Configuration grant status"]
pub type RegConfigGntR = crate::BitReader;
#[doc = "Field `reg_x16_mode` reader - Enable X16 mode"]
pub type RegX16ModeR = crate::BitReader;
#[doc = "Field `reg_x16_mode` writer - Enable X16 mode"]
pub type RegX16ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_wb_hyper3` reader - Winbond Hyperbus 3 mode"]
pub type RegWbHyper3R = crate::BitReader;
#[doc = "Field `reg_wb_hyper3` writer - Winbond Hyperbus 3 mode"]
pub type RegWbHyper3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_pck_s_div` reader - EMI AXI clock division"]
pub type RegPckSDivR = crate::FieldReader;
#[doc = "Field `reg_pck_s_div` writer - EMI AXI clock division"]
pub type RegPckSDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_clkn_free` reader - CLKN free running enable"]
pub type RegClknFreeR = crate::BitReader;
#[doc = "Field `reg_clkn_free` writer - CLKN free running enable"]
pub type RegClknFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_linear_bnd_b` reader - Linear burst boundary shift bit"]
pub type RegLinearBndBR = crate::FieldReader;
#[doc = "Field `reg_linear_bnd_b` writer - Linear burst boundary shift bit"]
pub type RegLinearBndBW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - PSRAM Vendor: 0:Winbond, 1:APM_XCELLA, 2:APM_HYPER"]
    #[inline(always)]
    pub fn reg_vendor_sel(&self) -> RegVendorSelR {
        RegVendorSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - APMemory pSRAM Mode Register R/W selection (MA0-MA8)"]
    #[inline(always)]
    pub fn reg_ap_mr(&self) -> RegApMrR {
        RegApMrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Winbond pSRAM Register R/W selection (ID/CR)"]
    #[inline(always)]
    pub fn reg_wb_reg_sel(&self) -> RegWbRegSelR {
        RegWbRegSelR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 14 - Status of config write done"]
    #[inline(always)]
    pub fn sts_config_w_done(&self) -> StsConfigWDoneR {
        StsConfigWDoneR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Status of config read done"]
    #[inline(always)]
    pub fn sts_config_r_done(&self) -> StsConfigRDoneR {
        StsConfigRDoneR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configuration request"]
    #[inline(always)]
    pub fn reg_config_req(&self) -> RegConfigReqR {
        RegConfigReqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configuration grant status"]
    #[inline(always)]
    pub fn reg_config_gnt(&self) -> RegConfigGntR {
        RegConfigGntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable X16 mode"]
    #[inline(always)]
    pub fn reg_x16_mode(&self) -> RegX16ModeR {
        RegX16ModeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Winbond Hyperbus 3 mode"]
    #[inline(always)]
    pub fn reg_wb_hyper3(&self) -> RegWbHyper3R {
        RegWbHyper3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - EMI AXI clock division"]
    #[inline(always)]
    pub fn reg_pck_s_div(&self) -> RegPckSDivR {
        RegPckSDivR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - CLKN free running enable"]
    #[inline(always)]
    pub fn reg_clkn_free(&self) -> RegClknFreeR {
        RegClknFreeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Linear burst boundary shift bit"]
    #[inline(always)]
    pub fn reg_linear_bnd_b(&self) -> RegLinearBndBR {
        RegLinearBndBR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PSRAM Vendor: 0:Winbond, 1:APM_XCELLA, 2:APM_HYPER"]
    #[inline(always)]
    pub fn reg_vendor_sel(&mut self) -> RegVendorSelW<'_, PsramConfigureSpec> {
        RegVendorSelW::new(self, 0)
    }
    #[doc = "Bits 4:6 - APMemory pSRAM Mode Register R/W selection (MA0-MA8)"]
    #[inline(always)]
    pub fn reg_ap_mr(&mut self) -> RegApMrW<'_, PsramConfigureSpec> {
        RegApMrW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Winbond pSRAM Register R/W selection (ID/CR)"]
    #[inline(always)]
    pub fn reg_wb_reg_sel(&mut self) -> RegWbRegSelW<'_, PsramConfigureSpec> {
        RegWbRegSelW::new(self, 8)
    }
    #[doc = "Bit 12 - Trigger configuration write pulse"]
    #[inline(always)]
    pub fn reg_config_w_pusle(&mut self) -> RegConfigWPusleW<'_, PsramConfigureSpec> {
        RegConfigWPusleW::new(self, 12)
    }
    #[doc = "Bit 13 - Trigger configuration read pulse"]
    #[inline(always)]
    pub fn reg_config_r_pusle(&mut self) -> RegConfigRPusleW<'_, PsramConfigureSpec> {
        RegConfigRPusleW::new(self, 13)
    }
    #[doc = "Bit 16 - Configuration request"]
    #[inline(always)]
    pub fn reg_config_req(&mut self) -> RegConfigReqW<'_, PsramConfigureSpec> {
        RegConfigReqW::new(self, 16)
    }
    #[doc = "Bit 18 - Enable X16 mode"]
    #[inline(always)]
    pub fn reg_x16_mode(&mut self) -> RegX16ModeW<'_, PsramConfigureSpec> {
        RegX16ModeW::new(self, 18)
    }
    #[doc = "Bit 19 - Winbond Hyperbus 3 mode"]
    #[inline(always)]
    pub fn reg_wb_hyper3(&mut self) -> RegWbHyper3W<'_, PsramConfigureSpec> {
        RegWbHyper3W::new(self, 19)
    }
    #[doc = "Bits 20:22 - EMI AXI clock division"]
    #[inline(always)]
    pub fn reg_pck_s_div(&mut self) -> RegPckSDivW<'_, PsramConfigureSpec> {
        RegPckSDivW::new(self, 20)
    }
    #[doc = "Bit 23 - CLKN free running enable"]
    #[inline(always)]
    pub fn reg_clkn_free(&mut self) -> RegClknFreeW<'_, PsramConfigureSpec> {
        RegClknFreeW::new(self, 23)
    }
    #[doc = "Bits 28:31 - Linear burst boundary shift bit"]
    #[inline(always)]
    pub fn reg_linear_bnd_b(&mut self) -> RegLinearBndBW<'_, PsramConfigureSpec> {
        RegLinearBndBW::new(self, 28)
    }
}
#[doc = "PSRAM Main Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_configure::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_configure::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsramConfigureSpec;
impl crate::RegisterSpec for PsramConfigureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_configure::R`](R) reader structure"]
impl crate::Readable for PsramConfigureSpec {}
#[doc = "`write(|w| ..)` method takes [`psram_configure::W`](W) writer structure"]
impl crate::Writable for PsramConfigureSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3000;
}
#[doc = "`reset()` method sets PSRAM_CONFIGURE to value 0xa080_c002"]
impl crate::Resettable for PsramConfigureSpec {
    const RESET_VALUE: u32 = 0xa080_c002;
}
