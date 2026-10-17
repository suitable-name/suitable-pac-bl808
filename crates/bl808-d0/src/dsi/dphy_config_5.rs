#[doc = "Register `DPHY_CONFIG_5` reader"]
pub type R = crate::R<DphyConfig5Spec>;
#[doc = "Register `DPHY_CONFIG_5` writer"]
pub type W = crate::W<DphyConfig5Spec>;
#[doc = "Field `reg_trig3_code` reader - "]
pub type RegTrig3CodeR = crate::FieldReader;
#[doc = "Field `reg_trig3_code` writer - "]
pub type RegTrig3CodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_trig2_code` reader - "]
pub type RegTrig2CodeR = crate::FieldReader;
#[doc = "Field `reg_trig2_code` writer - "]
pub type RegTrig2CodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_trig1_code` reader - "]
pub type RegTrig1CodeR = crate::FieldReader;
#[doc = "Field `reg_trig1_code` writer - "]
pub type RegTrig1CodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_trig0_code` reader - "]
pub type RegTrig0CodeR = crate::FieldReader;
#[doc = "Field `reg_trig0_code` writer - "]
pub type RegTrig0CodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_trig3_code(&self) -> RegTrig3CodeR {
        RegTrig3CodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_trig2_code(&self) -> RegTrig2CodeR {
        RegTrig2CodeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_trig1_code(&self) -> RegTrig1CodeR {
        RegTrig1CodeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_trig0_code(&self) -> RegTrig0CodeR {
        RegTrig0CodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_trig3_code(&mut self) -> RegTrig3CodeW<'_, DphyConfig5Spec> {
        RegTrig3CodeW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_trig2_code(&mut self) -> RegTrig2CodeW<'_, DphyConfig5Spec> {
        RegTrig2CodeW::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_trig1_code(&mut self) -> RegTrig1CodeW<'_, DphyConfig5Spec> {
        RegTrig1CodeW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_trig0_code(&mut self) -> RegTrig0CodeW<'_, DphyConfig5Spec> {
        RegTrig0CodeW::new(self, 24)
    }
}
#[doc = "DPHY Trigger Code Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig5Spec;
impl crate::RegisterSpec for DphyConfig5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_5::R`](R) reader structure"]
impl crate::Readable for DphyConfig5Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_5::W`](W) writer structure"]
impl crate::Writable for DphyConfig5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_5 to value 0x46ba_8405"]
impl crate::Resettable for DphyConfig5Spec {
    const RESET_VALUE: u32 = 0x46ba_8405;
}
