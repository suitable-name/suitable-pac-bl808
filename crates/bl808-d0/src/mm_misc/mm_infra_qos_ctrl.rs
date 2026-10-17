#[doc = "Register `MM_INFRA_QOS_CTRL` reader"]
pub type R = crate::R<MmInfraQosCtrlSpec>;
#[doc = "Register `MM_INFRA_QOS_CTRL` writer"]
pub type W = crate::W<MmInfraQosCtrlSpec>;
#[doc = "Field `reg_mmcpu0_awqos` reader - D0 core AXI write transaction QoS priority."]
pub type RegMmcpu0AwqosR = crate::BitReader;
#[doc = "Field `reg_mmcpu0_awqos` writer - D0 core AXI write transaction QoS priority."]
pub type RegMmcpu0AwqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mmcpu0_arqos` reader - D0 core AXI read transaction QoS priority."]
pub type RegMmcpu0ArqosR = crate::BitReader;
#[doc = "Field `reg_mmcpu0_arqos` writer - D0 core AXI read transaction QoS priority."]
pub type RegMmcpu0ArqosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_h_wthre_mm2conn` reader - MM to Connectivity bus write threshold configuration."]
pub type RegHWthreMm2connR = crate::FieldReader;
#[doc = "Field `reg_h_wthre_mm2conn` writer - MM to Connectivity bus write threshold configuration."]
pub type RegHWthreMm2connW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_h_wthre_conn2mm` reader - Connectivity to MM bus write threshold configuration."]
pub type RegHWthreConn2mmR = crate::FieldReader;
#[doc = "Field `reg_h_wthre_conn2mm` writer - Connectivity to MM bus write threshold configuration."]
pub type RegHWthreConn2mmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_x_wthre_mmhw2pA` reader - MM hardware to P-AXI write threshold."]
pub type RegXWthreMmhw2pAR = crate::FieldReader;
#[doc = "Field `reg_x_wthre_mmhw2pA` writer - MM hardware to P-AXI write threshold."]
pub type RegXWthreMmhw2pAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_x_wthre_mmhw2ext` reader - MM hardware to External bus write threshold."]
pub type RegXWthreMmhw2extR = crate::FieldReader;
#[doc = "Field `reg_x_wthre_mmhw2ext` writer - MM hardware to External bus write threshold."]
pub type RegXWthreMmhw2extW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_x_wthre_pUHS` reader - Peripheral UHS bus write threshold."]
pub type RegXWthrePUhsR = crate::FieldReader;
#[doc = "Field `reg_x_wthre_pUHS` writer - Peripheral UHS bus write threshold."]
pub type RegXWthrePUhsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 2 - D0 core AXI write transaction QoS priority."]
    #[inline(always)]
    pub fn reg_mmcpu0_awqos(&self) -> RegMmcpu0AwqosR {
        RegMmcpu0AwqosR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - D0 core AXI read transaction QoS priority."]
    #[inline(always)]
    pub fn reg_mmcpu0_arqos(&self) -> RegMmcpu0ArqosR {
        RegMmcpu0ArqosR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:17 - MM to Connectivity bus write threshold configuration."]
    #[inline(always)]
    pub fn reg_h_wthre_mm2conn(&self) -> RegHWthreMm2connR {
        RegHWthreMm2connR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Connectivity to MM bus write threshold configuration."]
    #[inline(always)]
    pub fn reg_h_wthre_conn2mm(&self) -> RegHWthreConn2mmR {
        RegHWthreConn2mmR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - MM hardware to P-AXI write threshold."]
    #[inline(always)]
    pub fn reg_x_wthre_mmhw2p_a(&self) -> RegXWthreMmhw2pAR {
        RegXWthreMmhw2pAR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - MM hardware to External bus write threshold."]
    #[inline(always)]
    pub fn reg_x_wthre_mmhw2ext(&self) -> RegXWthreMmhw2extR {
        RegXWthreMmhw2extR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Peripheral UHS bus write threshold."]
    #[inline(always)]
    pub fn reg_x_wthre_p_uhs(&self) -> RegXWthrePUhsR {
        RegXWthrePUhsR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - D0 core AXI write transaction QoS priority."]
    #[inline(always)]
    pub fn reg_mmcpu0_awqos(&mut self) -> RegMmcpu0AwqosW<'_, MmInfraQosCtrlSpec> {
        RegMmcpu0AwqosW::new(self, 2)
    }
    #[doc = "Bit 3 - D0 core AXI read transaction QoS priority."]
    #[inline(always)]
    pub fn reg_mmcpu0_arqos(&mut self) -> RegMmcpu0ArqosW<'_, MmInfraQosCtrlSpec> {
        RegMmcpu0ArqosW::new(self, 3)
    }
    #[doc = "Bits 16:17 - MM to Connectivity bus write threshold configuration."]
    #[inline(always)]
    pub fn reg_h_wthre_mm2conn(&mut self) -> RegHWthreMm2connW<'_, MmInfraQosCtrlSpec> {
        RegHWthreMm2connW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Connectivity to MM bus write threshold configuration."]
    #[inline(always)]
    pub fn reg_h_wthre_conn2mm(&mut self) -> RegHWthreConn2mmW<'_, MmInfraQosCtrlSpec> {
        RegHWthreConn2mmW::new(self, 18)
    }
    #[doc = "Bits 20:21 - MM hardware to P-AXI write threshold."]
    #[inline(always)]
    pub fn reg_x_wthre_mmhw2p_a(&mut self) -> RegXWthreMmhw2pAW<'_, MmInfraQosCtrlSpec> {
        RegXWthreMmhw2pAW::new(self, 20)
    }
    #[doc = "Bits 22:23 - MM hardware to External bus write threshold."]
    #[inline(always)]
    pub fn reg_x_wthre_mmhw2ext(&mut self) -> RegXWthreMmhw2extW<'_, MmInfraQosCtrlSpec> {
        RegXWthreMmhw2extW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Peripheral UHS bus write threshold."]
    #[inline(always)]
    pub fn reg_x_wthre_p_uhs(&mut self) -> RegXWthrePUhsW<'_, MmInfraQosCtrlSpec> {
        RegXWthrePUhsW::new(self, 24)
    }
}
#[doc = "Multimedia Infrastructure Quality-of-Service (QoS) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_infra_qos_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_infra_qos_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmInfraQosCtrlSpec;
impl crate::RegisterSpec for MmInfraQosCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_infra_qos_ctrl::R`](R) reader structure"]
impl crate::Readable for MmInfraQosCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mm_infra_qos_ctrl::W`](W) writer structure"]
impl crate::Writable for MmInfraQosCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_INFRA_QOS_CTRL to value 0"]
impl crate::Resettable for MmInfraQosCtrlSpec {}
