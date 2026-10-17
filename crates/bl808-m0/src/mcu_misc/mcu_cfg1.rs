#[doc = "Register `MCU_CFG1` reader"]
pub type R = crate::R<McuCfg1Spec>;
#[doc = "Register `MCU_CFG1` writer"]
pub type W = crate::W<McuCfg1Spec>;
#[doc = "Field `reg_mcu1_dfs_req` reader - E907 DFS (Dynamic Frequency Scaling) request"]
pub type RegMcu1DfsReqR = crate::BitReader;
#[doc = "Field `reg_mcu1_dfs_req` writer - E907 DFS (Dynamic Frequency Scaling) request"]
pub type RegMcu1DfsReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_mcu1_dfs_ack` reader - E907 DFS acknowledge status"]
pub type StsMcu1DfsAckR = crate::BitReader;
#[doc = "Field `reg_mcu1_srst_en` reader - E907 software reset enable"]
pub type RegMcu1SrstEnR = crate::FieldReader;
#[doc = "Field `reg_mcu1_srst_en` writer - E907 software reset enable"]
pub type RegMcu1SrstEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sts_mcu1_lpmd_b` reader - E907 low power mode status"]
pub type StsMcu1LpmdBR = crate::FieldReader;
#[doc = "Field `MCU1_WFI_FORCE` reader - Force E907 into WFI (Wait For Interrupt)"]
pub type Mcu1WfiForceR = crate::BitReader;
#[doc = "Field `MCU1_WFI_FORCE` writer - Force E907 into WFI (Wait For Interrupt)"]
pub type Mcu1WfiForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcu1_ndm_rstn_en` reader - E907 non-debug mode reset enable"]
pub type Mcu1NdmRstnEnR = crate::BitReader;
#[doc = "Field `mcu1_ndm_rstn_en` writer - E907 non-debug mode reset enable"]
pub type Mcu1NdmRstnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcu1_hart_rstn_en` reader - E907 HART reset enable"]
pub type Mcu1HartRstnEnR = crate::BitReader;
#[doc = "Field `mcu1_hart_rstn_en` writer - E907 HART reset enable"]
pub type Mcu1HartRstnEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - E907 DFS (Dynamic Frequency Scaling) request"]
    #[inline(always)]
    pub fn reg_mcu1_dfs_req(&self) -> RegMcu1DfsReqR {
        RegMcu1DfsReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - E907 DFS acknowledge status"]
    #[inline(always)]
    pub fn sts_mcu1_dfs_ack(&self) -> StsMcu1DfsAckR {
        StsMcu1DfsAckR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - E907 software reset enable"]
    #[inline(always)]
    pub fn reg_mcu1_srst_en(&self) -> RegMcu1SrstEnR {
        RegMcu1SrstEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 10:11 - E907 low power mode status"]
    #[inline(always)]
    pub fn sts_mcu1_lpmd_b(&self) -> StsMcu1LpmdBR {
        StsMcu1LpmdBR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - Force E907 into WFI (Wait For Interrupt)"]
    #[inline(always)]
    pub fn mcu1_wfi_force(&self) -> Mcu1WfiForceR {
        Mcu1WfiForceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - E907 non-debug mode reset enable"]
    #[inline(always)]
    pub fn mcu1_ndm_rstn_en(&self) -> Mcu1NdmRstnEnR {
        Mcu1NdmRstnEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - E907 HART reset enable"]
    #[inline(always)]
    pub fn mcu1_hart_rstn_en(&self) -> Mcu1HartRstnEnR {
        Mcu1HartRstnEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - E907 DFS (Dynamic Frequency Scaling) request"]
    #[inline(always)]
    pub fn reg_mcu1_dfs_req(&mut self) -> RegMcu1DfsReqW<'_, McuCfg1Spec> {
        RegMcu1DfsReqW::new(self, 0)
    }
    #[doc = "Bits 4:5 - E907 software reset enable"]
    #[inline(always)]
    pub fn reg_mcu1_srst_en(&mut self) -> RegMcu1SrstEnW<'_, McuCfg1Spec> {
        RegMcu1SrstEnW::new(self, 4)
    }
    #[doc = "Bit 16 - Force E907 into WFI (Wait For Interrupt)"]
    #[inline(always)]
    pub fn mcu1_wfi_force(&mut self) -> Mcu1WfiForceW<'_, McuCfg1Spec> {
        Mcu1WfiForceW::new(self, 16)
    }
    #[doc = "Bit 28 - E907 non-debug mode reset enable"]
    #[inline(always)]
    pub fn mcu1_ndm_rstn_en(&mut self) -> Mcu1NdmRstnEnW<'_, McuCfg1Spec> {
        Mcu1NdmRstnEnW::new(self, 28)
    }
    #[doc = "Bit 29 - E907 HART reset enable"]
    #[inline(always)]
    pub fn mcu1_hart_rstn_en(&mut self) -> Mcu1HartRstnEnW<'_, McuCfg1Spec> {
        Mcu1HartRstnEnW::new(self, 29)
    }
}
#[doc = "MCU Core Configuration 1 (E907 Reset/DFS)\n\nYou can [`read`](crate::Reg::read) this register and get [`mcu_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcu_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McuCfg1Spec;
impl crate::RegisterSpec for McuCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu_cfg1::R`](R) reader structure"]
impl crate::Readable for McuCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcu_cfg1::W`](W) writer structure"]
impl crate::Writable for McuCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCU_CFG1 to value 0x30"]
impl crate::Resettable for McuCfg1Spec {
    const RESET_VALUE: u32 = 0x30;
}
