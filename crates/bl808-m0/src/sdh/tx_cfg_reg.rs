#[doc = "Register `TX_CFG_REG` reader"]
pub type R = crate::R<TxCfgRegSpec>;
#[doc = "Register `TX_CFG_REG` writer"]
pub type W = crate::W<TxCfgRegSpec>;
#[doc = "Field `TX_HOLD_DELAY0` reader - "]
pub type TxHoldDelay0R = crate::FieldReader<u16>;
#[doc = "Field `TX_HOLD_DELAY0` writer - "]
pub type TxHoldDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_HOLD_DELAY1` reader - "]
pub type TxHoldDelay1R = crate::FieldReader<u16>;
#[doc = "Field `TX_HOLD_DELAY1` writer - "]
pub type TxHoldDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_INT_CLK_SEL` reader - "]
pub type TxIntClkSelR = crate::BitReader;
#[doc = "Field `TX_INT_CLK_SEL` writer - "]
pub type TxIntClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_MUX_SEL` reader - "]
pub type TxMuxSelR = crate::BitReader;
#[doc = "Field `TX_MUX_SEL` writer - "]
pub type TxMuxSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_hold_delay0(&self) -> TxHoldDelay0R {
        TxHoldDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tx_hold_delay1(&self) -> TxHoldDelay1R {
        TxHoldDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tx_int_clk_sel(&self) -> TxIntClkSelR {
        TxIntClkSelR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_mux_sel(&self) -> TxMuxSelR {
        TxMuxSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_hold_delay0(&mut self) -> TxHoldDelay0W<'_, TxCfgRegSpec> {
        TxHoldDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tx_hold_delay1(&mut self) -> TxHoldDelay1W<'_, TxCfgRegSpec> {
        TxHoldDelay1W::new(self, 16)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tx_int_clk_sel(&mut self) -> TxIntClkSelW<'_, TxCfgRegSpec> {
        TxIntClkSelW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tx_mux_sel(&mut self) -> TxMuxSelW<'_, TxCfgRegSpec> {
        TxMuxSelW::new(self, 31)
    }
}
#[doc = "Vendor specific TX clock output delay and muxing configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cfg_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_cfg_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCfgRegSpec;
impl crate::RegisterSpec for TxCfgRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cfg_reg::R`](R) reader structure"]
impl crate::Readable for TxCfgRegSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_cfg_reg::W`](W) writer structure"]
impl crate::Writable for TxCfgRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CFG_REG to value 0"]
impl crate::Resettable for TxCfgRegSpec {}
