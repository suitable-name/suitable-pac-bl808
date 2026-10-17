#[doc = "Register `PHY_CFG_30` reader"]
pub type R = crate::R<PhyCfg30Spec>;
#[doc = "Register `PHY_CFG_30` writer"]
pub type W = crate::W<PhyCfg30Spec>;
#[doc = "Field `phy_wl_dq_dig` reader - Write Latency DQ digital"]
pub type PhyWlDqDigR = crate::FieldReader;
#[doc = "Field `phy_wl_dq_dig` writer - Write Latency DQ digital"]
pub type PhyWlDqDigW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `phy_wl_dq_ana` reader - Write Latency DQ analog"]
pub type PhyWlDqAnaR = crate::FieldReader;
#[doc = "Field `phy_wl_dq_ana` writer - Write Latency DQ analog"]
pub type PhyWlDqAnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `phy_wl_dig` reader - Write Latency digital"]
pub type PhyWlDigR = crate::FieldReader;
#[doc = "Field `phy_wl_dig` writer - Write Latency digital"]
pub type PhyWlDigW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `phy_wl_ana` reader - Write Latency analog"]
pub type PhyWlAnaR = crate::FieldReader;
#[doc = "Field `phy_wl_ana` writer - Write Latency analog"]
pub type PhyWlAnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `phy_rl_dig` reader - Read Latency digital"]
pub type PhyRlDigR = crate::FieldReader;
#[doc = "Field `phy_rl_dig` writer - Read Latency digital"]
pub type PhyRlDigW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `phy_rl_ana` reader - Read Latency analog"]
pub type PhyRlAnaR = crate::FieldReader;
#[doc = "Field `phy_rl_ana` writer - Read Latency analog"]
pub type PhyRlAnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `oe_timer` reader - Output Enable timer"]
pub type OeTimerR = crate::FieldReader;
#[doc = "Field `oe_timer` writer - Output Enable timer"]
pub type OeTimerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `vref_mode` reader - VREF mode"]
pub type VrefModeR = crate::BitReader;
#[doc = "Field `vref_mode` writer - VREF mode"]
pub type VrefModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `oe_ctrl_hw` reader - Hardware OE control enable"]
pub type OeCtrlHwR = crate::BitReader;
#[doc = "Field `oe_ctrl_hw` writer - Hardware OE control enable"]
pub type OeCtrlHwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `odt_sel` reader - ODT selection"]
pub type OdtSelR = crate::FieldReader;
#[doc = "Field `odt_sel` writer - ODT selection"]
pub type OdtSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Write Latency DQ digital"]
    #[inline(always)]
    pub fn phy_wl_dq_dig(&self) -> PhyWlDqDigR {
        PhyWlDqDigR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Write Latency DQ analog"]
    #[inline(always)]
    pub fn phy_wl_dq_ana(&self) -> PhyWlDqAnaR {
        PhyWlDqAnaR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Write Latency digital"]
    #[inline(always)]
    pub fn phy_wl_dig(&self) -> PhyWlDigR {
        PhyWlDigR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Write Latency analog"]
    #[inline(always)]
    pub fn phy_wl_ana(&self) -> PhyWlAnaR {
        PhyWlAnaR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Read Latency digital"]
    #[inline(always)]
    pub fn phy_rl_dig(&self) -> PhyRlDigR {
        PhyRlDigR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Read Latency analog"]
    #[inline(always)]
    pub fn phy_rl_ana(&self) -> PhyRlAnaR {
        PhyRlAnaR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Output Enable timer"]
    #[inline(always)]
    pub fn oe_timer(&self) -> OeTimerR {
        OeTimerR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - VREF mode"]
    #[inline(always)]
    pub fn vref_mode(&self) -> VrefModeR {
        VrefModeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Hardware OE control enable"]
    #[inline(always)]
    pub fn oe_ctrl_hw(&self) -> OeCtrlHwR {
        OeCtrlHwR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - ODT selection"]
    #[inline(always)]
    pub fn odt_sel(&self) -> OdtSelR {
        OdtSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Write Latency DQ digital"]
    #[inline(always)]
    pub fn phy_wl_dq_dig(&mut self) -> PhyWlDqDigW<'_, PhyCfg30Spec> {
        PhyWlDqDigW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Write Latency DQ analog"]
    #[inline(always)]
    pub fn phy_wl_dq_ana(&mut self) -> PhyWlDqAnaW<'_, PhyCfg30Spec> {
        PhyWlDqAnaW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Write Latency digital"]
    #[inline(always)]
    pub fn phy_wl_dig(&mut self) -> PhyWlDigW<'_, PhyCfg30Spec> {
        PhyWlDigW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Write Latency analog"]
    #[inline(always)]
    pub fn phy_wl_ana(&mut self) -> PhyWlAnaW<'_, PhyCfg30Spec> {
        PhyWlAnaW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Read Latency digital"]
    #[inline(always)]
    pub fn phy_rl_dig(&mut self) -> PhyRlDigW<'_, PhyCfg30Spec> {
        PhyRlDigW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Read Latency analog"]
    #[inline(always)]
    pub fn phy_rl_ana(&mut self) -> PhyRlAnaW<'_, PhyCfg30Spec> {
        PhyRlAnaW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Output Enable timer"]
    #[inline(always)]
    pub fn oe_timer(&mut self) -> OeTimerW<'_, PhyCfg30Spec> {
        OeTimerW::new(self, 24)
    }
    #[doc = "Bit 26 - VREF mode"]
    #[inline(always)]
    pub fn vref_mode(&mut self) -> VrefModeW<'_, PhyCfg30Spec> {
        VrefModeW::new(self, 26)
    }
    #[doc = "Bit 27 - Hardware OE control enable"]
    #[inline(always)]
    pub fn oe_ctrl_hw(&mut self) -> OeCtrlHwW<'_, PhyCfg30Spec> {
        OeCtrlHwW::new(self, 27)
    }
    #[doc = "Bits 28:31 - ODT selection"]
    #[inline(always)]
    pub fn odt_sel(&mut self) -> OdtSelW<'_, PhyCfg30Spec> {
        OdtSelW::new(self, 28)
    }
}
#[doc = "PHY Latency and VREF Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg30Spec;
impl crate::RegisterSpec for PhyCfg30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_30::R`](R) reader structure"]
impl crate::Readable for PhyCfg30Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_30::W`](W) writer structure"]
impl crate::Writable for PhyCfg30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_30 to value 0xaa33_1020"]
impl crate::Resettable for PhyCfg30Spec {
    const RESET_VALUE: u32 = 0xaa33_1020;
}
