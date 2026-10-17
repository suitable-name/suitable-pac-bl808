#[doc = "Register `CGEN_CFG3` reader"]
pub type R = crate::R<CgenCfg3Spec>;
#[doc = "Register `CGEN_CFG3` writer"]
pub type W = crate::W<CgenCfg3Spec>;
#[doc = "Field `cgen_mm_wifipll_160m` reader - "]
pub type CgenMmWifipll160mR = crate::BitReader;
#[doc = "Field `cgen_mm_wifipll_160m` writer - "]
pub type CgenMmWifipll160mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_mm_wifipll_240m` reader - "]
pub type CgenMmWifipll240mR = crate::BitReader;
#[doc = "Field `cgen_mm_wifipll_240m` writer - "]
pub type CgenMmWifipll240mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_mm_wifipll_320m` reader - "]
pub type CgenMmWifipll320mR = crate::BitReader;
#[doc = "Field `cgen_mm_wifipll_320m` writer - "]
pub type CgenMmWifipll320mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_mm_aupll_div1` reader - "]
pub type CgenMmAupllDiv1R = crate::BitReader;
#[doc = "Field `cgen_mm_aupll_div1` writer - "]
pub type CgenMmAupllDiv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_mm_aupll_div2` reader - "]
pub type CgenMmAupllDiv2R = crate::BitReader;
#[doc = "Field `cgen_mm_aupll_div2` writer - "]
pub type CgenMmAupllDiv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_emi_cpupll_400m` reader - "]
pub type CgenEmiCpupll400mR = crate::BitReader;
#[doc = "Field `cgen_emi_cpupll_400m` writer - "]
pub type CgenEmiCpupll400mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_emi_cpupll_200m` reader - "]
pub type CgenEmiCpupll200mR = crate::BitReader;
#[doc = "Field `cgen_emi_cpupll_200m` writer - "]
pub type CgenEmiCpupll200mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_emi_wifipll_320m` reader - "]
pub type CgenEmiWifipll320mR = crate::BitReader;
#[doc = "Field `cgen_emi_wifipll_320m` writer - "]
pub type CgenEmiWifipll320mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_emi_aupll_div1` reader - "]
pub type CgenEmiAupllDiv1R = crate::BitReader;
#[doc = "Field `cgen_emi_aupll_div1` writer - "]
pub type CgenEmiAupllDiv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_cpupll_80m` reader - "]
pub type CgenTopCpupll80mR = crate::BitReader;
#[doc = "Field `cgen_top_cpupll_80m` writer - "]
pub type CgenTopCpupll80mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_cpupll_100m` reader - "]
pub type CgenTopCpupll100mR = crate::BitReader;
#[doc = "Field `cgen_top_cpupll_100m` writer - "]
pub type CgenTopCpupll100mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_cpupll_160m` reader - "]
pub type CgenTopCpupll160mR = crate::BitReader;
#[doc = "Field `cgen_top_cpupll_160m` writer - "]
pub type CgenTopCpupll160mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_cpupll_400m` reader - "]
pub type CgenTopCpupll400mR = crate::BitReader;
#[doc = "Field `cgen_top_cpupll_400m` writer - "]
pub type CgenTopCpupll400mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_wifipll_240m` reader - "]
pub type CgenTopWifipll240mR = crate::BitReader;
#[doc = "Field `cgen_top_wifipll_240m` writer - "]
pub type CgenTopWifipll240mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_wifipll_320m` reader - "]
pub type CgenTopWifipll320mR = crate::BitReader;
#[doc = "Field `cgen_top_wifipll_320m` writer - "]
pub type CgenTopWifipll320mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_top_aupll_div2` reader - "]
pub type CgenTopAupllDiv2R = crate::BitReader;
#[doc = "Field `cgen_top_aupll_div2` writer - "]
pub type CgenTopAupllDiv2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_mm_wifipll_160m(&self) -> CgenMmWifipll160mR {
        CgenMmWifipll160mR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cgen_mm_wifipll_240m(&self) -> CgenMmWifipll240mR {
        CgenMmWifipll240mR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_mm_wifipll_320m(&self) -> CgenMmWifipll320mR {
        CgenMmWifipll320mR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_mm_aupll_div1(&self) -> CgenMmAupllDiv1R {
        CgenMmAupllDiv1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_mm_aupll_div2(&self) -> CgenMmAupllDiv2R {
        CgenMmAupllDiv2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cgen_emi_cpupll_400m(&self) -> CgenEmiCpupll400mR {
        CgenEmiCpupll400mR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cgen_emi_cpupll_200m(&self) -> CgenEmiCpupll200mR {
        CgenEmiCpupll200mR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cgen_emi_wifipll_320m(&self) -> CgenEmiWifipll320mR {
        CgenEmiWifipll320mR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cgen_emi_aupll_div1(&self) -> CgenEmiAupllDiv1R {
        CgenEmiAupllDiv1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cgen_top_cpupll_80m(&self) -> CgenTopCpupll80mR {
        CgenTopCpupll80mR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cgen_top_cpupll_100m(&self) -> CgenTopCpupll100mR {
        CgenTopCpupll100mR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cgen_top_cpupll_160m(&self) -> CgenTopCpupll160mR {
        CgenTopCpupll160mR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cgen_top_cpupll_400m(&self) -> CgenTopCpupll400mR {
        CgenTopCpupll400mR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cgen_top_wifipll_240m(&self) -> CgenTopWifipll240mR {
        CgenTopWifipll240mR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cgen_top_wifipll_320m(&self) -> CgenTopWifipll320mR {
        CgenTopWifipll320mR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cgen_top_aupll_div2(&self) -> CgenTopAupllDiv2R {
        CgenTopAupllDiv2R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_mm_wifipll_160m(&mut self) -> CgenMmWifipll160mW<'_, CgenCfg3Spec> {
        CgenMmWifipll160mW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cgen_mm_wifipll_240m(&mut self) -> CgenMmWifipll240mW<'_, CgenCfg3Spec> {
        CgenMmWifipll240mW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_mm_wifipll_320m(&mut self) -> CgenMmWifipll320mW<'_, CgenCfg3Spec> {
        CgenMmWifipll320mW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_mm_aupll_div1(&mut self) -> CgenMmAupllDiv1W<'_, CgenCfg3Spec> {
        CgenMmAupllDiv1W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_mm_aupll_div2(&mut self) -> CgenMmAupllDiv2W<'_, CgenCfg3Spec> {
        CgenMmAupllDiv2W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cgen_emi_cpupll_400m(&mut self) -> CgenEmiCpupll400mW<'_, CgenCfg3Spec> {
        CgenEmiCpupll400mW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cgen_emi_cpupll_200m(&mut self) -> CgenEmiCpupll200mW<'_, CgenCfg3Spec> {
        CgenEmiCpupll200mW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cgen_emi_wifipll_320m(&mut self) -> CgenEmiWifipll320mW<'_, CgenCfg3Spec> {
        CgenEmiWifipll320mW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cgen_emi_aupll_div1(&mut self) -> CgenEmiAupllDiv1W<'_, CgenCfg3Spec> {
        CgenEmiAupllDiv1W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cgen_top_cpupll_80m(&mut self) -> CgenTopCpupll80mW<'_, CgenCfg3Spec> {
        CgenTopCpupll80mW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cgen_top_cpupll_100m(&mut self) -> CgenTopCpupll100mW<'_, CgenCfg3Spec> {
        CgenTopCpupll100mW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cgen_top_cpupll_160m(&mut self) -> CgenTopCpupll160mW<'_, CgenCfg3Spec> {
        CgenTopCpupll160mW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cgen_top_cpupll_400m(&mut self) -> CgenTopCpupll400mW<'_, CgenCfg3Spec> {
        CgenTopCpupll400mW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cgen_top_wifipll_240m(&mut self) -> CgenTopWifipll240mW<'_, CgenCfg3Spec> {
        CgenTopWifipll240mW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cgen_top_wifipll_320m(&mut self) -> CgenTopWifipll320mW<'_, CgenCfg3Spec> {
        CgenTopWifipll320mW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cgen_top_aupll_div2(&mut self) -> CgenTopAupllDiv2W<'_, CgenCfg3Spec> {
        CgenTopAupllDiv2W::new(self, 15)
    }
}
#[doc = "Clock Gate Config 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgenCfg3Spec;
impl crate::RegisterSpec for CgenCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_cfg3::R`](R) reader structure"]
impl crate::Readable for CgenCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`cgen_cfg3::W`](W) writer structure"]
impl crate::Writable for CgenCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CGEN_CFG3 to value 0"]
impl crate::Resettable for CgenCfg3Spec {}
