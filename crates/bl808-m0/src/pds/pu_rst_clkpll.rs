#[doc = "Register `PU_RST_CLKPLL` reader"]
pub type R = crate::R<PuRstClkpllSpec>;
#[doc = "Register `PU_RST_CLKPLL` writer"]
pub type W = crate::W<PuRstClkpllSpec>;
#[doc = "Field `cr_pds_pu_clkpll_sfreg` reader - "]
pub type CrPdsPuClkpllSfregR = crate::BitReader;
#[doc = "Field `cr_pds_pu_clkpll_sfreg` writer - "]
pub type CrPdsPuClkpllSfregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pu_clkpll` reader - "]
pub type CrPdsPuClkpllR = crate::BitReader;
#[doc = "Field `cr_pds_pu_clkpll` writer - "]
pub type CrPdsPuClkpllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_pu_clkpll_sfreg(&self) -> CrPdsPuClkpllSfregR {
        CrPdsPuClkpllSfregR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_pu_clkpll(&self) -> CrPdsPuClkpllR {
        CrPdsPuClkpllR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_pu_clkpll_sfreg(&mut self) -> CrPdsPuClkpllSfregW<'_, PuRstClkpllSpec> {
        CrPdsPuClkpllSfregW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_pu_clkpll(&mut self) -> CrPdsPuClkpllW<'_, PuRstClkpllSpec> {
        CrPdsPuClkpllW::new(self, 10)
    }
}
#[doc = "Clock PLL power up and reset control\n\nYou can [`read`](crate::Reg::read) this register and get [`pu_rst_clkpll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pu_rst_clkpll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PuRstClkpllSpec;
impl crate::RegisterSpec for PuRstClkpllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pu_rst_clkpll::R`](R) reader structure"]
impl crate::Readable for PuRstClkpllSpec {}
#[doc = "`write(|w| ..)` method takes [`pu_rst_clkpll::W`](W) writer structure"]
impl crate::Writable for PuRstClkpllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PU_RST_CLKPLL to value 0"]
impl crate::Resettable for PuRstClkpllSpec {}
