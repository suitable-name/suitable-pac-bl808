#[doc = "Register `DPHY_CONFIG_5` reader"]
pub type R = crate::R<DphyConfig5Spec>;
#[doc = "Register `DPHY_CONFIG_5` writer"]
pub type W = crate::W<DphyConfig5Spec>;
#[doc = "Field `reg_csi_byte_clk_inv` reader - Invert CSI byte clock"]
pub type RegCsiByteClkInvR = crate::BitReader;
#[doc = "Field `reg_csi_byte_clk_inv` writer - Invert CSI byte clock"]
pub type RegCsiByteClkInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_csi_ddr_clk_inv` reader - Invert CSI DDR clock"]
pub type RegCsiDdrClkInvR = crate::BitReader;
#[doc = "Field `reg_csi_ddr_clk_inv` writer - Invert CSI DDR clock"]
pub type RegCsiDdrClkInvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invert CSI byte clock"]
    #[inline(always)]
    pub fn reg_csi_byte_clk_inv(&self) -> RegCsiByteClkInvR {
        RegCsiByteClkInvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invert CSI DDR clock"]
    #[inline(always)]
    pub fn reg_csi_ddr_clk_inv(&self) -> RegCsiDdrClkInvR {
        RegCsiDdrClkInvR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invert CSI byte clock"]
    #[inline(always)]
    pub fn reg_csi_byte_clk_inv(&mut self) -> RegCsiByteClkInvW<'_, DphyConfig5Spec> {
        RegCsiByteClkInvW::new(self, 0)
    }
    #[doc = "Bit 1 - Invert CSI DDR clock"]
    #[inline(always)]
    pub fn reg_csi_ddr_clk_inv(&mut self) -> RegCsiDdrClkInvW<'_, DphyConfig5Spec> {
        RegCsiDdrClkInvW::new(self, 1)
    }
}
#[doc = "D-PHY Clock Inversion Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets DPHY_CONFIG_5 to value 0"]
impl crate::Resettable for DphyConfig5Spec {}
