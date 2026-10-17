#[doc = "Register `MCU_BUS_CFG0` reader"]
pub type R = crate::R<McuBusCfg0Spec>;
#[doc = "Register `MCU_BUS_CFG0` writer"]
pub type W = crate::W<McuBusCfg0Spec>;
#[doc = "Field `reg_mcu_infra_timeout_en` reader - MCU infrastructure timeout enable"]
pub type RegMcuInfraTimeoutEnR = crate::BitReader;
#[doc = "Field `reg_mcu_infra_timeout_en` writer - MCU infrastructure timeout enable"]
pub type RegMcuInfraTimeoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_infra_timeout_clr` reader - Clear MCU infrastructure timeout status"]
pub type RegMcuInfraTimeoutClrR = crate::BitReader;
#[doc = "Field `reg_mcu_infra_timeout_clr` writer - Clear MCU infrastructure timeout status"]
pub type RegMcuInfraTimeoutClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_mcu_infra_timeout` reader - MCU infrastructure timeout status"]
pub type StsMcuInfraTimeoutR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - MCU infrastructure timeout enable"]
    #[inline(always)]
    pub fn reg_mcu_infra_timeout_en(&self) -> RegMcuInfraTimeoutEnR {
        RegMcuInfraTimeoutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear MCU infrastructure timeout status"]
    #[inline(always)]
    pub fn reg_mcu_infra_timeout_clr(&self) -> RegMcuInfraTimeoutClrR {
        RegMcuInfraTimeoutClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - MCU infrastructure timeout status"]
    #[inline(always)]
    pub fn sts_mcu_infra_timeout(&self) -> StsMcuInfraTimeoutR {
        StsMcuInfraTimeoutR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCU infrastructure timeout enable"]
    #[inline(always)]
    pub fn reg_mcu_infra_timeout_en(&mut self) -> RegMcuInfraTimeoutEnW<'_, McuBusCfg0Spec> {
        RegMcuInfraTimeoutEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear MCU infrastructure timeout status"]
    #[inline(always)]
    pub fn reg_mcu_infra_timeout_clr(&mut self) -> RegMcuInfraTimeoutClrW<'_, McuBusCfg0Spec> {
        RegMcuInfraTimeoutClrW::new(self, 1)
    }
}
#[doc = "MCU Bus Configuration 0 (Timeout)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu_bus_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcu_bus_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McuBusCfg0Spec;
impl crate::RegisterSpec for McuBusCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu_bus_cfg0::R`](R) reader structure"]
impl crate::Readable for McuBusCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`mcu_bus_cfg0::W`](W) writer structure"]
impl crate::Writable for McuBusCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCU_BUS_CFG0 to value 0"]
impl crate::Resettable for McuBusCfg0Spec {}
