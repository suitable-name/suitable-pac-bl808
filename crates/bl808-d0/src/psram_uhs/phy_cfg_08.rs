#[doc = "Register `PHY_CFG_08` reader"]
pub type R = crate::R<PhyCfg08Spec>;
#[doc = "Register `PHY_CFG_08` writer"]
pub type W = crate::W<PhyCfg08Spec>;
#[doc = "Field `dq1_sr` reader - DQ1 slew rate"]
pub type Dq1SrR = crate::FieldReader;
#[doc = "Field `dq1_sr` writer - DQ1 slew rate"]
pub type Dq1SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq1_dly_rx` reader - DQ1 RX delay"]
pub type Dq1DlyRxR = crate::FieldReader;
#[doc = "Field `dq1_dly_rx` writer - DQ1 RX delay"]
pub type Dq1DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq1_dly_drv` reader - DQ1 drive delay"]
pub type Dq1DlyDrvR = crate::FieldReader;
#[doc = "Field `dq1_dly_drv` writer - DQ1 drive delay"]
pub type Dq1DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq0_sr` reader - DQ0 slew rate"]
pub type Dq0SrR = crate::FieldReader;
#[doc = "Field `dq0_sr` writer - DQ0 slew rate"]
pub type Dq0SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq0_dly_rx` reader - DQ0 RX delay"]
pub type Dq0DlyRxR = crate::FieldReader;
#[doc = "Field `dq0_dly_rx` writer - DQ0 RX delay"]
pub type Dq0DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq0_dly_drv` reader - DQ0 drive delay"]
pub type Dq0DlyDrvR = crate::FieldReader;
#[doc = "Field `dq0_dly_drv` writer - DQ0 drive delay"]
pub type Dq0DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - DQ1 slew rate"]
    #[inline(always)]
    pub fn dq1_sr(&self) -> Dq1SrR {
        Dq1SrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - DQ1 RX delay"]
    #[inline(always)]
    pub fn dq1_dly_rx(&self) -> Dq1DlyRxR {
        Dq1DlyRxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQ1 drive delay"]
    #[inline(always)]
    pub fn dq1_dly_drv(&self) -> Dq1DlyDrvR {
        Dq1DlyDrvR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - DQ0 slew rate"]
    #[inline(always)]
    pub fn dq0_sr(&self) -> Dq0SrR {
        Dq0SrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DQ0 RX delay"]
    #[inline(always)]
    pub fn dq0_dly_rx(&self) -> Dq0DlyRxR {
        Dq0DlyRxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQ0 drive delay"]
    #[inline(always)]
    pub fn dq0_dly_drv(&self) -> Dq0DlyDrvR {
        Dq0DlyDrvR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DQ1 slew rate"]
    #[inline(always)]
    pub fn dq1_sr(&mut self) -> Dq1SrW<'_, PhyCfg08Spec> {
        Dq1SrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQ1 RX delay"]
    #[inline(always)]
    pub fn dq1_dly_rx(&mut self) -> Dq1DlyRxW<'_, PhyCfg08Spec> {
        Dq1DlyRxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DQ1 drive delay"]
    #[inline(always)]
    pub fn dq1_dly_drv(&mut self) -> Dq1DlyDrvW<'_, PhyCfg08Spec> {
        Dq1DlyDrvW::new(self, 12)
    }
    #[doc = "Bits 16:17 - DQ0 slew rate"]
    #[inline(always)]
    pub fn dq0_sr(&mut self) -> Dq0SrW<'_, PhyCfg08Spec> {
        Dq0SrW::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQ0 RX delay"]
    #[inline(always)]
    pub fn dq0_dly_rx(&mut self) -> Dq0DlyRxW<'_, PhyCfg08Spec> {
        Dq0DlyRxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQ0 drive delay"]
    #[inline(always)]
    pub fn dq0_dly_drv(&mut self) -> Dq0DlyDrvW<'_, PhyCfg08Spec> {
        Dq0DlyDrvW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 08 (DQ1/DQ0)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg08Spec;
impl crate::RegisterSpec for PhyCfg08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_08::R`](R) reader structure"]
impl crate::Readable for PhyCfg08Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_08::W`](W) writer structure"]
impl crate::Writable for PhyCfg08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_08 to value 0x8100_8100"]
impl crate::Resettable for PhyCfg08Spec {
    const RESET_VALUE: u32 = 0x8100_8100;
}
