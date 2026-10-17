#[doc = "Register `PHY_CFG_28` reader"]
pub type R = crate::R<PhyCfg28Spec>;
#[doc = "Register `PHY_CFG_28` writer"]
pub type W = crate::W<PhyCfg28Spec>;
#[doc = "Field `dqs0n_dly_rx` reader - DQS0N RX delay"]
pub type Dqs0nDlyRxR = crate::FieldReader;
#[doc = "Field `dqs0n_dly_rx` writer - DQS0N RX delay"]
pub type Dqs0nDlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dqs0_sr` reader - DQS0 slew rate"]
pub type Dqs0SrR = crate::FieldReader;
#[doc = "Field `dqs0_sr` writer - DQS0 slew rate"]
pub type Dqs0SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dqs0_sel` reader - DQS0 source selection"]
pub type Dqs0SelR = crate::FieldReader;
#[doc = "Field `dqs0_sel` writer - DQS0 source selection"]
pub type Dqs0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dqs0_dly_rx` reader - DQS0 RX delay"]
pub type Dqs0DlyRxR = crate::FieldReader;
#[doc = "Field `dqs0_dly_rx` writer - DQS0 RX delay"]
pub type Dqs0DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dqs0_dly_drv` reader - DQS0 drive delay"]
pub type Dqs0DlyDrvR = crate::FieldReader;
#[doc = "Field `dqs0_dly_drv` writer - DQS0 drive delay"]
pub type Dqs0DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dqs0_diff_dly_rx` reader - DQS0 differential RX delay"]
pub type Dqs0DiffDlyRxR = crate::FieldReader;
#[doc = "Field `dqs0_diff_dly_rx` writer - DQS0 differential RX delay"]
pub type Dqs0DiffDlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:11 - DQS0N RX delay"]
    #[inline(always)]
    pub fn dqs0n_dly_rx(&self) -> Dqs0nDlyRxR {
        Dqs0nDlyRxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - DQS0 slew rate"]
    #[inline(always)]
    pub fn dqs0_sr(&self) -> Dqs0SrR {
        Dqs0SrR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DQS0 source selection"]
    #[inline(always)]
    pub fn dqs0_sel(&self) -> Dqs0SelR {
        Dqs0SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 20:23 - DQS0 RX delay"]
    #[inline(always)]
    pub fn dqs0_dly_rx(&self) -> Dqs0DlyRxR {
        Dqs0DlyRxR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DQS0 drive delay"]
    #[inline(always)]
    pub fn dqs0_dly_drv(&self) -> Dqs0DlyDrvR {
        Dqs0DlyDrvR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQS0 differential RX delay"]
    #[inline(always)]
    pub fn dqs0_diff_dly_rx(&self) -> Dqs0DiffDlyRxR {
        Dqs0DiffDlyRxR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - DQS0N RX delay"]
    #[inline(always)]
    pub fn dqs0n_dly_rx(&mut self) -> Dqs0nDlyRxW<'_, PhyCfg28Spec> {
        Dqs0nDlyRxW::new(self, 8)
    }
    #[doc = "Bits 12:13 - DQS0 slew rate"]
    #[inline(always)]
    pub fn dqs0_sr(&mut self) -> Dqs0SrW<'_, PhyCfg28Spec> {
        Dqs0SrW::new(self, 12)
    }
    #[doc = "Bits 14:15 - DQS0 source selection"]
    #[inline(always)]
    pub fn dqs0_sel(&mut self) -> Dqs0SelW<'_, PhyCfg28Spec> {
        Dqs0SelW::new(self, 14)
    }
    #[doc = "Bits 20:23 - DQS0 RX delay"]
    #[inline(always)]
    pub fn dqs0_dly_rx(&mut self) -> Dqs0DlyRxW<'_, PhyCfg28Spec> {
        Dqs0DlyRxW::new(self, 20)
    }
    #[doc = "Bits 24:27 - DQS0 drive delay"]
    #[inline(always)]
    pub fn dqs0_dly_drv(&mut self) -> Dqs0DlyDrvW<'_, PhyCfg28Spec> {
        Dqs0DlyDrvW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQS0 differential RX delay"]
    #[inline(always)]
    pub fn dqs0_diff_dly_rx(&mut self) -> Dqs0DiffDlyRxW<'_, PhyCfg28Spec> {
        Dqs0DiffDlyRxW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 28 (DQS0)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg28Spec;
impl crate::RegisterSpec for PhyCfg28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_28::R`](R) reader structure"]
impl crate::Readable for PhyCfg28Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_28::W`](W) writer structure"]
impl crate::Writable for PhyCfg28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_28 to value 0x3830_0300"]
impl crate::Resettable for PhyCfg28Spec {
    const RESET_VALUE: u32 = 0x3830_0300;
}
