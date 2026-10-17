#[doc = "Register `PHY_CFG_40` reader"]
pub type R = crate::R<PhyCfg40Spec>;
#[doc = "Register `PHY_CFG_40` writer"]
pub type W = crate::W<PhyCfg40Spec>;
#[doc = "Field `vref_sel` reader - VREF source selection"]
pub type VrefSelR = crate::FieldReader;
#[doc = "Field `vref_sel` writer - VREF source selection"]
pub type VrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vref_dq_sel` reader - VREF DQ selection"]
pub type VrefDqSelR = crate::FieldReader;
#[doc = "Field `vref_dq_sel` writer - VREF DQ selection"]
pub type VrefDqSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_uhs_dmy0` reader - UHS dummy 0"]
pub type RegUhsDmy0R = crate::FieldReader;
#[doc = "Field `reg_uhs_dmy0` writer - UHS dummy 0"]
pub type RegUhsDmy0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_uhs_dmy1` reader - UHS dummy 1"]
pub type RegUhsDmy1R = crate::FieldReader;
#[doc = "Field `reg_uhs_dmy1` writer - UHS dummy 1"]
pub type RegUhsDmy1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_uhs_phy_ten` reader - PHY test enable"]
pub type RegUhsPhyTenR = crate::BitReader;
#[doc = "Field `reg_uhs_phy_ten` writer - PHY test enable"]
pub type RegUhsPhyTenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `soc_en_aon` reader - SoC AON enable"]
pub type SocEnAonR = crate::BitReader;
#[doc = "Field `soc_en_aon` writer - SoC AON enable"]
pub type SocEnAonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_uhs_phy` reader - PHY enable"]
pub type TenUhsPhyR = crate::BitReader;
#[doc = "Field `ten_uhs_phy` writer - PHY enable"]
pub type TenUhsPhyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_uhs_phy_dig` reader - PHY digital enable"]
pub type TenUhsPhyDigR = crate::BitReader;
#[doc = "Field `ten_uhs_phy_dig` writer - PHY digital enable"]
pub type TenUhsPhyDigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_clktree_gate_hw` reader - Hardware clock gate enable"]
pub type TxClktreeGateHwR = crate::BitReader;
#[doc = "Field `tx_clktree_gate_hw` writer - Hardware clock gate enable"]
pub type TxClktreeGateHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhs_dc_tp_out_en` reader - UHS DC test point output enable"]
pub type UhsDcTpOutEnR = crate::BitReader;
#[doc = "Field `uhs_dc_tp_out_en` writer - UHS DC test point output enable"]
pub type UhsDcTpOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhs_phy_dqs_diff` reader - UHS PHY differential DQS enable"]
pub type UhsPhyDqsDiffR = crate::BitReader;
#[doc = "Field `uhs_phy_dqs_diff` writer - UHS PHY differential DQS enable"]
pub type UhsPhyDqsDiffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - VREF source selection"]
    #[inline(always)]
    pub fn vref_sel(&self) -> VrefSelR {
        VrefSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - VREF DQ selection"]
    #[inline(always)]
    pub fn vref_dq_sel(&self) -> VrefDqSelR {
        VrefDqSelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - UHS dummy 0"]
    #[inline(always)]
    pub fn reg_uhs_dmy0(&self) -> RegUhsDmy0R {
        RegUhsDmy0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - UHS dummy 1"]
    #[inline(always)]
    pub fn reg_uhs_dmy1(&self) -> RegUhsDmy1R {
        RegUhsDmy1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - PHY test enable"]
    #[inline(always)]
    pub fn reg_uhs_phy_ten(&self) -> RegUhsPhyTenR {
        RegUhsPhyTenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SoC AON enable"]
    #[inline(always)]
    pub fn soc_en_aon(&self) -> SocEnAonR {
        SocEnAonR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PHY enable"]
    #[inline(always)]
    pub fn ten_uhs_phy(&self) -> TenUhsPhyR {
        TenUhsPhyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PHY digital enable"]
    #[inline(always)]
    pub fn ten_uhs_phy_dig(&self) -> TenUhsPhyDigR {
        TenUhsPhyDigR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Hardware clock gate enable"]
    #[inline(always)]
    pub fn tx_clktree_gate_hw(&self) -> TxClktreeGateHwR {
        TxClktreeGateHwR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - UHS DC test point output enable"]
    #[inline(always)]
    pub fn uhs_dc_tp_out_en(&self) -> UhsDcTpOutEnR {
        UhsDcTpOutEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UHS PHY differential DQS enable"]
    #[inline(always)]
    pub fn uhs_phy_dqs_diff(&self) -> UhsPhyDqsDiffR {
        UhsPhyDqsDiffR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - VREF source selection"]
    #[inline(always)]
    pub fn vref_sel(&mut self) -> VrefSelW<'_, PhyCfg40Spec> {
        VrefSelW::new(self, 0)
    }
    #[doc = "Bits 4:7 - VREF DQ selection"]
    #[inline(always)]
    pub fn vref_dq_sel(&mut self) -> VrefDqSelW<'_, PhyCfg40Spec> {
        VrefDqSelW::new(self, 4)
    }
    #[doc = "Bits 8:15 - UHS dummy 0"]
    #[inline(always)]
    pub fn reg_uhs_dmy0(&mut self) -> RegUhsDmy0W<'_, PhyCfg40Spec> {
        RegUhsDmy0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - UHS dummy 1"]
    #[inline(always)]
    pub fn reg_uhs_dmy1(&mut self) -> RegUhsDmy1W<'_, PhyCfg40Spec> {
        RegUhsDmy1W::new(self, 16)
    }
    #[doc = "Bit 24 - PHY test enable"]
    #[inline(always)]
    pub fn reg_uhs_phy_ten(&mut self) -> RegUhsPhyTenW<'_, PhyCfg40Spec> {
        RegUhsPhyTenW::new(self, 24)
    }
    #[doc = "Bit 25 - SoC AON enable"]
    #[inline(always)]
    pub fn soc_en_aon(&mut self) -> SocEnAonW<'_, PhyCfg40Spec> {
        SocEnAonW::new(self, 25)
    }
    #[doc = "Bit 26 - PHY enable"]
    #[inline(always)]
    pub fn ten_uhs_phy(&mut self) -> TenUhsPhyW<'_, PhyCfg40Spec> {
        TenUhsPhyW::new(self, 26)
    }
    #[doc = "Bit 27 - PHY digital enable"]
    #[inline(always)]
    pub fn ten_uhs_phy_dig(&mut self) -> TenUhsPhyDigW<'_, PhyCfg40Spec> {
        TenUhsPhyDigW::new(self, 27)
    }
    #[doc = "Bit 29 - Hardware clock gate enable"]
    #[inline(always)]
    pub fn tx_clktree_gate_hw(&mut self) -> TxClktreeGateHwW<'_, PhyCfg40Spec> {
        TxClktreeGateHwW::new(self, 29)
    }
    #[doc = "Bit 30 - UHS DC test point output enable"]
    #[inline(always)]
    pub fn uhs_dc_tp_out_en(&mut self) -> UhsDcTpOutEnW<'_, PhyCfg40Spec> {
        UhsDcTpOutEnW::new(self, 30)
    }
    #[doc = "Bit 31 - UHS PHY differential DQS enable"]
    #[inline(always)]
    pub fn uhs_phy_dqs_diff(&mut self) -> UhsPhyDqsDiffW<'_, PhyCfg40Spec> {
        UhsPhyDqsDiffW::new(self, 31)
    }
}
#[doc = "PHY Power and VREF Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg40Spec;
impl crate::RegisterSpec for PhyCfg40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_40::R`](R) reader structure"]
impl crate::Readable for PhyCfg40Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_40::W`](W) writer structure"]
impl crate::Writable for PhyCfg40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_40 to value 0xa2ff_0000"]
impl crate::Resettable for PhyCfg40Spec {
    const RESET_VALUE: u32 = 0xa2ff_0000;
}
