#[doc = "Register `PHY_CFG_14` reader"]
pub type R = crate::R<PhyCfg14Spec>;
#[doc = "Register `PHY_CFG_14` writer"]
pub type W = crate::W<PhyCfg14Spec>;
#[doc = "Field `dq7_sr` reader - DQ7 slew rate"]
pub type Dq7SrR = crate::FieldReader;
#[doc = "Field `dq7_sr` writer - DQ7 slew rate"]
pub type Dq7SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq7_dly_rx` reader - DQ7 RX delay"]
pub type Dq7DlyRxR = crate::FieldReader;
#[doc = "Field `dq7_dly_rx` writer - DQ7 RX delay"]
pub type Dq7DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq7_dly_drv` reader - DQ7 drive delay"]
pub type Dq7DlyDrvR = crate::FieldReader;
#[doc = "Field `dq7_dly_drv` writer - DQ7 drive delay"]
pub type Dq7DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq6_sr` reader - DQ6 slew rate"]
pub type Dq6SrR = crate::FieldReader;
#[doc = "Field `dq6_sr` writer - DQ6 slew rate"]
pub type Dq6SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq6_dly_rx` reader - DQ6 RX delay"]
pub type Dq6DlyRxR = crate::FieldReader;
#[doc = "Field `dq6_dly_rx` writer - DQ6 RX delay"]
pub type Dq6DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq6_dly_drv` reader - DQ6 drive delay"]
pub type Dq6DlyDrvR = crate::FieldReader;
#[doc = "Field `dq6_dly_drv` writer - DQ6 drive delay"]
pub type Dq6DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - DQ7 slew rate"]
    #[inline(always)]
    pub fn dq7_sr(&self) -> Dq7SrR {
        Dq7SrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - DQ7 RX delay"]
    #[inline(always)]
    pub fn dq7_dly_rx(&self) -> Dq7DlyRxR {
        Dq7DlyRxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQ7 drive delay"]
    #[inline(always)]
    pub fn dq7_dly_drv(&self) -> Dq7DlyDrvR {
        Dq7DlyDrvR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - DQ6 slew rate"]
    #[inline(always)]
    pub fn dq6_sr(&self) -> Dq6SrR {
        Dq6SrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DQ6 RX delay"]
    #[inline(always)]
    pub fn dq6_dly_rx(&self) -> Dq6DlyRxR {
        Dq6DlyRxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQ6 drive delay"]
    #[inline(always)]
    pub fn dq6_dly_drv(&self) -> Dq6DlyDrvR {
        Dq6DlyDrvR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DQ7 slew rate"]
    #[inline(always)]
    pub fn dq7_sr(&mut self) -> Dq7SrW<'_, PhyCfg14Spec> {
        Dq7SrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQ7 RX delay"]
    #[inline(always)]
    pub fn dq7_dly_rx(&mut self) -> Dq7DlyRxW<'_, PhyCfg14Spec> {
        Dq7DlyRxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DQ7 drive delay"]
    #[inline(always)]
    pub fn dq7_dly_drv(&mut self) -> Dq7DlyDrvW<'_, PhyCfg14Spec> {
        Dq7DlyDrvW::new(self, 12)
    }
    #[doc = "Bits 16:17 - DQ6 slew rate"]
    #[inline(always)]
    pub fn dq6_sr(&mut self) -> Dq6SrW<'_, PhyCfg14Spec> {
        Dq6SrW::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQ6 RX delay"]
    #[inline(always)]
    pub fn dq6_dly_rx(&mut self) -> Dq6DlyRxW<'_, PhyCfg14Spec> {
        Dq6DlyRxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQ6 drive delay"]
    #[inline(always)]
    pub fn dq6_dly_drv(&mut self) -> Dq6DlyDrvW<'_, PhyCfg14Spec> {
        Dq6DlyDrvW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 14 (DQ7/DQ6)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg14Spec;
impl crate::RegisterSpec for PhyCfg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_14::R`](R) reader structure"]
impl crate::Readable for PhyCfg14Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_14::W`](W) writer structure"]
impl crate::Writable for PhyCfg14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_14 to value 0x8100_8100"]
impl crate::Resettable for PhyCfg14Spec {
    const RESET_VALUE: u32 = 0x8100_8100;
}
