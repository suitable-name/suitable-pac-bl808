#[doc = "Register `PHY_CFG_1C` reader"]
pub type R = crate::R<PhyCfg1cSpec>;
#[doc = "Register `PHY_CFG_1C` writer"]
pub type W = crate::W<PhyCfg1cSpec>;
#[doc = "Field `dq11_sr` reader - DQ11 slew rate"]
pub type Dq11SrR = crate::FieldReader;
#[doc = "Field `dq11_sr` writer - DQ11 slew rate"]
pub type Dq11SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq11_dly_rx` reader - DQ11 RX delay"]
pub type Dq11DlyRxR = crate::FieldReader;
#[doc = "Field `dq11_dly_rx` writer - DQ11 RX delay"]
pub type Dq11DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq11_dly_drv` reader - DQ11 drive delay"]
pub type Dq11DlyDrvR = crate::FieldReader;
#[doc = "Field `dq11_dly_drv` writer - DQ11 drive delay"]
pub type Dq11DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq10_sr` reader - DQ10 slew rate"]
pub type Dq10SrR = crate::FieldReader;
#[doc = "Field `dq10_sr` writer - DQ10 slew rate"]
pub type Dq10SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dq10_dly_rx` reader - DQ10 RX delay"]
pub type Dq10DlyRxR = crate::FieldReader;
#[doc = "Field `dq10_dly_rx` writer - DQ10 RX delay"]
pub type Dq10DlyRxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dq10_dly_drv` reader - DQ10 drive delay"]
pub type Dq10DlyDrvR = crate::FieldReader;
#[doc = "Field `dq10_dly_drv` writer - DQ10 drive delay"]
pub type Dq10DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - DQ11 slew rate"]
    #[inline(always)]
    pub fn dq11_sr(&self) -> Dq11SrR {
        Dq11SrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - DQ11 RX delay"]
    #[inline(always)]
    pub fn dq11_dly_rx(&self) -> Dq11DlyRxR {
        Dq11DlyRxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQ11 drive delay"]
    #[inline(always)]
    pub fn dq11_dly_drv(&self) -> Dq11DlyDrvR {
        Dq11DlyDrvR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - DQ10 slew rate"]
    #[inline(always)]
    pub fn dq10_sr(&self) -> Dq10SrR {
        Dq10SrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DQ10 RX delay"]
    #[inline(always)]
    pub fn dq10_dly_rx(&self) -> Dq10DlyRxR {
        Dq10DlyRxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQ10 drive delay"]
    #[inline(always)]
    pub fn dq10_dly_drv(&self) -> Dq10DlyDrvR {
        Dq10DlyDrvR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DQ11 slew rate"]
    #[inline(always)]
    pub fn dq11_sr(&mut self) -> Dq11SrW<'_, PhyCfg1cSpec> {
        Dq11SrW::new(self, 0)
    }
    #[doc = "Bits 8:11 - DQ11 RX delay"]
    #[inline(always)]
    pub fn dq11_dly_rx(&mut self) -> Dq11DlyRxW<'_, PhyCfg1cSpec> {
        Dq11DlyRxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DQ11 drive delay"]
    #[inline(always)]
    pub fn dq11_dly_drv(&mut self) -> Dq11DlyDrvW<'_, PhyCfg1cSpec> {
        Dq11DlyDrvW::new(self, 12)
    }
    #[doc = "Bits 16:17 - DQ10 slew rate"]
    #[inline(always)]
    pub fn dq10_sr(&mut self) -> Dq10SrW<'_, PhyCfg1cSpec> {
        Dq10SrW::new(self, 16)
    }
    #[doc = "Bits 24:27 - DQ10 RX delay"]
    #[inline(always)]
    pub fn dq10_dly_rx(&mut self) -> Dq10DlyRxW<'_, PhyCfg1cSpec> {
        Dq10DlyRxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQ10 drive delay"]
    #[inline(always)]
    pub fn dq10_dly_drv(&mut self) -> Dq10DlyDrvW<'_, PhyCfg1cSpec> {
        Dq10DlyDrvW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 1C (DQ11/DQ10)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg1cSpec;
impl crate::RegisterSpec for PhyCfg1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_1c::R`](R) reader structure"]
impl crate::Readable for PhyCfg1cSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_1c::W`](W) writer structure"]
impl crate::Writable for PhyCfg1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_1C to value 0x8100_8100"]
impl crate::Resettable for PhyCfg1cSpec {
    const RESET_VALUE: u32 = 0x8100_8100;
}
