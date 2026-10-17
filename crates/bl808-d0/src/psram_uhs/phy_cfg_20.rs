#[doc = "Register `PHY_CFG_20` reader"]
pub type R = crate::R<PhyCfg20Spec>;
#[doc = "Register `PHY_CFG_20` writer"]
pub type W = crate::W<PhyCfg20Spec>;
#[doc = "Field `dq13_sr` reader - DQ13 slew rate"]
pub type Dq13SrR = crate::FieldReader;
#[doc = "Field `dq13_sr` writer - DQ13 slew rate"]
pub type Dq13SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq13_dly_rx` reader - DQ13 RX delay"]
pub type Dq13DlyRxR = crate::FieldReader;
#[doc = "Field `dq13_dly_rx` writer - DQ13 RX delay"]
pub type Dq13DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq13_dly_drv` reader - DQ13 drive delay"]
pub type Dq13DlyDrvR = crate::FieldReader;
#[doc = "Field `dq13_dly_drv` writer - DQ13 drive delay"]
pub type Dq13DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq12_sr` reader - DQ12 slew rate"]
pub type Dq12SrR = crate::FieldReader;
#[doc = "Field `dq12_sr` writer - DQ12 slew rate"]
pub type Dq12SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq12_dly_rx` reader - DQ12 RX delay"]
pub type Dq12DlyRxR = crate::FieldReader;
#[doc = "Field `dq12_dly_rx` writer - DQ12 RX delay"]
pub type Dq12DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq12_dly_drv` reader - DQ12 drive delay"]
pub type Dq12DlyDrvR = crate::FieldReader;
#[doc = "Field `dq12_dly_drv` writer - DQ12 drive delay"]
pub type Dq12DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - DQ13 slew rate"]
    #[inline(always)]
    pub fn dq13_sr(&self) -> Dq13SrR {
        Dq13SrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - DQ13 RX delay"]
    #[inline(always)]
    pub fn dq13_dly_rx(&self) -> Dq13DlyRxR {
        Dq13DlyRxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQ13 drive delay"]
    #[inline(always)]
    pub fn dq13_dly_drv(&self) -> Dq13DlyDrvR {
        Dq13DlyDrvR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - DQ12 slew rate"]
    #[inline(always)]
    pub fn dq12_sr(&self) -> Dq12SrR {
        Dq12SrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DQ12 RX delay"]
    #[inline(always)]
    pub fn dq12_dly_rx(&self) -> Dq12DlyRxR {
        Dq12DlyRxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQ12 drive delay"]
    #[inline(always)]
    pub fn dq12_dly_drv(&self) -> Dq12DlyDrvR {
        Dq12DlyDrvR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DQ13 slew rate"]
    #[inline(always)]
    pub fn dq13_sr(&mut self) -> Dq13SrW<'_, PhyCfg20Spec> {
        Dq13SrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQ13 RX delay"]
    #[inline(always)]
    pub fn dq13_dly_rx(&mut self) -> Dq13DlyRxW<'_, PhyCfg20Spec> {
        Dq13DlyRxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DQ13 drive delay"]
    #[inline(always)]
    pub fn dq13_dly_drv(&mut self) -> Dq13DlyDrvW<'_, PhyCfg20Spec> {
        Dq13DlyDrvW::new(self, 12)
    }
    #[doc = "Bits 16:17 - DQ12 slew rate"]
    #[inline(always)]
    pub fn dq12_sr(&mut self) -> Dq12SrW<'_, PhyCfg20Spec> {
        Dq12SrW::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQ12 RX delay"]
    #[inline(always)]
    pub fn dq12_dly_rx(&mut self) -> Dq12DlyRxW<'_, PhyCfg20Spec> {
        Dq12DlyRxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQ12 drive delay"]
    #[inline(always)]
    pub fn dq12_dly_drv(&mut self) -> Dq12DlyDrvW<'_, PhyCfg20Spec> {
        Dq12DlyDrvW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 20 (DQ13/DQ12)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg20Spec;
impl crate::RegisterSpec for PhyCfg20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_20::R`](R) reader structure"]
impl crate::Readable for PhyCfg20Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_20::W`](W) writer structure"]
impl crate::Writable for PhyCfg20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_20 to value 0x8100_8100"]
impl crate::Resettable for PhyCfg20Spec {
    const RESET_VALUE: u32 = 0x8100_8100;
}
