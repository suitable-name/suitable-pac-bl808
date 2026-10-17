#[doc = "Register `DPHY_CONFIG_16` reader"]
pub type R = crate::R<DphyConfig16Spec>;
#[doc = "Register `DPHY_CONFIG_16` writer"]
pub type W = crate::W<DphyConfig16Spec>;
#[doc = "Field `reg_dsi_byte_clk_inv` reader - "]
pub type RegDsiByteClkInvR = crate::BitReader;
#[doc = "Field `reg_dsi_byte_clk_inv` writer - "]
pub type RegDsiByteClkInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dsi_lprx_clk_inv` reader - "]
pub type RegDsiLprxClkInvR = crate::BitReader;
#[doc = "Field `reg_dsi_lprx_clk_inv` writer - "]
pub type RegDsiLprxClkInvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dsi_byte_clk_inv(&self) -> RegDsiByteClkInvR {
        RegDsiByteClkInvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_dsi_lprx_clk_inv(&self) -> RegDsiLprxClkInvR {
        RegDsiLprxClkInvR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dsi_byte_clk_inv(&mut self) -> RegDsiByteClkInvW<'_, DphyConfig16Spec> {
        RegDsiByteClkInvW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_dsi_lprx_clk_inv(&mut self) -> RegDsiLprxClkInvW<'_, DphyConfig16Spec> {
        RegDsiLprxClkInvW::new(self, 1)
    }
}
#[doc = "DPHY Clock Inversion Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig16Spec;
impl crate::RegisterSpec for DphyConfig16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_16::R`](R) reader structure"]
impl crate::Readable for DphyConfig16Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_16::W`](W) writer structure"]
impl crate::Writable for DphyConfig16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_16 to value 0"]
impl crate::Resettable for DphyConfig16Spec {}
