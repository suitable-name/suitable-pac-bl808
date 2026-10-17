#[doc = "Register `AUDIO_PLL_CFG11` reader"]
pub type R = crate::R<AudioPllCfg11Spec>;
#[doc = "Register `AUDIO_PLL_CFG11` writer"]
pub type W = crate::W<AudioPllCfg11Spec>;
#[doc = "Field `aupll_resv` reader - "]
pub type AupllResvR = crate::FieldReader<u16>;
#[doc = "Field `aupll_resv` writer - "]
pub type AupllResvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `aupll_dl_ctrl_15` reader - "]
pub type AupllDlCtrl15R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_15` writer - "]
pub type AupllDlCtrl15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_10` reader - "]
pub type AupllDlCtrl10R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_10` writer - "]
pub type AupllDlCtrl10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_6` reader - "]
pub type AupllDlCtrl6R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_6` writer - "]
pub type AupllDlCtrl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_5` reader - "]
pub type AupllDlCtrl5R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_5` writer - "]
pub type AupllDlCtrl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_4` reader - "]
pub type AupllDlCtrl4R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_4` writer - "]
pub type AupllDlCtrl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_3` reader - "]
pub type AupllDlCtrl3R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_3` writer - "]
pub type AupllDlCtrl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_2p5` reader - "]
pub type AupllDlCtrl2p5R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_2p5` writer - "]
pub type AupllDlCtrl2p5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_2` reader - "]
pub type AupllDlCtrl2R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_2` writer - "]
pub type AupllDlCtrl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_dl_ctrl_1` reader - "]
pub type AupllDlCtrl1R = crate::BitReader;
#[doc = "Field `aupll_dl_ctrl_1` writer - "]
pub type AupllDlCtrl1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn aupll_resv(&self) -> AupllResvR {
        AupllResvR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_15(&self) -> AupllDlCtrl15R {
        AupllDlCtrl15R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_10(&self) -> AupllDlCtrl10R {
        AupllDlCtrl10R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_6(&self) -> AupllDlCtrl6R {
        AupllDlCtrl6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_5(&self) -> AupllDlCtrl5R {
        AupllDlCtrl5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_4(&self) -> AupllDlCtrl4R {
        AupllDlCtrl4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_3(&self) -> AupllDlCtrl3R {
        AupllDlCtrl3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_2p5(&self) -> AupllDlCtrl2p5R {
        AupllDlCtrl2p5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_2(&self) -> AupllDlCtrl2R {
        AupllDlCtrl2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_1(&self) -> AupllDlCtrl1R {
        AupllDlCtrl1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn aupll_resv(&mut self) -> AupllResvW<'_, AudioPllCfg11Spec> {
        AupllResvW::new(self, 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_15(&mut self) -> AupllDlCtrl15W<'_, AudioPllCfg11Spec> {
        AupllDlCtrl15W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_10(&mut self) -> AupllDlCtrl10W<'_, AudioPllCfg11Spec> {
        AupllDlCtrl10W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_6(&mut self) -> AupllDlCtrl6W<'_, AudioPllCfg11Spec> {
        AupllDlCtrl6W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_5(&mut self) -> AupllDlCtrl5W<'_, AudioPllCfg11Spec> {
        AupllDlCtrl5W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_4(&mut self) -> AupllDlCtrl4W<'_, AudioPllCfg11Spec> {
        AupllDlCtrl4W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_3(&mut self) -> AupllDlCtrl3W<'_, AudioPllCfg11Spec> {
        AupllDlCtrl3W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_2p5(&mut self) -> AupllDlCtrl2p5W<'_, AudioPllCfg11Spec> {
        AupllDlCtrl2p5W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_2(&mut self) -> AupllDlCtrl2W<'_, AudioPllCfg11Spec> {
        AupllDlCtrl2W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn aupll_dl_ctrl_1(&mut self) -> AupllDlCtrl1W<'_, AudioPllCfg11Spec> {
        AupllDlCtrl1W::new(self, 31)
    }
}
#[doc = "Audio PLL Configuration 11 - Reserved and DL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`audio_pll_cfg11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`audio_pll_cfg11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudioPllCfg11Spec;
impl crate::RegisterSpec for AudioPllCfg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg11::R`](R) reader structure"]
impl crate::Readable for AudioPllCfg11Spec {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg11::W`](W) writer structure"]
impl crate::Writable for AudioPllCfg11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUDIO_PLL_CFG11 to value 0"]
impl crate::Resettable for AudioPllCfg11Spec {}
