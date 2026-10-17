#[doc = "Register `PHY_CFG_18` reader"]
pub type R = crate::R<PhyCfg18Spec>;
#[doc = "Register `PHY_CFG_18` writer"]
pub type W = crate::W<PhyCfg18Spec>;
#[doc = "Field `dq9_sr` reader - DQ9 slew rate"]
pub type Dq9SrR = crate::FieldReader;
#[doc = "Field `dq9_sr` writer - DQ9 slew rate"]
pub type Dq9SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq9_dly_rx` reader - DQ9 RX delay"]
pub type Dq9DlyRxR = crate::FieldReader;
#[doc = "Field `dq9_dly_rx` writer - DQ9 RX delay"]
pub type Dq9DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq9_dly_drv` reader - DQ9 drive delay"]
pub type Dq9DlyDrvR = crate::FieldReader;
#[doc = "Field `dq9_dly_drv` writer - DQ9 drive delay"]
pub type Dq9DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq8_sr` reader - DQ8 slew rate"]
pub type Dq8SrR = crate::FieldReader;
#[doc = "Field `dq8_sr` writer - DQ8 slew rate"]
pub type Dq8SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq8_dly_rx` reader - DQ8 RX delay"]
pub type Dq8DlyRxR = crate::FieldReader;
#[doc = "Field `dq8_dly_rx` writer - DQ8 RX delay"]
pub type Dq8DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq8_dly_drv` reader - DQ8 drive delay"]
pub type Dq8DlyDrvR = crate::FieldReader;
#[doc = "Field `dq8_dly_drv` writer - DQ8 drive delay"]
pub type Dq8DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - DQ9 slew rate"]
    #[inline(always)]
    pub fn dq9_sr(&self) -> Dq9SrR {
        Dq9SrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - DQ9 RX delay"]
    #[inline(always)]
    pub fn dq9_dly_rx(&self) -> Dq9DlyRxR {
        Dq9DlyRxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQ9 drive delay"]
    #[inline(always)]
    pub fn dq9_dly_drv(&self) -> Dq9DlyDrvR {
        Dq9DlyDrvR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - DQ8 slew rate"]
    #[inline(always)]
    pub fn dq8_sr(&self) -> Dq8SrR {
        Dq8SrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DQ8 RX delay"]
    #[inline(always)]
    pub fn dq8_dly_rx(&self) -> Dq8DlyRxR {
        Dq8DlyRxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQ8 drive delay"]
    #[inline(always)]
    pub fn dq8_dly_drv(&self) -> Dq8DlyDrvR {
        Dq8DlyDrvR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DQ9 slew rate"]
    #[inline(always)]
    pub fn dq9_sr(&mut self) -> Dq9SrW<'_, PhyCfg18Spec> {
        Dq9SrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQ9 RX delay"]
    #[inline(always)]
    pub fn dq9_dly_rx(&mut self) -> Dq9DlyRxW<'_, PhyCfg18Spec> {
        Dq9DlyRxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DQ9 drive delay"]
    #[inline(always)]
    pub fn dq9_dly_drv(&mut self) -> Dq9DlyDrvW<'_, PhyCfg18Spec> {
        Dq9DlyDrvW::new(self, 12)
    }
    #[doc = "Bits 16:17 - DQ8 slew rate"]
    #[inline(always)]
    pub fn dq8_sr(&mut self) -> Dq8SrW<'_, PhyCfg18Spec> {
        Dq8SrW::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQ8 RX delay"]
    #[inline(always)]
    pub fn dq8_dly_rx(&mut self) -> Dq8DlyRxW<'_, PhyCfg18Spec> {
        Dq8DlyRxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQ8 drive delay"]
    #[inline(always)]
    pub fn dq8_dly_drv(&mut self) -> Dq8DlyDrvW<'_, PhyCfg18Spec> {
        Dq8DlyDrvW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 18 (DQ9/DQ8)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg18Spec;
impl crate::RegisterSpec for PhyCfg18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_18::R`](R) reader structure"]
impl crate::Readable for PhyCfg18Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_18::W`](W) writer structure"]
impl crate::Writable for PhyCfg18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_18 to value 0x8100_8100"]
impl crate::Resettable for PhyCfg18Spec {
    const RESET_VALUE: u32 = 0x8100_8100;
}
