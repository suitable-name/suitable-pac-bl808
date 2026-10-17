#[doc = "Register `CPU_PLL_CFG8` reader"]
pub type R = crate::R<CpuPllCfg8Spec>;
#[doc = "Register `CPU_PLL_CFG8` writer"]
pub type W = crate::W<CpuPllCfg8Spec>;
#[doc = "Field `cpupll_en_div1` reader - "]
pub type CpupllEnDiv1R = crate::BitReader;
#[doc = "Field `cpupll_en_div1` writer - "]
pub type CpupllEnDiv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_en_div2` reader - "]
pub type CpupllEnDiv2R = crate::BitReader;
#[doc = "Field `cpupll_en_div2` writer - "]
pub type CpupllEnDiv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_en_div2p5` reader - "]
pub type CpupllEnDiv2p5R = crate::BitReader;
#[doc = "Field `cpupll_en_div2p5` writer - "]
pub type CpupllEnDiv2p5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_en_div3` reader - "]
pub type CpupllEnDiv3R = crate::BitReader;
#[doc = "Field `cpupll_en_div3` writer - "]
pub type CpupllEnDiv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_en_div4` reader - "]
pub type CpupllEnDiv4R = crate::BitReader;
#[doc = "Field `cpupll_en_div4` writer - "]
pub type CpupllEnDiv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_en_div5` reader - "]
pub type CpupllEnDiv5R = crate::BitReader;
#[doc = "Field `cpupll_en_div5` writer - "]
pub type CpupllEnDiv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_en_div6` reader - "]
pub type CpupllEnDiv6R = crate::BitReader;
#[doc = "Field `cpupll_en_div6` writer - "]
pub type CpupllEnDiv6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_en_div10` reader - "]
pub type CpupllEnDiv10R = crate::BitReader;
#[doc = "Field `cpupll_en_div10` writer - "]
pub type CpupllEnDiv10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_en_div15` reader - "]
pub type CpupllEnDiv15R = crate::BitReader;
#[doc = "Field `cpupll_en_div15` writer - "]
pub type CpupllEnDiv15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_sel_div1_div2` reader - "]
pub type CpupllSelDiv1Div2R = crate::BitReader;
#[doc = "Field `cpupll_sel_div1_div2` writer - "]
pub type CpupllSelDiv1Div2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_en_div1(&self) -> CpupllEnDiv1R {
        CpupllEnDiv1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cpupll_en_div2(&self) -> CpupllEnDiv2R {
        CpupllEnDiv2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cpupll_en_div2p5(&self) -> CpupllEnDiv2p5R {
        CpupllEnDiv2p5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cpupll_en_div3(&self) -> CpupllEnDiv3R {
        CpupllEnDiv3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cpupll_en_div4(&self) -> CpupllEnDiv4R {
        CpupllEnDiv4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cpupll_en_div5(&self) -> CpupllEnDiv5R {
        CpupllEnDiv5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cpupll_en_div6(&self) -> CpupllEnDiv6R {
        CpupllEnDiv6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cpupll_en_div10(&self) -> CpupllEnDiv10R {
        CpupllEnDiv10R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_en_div15(&self) -> CpupllEnDiv15R {
        CpupllEnDiv15R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cpupll_sel_div1_div2(&self) -> CpupllSelDiv1Div2R {
        CpupllSelDiv1Div2R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_en_div1(&mut self) -> CpupllEnDiv1W<'_, CpuPllCfg8Spec> {
        CpupllEnDiv1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cpupll_en_div2(&mut self) -> CpupllEnDiv2W<'_, CpuPllCfg8Spec> {
        CpupllEnDiv2W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cpupll_en_div2p5(&mut self) -> CpupllEnDiv2p5W<'_, CpuPllCfg8Spec> {
        CpupllEnDiv2p5W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cpupll_en_div3(&mut self) -> CpupllEnDiv3W<'_, CpuPllCfg8Spec> {
        CpupllEnDiv3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cpupll_en_div4(&mut self) -> CpupllEnDiv4W<'_, CpuPllCfg8Spec> {
        CpupllEnDiv4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cpupll_en_div5(&mut self) -> CpupllEnDiv5W<'_, CpuPllCfg8Spec> {
        CpupllEnDiv5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cpupll_en_div6(&mut self) -> CpupllEnDiv6W<'_, CpuPllCfg8Spec> {
        CpupllEnDiv6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cpupll_en_div10(&mut self) -> CpupllEnDiv10W<'_, CpuPllCfg8Spec> {
        CpupllEnDiv10W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpupll_en_div15(&mut self) -> CpupllEnDiv15W<'_, CpuPllCfg8Spec> {
        CpupllEnDiv15W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cpupll_sel_div1_div2(&mut self) -> CpupllSelDiv1Div2W<'_, CpuPllCfg8Spec> {
        CpupllSelDiv1Div2W::new(self, 9)
    }
}
#[doc = "CPU PLL Configuration 8 - Output Dividers\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg8Spec;
impl crate::RegisterSpec for CpuPllCfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg8::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg8::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG8 to value 0x01"]
impl crate::Resettable for CpuPllCfg8Spec {
    const RESET_VALUE: u32 = 0x01;
}
