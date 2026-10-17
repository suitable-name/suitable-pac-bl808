#[doc = "Register `CPU_PLL_CFG11` reader"]
pub type R = crate::R<CpuPllCfg11Spec>;
#[doc = "Register `CPU_PLL_CFG11` writer"]
pub type W = crate::W<CpuPllCfg11Spec>;
#[doc = "Field `cpupll_resv` reader - "]
pub type CpupllResvR = crate::FieldReader<u16>;
#[doc = "Field `cpupll_resv` writer - "]
pub type CpupllResvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `cpupll_dl_ctrl_15` reader - "]
pub type CpupllDlCtrl15R = crate::BitReader;
#[doc = "Field `cpupll_dl_ctrl_15` writer - "]
pub type CpupllDlCtrl15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_dl_ctrl_10` reader - "]
pub type CpupllDlCtrl10R = crate::BitReader;
#[doc = "Field `cpupll_dl_ctrl_10` writer - "]
pub type CpupllDlCtrl10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_dl_ctrl_6` reader - "]
pub type CpupllDlCtrl6R = crate::BitReader;
#[doc = "Field `cpupll_dl_ctrl_6` writer - "]
pub type CpupllDlCtrl6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_dl_ctrl_5` reader - "]
pub type CpupllDlCtrl5R = crate::BitReader;
#[doc = "Field `cpupll_dl_ctrl_5` writer - "]
pub type CpupllDlCtrl5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_dl_ctrl_4` reader - "]
pub type CpupllDlCtrl4R = crate::BitReader;
#[doc = "Field `cpupll_dl_ctrl_4` writer - "]
pub type CpupllDlCtrl4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_dl_ctrl_3` reader - "]
pub type CpupllDlCtrl3R = crate::BitReader;
#[doc = "Field `cpupll_dl_ctrl_3` writer - "]
pub type CpupllDlCtrl3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_dl_ctrl_2p5` reader - "]
pub type CpupllDlCtrl2p5R = crate::BitReader;
#[doc = "Field `cpupll_dl_ctrl_2p5` writer - "]
pub type CpupllDlCtrl2p5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_dl_ctrl_2` reader - "]
pub type CpupllDlCtrl2R = crate::BitReader;
#[doc = "Field `cpupll_dl_ctrl_2` writer - "]
pub type CpupllDlCtrl2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_dl_ctrl_1` reader - "]
pub type CpupllDlCtrl1R = crate::BitReader;
#[doc = "Field `cpupll_dl_ctrl_1` writer - "]
pub type CpupllDlCtrl1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cpupll_resv(&self) -> CpupllResvR {
        CpupllResvR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_15(&self) -> CpupllDlCtrl15R {
        CpupllDlCtrl15R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_10(&self) -> CpupllDlCtrl10R {
        CpupllDlCtrl10R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_6(&self) -> CpupllDlCtrl6R {
        CpupllDlCtrl6R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_5(&self) -> CpupllDlCtrl5R {
        CpupllDlCtrl5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_4(&self) -> CpupllDlCtrl4R {
        CpupllDlCtrl4R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_3(&self) -> CpupllDlCtrl3R {
        CpupllDlCtrl3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_2p5(&self) -> CpupllDlCtrl2p5R {
        CpupllDlCtrl2p5R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_2(&self) -> CpupllDlCtrl2R {
        CpupllDlCtrl2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_1(&self) -> CpupllDlCtrl1R {
        CpupllDlCtrl1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cpupll_resv(&mut self) -> CpupllResvW<'_, CpuPllCfg11Spec> {
        CpupllResvW::new(self, 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_15(&mut self) -> CpupllDlCtrl15W<'_, CpuPllCfg11Spec> {
        CpupllDlCtrl15W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_10(&mut self) -> CpupllDlCtrl10W<'_, CpuPllCfg11Spec> {
        CpupllDlCtrl10W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_6(&mut self) -> CpupllDlCtrl6W<'_, CpuPllCfg11Spec> {
        CpupllDlCtrl6W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_5(&mut self) -> CpupllDlCtrl5W<'_, CpuPllCfg11Spec> {
        CpupllDlCtrl5W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_4(&mut self) -> CpupllDlCtrl4W<'_, CpuPllCfg11Spec> {
        CpupllDlCtrl4W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_3(&mut self) -> CpupllDlCtrl3W<'_, CpuPllCfg11Spec> {
        CpupllDlCtrl3W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_2p5(&mut self) -> CpupllDlCtrl2p5W<'_, CpuPllCfg11Spec> {
        CpupllDlCtrl2p5W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_2(&mut self) -> CpupllDlCtrl2W<'_, CpuPllCfg11Spec> {
        CpupllDlCtrl2W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cpupll_dl_ctrl_1(&mut self) -> CpupllDlCtrl1W<'_, CpuPllCfg11Spec> {
        CpupllDlCtrl1W::new(self, 31)
    }
}
#[doc = "CPU PLL Configuration 11 - Reserved and DL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg11Spec;
impl crate::RegisterSpec for CpuPllCfg11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg11::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg11Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg11::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG11 to value 0"]
impl crate::Resettable for CpuPllCfg11Spec {}
