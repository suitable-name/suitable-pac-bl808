#[doc = "Register `PHY_CFG_00` reader"]
pub type R = crate::R<PhyCfg00Spec>;
#[doc = "Register `PHY_CFG_00` writer"]
pub type W = crate::W<PhyCfg00Spec>;
#[doc = "Field `dqs_rdy` reader - DQS ready flag"]
pub type DqsRdyR = crate::BitReader;
#[doc = "Field `ck_sr` reader - Clock slew rate"]
pub type CkSrR = crate::FieldReader;
#[doc = "Field `ck_sr` writer - Clock slew rate"]
pub type CkSrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clk0_polarity` reader - Clock polarity"]
pub type Clk0PolarityR = crate::BitReader;
#[doc = "Field `clk0_polarity` writer - Clock polarity"]
pub type Clk0PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ck_dly_drv` reader - Clock delay drive strength"]
pub type CkDlyDrvR = crate::FieldReader;
#[doc = "Field `ck_dly_drv` writer - Clock delay drive strength"]
pub type CkDlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cen_sr` reader - CEN slew rate"]
pub type CenSrR = crate::FieldReader;
#[doc = "Field `cen_sr` writer - CEN slew rate"]
pub type CenSrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cen_dly_drv` reader - CEN delay drive strength"]
pub type CenDlyDrvR = crate::FieldReader;
#[doc = "Field `cen_dly_drv` writer - CEN delay drive strength"]
pub type CenDlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - DQS ready flag"]
    #[inline(always)]
    pub fn dqs_rdy(&self) -> DqsRdyR {
        DqsRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock slew rate"]
    #[inline(always)]
    pub fn ck_sr(&self) -> CkSrR {
        CkSrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Clock polarity"]
    #[inline(always)]
    pub fn clk0_polarity(&self) -> Clk0PolarityR {
        Clk0PolarityR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Clock delay drive strength"]
    #[inline(always)]
    pub fn ck_dly_drv(&self) -> CkDlyDrvR {
        CkDlyDrvR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - CEN slew rate"]
    #[inline(always)]
    pub fn cen_sr(&self) -> CenSrR {
        CenSrR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:31 - CEN delay drive strength"]
    #[inline(always)]
    pub fn cen_dly_drv(&self) -> CenDlyDrvR {
        CenDlyDrvR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Clock slew rate"]
    #[inline(always)]
    pub fn ck_sr(&mut self) -> CkSrW<'_, PhyCfg00Spec> {
        CkSrW::new(self, 8)
    }
    #[doc = "Bit 15 - Clock polarity"]
    #[inline(always)]
    pub fn clk0_polarity(&mut self) -> Clk0PolarityW<'_, PhyCfg00Spec> {
        Clk0PolarityW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Clock delay drive strength"]
    #[inline(always)]
    pub fn ck_dly_drv(&mut self) -> CkDlyDrvW<'_, PhyCfg00Spec> {
        CkDlyDrvW::new(self, 16)
    }
    #[doc = "Bits 20:21 - CEN slew rate"]
    #[inline(always)]
    pub fn cen_sr(&mut self) -> CenSrW<'_, PhyCfg00Spec> {
        CenSrW::new(self, 20)
    }
    #[doc = "Bits 28:31 - CEN delay drive strength"]
    #[inline(always)]
    pub fn cen_dly_drv(&mut self) -> CenDlyDrvW<'_, PhyCfg00Spec> {
        CenDlyDrvW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 00 (DQS/CK/CEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg00Spec;
impl crate::RegisterSpec for PhyCfg00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_00::R`](R) reader structure"]
impl crate::Readable for PhyCfg00Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_00::W`](W) writer structure"]
impl crate::Writable for PhyCfg00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_00 to value 0x8008_0000"]
impl crate::Resettable for PhyCfg00Spec {
    const RESET_VALUE: u32 = 0x8008_0000;
}
