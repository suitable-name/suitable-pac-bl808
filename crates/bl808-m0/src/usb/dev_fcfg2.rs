#[doc = "Register `DEV_FCFG2` reader"]
pub type R = crate::R<DevFcfg2Spec>;
#[doc = "Register `DEV_FCFG2` writer"]
pub type W = crate::W<DevFcfg2Spec>;
#[doc = "Field `BLK_TYP_F4` reader - "]
pub type BlkTypF4R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F4` writer - "]
pub type BlkTypF4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F4` reader - "]
pub type BlknoF4R = crate::FieldReader;
#[doc = "Field `BLKNO_F4` writer - "]
pub type BlknoF4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F4` reader - "]
pub type BlkszF4R = crate::BitReader;
#[doc = "Field `BLKSZ_F4` writer - "]
pub type BlkszF4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F4` reader - "]
pub type EnF4R = crate::BitReader;
#[doc = "Field `EN_F4` writer - "]
pub type EnF4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F5` reader - "]
pub type BlkTypF5R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F5` writer - "]
pub type BlkTypF5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F5` reader - "]
pub type BlknoF5R = crate::FieldReader;
#[doc = "Field `BLKNO_F5` writer - "]
pub type BlknoF5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F5` reader - "]
pub type BlkszF5R = crate::BitReader;
#[doc = "Field `BLKSZ_F5` writer - "]
pub type BlkszF5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F5` reader - "]
pub type EnF5R = crate::BitReader;
#[doc = "Field `EN_F5` writer - "]
pub type EnF5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F6` reader - "]
pub type BlkTypF6R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F6` writer - "]
pub type BlkTypF6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F6` reader - "]
pub type BlknoF6R = crate::FieldReader;
#[doc = "Field `BLKNO_F6` writer - "]
pub type BlknoF6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F6` reader - "]
pub type BlkszF6R = crate::BitReader;
#[doc = "Field `BLKSZ_F6` writer - "]
pub type BlkszF6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F6` reader - "]
pub type EnF6R = crate::BitReader;
#[doc = "Field `EN_F6` writer - "]
pub type EnF6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F7` reader - "]
pub type BlkTypF7R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F7` writer - "]
pub type BlkTypF7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F7` reader - "]
pub type BlknoF7R = crate::FieldReader;
#[doc = "Field `BLKNO_F7` writer - "]
pub type BlknoF7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F7` reader - "]
pub type BlkszF7R = crate::BitReader;
#[doc = "Field `BLKSZ_F7` writer - "]
pub type BlkszF7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F7` reader - "]
pub type EnF7R = crate::BitReader;
#[doc = "Field `EN_F7` writer - "]
pub type EnF7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn blk_typ_f4(&self) -> BlkTypF4R {
        BlkTypF4R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn blkno_f4(&self) -> BlknoF4R {
        BlknoF4R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn blksz_f4(&self) -> BlkszF4R {
        BlkszF4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn en_f4(&self) -> EnF4R {
        EnF4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn blk_typ_f5(&self) -> BlkTypF5R {
        BlkTypF5R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn blkno_f5(&self) -> BlknoF5R {
        BlknoF5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn blksz_f5(&self) -> BlkszF5R {
        BlkszF5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn en_f5(&self) -> EnF5R {
        EnF5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn blk_typ_f6(&self) -> BlkTypF6R {
        BlkTypF6R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn blkno_f6(&self) -> BlknoF6R {
        BlknoF6R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn blksz_f6(&self) -> BlkszF6R {
        BlkszF6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn en_f6(&self) -> EnF6R {
        EnF6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn blk_typ_f7(&self) -> BlkTypF7R {
        BlkTypF7R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn blkno_f7(&self) -> BlknoF7R {
        BlknoF7R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn blksz_f7(&self) -> BlkszF7R {
        BlkszF7R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn en_f7(&self) -> EnF7R {
        EnF7R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn blk_typ_f4(&mut self) -> BlkTypF4W<'_, DevFcfg2Spec> {
        BlkTypF4W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn blkno_f4(&mut self) -> BlknoF4W<'_, DevFcfg2Spec> {
        BlknoF4W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn blksz_f4(&mut self) -> BlkszF4W<'_, DevFcfg2Spec> {
        BlkszF4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn en_f4(&mut self) -> EnF4W<'_, DevFcfg2Spec> {
        EnF4W::new(self, 5)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn blk_typ_f5(&mut self) -> BlkTypF5W<'_, DevFcfg2Spec> {
        BlkTypF5W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn blkno_f5(&mut self) -> BlknoF5W<'_, DevFcfg2Spec> {
        BlknoF5W::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn blksz_f5(&mut self) -> BlkszF5W<'_, DevFcfg2Spec> {
        BlkszF5W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn en_f5(&mut self) -> EnF5W<'_, DevFcfg2Spec> {
        EnF5W::new(self, 13)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn blk_typ_f6(&mut self) -> BlkTypF6W<'_, DevFcfg2Spec> {
        BlkTypF6W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn blkno_f6(&mut self) -> BlknoF6W<'_, DevFcfg2Spec> {
        BlknoF6W::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn blksz_f6(&mut self) -> BlkszF6W<'_, DevFcfg2Spec> {
        BlkszF6W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn en_f6(&mut self) -> EnF6W<'_, DevFcfg2Spec> {
        EnF6W::new(self, 21)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn blk_typ_f7(&mut self) -> BlkTypF7W<'_, DevFcfg2Spec> {
        BlkTypF7W::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn blkno_f7(&mut self) -> BlknoF7W<'_, DevFcfg2Spec> {
        BlknoF7W::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn blksz_f7(&mut self) -> BlkszF7W<'_, DevFcfg2Spec> {
        BlkszF7W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn en_f7(&mut self) -> EnF7W<'_, DevFcfg2Spec> {
        EnF7W::new(self, 29)
    }
}
#[doc = "Device FIFO Configuration 2 (FIFO 4-7)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fcfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFcfg2Spec;
impl crate::RegisterSpec for DevFcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fcfg2::R`](R) reader structure"]
impl crate::Readable for DevFcfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fcfg2::W`](W) writer structure"]
impl crate::Writable for DevFcfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FCFG2 to value 0"]
impl crate::Resettable for DevFcfg2Spec {}
