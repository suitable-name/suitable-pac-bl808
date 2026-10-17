#[doc = "Register `PHY_CFG_2C` reader"]
pub type R = crate::R<PhyCfg2cSpec>;
#[doc = "Register `PHY_CFG_2C` writer"]
pub type W = crate::W<PhyCfg2cSpec>;
#[doc = "Field `ipp5un_lpddr` reader - LPDDR bias control"]
pub type Ipp5unLpddrR = crate::BitReader;
#[doc = "Field `ipp5un_lpddr` writer - LPDDR bias control"]
pub type Ipp5unLpddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_rx_fe` reader - RX Front-End enable"]
pub type EnRxFeR = crate::BitReader;
#[doc = "Field `en_rx_fe` writer - RX Front-End enable"]
pub type EnRxFeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_bias` reader - PHY bias enable"]
pub type EnBiasR = crate::BitReader;
#[doc = "Field `en_bias` writer - PHY bias enable"]
pub type EnBiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dqs1n_dly_rx` reader - DQS1N RX delay"]
pub type Dqs1nDlyRxR = crate::FieldReader;
#[doc = "Field `dqs1n_dly_rx` writer - DQS1N RX delay"]
pub type Dqs1nDlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dqs1_sr` reader - DQS1 slew rate"]
pub type Dqs1SrR = crate::FieldReader;
#[doc = "Field `dqs1_sr` writer - DQS1 slew rate"]
pub type Dqs1SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dqs1_sel` reader - DQS1 source selection"]
pub type Dqs1SelR = crate::FieldReader;
#[doc = "Field `dqs1_sel` writer - DQS1 source selection"]
pub type Dqs1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dqs1_dly_rx` reader - DQS1 RX delay"]
pub type Dqs1DlyRxR = crate::FieldReader;
#[doc = "Field `dqs1_dly_rx` writer - DQS1 RX delay"]
pub type Dqs1DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dqs1_dly_drv` reader - DQS1 drive delay"]
pub type Dqs1DlyDrvR = crate::FieldReader;
#[doc = "Field `dqs1_dly_drv` writer - DQS1 drive delay"]
pub type Dqs1DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dqs1_diff_dly_rx` reader - DQS1 differential RX delay"]
pub type Dqs1DiffDlyRxR = crate::FieldReader;
#[doc = "Field `dqs1_diff_dly_rx` writer - DQS1 differential RX delay"]
pub type Dqs1DiffDlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - LPDDR bias control"]
    #[inline(always)]
    pub fn ipp5un_lpddr(&self) -> Ipp5unLpddrR {
        Ipp5unLpddrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX Front-End enable"]
    #[inline(always)]
    pub fn en_rx_fe(&self) -> EnRxFeR {
        EnRxFeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PHY bias enable"]
    #[inline(always)]
    pub fn en_bias(&self) -> EnBiasR {
        EnBiasR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - DQS1N RX delay"]
    #[inline(always)]
    pub fn dqs1n_dly_rx(&self) -> Dqs1nDlyRxR {
        Dqs1nDlyRxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - DQS1 slew rate"]
    #[inline(always)]
    pub fn dqs1_sr(&self) -> Dqs1SrR {
        Dqs1SrR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DQS1 source selection"]
    #[inline(always)]
    pub fn dqs1_sel(&self) -> Dqs1SelR {
        Dqs1SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 20:23 - DQS1 RX delay"]
    #[inline(always)]
    pub fn dqs1_dly_rx(&self) -> Dqs1DlyRxR {
        Dqs1DlyRxR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DQS1 drive delay"]
    #[inline(always)]
    pub fn dqs1_dly_drv(&self) -> Dqs1DlyDrvR {
        Dqs1DlyDrvR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQS1 differential RX delay"]
    #[inline(always)]
    pub fn dqs1_diff_dly_rx(&self) -> Dqs1DiffDlyRxR {
        Dqs1DiffDlyRxR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPDDR bias control"]
    #[inline(always)]
    pub fn ipp5un_lpddr(&mut self) -> Ipp5unLpddrW<'_, PhyCfg2cSpec> {
        Ipp5unLpddrW::new(self, 0)
    }
    #[doc = "Bit 1 - RX Front-End enable"]
    #[inline(always)]
    pub fn en_rx_fe(&mut self) -> EnRxFeW<'_, PhyCfg2cSpec> {
        EnRxFeW::new(self, 1)
    }
    #[doc = "Bit 2 - PHY bias enable"]
    #[inline(always)]
    pub fn en_bias(&mut self) -> EnBiasW<'_, PhyCfg2cSpec> {
        EnBiasW::new(self, 2)
    }
    #[doc = "Bits 8:11 - DQS1N RX delay"]
    #[inline(always)]
    pub fn dqs1n_dly_rx(&mut self) -> Dqs1nDlyRxW<'_, PhyCfg2cSpec> {
        Dqs1nDlyRxW::new(self, 8)
    }
    #[doc = "Bits 12:13 - DQS1 slew rate"]
    #[inline(always)]
    pub fn dqs1_sr(&mut self) -> Dqs1SrW<'_, PhyCfg2cSpec> {
        Dqs1SrW::new(self, 12)
    }
    #[doc = "Bits 14:15 - DQS1 source selection"]
    #[inline(always)]
    pub fn dqs1_sel(&mut self) -> Dqs1SelW<'_, PhyCfg2cSpec> {
        Dqs1SelW::new(self, 14)
    }
    #[doc = "Bits 20:23 - DQS1 RX delay"]
    #[inline(always)]
    pub fn dqs1_dly_rx(&mut self) -> Dqs1DlyRxW<'_, PhyCfg2cSpec> {
        Dqs1DlyRxW::new(self, 20)
    }
    #[doc = "Bits 24:27 - DQS1 drive delay"]
    #[inline(always)]
    pub fn dqs1_dly_drv(&mut self) -> Dqs1DlyDrvW<'_, PhyCfg2cSpec> {
        Dqs1DlyDrvW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQS1 differential RX delay"]
    #[inline(always)]
    pub fn dqs1_diff_dly_rx(&mut self) -> Dqs1DiffDlyRxW<'_, PhyCfg2cSpec> {
        Dqs1DiffDlyRxW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 2C (DQS1)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg2cSpec;
impl crate::RegisterSpec for PhyCfg2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_2c::R`](R) reader structure"]
impl crate::Readable for PhyCfg2cSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_2c::W`](W) writer structure"]
impl crate::Writable for PhyCfg2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_2C to value 0x3830_0306"]
impl crate::Resettable for PhyCfg2cSpec {
    const RESET_VALUE: u32 = 0x3830_0306;
}
