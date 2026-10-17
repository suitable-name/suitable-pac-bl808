#[doc = "Register `PHY_CFG_0C` reader"]
pub type R = crate::R<PhyCfg0cSpec>;
#[doc = "Register `PHY_CFG_0C` writer"]
pub type W = crate::W<PhyCfg0cSpec>;
#[doc = "Field `dq3_sr` reader - DQ3 slew rate"]
pub type Dq3SrR = crate::FieldReader;
#[doc = "Field `dq3_sr` writer - DQ3 slew rate"]
pub type Dq3SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq3_dly_rx` reader - DQ3 RX delay"]
pub type Dq3DlyRxR = crate::FieldReader;
#[doc = "Field `dq3_dly_rx` writer - DQ3 RX delay"]
pub type Dq3DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq3_dly_drv` reader - DQ3 drive delay"]
pub type Dq3DlyDrvR = crate::FieldReader;
#[doc = "Field `dq3_dly_drv` writer - DQ3 drive delay"]
pub type Dq3DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq2_sr` reader - DQ2 slew rate"]
pub type Dq2SrR = crate::FieldReader;
#[doc = "Field `dq2_sr` writer - DQ2 slew rate"]
pub type Dq2SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq2_dly_rx` reader - DQ2 RX delay"]
pub type Dq2DlyRxR = crate::FieldReader;
#[doc = "Field `dq2_dly_rx` writer - DQ2 RX delay"]
pub type Dq2DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq2_dly_drv` reader - DQ2 drive delay"]
pub type Dq2DlyDrvR = crate::FieldReader;
#[doc = "Field `dq2_dly_drv` writer - DQ2 drive delay"]
pub type Dq2DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - DQ3 slew rate"]
    #[inline(always)]
    pub fn dq3_sr(&self) -> Dq3SrR {
        Dq3SrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - DQ3 RX delay"]
    #[inline(always)]
    pub fn dq3_dly_rx(&self) -> Dq3DlyRxR {
        Dq3DlyRxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQ3 drive delay"]
    #[inline(always)]
    pub fn dq3_dly_drv(&self) -> Dq3DlyDrvR {
        Dq3DlyDrvR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - DQ2 slew rate"]
    #[inline(always)]
    pub fn dq2_sr(&self) -> Dq2SrR {
        Dq2SrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DQ2 RX delay"]
    #[inline(always)]
    pub fn dq2_dly_rx(&self) -> Dq2DlyRxR {
        Dq2DlyRxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQ2 drive delay"]
    #[inline(always)]
    pub fn dq2_dly_drv(&self) -> Dq2DlyDrvR {
        Dq2DlyDrvR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DQ3 slew rate"]
    #[inline(always)]
    pub fn dq3_sr(&mut self) -> Dq3SrW<'_, PhyCfg0cSpec> {
        Dq3SrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQ3 RX delay"]
    #[inline(always)]
    pub fn dq3_dly_rx(&mut self) -> Dq3DlyRxW<'_, PhyCfg0cSpec> {
        Dq3DlyRxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DQ3 drive delay"]
    #[inline(always)]
    pub fn dq3_dly_drv(&mut self) -> Dq3DlyDrvW<'_, PhyCfg0cSpec> {
        Dq3DlyDrvW::new(self, 12)
    }
    #[doc = "Bits 16:17 - DQ2 slew rate"]
    #[inline(always)]
    pub fn dq2_sr(&mut self) -> Dq2SrW<'_, PhyCfg0cSpec> {
        Dq2SrW::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQ2 RX delay"]
    #[inline(always)]
    pub fn dq2_dly_rx(&mut self) -> Dq2DlyRxW<'_, PhyCfg0cSpec> {
        Dq2DlyRxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQ2 drive delay"]
    #[inline(always)]
    pub fn dq2_dly_drv(&mut self) -> Dq2DlyDrvW<'_, PhyCfg0cSpec> {
        Dq2DlyDrvW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 0C (DQ3/DQ2)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg0cSpec;
impl crate::RegisterSpec for PhyCfg0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_0c::R`](R) reader structure"]
impl crate::Readable for PhyCfg0cSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_0c::W`](W) writer structure"]
impl crate::Writable for PhyCfg0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_0C to value 0x8100_8100"]
impl crate::Resettable for PhyCfg0cSpec {
    const RESET_VALUE: u32 = 0x8100_8100;
}
