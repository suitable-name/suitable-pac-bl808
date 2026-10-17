#[doc = "Register `ETH_CFG0` reader"]
pub type R = crate::R<EthCfg0Spec>;
#[doc = "Register `ETH_CFG0` writer"]
pub type W = crate::W<EthCfg0Spec>;
#[doc = "Field `cfg_sel_eth_ref_clk_o` reader - "]
pub type CfgSelEthRefClkOR = crate::BitReader;
#[doc = "Field `cfg_sel_eth_ref_clk_o` writer - "]
pub type CfgSelEthRefClkOW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_inv_eth_ref_clk_o` reader - "]
pub type CfgInvEthRefClkOR = crate::BitReader;
#[doc = "Field `cfg_inv_eth_ref_clk_o` writer - "]
pub type CfgInvEthRefClkOW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_inv_eth_tx_clk` reader - "]
pub type CfgInvEthTxClkR = crate::BitReader;
#[doc = "Field `cfg_inv_eth_tx_clk` writer - "]
pub type CfgInvEthTxClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_inv_eth_rx_clk` reader - "]
pub type CfgInvEthRxClkR = crate::BitReader;
#[doc = "Field `cfg_inv_eth_rx_clk` writer - "]
pub type CfgInvEthRxClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cfg_sel_eth_ref_clk_o(&self) -> CfgSelEthRefClkOR {
        CfgSelEthRefClkOR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cfg_inv_eth_ref_clk_o(&self) -> CfgInvEthRefClkOR {
        CfgInvEthRefClkOR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_inv_eth_tx_clk(&self) -> CfgInvEthTxClkR {
        CfgInvEthTxClkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_inv_eth_rx_clk(&self) -> CfgInvEthRxClkR {
        CfgInvEthRxClkR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cfg_sel_eth_ref_clk_o(&mut self) -> CfgSelEthRefClkOW<'_, EthCfg0Spec> {
        CfgSelEthRefClkOW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cfg_inv_eth_ref_clk_o(&mut self) -> CfgInvEthRefClkOW<'_, EthCfg0Spec> {
        CfgInvEthRefClkOW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_inv_eth_tx_clk(&mut self) -> CfgInvEthTxClkW<'_, EthCfg0Spec> {
        CfgInvEthTxClkW::new(self, 7)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_inv_eth_rx_clk(&mut self) -> CfgInvEthRxClkW<'_, EthCfg0Spec> {
        CfgInvEthRxClkW::new(self, 10)
    }
}
#[doc = "Ethernet Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`eth_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EthCfg0Spec;
impl crate::RegisterSpec for EthCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_cfg0::R`](R) reader structure"]
impl crate::Readable for EthCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`eth_cfg0::W`](W) writer structure"]
impl crate::Writable for EthCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETH_CFG0 to value 0"]
impl crate::Resettable for EthCfg0Spec {}
