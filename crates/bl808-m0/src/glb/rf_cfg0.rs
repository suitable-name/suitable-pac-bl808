#[doc = "Register `RF_CFG0` reader"]
pub type R = crate::R<RfCfg0Spec>;
#[doc = "Register `RF_CFG0` writer"]
pub type W = crate::W<RfCfg0Spec>;
#[doc = "Field `cfg_inv_rf2_test_clk_o` reader - "]
pub type CfgInvRf2TestClkOR = crate::BitReader;
#[doc = "Field `cfg_inv_rf2_test_clk_o` writer - "]
pub type CfgInvRf2TestClkOW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_inv_rf2_test_clk_o(&self) -> CfgInvRf2TestClkOR {
        CfgInvRf2TestClkOR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_inv_rf2_test_clk_o(&mut self) -> CfgInvRf2TestClkOW<'_, RfCfg0Spec> {
        CfgInvRf2TestClkOW::new(self, 9)
    }
}
#[doc = "RF Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfCfg0Spec;
impl crate::RegisterSpec for RfCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_cfg0::R`](R) reader structure"]
impl crate::Readable for RfCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`rf_cfg0::W`](W) writer structure"]
impl crate::Writable for RfCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF_CFG0 to value 0"]
impl crate::Resettable for RfCfg0Spec {}
