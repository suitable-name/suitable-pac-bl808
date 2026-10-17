#[doc = "Register `PHY_CFG_24` reader"]
pub type R = crate::R<PhyCfg24Spec>;
#[doc = "Register `PHY_CFG_24` writer"]
pub type W = crate::W<PhyCfg24Spec>;
#[doc = "Field `dq15_sr` reader - DQ15 slew rate"]
pub type Dq15SrR = crate::FieldReader;
#[doc = "Field `dq15_sr` writer - DQ15 slew rate"]
pub type Dq15SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq15_dly_rx` reader - DQ15 RX delay"]
pub type Dq15DlyRxR = crate::FieldReader;
#[doc = "Field `dq15_dly_rx` writer - DQ15 RX delay"]
pub type Dq15DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq15_dly_drv` reader - DQ15 drive delay"]
pub type Dq15DlyDrvR = crate::FieldReader;
#[doc = "Field `dq15_dly_drv` writer - DQ15 drive delay"]
pub type Dq15DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq14_sr` reader - DQ14 slew rate"]
pub type Dq14SrR = crate::FieldReader;
#[doc = "Field `dq14_sr` writer - DQ14 slew rate"]
pub type Dq14SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq14_dly_rx` reader - DQ14 RX delay"]
pub type Dq14DlyRxR = crate::FieldReader;
#[doc = "Field `dq14_dly_rx` writer - DQ14 RX delay"]
pub type Dq14DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq14_dly_drv` reader - DQ14 drive delay"]
pub type Dq14DlyDrvR = crate::FieldReader;
#[doc = "Field `dq14_dly_drv` writer - DQ14 drive delay"]
pub type Dq14DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - DQ15 slew rate"]
    #[inline(always)]
    pub fn dq15_sr(&self) -> Dq15SrR {
        Dq15SrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - DQ15 RX delay"]
    #[inline(always)]
    pub fn dq15_dly_rx(&self) -> Dq15DlyRxR {
        Dq15DlyRxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQ15 drive delay"]
    #[inline(always)]
    pub fn dq15_dly_drv(&self) -> Dq15DlyDrvR {
        Dq15DlyDrvR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - DQ14 slew rate"]
    #[inline(always)]
    pub fn dq14_sr(&self) -> Dq14SrR {
        Dq14SrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DQ14 RX delay"]
    #[inline(always)]
    pub fn dq14_dly_rx(&self) -> Dq14DlyRxR {
        Dq14DlyRxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQ14 drive delay"]
    #[inline(always)]
    pub fn dq14_dly_drv(&self) -> Dq14DlyDrvR {
        Dq14DlyDrvR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DQ15 slew rate"]
    #[inline(always)]
    pub fn dq15_sr(&mut self) -> Dq15SrW<'_, PhyCfg24Spec> {
        Dq15SrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQ15 RX delay"]
    #[inline(always)]
    pub fn dq15_dly_rx(&mut self) -> Dq15DlyRxW<'_, PhyCfg24Spec> {
        Dq15DlyRxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DQ15 drive delay"]
    #[inline(always)]
    pub fn dq15_dly_drv(&mut self) -> Dq15DlyDrvW<'_, PhyCfg24Spec> {
        Dq15DlyDrvW::new(self, 12)
    }
    #[doc = "Bits 16:17 - DQ14 slew rate"]
    #[inline(always)]
    pub fn dq14_sr(&mut self) -> Dq14SrW<'_, PhyCfg24Spec> {
        Dq14SrW::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQ14 RX delay"]
    #[inline(always)]
    pub fn dq14_dly_rx(&mut self) -> Dq14DlyRxW<'_, PhyCfg24Spec> {
        Dq14DlyRxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQ14 drive delay"]
    #[inline(always)]
    pub fn dq14_dly_drv(&mut self) -> Dq14DlyDrvW<'_, PhyCfg24Spec> {
        Dq14DlyDrvW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 24 (DQ15/DQ14)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg24Spec;
impl crate::RegisterSpec for PhyCfg24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_24::R`](R) reader structure"]
impl crate::Readable for PhyCfg24Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_24::W`](W) writer structure"]
impl crate::Writable for PhyCfg24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_24 to value 0x8100_8100"]
impl crate::Resettable for PhyCfg24Spec {
    const RESET_VALUE: u32 = 0x8100_8100;
}
