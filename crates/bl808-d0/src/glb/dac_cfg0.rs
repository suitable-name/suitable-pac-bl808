#[doc = "Register `DAC_CFG0` reader"]
pub type R = crate::R<DacCfg0Spec>;
#[doc = "Register `DAC_CFG0` writer"]
pub type W = crate::W<DacCfg0Spec>;
#[doc = "Field `gpdaca_rstn_ana` reader - "]
pub type GpdacaRstnAnaR = crate::BitReader;
#[doc = "Field `gpdaca_rstn_ana` writer - "]
pub type GpdacaRstnAnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdacb_rstn_ana` reader - "]
pub type GpdacbRstnAnaR = crate::BitReader;
#[doc = "Field `gpdacb_rstn_ana` writer - "]
pub type GpdacbRstnAnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_test_en` reader - "]
pub type GpdacTestEnR = crate::BitReader;
#[doc = "Field `gpdac_test_en` writer - "]
pub type GpdacTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_ref_sel` reader - "]
pub type GpdacRefSelR = crate::BitReader;
#[doc = "Field `gpdac_ref_sel` writer - "]
pub type GpdacRefSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_test_sel` reader - "]
pub type GpdacTestSelR = crate::FieldReader;
#[doc = "Field `gpdac_test_sel` writer - "]
pub type GpdacTestSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpdac_reserved` reader - "]
pub type GpdacReservedR = crate::FieldReader;
#[doc = "Field `gpdac_reserved` writer - "]
pub type GpdacReservedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdaca_rstn_ana(&self) -> GpdacaRstnAnaR {
        GpdacaRstnAnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdacb_rstn_ana(&self) -> GpdacbRstnAnaR {
        GpdacbRstnAnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpdac_test_en(&self) -> GpdacTestEnR {
        GpdacTestEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpdac_ref_sel(&self) -> GpdacRefSelR {
        GpdacRefSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn gpdac_test_sel(&self) -> GpdacTestSelR {
        GpdacTestSelR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gpdac_reserved(&self) -> GpdacReservedR {
        GpdacReservedR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdaca_rstn_ana(&mut self) -> GpdacaRstnAnaW<'_, DacCfg0Spec> {
        GpdacaRstnAnaW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdacb_rstn_ana(&mut self) -> GpdacbRstnAnaW<'_, DacCfg0Spec> {
        GpdacbRstnAnaW::new(self, 1)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpdac_test_en(&mut self) -> GpdacTestEnW<'_, DacCfg0Spec> {
        GpdacTestEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpdac_ref_sel(&mut self) -> GpdacRefSelW<'_, DacCfg0Spec> {
        GpdacRefSelW::new(self, 8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn gpdac_test_sel(&mut self) -> GpdacTestSelW<'_, DacCfg0Spec> {
        GpdacTestSelW::new(self, 9)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gpdac_reserved(&mut self) -> GpdacReservedW<'_, DacCfg0Spec> {
        GpdacReservedW::new(self, 24)
    }
}
#[doc = "GPDAC Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacCfg0Spec;
impl crate::RegisterSpec for DacCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_cfg0::R`](R) reader structure"]
impl crate::Readable for DacCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_cfg0::W`](W) writer structure"]
impl crate::Writable for DacCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAC_CFG0 to value 0"]
impl crate::Resettable for DacCfg0Spec {}
