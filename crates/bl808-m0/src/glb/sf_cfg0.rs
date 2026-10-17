#[doc = "Register `SF_CFG0` reader"]
pub type R = crate::R<SfCfg0Spec>;
#[doc = "Register `SF_CFG0` writer"]
pub type W = crate::W<SfCfg0Spec>;
#[doc = "Field `sf_clk_div` reader - "]
pub type SfClkDivR = crate::FieldReader;
#[doc = "Field `sf_clk_div` writer - "]
pub type SfClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_clk_en` reader - "]
pub type SfClkEnR = crate::BitReader;
#[doc = "Field `sf_clk_en` writer - "]
pub type SfClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_clk_sel` reader - "]
pub type SfClkSelR = crate::FieldReader;
#[doc = "Field `sf_clk_sel` writer - "]
pub type SfClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_clk_sel2` reader - "]
pub type SfClkSel2R = crate::FieldReader;
#[doc = "Field `sf_clk_sel2` writer - "]
pub type SfClkSel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_clk_div(&self) -> SfClkDivR {
        SfClkDivR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_clk_en(&self) -> SfClkEnR {
        SfClkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sf_clk_sel(&self) -> SfClkSelR {
        SfClkSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sf_clk_sel2(&self) -> SfClkSel2R {
        SfClkSel2R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_clk_div(&mut self) -> SfClkDivW<'_, SfCfg0Spec> {
        SfClkDivW::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_clk_en(&mut self) -> SfClkEnW<'_, SfCfg0Spec> {
        SfClkEnW::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sf_clk_sel(&mut self) -> SfClkSelW<'_, SfCfg0Spec> {
        SfClkSelW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn sf_clk_sel2(&mut self) -> SfClkSel2W<'_, SfCfg0Spec> {
        SfClkSel2W::new(self, 14)
    }
}
#[doc = "Serial Flash Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfCfg0Spec;
impl crate::RegisterSpec for SfCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_cfg0::R`](R) reader structure"]
impl crate::Readable for SfCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_cfg0::W`](W) writer structure"]
impl crate::Writable for SfCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_CFG0 to value 0"]
impl crate::Resettable for SfCfg0Spec {}
