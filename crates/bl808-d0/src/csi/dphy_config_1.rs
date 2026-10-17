#[doc = "Register `DPHY_CONFIG_1` reader"]
pub type R = crate::R<DphyConfig1Spec>;
#[doc = "Register `DPHY_CONFIG_1` writer"]
pub type W = crate::W<DphyConfig1Spec>;
#[doc = "Field `reg_time_ck_settle` reader - Clock lane settling time"]
pub type RegTimeCkSettleR = crate::FieldReader;
#[doc = "Field `reg_time_ck_settle` writer - Clock lane settling time"]
pub type RegTimeCkSettleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_ck_term_en` reader - Clock lane termination enable time"]
pub type RegTimeCkTermEnR = crate::FieldReader;
#[doc = "Field `reg_time_ck_term_en` writer - Clock lane termination enable time"]
pub type RegTimeCkTermEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_hs_settle` reader - Data lane High Speed settling time"]
pub type RegTimeHsSettleR = crate::FieldReader;
#[doc = "Field `reg_time_hs_settle` writer - Data lane High Speed settling time"]
pub type RegTimeHsSettleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_time_hs_term_en` reader - Data lane High Speed termination enable time"]
pub type RegTimeHsTermEnR = crate::FieldReader;
#[doc = "Field `reg_time_hs_term_en` writer - Data lane High Speed termination enable time"]
pub type RegTimeHsTermEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock lane settling time"]
    #[inline(always)]
    pub fn reg_time_ck_settle(&self) -> RegTimeCkSettleR {
        RegTimeCkSettleR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock lane termination enable time"]
    #[inline(always)]
    pub fn reg_time_ck_term_en(&self) -> RegTimeCkTermEnR {
        RegTimeCkTermEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data lane High Speed settling time"]
    #[inline(always)]
    pub fn reg_time_hs_settle(&self) -> RegTimeHsSettleR {
        RegTimeHsSettleR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data lane High Speed termination enable time"]
    #[inline(always)]
    pub fn reg_time_hs_term_en(&self) -> RegTimeHsTermEnR {
        RegTimeHsTermEnR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock lane settling time"]
    #[inline(always)]
    pub fn reg_time_ck_settle(&mut self) -> RegTimeCkSettleW<'_, DphyConfig1Spec> {
        RegTimeCkSettleW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock lane termination enable time"]
    #[inline(always)]
    pub fn reg_time_ck_term_en(&mut self) -> RegTimeCkTermEnW<'_, DphyConfig1Spec> {
        RegTimeCkTermEnW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data lane High Speed settling time"]
    #[inline(always)]
    pub fn reg_time_hs_settle(&mut self) -> RegTimeHsSettleW<'_, DphyConfig1Spec> {
        RegTimeHsSettleW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data lane High Speed termination enable time"]
    #[inline(always)]
    pub fn reg_time_hs_term_en(&mut self) -> RegTimeHsTermEnW<'_, DphyConfig1Spec> {
        RegTimeHsTermEnW::new(self, 24)
    }
}
#[doc = "D-PHY Timing Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig1Spec;
impl crate::RegisterSpec for DphyConfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_1::R`](R) reader structure"]
impl crate::Readable for DphyConfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_1::W`](W) writer structure"]
impl crate::Writable for DphyConfig1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_1 to value 0x0f2f_010a"]
impl crate::Resettable for DphyConfig1Spec {
    const RESET_VALUE: u32 = 0x0f2f_010a;
}
