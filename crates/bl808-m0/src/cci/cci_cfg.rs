#[doc = "Register `CCI_CFG` reader"]
pub type R = crate::R<CciCfgSpec>;
#[doc = "Register `CCI_CFG` writer"]
pub type W = crate::W<CciCfgSpec>;
#[doc = "Field `cci_en` reader - CCI module enable"]
pub type CciEnR = crate::BitReader;
#[doc = "Field `cci_en` writer - CCI module enable"]
pub type CciEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cci_slv_sel_cci2` reader - Slave select control (for internal routing to CCI2/D0 side)"]
pub type CciSlvSelCci2R = crate::BitReader;
#[doc = "Field `cci_slv_sel_cci2` writer - Slave select control (for internal routing to CCI2/D0 side)"]
pub type CciSlvSelCci2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cci_mas_sel_cci2` reader - Master select control (for internal routing from CCI2/D0 side)"]
pub type CciMasSelCci2R = crate::BitReader;
#[doc = "Field `cci_mas_sel_cci2` writer - Master select control (for internal routing from CCI2/D0 side)"]
pub type CciMasSelCci2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cci_mas_hw_mode` reader - Master hardware mode enable"]
pub type CciMasHwModeR = crate::BitReader;
#[doc = "Field `cci_mas_hw_mode` writer - Master hardware mode enable"]
pub type CciMasHwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_m_cci_sclk_en` reader - Master CCI SCLK enable"]
pub type RegMCciSclkEnR = crate::BitReader;
#[doc = "Field `reg_m_cci_sclk_en` writer - Master CCI SCLK enable"]
pub type RegMCciSclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_div_m_cci_sclk` reader - Master CCI SCLK clock division ratio"]
pub type RegDivMCciSclkR = crate::FieldReader;
#[doc = "Field `reg_div_m_cci_sclk` writer - Master CCI SCLK clock division ratio"]
pub type RegDivMCciSclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cfg_cci1_pre_read` reader - CCI1 pre-read configuration"]
pub type CfgCci1PreReadR = crate::BitReader;
#[doc = "Field `cfg_cci1_pre_read` writer - CCI1 pre-read configuration"]
pub type CfgCci1PreReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_scci_clk_inv` reader - Slave CCI clock inversion"]
pub type RegScciClkInvR = crate::BitReader;
#[doc = "Field `reg_scci_clk_inv` writer - Slave CCI clock inversion"]
pub type RegScciClkInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcci_clk_inv` reader - Master CCI clock inversion"]
pub type RegMcciClkInvR = crate::BitReader;
#[doc = "Field `reg_mcci_clk_inv` writer - Master CCI clock inversion"]
pub type RegMcciClkInvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCI module enable"]
    #[inline(always)]
    pub fn cci_en(&self) -> CciEnR {
        CciEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave select control (for internal routing to CCI2/D0 side)"]
    #[inline(always)]
    pub fn cci_slv_sel_cci2(&self) -> CciSlvSelCci2R {
        CciSlvSelCci2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master select control (for internal routing from CCI2/D0 side)"]
    #[inline(always)]
    pub fn cci_mas_sel_cci2(&self) -> CciMasSelCci2R {
        CciMasSelCci2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master hardware mode enable"]
    #[inline(always)]
    pub fn cci_mas_hw_mode(&self) -> CciMasHwModeR {
        CciMasHwModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master CCI SCLK enable"]
    #[inline(always)]
    pub fn reg_m_cci_sclk_en(&self) -> RegMCciSclkEnR {
        RegMCciSclkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Master CCI SCLK clock division ratio"]
    #[inline(always)]
    pub fn reg_div_m_cci_sclk(&self) -> RegDivMCciSclkR {
        RegDivMCciSclkR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - CCI1 pre-read configuration"]
    #[inline(always)]
    pub fn cfg_cci1_pre_read(&self) -> CfgCci1PreReadR {
        CfgCci1PreReadR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave CCI clock inversion"]
    #[inline(always)]
    pub fn reg_scci_clk_inv(&self) -> RegScciClkInvR {
        RegScciClkInvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master CCI clock inversion"]
    #[inline(always)]
    pub fn reg_mcci_clk_inv(&self) -> RegMcciClkInvR {
        RegMcciClkInvR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCI module enable"]
    #[inline(always)]
    pub fn cci_en(&mut self) -> CciEnW<'_, CciCfgSpec> {
        CciEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave select control (for internal routing to CCI2/D0 side)"]
    #[inline(always)]
    pub fn cci_slv_sel_cci2(&mut self) -> CciSlvSelCci2W<'_, CciCfgSpec> {
        CciSlvSelCci2W::new(self, 1)
    }
    #[doc = "Bit 2 - Master select control (for internal routing from CCI2/D0 side)"]
    #[inline(always)]
    pub fn cci_mas_sel_cci2(&mut self) -> CciMasSelCci2W<'_, CciCfgSpec> {
        CciMasSelCci2W::new(self, 2)
    }
    #[doc = "Bit 3 - Master hardware mode enable"]
    #[inline(always)]
    pub fn cci_mas_hw_mode(&mut self) -> CciMasHwModeW<'_, CciCfgSpec> {
        CciMasHwModeW::new(self, 3)
    }
    #[doc = "Bit 4 - Master CCI SCLK enable"]
    #[inline(always)]
    pub fn reg_m_cci_sclk_en(&mut self) -> RegMCciSclkEnW<'_, CciCfgSpec> {
        RegMCciSclkEnW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Master CCI SCLK clock division ratio"]
    #[inline(always)]
    pub fn reg_div_m_cci_sclk(&mut self) -> RegDivMCciSclkW<'_, CciCfgSpec> {
        RegDivMCciSclkW::new(self, 5)
    }
    #[doc = "Bit 7 - CCI1 pre-read configuration"]
    #[inline(always)]
    pub fn cfg_cci1_pre_read(&mut self) -> CfgCci1PreReadW<'_, CciCfgSpec> {
        CfgCci1PreReadW::new(self, 7)
    }
    #[doc = "Bit 8 - Slave CCI clock inversion"]
    #[inline(always)]
    pub fn reg_scci_clk_inv(&mut self) -> RegScciClkInvW<'_, CciCfgSpec> {
        RegScciClkInvW::new(self, 8)
    }
    #[doc = "Bit 9 - Master CCI clock inversion"]
    #[inline(always)]
    pub fn reg_mcci_clk_inv(&mut self) -> RegMcciClkInvW<'_, CciCfgSpec> {
        RegMcciClkInvW::new(self, 9)
    }
}
#[doc = "CCI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciCfgSpec;
impl crate::RegisterSpec for CciCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci_cfg::R`](R) reader structure"]
impl crate::Readable for CciCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cci_cfg::W`](W) writer structure"]
impl crate::Writable for CciCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCI_CFG to value 0x0261"]
impl crate::Resettable for CciCfgSpec {
    const RESET_VALUE: u32 = 0x0261;
}
