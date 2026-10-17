#[doc = "Register `DPHY_CONFIG_6` reader"]
pub type R = crate::R<DphyConfig6Spec>;
#[doc = "Register `DPHY_CONFIG_6` writer"]
pub type W = crate::W<DphyConfig6Spec>;
#[doc = "Field `reg_ulps_code` reader - "]
pub type RegUlpsCodeR = crate::FieldReader;
#[doc = "Field `reg_ulps_code` writer - "]
pub type RegUlpsCodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_lpdt_code` reader - "]
pub type RegLpdtCodeR = crate::FieldReader;
#[doc = "Field `reg_lpdt_code` writer - "]
pub type RegLpdtCodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_ulps_code(&self) -> RegUlpsCodeR {
        RegUlpsCodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_lpdt_code(&self) -> RegLpdtCodeR {
        RegLpdtCodeR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn reg_ulps_code(&mut self) -> RegUlpsCodeW<'_, DphyConfig6Spec> {
        RegUlpsCodeW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_lpdt_code(&mut self) -> RegLpdtCodeW<'_, DphyConfig6Spec> {
        RegLpdtCodeW::new(self, 8)
    }
}
#[doc = "DPHY ULPS and LPDT Code Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig6Spec;
impl crate::RegisterSpec for DphyConfig6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_6::R`](R) reader structure"]
impl crate::Readable for DphyConfig6Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_6::W`](W) writer structure"]
impl crate::Writable for DphyConfig6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_6 to value 0x8778"]
impl crate::Resettable for DphyConfig6Spec {
    const RESET_VALUE: u32 = 0x8778;
}
