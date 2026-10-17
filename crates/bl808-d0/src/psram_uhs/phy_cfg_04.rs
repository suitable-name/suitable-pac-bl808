#[doc = "Register `PHY_CFG_04` reader"]
pub type R = crate::R<PhyCfg04Spec>;
#[doc = "Register `PHY_CFG_04` writer"]
pub type W = crate::W<PhyCfg04Spec>;
#[doc = "Field `dm1_sr` reader - DM1 slew rate"]
pub type Dm1SrR = crate::FieldReader;
#[doc = "Field `dm1_sr` writer - DM1 slew rate"]
pub type Dm1SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dm1_dly_drv` reader - DM1 delay drive strength"]
pub type Dm1DlyDrvR = crate::FieldReader;
#[doc = "Field `dm1_dly_drv` writer - DM1 delay drive strength"]
pub type Dm1DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dm0_sr` reader - DM0 slew rate"]
pub type Dm0SrR = crate::FieldReader;
#[doc = "Field `dm0_sr` writer - DM0 slew rate"]
pub type Dm0SrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dm0_dly_drv` reader - DM0 delay drive strength"]
pub type Dm0DlyDrvR = crate::FieldReader;
#[doc = "Field `dm0_dly_drv` writer - DM0 delay drive strength"]
pub type Dm0DlyDrvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:5 - DM1 slew rate"]
    #[inline(always)]
    pub fn dm1_sr(&self) -> Dm1SrR {
        Dm1SrR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:15 - DM1 delay drive strength"]
    #[inline(always)]
    pub fn dm1_dly_drv(&self) -> Dm1DlyDrvR {
        Dm1DlyDrvR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - DM0 slew rate"]
    #[inline(always)]
    pub fn dm0_sr(&self) -> Dm0SrR {
        Dm0SrR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:31 - DM0 delay drive strength"]
    #[inline(always)]
    pub fn dm0_dly_drv(&self) -> Dm0DlyDrvR {
        Dm0DlyDrvR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - DM1 slew rate"]
    #[inline(always)]
    pub fn dm1_sr(&mut self) -> Dm1SrW<'_, PhyCfg04Spec> {
        Dm1SrW::new(self, 4)
    }
    #[doc = "Bits 12:15 - DM1 delay drive strength"]
    #[inline(always)]
    pub fn dm1_dly_drv(&mut self) -> Dm1DlyDrvW<'_, PhyCfg04Spec> {
        Dm1DlyDrvW::new(self, 12)
    }
    #[doc = "Bits 20:21 - DM0 slew rate"]
    #[inline(always)]
    pub fn dm0_sr(&mut self) -> Dm0SrW<'_, PhyCfg04Spec> {
        Dm0SrW::new(self, 20)
    }
    #[doc = "Bits 28:31 - DM0 delay drive strength"]
    #[inline(always)]
    pub fn dm0_dly_drv(&mut self) -> Dm0DlyDrvW<'_, PhyCfg04Spec> {
        Dm0DlyDrvW::new(self, 28)
    }
}
#[doc = "PHY Pin Configuration 04 (DM1/DM0)\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg04Spec;
impl crate::RegisterSpec for PhyCfg04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_04::R`](R) reader structure"]
impl crate::Readable for PhyCfg04Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_04::W`](W) writer structure"]
impl crate::Writable for PhyCfg04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_04 to value 0x8000_8000"]
impl crate::Resettable for PhyCfg04Spec {
    const RESET_VALUE: u32 = 0x8000_8000;
}
