#[doc = "Register `PHY_CFG_10` reader"]
pub type R = crate::R<PhyCfg10Spec>;
#[doc = "Register `PHY_CFG_10` writer"]
pub type W = crate::W<PhyCfg10Spec>;
#[doc = "Field `dq5_sr` reader - DQ5 slew rate"]
pub type Dq5SrR = crate::FieldReader;
#[doc = "Field `dq5_sr` writer - DQ5 slew rate"]
pub type Dq5SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq5_dly_rx` reader - DQ5 RX delay"]
pub type Dq5DlyRxR = crate::FieldReader;
#[doc = "Field `dq5_dly_rx` writer - DQ5 RX delay"]
pub type Dq5DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq5_dly_drv` reader - DQ5 drive delay"]
pub type Dq5DlyDrvR = crate::FieldReader;
#[doc = "Field `dq5_dly_drv` writer - DQ5 drive delay"]
pub type Dq5DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq4_sr` reader - DQ4 slew rate"]
pub type Dq4SrR = crate::FieldReader;
#[doc = "Field `dq4_sr` writer - DQ4 slew rate"]
pub type Dq4SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq4_dly_rx` reader - DQ4 RX delay"]
pub type Dq4DlyRxR = crate::FieldReader;
#[doc = "Field `dq4_dly_rx` writer - DQ4 RX delay"]
pub type Dq4DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq4_dly_drv` reader - DQ4 drive delay"]
pub type Dq4DlyDrvR = crate::FieldReader;
#[doc = "Field `dq4_dly_drv` writer - DQ4 drive delay"]
pub type Dq4DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - DQ5 slew rate"]
    #[inline(always)]
    pub fn dq5_sr(&self) -> Dq5SrR {
        Dq5SrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - DQ5 RX delay"]
    #[inline(always)]
    pub fn dq5_dly_rx(&self) -> Dq5DlyRxR {
        Dq5DlyRxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQ5 drive delay"]
    #[inline(always)]
    pub fn dq5_dly_drv(&self) -> Dq5DlyDrvR {
        Dq5DlyDrvR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - DQ4 slew rate"]
    #[inline(always)]
    pub fn dq4_sr(&self) -> Dq4SrR {
        Dq4SrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DQ4 RX delay"]
    #[inline(always)]
    pub fn dq4_dly_rx(&self) -> Dq4DlyRxR {
        Dq4DlyRxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQ4 drive delay"]
    #[inline(always)]
    pub fn dq4_dly_drv(&self) -> Dq4DlyDrvR {
        Dq4DlyDrvR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DQ5 slew rate"]
    #[inline(always)]
    pub fn dq5_sr(&mut self) -> Dq5SrW<'_, PhyCfg10Spec> {
        Dq5SrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQ5 RX delay"]
    #[inline(always)]
    pub fn dq5_dly_rx(&mut self) -> Dq5DlyRxW<'_, PhyCfg10Spec> {
        Dq5DlyRxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DQ5 drive delay"]
    #[inline(always)]
    pub fn dq5_dly_drv(&mut self) -> Dq5DlyDrvW<'_, PhyCfg10Spec> {
        Dq5DlyDrvW::new(self, 12)
    }
    #[doc = "Bits 16:17 - DQ4 slew rate"]
    #[inline(always)]
    pub fn dq4_sr(&mut self) -> Dq4SrW<'_, PhyCfg10Spec> {
        Dq4SrW::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQ4 RX delay"]
    #[inline(always)]
    pub fn dq4_dly_rx(&mut self) -> Dq4DlyRxW<'_, PhyCfg10Spec> {
        Dq4DlyRxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQ4 drive delay"]
    #[inline(always)]
    pub fn dq4_dly_drv(&mut self) -> Dq4DlyDrvW<'_, PhyCfg10Spec> {
        Dq4DlyDrvW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 10 (DQ5/DQ4)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg10Spec;
impl crate::RegisterSpec for PhyCfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_10::R`](R) reader structure"]
impl crate::Readable for PhyCfg10Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_10::W`](W) writer structure"]
impl crate::Writable for PhyCfg10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_10 to value 0x8100_8100"]
impl crate::Resettable for PhyCfg10Spec {
    const RESET_VALUE: u32 = 0x8100_8100;
}
