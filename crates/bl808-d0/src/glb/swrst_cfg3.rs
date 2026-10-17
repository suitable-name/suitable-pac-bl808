#[doc = "Register `SWRST_CFG3` reader"]
pub type R = crate::R<SwrstCfg3Spec>;
#[doc = "Register `SWRST_CFG3` writer"]
pub type W = crate::W<SwrstCfg3Spec>;
#[doc = "Field `disrst_s12` reader - "]
pub type DisrstS12R = crate::BitReader;
#[doc = "Field `disrst_s12` writer - "]
pub type DisrstS12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s14` reader - "]
pub type DisrstS14R = crate::BitReader;
#[doc = "Field `disrst_s14` writer - "]
pub type DisrstS14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s18` reader - "]
pub type DisrstS18R = crate::BitReader;
#[doc = "Field `disrst_s18` writer - "]
pub type DisrstS18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1b` reader - "]
pub type DisrstS1bR = crate::BitReader;
#[doc = "Field `disrst_s1b` writer - "]
pub type DisrstS1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a0` reader - "]
pub type DisrstS1a0R = crate::BitReader;
#[doc = "Field `disrst_s1a0` writer - "]
pub type DisrstS1a0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a1` reader - "]
pub type DisrstS1a1R = crate::BitReader;
#[doc = "Field `disrst_s1a1` writer - "]
pub type DisrstS1a1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a2` reader - "]
pub type DisrstS1a2R = crate::BitReader;
#[doc = "Field `disrst_s1a2` writer - "]
pub type DisrstS1a2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a3` reader - "]
pub type DisrstS1a3R = crate::BitReader;
#[doc = "Field `disrst_s1a3` writer - "]
pub type DisrstS1a3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a4` reader - "]
pub type DisrstS1a4R = crate::BitReader;
#[doc = "Field `disrst_s1a4` writer - "]
pub type DisrstS1a4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a5` reader - "]
pub type DisrstS1a5R = crate::BitReader;
#[doc = "Field `disrst_s1a5` writer - "]
pub type DisrstS1a5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a6` reader - "]
pub type DisrstS1a6R = crate::BitReader;
#[doc = "Field `disrst_s1a6` writer - "]
pub type DisrstS1a6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a7` reader - "]
pub type DisrstS1a7R = crate::BitReader;
#[doc = "Field `disrst_s1a7` writer - "]
pub type DisrstS1a7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a8` reader - "]
pub type DisrstS1a8R = crate::BitReader;
#[doc = "Field `disrst_s1a8` writer - "]
pub type DisrstS1a8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a9` reader - "]
pub type DisrstS1a9R = crate::BitReader;
#[doc = "Field `disrst_s1a9` writer - "]
pub type DisrstS1a9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1aa` reader - "]
pub type DisrstS1aaR = crate::BitReader;
#[doc = "Field `disrst_s1aa` writer - "]
pub type DisrstS1aaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn disrst_s12(&self) -> DisrstS12R {
        DisrstS12R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn disrst_s14(&self) -> DisrstS14R {
        DisrstS14R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn disrst_s18(&self) -> DisrstS18R {
        DisrstS18R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn disrst_s1b(&self) -> DisrstS1bR {
        DisrstS1bR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn disrst_s1a0(&self) -> DisrstS1a0R {
        DisrstS1a0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn disrst_s1a1(&self) -> DisrstS1a1R {
        DisrstS1a1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn disrst_s1a2(&self) -> DisrstS1a2R {
        DisrstS1a2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn disrst_s1a3(&self) -> DisrstS1a3R {
        DisrstS1a3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn disrst_s1a4(&self) -> DisrstS1a4R {
        DisrstS1a4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn disrst_s1a5(&self) -> DisrstS1a5R {
        DisrstS1a5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn disrst_s1a6(&self) -> DisrstS1a6R {
        DisrstS1a6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn disrst_s1a7(&self) -> DisrstS1a7R {
        DisrstS1a7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn disrst_s1a8(&self) -> DisrstS1a8R {
        DisrstS1a8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn disrst_s1a9(&self) -> DisrstS1a9R {
        DisrstS1a9R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn disrst_s1aa(&self) -> DisrstS1aaR {
        DisrstS1aaR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn disrst_s12(&mut self) -> DisrstS12W<'_, SwrstCfg3Spec> {
        DisrstS12W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn disrst_s14(&mut self) -> DisrstS14W<'_, SwrstCfg3Spec> {
        DisrstS14W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn disrst_s18(&mut self) -> DisrstS18W<'_, SwrstCfg3Spec> {
        DisrstS18W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn disrst_s1b(&mut self) -> DisrstS1bW<'_, SwrstCfg3Spec> {
        DisrstS1bW::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn disrst_s1a0(&mut self) -> DisrstS1a0W<'_, SwrstCfg3Spec> {
        DisrstS1a0W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn disrst_s1a1(&mut self) -> DisrstS1a1W<'_, SwrstCfg3Spec> {
        DisrstS1a1W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn disrst_s1a2(&mut self) -> DisrstS1a2W<'_, SwrstCfg3Spec> {
        DisrstS1a2W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn disrst_s1a3(&mut self) -> DisrstS1a3W<'_, SwrstCfg3Spec> {
        DisrstS1a3W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn disrst_s1a4(&mut self) -> DisrstS1a4W<'_, SwrstCfg3Spec> {
        DisrstS1a4W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn disrst_s1a5(&mut self) -> DisrstS1a5W<'_, SwrstCfg3Spec> {
        DisrstS1a5W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn disrst_s1a6(&mut self) -> DisrstS1a6W<'_, SwrstCfg3Spec> {
        DisrstS1a6W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn disrst_s1a7(&mut self) -> DisrstS1a7W<'_, SwrstCfg3Spec> {
        DisrstS1a7W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn disrst_s1a8(&mut self) -> DisrstS1a8W<'_, SwrstCfg3Spec> {
        DisrstS1a8W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn disrst_s1a9(&mut self) -> DisrstS1a9W<'_, SwrstCfg3Spec> {
        DisrstS1a9W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn disrst_s1aa(&mut self) -> DisrstS1aaW<'_, SwrstCfg3Spec> {
        DisrstS1aaW::new(self, 26)
    }
}
#[doc = "HRESET Disable Config (SWRST_CFG3)\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst_cfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst_cfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwrstCfg3Spec;
impl crate::RegisterSpec for SwrstCfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_cfg3::R`](R) reader structure"]
impl crate::Readable for SwrstCfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`swrst_cfg3::W`](W) writer structure"]
impl crate::Writable for SwrstCfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWRST_CFG3 to value 0"]
impl crate::Resettable for SwrstCfg3Spec {}
