#[doc = "Register `DEV_FCFG3` reader"]
pub type R = crate::R<DevFcfg3Spec>;
#[doc = "Register `DEV_FCFG3` writer"]
pub type W = crate::W<DevFcfg3Spec>;
#[doc = "Field `BLK_TYP_F8` reader - "]
pub type BlkTypF8R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F8` writer - "]
pub type BlkTypF8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F8` reader - "]
pub type BlknoF8R = crate::FieldReader;
#[doc = "Field `BLKNO_F8` writer - "]
pub type BlknoF8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F8` reader - "]
pub type BlkszF8R = crate::BitReader;
#[doc = "Field `BLKSZ_F8` writer - "]
pub type BlkszF8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F8` reader - "]
pub type EnF8R = crate::BitReader;
#[doc = "Field `EN_F8` writer - "]
pub type EnF8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F9` reader - "]
pub type BlkTypF9R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F9` writer - "]
pub type BlkTypF9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F9` reader - "]
pub type BlknoF9R = crate::FieldReader;
#[doc = "Field `BLKNO_F9` writer - "]
pub type BlknoF9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F9` reader - "]
pub type BlkszF9R = crate::BitReader;
#[doc = "Field `BLKSZ_F9` writer - "]
pub type BlkszF9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F9` reader - "]
pub type EnF9R = crate::BitReader;
#[doc = "Field `EN_F9` writer - "]
pub type EnF9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F10` reader - "]
pub type BlkTypF10R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F10` writer - "]
pub type BlkTypF10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F10` reader - "]
pub type BlknoF10R = crate::FieldReader;
#[doc = "Field `BLKNO_F10` writer - "]
pub type BlknoF10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F10` reader - "]
pub type BlkszF10R = crate::BitReader;
#[doc = "Field `BLKSZ_F10` writer - "]
pub type BlkszF10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F10` reader - "]
pub type EnF10R = crate::BitReader;
#[doc = "Field `EN_F10` writer - "]
pub type EnF10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F11` reader - "]
pub type BlkTypF11R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F11` writer - "]
pub type BlkTypF11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F11` reader - "]
pub type BlknoF11R = crate::FieldReader;
#[doc = "Field `BLKNO_F11` writer - "]
pub type BlknoF11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F11` reader - "]
pub type BlkszF11R = crate::BitReader;
#[doc = "Field `BLKSZ_F11` writer - "]
pub type BlkszF11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F11` reader - "]
pub type EnF11R = crate::BitReader;
#[doc = "Field `EN_F11` writer - "]
pub type EnF11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn blk_typ_f8(&self) -> BlkTypF8R {
        BlkTypF8R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn blkno_f8(&self) -> BlknoF8R {
        BlknoF8R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn blksz_f8(&self) -> BlkszF8R {
        BlkszF8R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn en_f8(&self) -> EnF8R {
        EnF8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn blk_typ_f9(&self) -> BlkTypF9R {
        BlkTypF9R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn blkno_f9(&self) -> BlknoF9R {
        BlknoF9R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn blksz_f9(&self) -> BlkszF9R {
        BlkszF9R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn en_f9(&self) -> EnF9R {
        EnF9R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn blk_typ_f10(&self) -> BlkTypF10R {
        BlkTypF10R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn blkno_f10(&self) -> BlknoF10R {
        BlknoF10R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn blksz_f10(&self) -> BlkszF10R {
        BlkszF10R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn en_f10(&self) -> EnF10R {
        EnF10R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn blk_typ_f11(&self) -> BlkTypF11R {
        BlkTypF11R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn blkno_f11(&self) -> BlknoF11R {
        BlknoF11R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn blksz_f11(&self) -> BlkszF11R {
        BlkszF11R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn en_f11(&self) -> EnF11R {
        EnF11R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn blk_typ_f8(&mut self) -> BlkTypF8W<'_, DevFcfg3Spec> {
        BlkTypF8W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn blkno_f8(&mut self) -> BlknoF8W<'_, DevFcfg3Spec> {
        BlknoF8W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn blksz_f8(&mut self) -> BlkszF8W<'_, DevFcfg3Spec> {
        BlkszF8W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn en_f8(&mut self) -> EnF8W<'_, DevFcfg3Spec> {
        EnF8W::new(self, 5)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn blk_typ_f9(&mut self) -> BlkTypF9W<'_, DevFcfg3Spec> {
        BlkTypF9W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn blkno_f9(&mut self) -> BlknoF9W<'_, DevFcfg3Spec> {
        BlknoF9W::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn blksz_f9(&mut self) -> BlkszF9W<'_, DevFcfg3Spec> {
        BlkszF9W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn en_f9(&mut self) -> EnF9W<'_, DevFcfg3Spec> {
        EnF9W::new(self, 13)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn blk_typ_f10(&mut self) -> BlkTypF10W<'_, DevFcfg3Spec> {
        BlkTypF10W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn blkno_f10(&mut self) -> BlknoF10W<'_, DevFcfg3Spec> {
        BlknoF10W::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn blksz_f10(&mut self) -> BlkszF10W<'_, DevFcfg3Spec> {
        BlkszF10W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn en_f10(&mut self) -> EnF10W<'_, DevFcfg3Spec> {
        EnF10W::new(self, 21)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn blk_typ_f11(&mut self) -> BlkTypF11W<'_, DevFcfg3Spec> {
        BlkTypF11W::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn blkno_f11(&mut self) -> BlknoF11W<'_, DevFcfg3Spec> {
        BlknoF11W::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn blksz_f11(&mut self) -> BlkszF11W<'_, DevFcfg3Spec> {
        BlkszF11W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn en_f11(&mut self) -> EnF11W<'_, DevFcfg3Spec> {
        EnF11W::new(self, 29)
    }
}
#[doc = "Device FIFO Configuration 3 (FIFO 8-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fcfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fcfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFcfg3Spec;
impl crate::RegisterSpec for DevFcfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fcfg3::R`](R) reader structure"]
impl crate::Readable for DevFcfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fcfg3::W`](W) writer structure"]
impl crate::Writable for DevFcfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FCFG3 to value 0"]
impl crate::Resettable for DevFcfg3Spec {}
