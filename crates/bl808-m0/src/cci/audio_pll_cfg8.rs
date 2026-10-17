#[doc = "Register `AUDIO_PLL_CFG8` reader"]
pub type R = crate::R<AudioPllCfg8Spec>;
#[doc = "Register `AUDIO_PLL_CFG8` writer"]
pub type W = crate::W<AudioPllCfg8Spec>;
#[doc = "Field `aupll_en_div1` reader - "]
pub type AupllEnDiv1R = crate::BitReader;
#[doc = "Field `aupll_en_div1` writer - "]
pub type AupllEnDiv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_en_div2` reader - "]
pub type AupllEnDiv2R = crate::BitReader;
#[doc = "Field `aupll_en_div2` writer - "]
pub type AupllEnDiv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_en_div2p5` reader - "]
pub type AupllEnDiv2p5R = crate::BitReader;
#[doc = "Field `aupll_en_div2p5` writer - "]
pub type AupllEnDiv2p5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_en_div3` reader - "]
pub type AupllEnDiv3R = crate::BitReader;
#[doc = "Field `aupll_en_div3` writer - "]
pub type AupllEnDiv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_en_div4` reader - "]
pub type AupllEnDiv4R = crate::BitReader;
#[doc = "Field `aupll_en_div4` writer - "]
pub type AupllEnDiv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_en_div5` reader - "]
pub type AupllEnDiv5R = crate::BitReader;
#[doc = "Field `aupll_en_div5` writer - "]
pub type AupllEnDiv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_en_div6` reader - "]
pub type AupllEnDiv6R = crate::BitReader;
#[doc = "Field `aupll_en_div6` writer - "]
pub type AupllEnDiv6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_en_div10` reader - "]
pub type AupllEnDiv10R = crate::BitReader;
#[doc = "Field `aupll_en_div10` writer - "]
pub type AupllEnDiv10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_en_div15` reader - "]
pub type AupllEnDiv15R = crate::BitReader;
#[doc = "Field `aupll_en_div15` writer - "]
pub type AupllEnDiv15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_sel_div1_div2` reader - "]
pub type AupllSelDiv1Div2R = crate::BitReader;
#[doc = "Field `aupll_sel_div1_div2` writer - "]
pub type AupllSelDiv1Div2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_en_div1(&self) -> AupllEnDiv1R {
        AupllEnDiv1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn aupll_en_div2(&self) -> AupllEnDiv2R {
        AupllEnDiv2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn aupll_en_div2p5(&self) -> AupllEnDiv2p5R {
        AupllEnDiv2p5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn aupll_en_div3(&self) -> AupllEnDiv3R {
        AupllEnDiv3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn aupll_en_div4(&self) -> AupllEnDiv4R {
        AupllEnDiv4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn aupll_en_div5(&self) -> AupllEnDiv5R {
        AupllEnDiv5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn aupll_en_div6(&self) -> AupllEnDiv6R {
        AupllEnDiv6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn aupll_en_div10(&self) -> AupllEnDiv10R {
        AupllEnDiv10R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn aupll_en_div15(&self) -> AupllEnDiv15R {
        AupllEnDiv15R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn aupll_sel_div1_div2(&self) -> AupllSelDiv1Div2R {
        AupllSelDiv1Div2R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_en_div1(&mut self) -> AupllEnDiv1W<'_, AudioPllCfg8Spec> {
        AupllEnDiv1W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn aupll_en_div2(&mut self) -> AupllEnDiv2W<'_, AudioPllCfg8Spec> {
        AupllEnDiv2W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn aupll_en_div2p5(&mut self) -> AupllEnDiv2p5W<'_, AudioPllCfg8Spec> {
        AupllEnDiv2p5W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn aupll_en_div3(&mut self) -> AupllEnDiv3W<'_, AudioPllCfg8Spec> {
        AupllEnDiv3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn aupll_en_div4(&mut self) -> AupllEnDiv4W<'_, AudioPllCfg8Spec> {
        AupllEnDiv4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn aupll_en_div5(&mut self) -> AupllEnDiv5W<'_, AudioPllCfg8Spec> {
        AupllEnDiv5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn aupll_en_div6(&mut self) -> AupllEnDiv6W<'_, AudioPllCfg8Spec> {
        AupllEnDiv6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn aupll_en_div10(&mut self) -> AupllEnDiv10W<'_, AudioPllCfg8Spec> {
        AupllEnDiv10W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn aupll_en_div15(&mut self) -> AupllEnDiv15W<'_, AudioPllCfg8Spec> {
        AupllEnDiv15W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn aupll_sel_div1_div2(&mut self) -> AupllSelDiv1Div2W<'_, AudioPllCfg8Spec> {
        AupllSelDiv1Div2W::new(self, 9)
    }
}
#[doc = "Audio PLL Configuration 8 - Output Dividers\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg8Spec;
impl crate::RegisterSpec for AudioPllCfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg8::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg8::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG8 to value 0"]
impl crate::Resettable for AudioPllCfg8Spec {}
