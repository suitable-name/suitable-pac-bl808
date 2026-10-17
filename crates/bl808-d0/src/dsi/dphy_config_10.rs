#[doc = "Register `DPHY_CONFIG_10` reader"]
pub type R = crate::R<DphyConfig10Spec>;
#[doc = "Register `DPHY_CONFIG_10` writer"]
pub type W = crate::W<DphyConfig10Spec>;
#[doc = "Field `reg_testbus_sel_lo` reader - "]
pub type RegTestbusSelLoR = crate::FieldReader;
#[doc = "Field `reg_testbus_sel_lo` writer - "]
pub type RegTestbusSelLoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_testbus_sel_hi` reader - "]
pub type RegTestbusSelHiR = crate::FieldReader;
#[doc = "Field `reg_testbus_sel_hi` writer - "]
pub type RegTestbusSelHiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_testbus_hi8bsel_8bmode` reader - "]
pub type RegTestbusHi8bsel8bmodeR = crate::BitReader;
#[doc = "Field `reg_testbus_hi8bsel_8bmode` writer - "]
pub type RegTestbusHi8bsel8bmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_imp_pu_code` reader - "]
pub type RegImpPuCodeR = crate::FieldReader;
#[doc = "Field `reg_imp_pu_code` writer - "]
pub type RegImpPuCodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `reg_imp_pd_code` reader - "]
pub type RegImpPdCodeR = crate::FieldReader;
#[doc = "Field `reg_imp_pd_code` writer - "]
pub type RegImpPdCodeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `reg_dphy_vreg400mv_trim` reader - "]
pub type RegDphyVreg400mvTrimR = crate::FieldReader;
#[doc = "Field `reg_dphy_vreg400mv_trim` writer - "]
pub type RegDphyVreg400mvTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_dphy_bg_vref_en` reader - "]
pub type RegDphyBgVrefEnR = crate::BitReader;
#[doc = "Field `reg_dphy_bg_vref_en` writer - "]
pub type RegDphyBgVrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ana_bg_vref_en` reader - "]
pub type RegAnaBgVrefEnR = crate::BitReader;
#[doc = "Field `reg_ana_bg_vref_en` writer - "]
pub type RegAnaBgVrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ana_vreg400mv_trim` reader - "]
pub type RegAnaVreg400mvTrimR = crate::FieldReader;
#[doc = "Field `reg_ana_vreg400mv_trim` writer - "]
pub type RegAnaVreg400mvTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_ana_vref_en` reader - "]
pub type RegAnaVrefEnR = crate::BitReader;
#[doc = "Field `reg_ana_vref_en` writer - "]
pub type RegAnaVrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_testbus_sel_lo(&self) -> RegTestbusSelLoR {
        RegTestbusSelLoR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_testbus_sel_hi(&self) -> RegTestbusSelHiR {
        RegTestbusSelHiR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_testbus_hi8bsel_8bmode(&self) -> RegTestbusHi8bsel8bmodeR {
        RegTestbusHi8bsel8bmodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn reg_imp_pu_code(&self) -> RegImpPuCodeR {
        RegImpPuCodeR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18"]
    #[inline(always)]
    pub fn reg_imp_pd_code(&self) -> RegImpPdCodeR {
        RegImpPdCodeR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn reg_dphy_vreg400mv_trim(&self) -> RegDphyVreg400mvTrimR {
        RegDphyVreg400mvTrimR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_dphy_bg_vref_en(&self) -> RegDphyBgVrefEnR {
        RegDphyBgVrefEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_ana_bg_vref_en(&self) -> RegAnaBgVrefEnR {
        RegAnaBgVrefEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn reg_ana_vreg400mv_trim(&self) -> RegAnaVreg400mvTrimR {
        RegAnaVreg400mvTrimR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_ana_vref_en(&self) -> RegAnaVrefEnR {
        RegAnaVrefEnR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_testbus_sel_lo(&mut self) -> RegTestbusSelLoW<'_, DphyConfig10Spec> {
        RegTestbusSelLoW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_testbus_sel_hi(&mut self) -> RegTestbusSelHiW<'_, DphyConfig10Spec> {
        RegTestbusSelHiW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_testbus_hi8bsel_8bmode(&mut self) -> RegTestbusHi8bsel8bmodeW<'_, DphyConfig10Spec> {
        RegTestbusHi8bsel8bmodeW::new(self, 8)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn reg_imp_pu_code(&mut self) -> RegImpPuCodeW<'_, DphyConfig10Spec> {
        RegImpPuCodeW::new(self, 9)
    }
    #[doc = "Bits 14:18"]
    #[inline(always)]
    pub fn reg_imp_pd_code(&mut self) -> RegImpPdCodeW<'_, DphyConfig10Spec> {
        RegImpPdCodeW::new(self, 14)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn reg_dphy_vreg400mv_trim(&mut self) -> RegDphyVreg400mvTrimW<'_, DphyConfig10Spec> {
        RegDphyVreg400mvTrimW::new(self, 19)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_dphy_bg_vref_en(&mut self) -> RegDphyBgVrefEnW<'_, DphyConfig10Spec> {
        RegDphyBgVrefEnW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_ana_bg_vref_en(&mut self) -> RegAnaBgVrefEnW<'_, DphyConfig10Spec> {
        RegAnaBgVrefEnW::new(self, 23)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn reg_ana_vreg400mv_trim(&mut self) -> RegAnaVreg400mvTrimW<'_, DphyConfig10Spec> {
        RegAnaVreg400mvTrimW::new(self, 24)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_ana_vref_en(&mut self) -> RegAnaVrefEnW<'_, DphyConfig10Spec> {
        RegAnaVrefEnW::new(self, 27)
    }
}
#[doc = "DPHY Impedance and Voltage Trim\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig10Spec;
impl crate::RegisterSpec for DphyConfig10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_10::R`](R) reader structure"]
impl crate::Readable for DphyConfig10Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_10::W`](W) writer structure"]
impl crate::Writable for DphyConfig10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_10 to value 0x0002_5000"]
impl crate::Resettable for DphyConfig10Spec {
    const RESET_VALUE: u32 = 0x0002_5000;
}
