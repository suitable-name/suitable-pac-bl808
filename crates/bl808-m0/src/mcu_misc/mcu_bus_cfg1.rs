#[doc = "Register `MCU_BUS_CFG1` reader"]
pub type R = crate::R<McuBusCfg1Spec>;
#[doc = "Register `MCU_BUS_CFG1` writer"]
pub type W = crate::W<McuBusCfg1Spec>;
#[doc = "Field `reg_mcu1_hqos` reader - MCU1 (E907) QoS enable"]
pub type RegMcu1HqosR = crate::BitReader;
#[doc = "Field `reg_mcu1_hqos` writer - MCU1 (E907) QoS enable"]
pub type RegMcu1HqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu1_awqos` reader - MCU1 (E907) write QoS priority"]
pub type RegMcu1AwqosR = crate::BitReader;
#[doc = "Field `reg_mcu1_awqos` writer - MCU1 (E907) write QoS priority"]
pub type RegMcu1AwqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu1_arqos` reader - MCU1 (E907) read QoS priority"]
pub type RegMcu1ArqosR = crate::BitReader;
#[doc = "Field `reg_mcu1_arqos` writer - MCU1 (E907) read QoS priority"]
pub type RegMcu1ArqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_x2hs_sp_bypass` reader - X2HS single port bypass"]
pub type RegMcuX2hsSpBypassR = crate::BitReader;
#[doc = "Field `reg_mcu_x2hs_sp_bypass` writer - X2HS single port bypass"]
pub type RegMcuX2hsSpBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_x_wthre_mcu2ext` reader - Write threshold limit for MCU to External Bus"]
pub type RegXWthreMcu2extR = crate::FieldReader;
#[doc = "Field `reg_x_wthre_mcu2ext` writer - Write threshold limit for MCU to External Bus"]
pub type RegXWthreMcu2extW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_mcu_infra_arb_mode` reader - MCU infrastructure arbitration mode"]
pub type RegMcuInfraArbModeR = crate::BitReader;
#[doc = "Field `reg_mcu_infra_arb_mode` writer - MCU infrastructure arbitration mode"]
pub type RegMcuInfraArbModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCU1 (E907) QoS enable"]
    #[inline(always)]
    pub fn reg_mcu1_hqos(&self) -> RegMcu1HqosR {
        RegMcu1HqosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCU1 (E907) write QoS priority"]
    #[inline(always)]
    pub fn reg_mcu1_awqos(&self) -> RegMcu1AwqosR {
        RegMcu1AwqosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCU1 (E907) read QoS priority"]
    #[inline(always)]
    pub fn reg_mcu1_arqos(&self) -> RegMcu1ArqosR {
        RegMcu1ArqosR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - X2HS single port bypass"]
    #[inline(always)]
    pub fn reg_mcu_x2hs_sp_bypass(&self) -> RegMcuX2hsSpBypassR {
        RegMcuX2hsSpBypassR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Write threshold limit for MCU to External Bus"]
    #[inline(always)]
    pub fn reg_x_wthre_mcu2ext(&self) -> RegXWthreMcu2extR {
        RegXWthreMcu2extR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 16 - MCU infrastructure arbitration mode"]
    #[inline(always)]
    pub fn reg_mcu_infra_arb_mode(&self) -> RegMcuInfraArbModeR {
        RegMcuInfraArbModeR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCU1 (E907) QoS enable"]
    #[inline(always)]
    pub fn reg_mcu1_hqos(&mut self) -> RegMcu1HqosW<'_, McuBusCfg1Spec> {
        RegMcu1HqosW::new(self, 0)
    }
    #[doc = "Bit 1 - MCU1 (E907) write QoS priority"]
    #[inline(always)]
    pub fn reg_mcu1_awqos(&mut self) -> RegMcu1AwqosW<'_, McuBusCfg1Spec> {
        RegMcu1AwqosW::new(self, 1)
    }
    #[doc = "Bit 2 - MCU1 (E907) read QoS priority"]
    #[inline(always)]
    pub fn reg_mcu1_arqos(&mut self) -> RegMcu1ArqosW<'_, McuBusCfg1Spec> {
        RegMcu1ArqosW::new(self, 2)
    }
    #[doc = "Bit 3 - X2HS single port bypass"]
    #[inline(always)]
    pub fn reg_mcu_x2hs_sp_bypass(&mut self) -> RegMcuX2hsSpBypassW<'_, McuBusCfg1Spec> {
        RegMcuX2hsSpBypassW::new(self, 3)
    }
    #[doc = "Bits 7:8 - Write threshold limit for MCU to External Bus"]
    #[inline(always)]
    pub fn reg_x_wthre_mcu2ext(&mut self) -> RegXWthreMcu2extW<'_, McuBusCfg1Spec> {
        RegXWthreMcu2extW::new(self, 7)
    }
    #[doc = "Bit 16 - MCU infrastructure arbitration mode"]
    #[inline(always)]
    pub fn reg_mcu_infra_arb_mode(&mut self) -> RegMcuInfraArbModeW<'_, McuBusCfg1Spec> {
        RegMcuInfraArbModeW::new(self, 16)
    }
}
#[doc = "MCU Bus Configuration 1 (QoS and Arbitration)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu_bus_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcu_bus_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McuBusCfg1Spec;
impl crate::RegisterSpec for McuBusCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu_bus_cfg1::R`](R) reader structure"]
impl crate::Readable for McuBusCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcu_bus_cfg1::W`](W) writer structure"]
impl crate::Writable for McuBusCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCU_BUS_CFG1 to value 0"]
impl crate::Resettable for McuBusCfg1Spec {}
