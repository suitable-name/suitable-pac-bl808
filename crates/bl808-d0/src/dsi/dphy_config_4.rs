#[doc = "Register `DPHY_CONFIG_4` reader"]
pub type R = crate::R<DphyConfig4Spec>;
#[doc = "Register `DPHY_CONFIG_4` writer"]
pub type W = crate::W<DphyConfig4Spec>;
#[doc = "Field `reg_time_wakeup` reader - "]
pub type RegTimeWakeupR = crate::FieldReader<u16>;
#[doc = "Field `reg_time_wakeup` writer - "]
pub type RegTimeWakeupW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_time_wakeup(&self) -> RegTimeWakeupR {
        RegTimeWakeupR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_time_wakeup(&mut self) -> RegTimeWakeupW<'_, DphyConfig4Spec> {
        RegTimeWakeupW::new(self, 0)
    }
}
#[doc = "DPHY Wakeup Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig4Spec;
impl crate::RegisterSpec for DphyConfig4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_4::R`](R) reader structure"]
impl crate::Readable for DphyConfig4Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_4::W`](W) writer structure"]
impl crate::Writable for DphyConfig4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_4 to value 0x9c41"]
impl crate::Resettable for DphyConfig4Spec {
    const RESET_VALUE: u32 = 0x9c41;
}
