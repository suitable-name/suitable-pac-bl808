#[doc = "Register `PHY_CFG_50` reader"]
pub type R = crate::R<PhyCfg50Spec>;
#[doc = "Register `PHY_CFG_50` writer"]
pub type W = crate::W<PhyCfg50Spec>;
#[doc = "Field `dq_oe_up_p_reg` reader - DQ OE Up P-channel tuning"]
pub type DqOeUpPRegR = crate::FieldReader;
#[doc = "Field `dq_oe_up_p_reg` writer - DQ OE Up P-channel tuning"]
pub type DqOeUpPRegW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dq_oe_up_n_reg` reader - DQ OE Up N-channel tuning"]
pub type DqOeUpNRegR = crate::FieldReader;
#[doc = "Field `dq_oe_up_n_reg` writer - DQ OE Up N-channel tuning"]
pub type DqOeUpNRegW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dq_oe_mid_p_reg` reader - DQ OE Mid P-channel tuning"]
pub type DqOeMidPRegR = crate::FieldReader;
#[doc = "Field `dq_oe_mid_p_reg` writer - DQ OE Mid P-channel tuning"]
pub type DqOeMidPRegW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dq_oe_mid_n_reg` reader - DQ OE Mid N-channel tuning"]
pub type DqOeMidNRegR = crate::FieldReader;
#[doc = "Field `dq_oe_mid_n_reg` writer - DQ OE Mid N-channel tuning"]
pub type DqOeMidNRegW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dq_oe_dn_p_reg` reader - DQ OE Down P-channel tuning"]
pub type DqOeDnPRegR = crate::FieldReader;
#[doc = "Field `dq_oe_dn_p_reg` writer - DQ OE Down P-channel tuning"]
pub type DqOeDnPRegW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dq_oe_dn_n_reg` reader - DQ OE Down N-channel tuning"]
pub type DqOeDnNRegR = crate::FieldReader;
#[doc = "Field `dq_oe_dn_n_reg` writer - DQ OE Down N-channel tuning"]
pub type DqOeDnNRegW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `phy_wl_cen_ana` reader - PHY write-latency CEN analog tuning"]
pub type PhyWlCenAnaR = crate::FieldReader;
#[doc = "Field `phy_wl_cen_ana` writer - PHY write-latency CEN analog tuning"]
pub type PhyWlCenAnaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - DQ OE Up P-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_up_p_reg(&self) -> DqOeUpPRegR {
        DqOeUpPRegR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - DQ OE Up N-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_up_n_reg(&self) -> DqOeUpNRegR {
        DqOeUpNRegR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - DQ OE Mid P-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_mid_p_reg(&self) -> DqOeMidPRegR {
        DqOeMidPRegR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - DQ OE Mid N-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_mid_n_reg(&self) -> DqOeMidNRegR {
        DqOeMidNRegR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - DQ OE Down P-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_dn_p_reg(&self) -> DqOeDnPRegR {
        DqOeDnPRegR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - DQ OE Down N-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_dn_n_reg(&self) -> DqOeDnNRegR {
        DqOeDnNRegR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - PHY write-latency CEN analog tuning"]
    #[inline(always)]
    pub fn phy_wl_cen_ana(&self) -> PhyWlCenAnaR {
        PhyWlCenAnaR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DQ OE Up P-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_up_p_reg(&mut self) -> DqOeUpPRegW<'_, PhyCfg50Spec> {
        DqOeUpPRegW::new(self, 0)
    }
    #[doc = "Bits 4:6 - DQ OE Up N-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_up_n_reg(&mut self) -> DqOeUpNRegW<'_, PhyCfg50Spec> {
        DqOeUpNRegW::new(self, 4)
    }
    #[doc = "Bits 8:10 - DQ OE Mid P-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_mid_p_reg(&mut self) -> DqOeMidPRegW<'_, PhyCfg50Spec> {
        DqOeMidPRegW::new(self, 8)
    }
    #[doc = "Bits 12:14 - DQ OE Mid N-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_mid_n_reg(&mut self) -> DqOeMidNRegW<'_, PhyCfg50Spec> {
        DqOeMidNRegW::new(self, 12)
    }
    #[doc = "Bits 16:18 - DQ OE Down P-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_dn_p_reg(&mut self) -> DqOeDnPRegW<'_, PhyCfg50Spec> {
        DqOeDnPRegW::new(self, 16)
    }
    #[doc = "Bits 20:22 - DQ OE Down N-channel tuning"]
    #[inline(always)]
    pub fn dq_oe_dn_n_reg(&mut self) -> DqOeDnNRegW<'_, PhyCfg50Spec> {
        DqOeDnNRegW::new(self, 20)
    }
    #[doc = "Bits 24:26 - PHY write-latency CEN analog tuning"]
    #[inline(always)]
    pub fn phy_wl_cen_ana(&mut self) -> PhyWlCenAnaW<'_, PhyCfg50Spec> {
        PhyWlCenAnaW::new(self, 24)
    }
}
#[doc = "PHY Output Enable (OE) Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_cfg_50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_cfg_50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyCfg50Spec;
impl crate::RegisterSpec for PhyCfg50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_cfg_50::R`](R) reader structure"]
impl crate::Readable for PhyCfg50Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_cfg_50::W`](W) writer structure"]
impl crate::Writable for PhyCfg50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_CFG_50 to value 0x0144_4444"]
impl crate::Resettable for PhyCfg50Spec {
    const RESET_VALUE: u32 = 0x0144_4444;
}
