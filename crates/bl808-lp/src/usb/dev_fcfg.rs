#[doc = "Register `DEV_FCFG` reader"]
pub type R = crate::R<DevFcfgSpec>;
#[doc = "Register `DEV_FCFG` writer"]
pub type W = crate::W<DevFcfgSpec>;
#[doc = "Field `BLK_TYP_F0` reader - Block Type for FIFO 0"]
pub type BlkTypF0R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F0` writer - Block Type for FIFO 0"]
pub type BlkTypF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F0` reader - Number of blocks for FIFO 0"]
pub type BlknoF0R = crate::FieldReader;
#[doc = "Field `BLKNO_F0` writer - Number of blocks for FIFO 0"]
pub type BlknoF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F0` reader - Block size for FIFO 0"]
pub type BlkszF0R = crate::BitReader;
#[doc = "Field `BLKSZ_F0` writer - Block size for FIFO 0"]
pub type BlkszF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F0` reader - FIFO 0 enable"]
pub type EnF0R = crate::BitReader;
#[doc = "Field `EN_F0` writer - FIFO 0 enable"]
pub type EnF0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F1` reader - "]
pub type BlkTypF1R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F1` writer - "]
pub type BlkTypF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F1` reader - "]
pub type BlknoF1R = crate::FieldReader;
#[doc = "Field `BLKNO_F1` writer - "]
pub type BlknoF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F1` reader - "]
pub type BlkszF1R = crate::BitReader;
#[doc = "Field `BLKSZ_F1` writer - "]
pub type BlkszF1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F1` reader - "]
pub type EnF1R = crate::BitReader;
#[doc = "Field `EN_F1` writer - "]
pub type EnF1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F2` reader - "]
pub type BlkTypF2R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F2` writer - "]
pub type BlkTypF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F2` reader - "]
pub type BlknoF2R = crate::FieldReader;
#[doc = "Field `BLKNO_F2` writer - "]
pub type BlknoF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F2` reader - "]
pub type BlkszF2R = crate::BitReader;
#[doc = "Field `BLKSZ_F2` writer - "]
pub type BlkszF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F2` reader - "]
pub type EnF2R = crate::BitReader;
#[doc = "Field `EN_F2` writer - "]
pub type EnF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F3` reader - "]
pub type BlkTypF3R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F3` writer - "]
pub type BlkTypF3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F3` reader - "]
pub type BlknoF3R = crate::FieldReader;
#[doc = "Field `BLKNO_F3` writer - "]
pub type BlknoF3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F3` reader - "]
pub type BlkszF3R = crate::BitReader;
#[doc = "Field `BLKSZ_F3` writer - "]
pub type BlkszF3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F3` reader - "]
pub type EnF3R = crate::BitReader;
#[doc = "Field `EN_F3` writer - "]
pub type EnF3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Block Type for FIFO 0"]
    #[inline(always)]
    pub fn blk_typ_f0(&self) -> BlkTypF0R {
        BlkTypF0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Number of blocks for FIFO 0"]
    #[inline(always)]
    pub fn blkno_f0(&self) -> BlknoF0R {
        BlknoF0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Block size for FIFO 0"]
    #[inline(always)]
    pub fn blksz_f0(&self) -> BlkszF0R {
        BlkszF0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO 0 enable"]
    #[inline(always)]
    pub fn en_f0(&self) -> EnF0R {
        EnF0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn blk_typ_f1(&self) -> BlkTypF1R {
        BlkTypF1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn blkno_f1(&self) -> BlknoF1R {
        BlknoF1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn blksz_f1(&self) -> BlkszF1R {
        BlkszF1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn en_f1(&self) -> EnF1R {
        EnF1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn blk_typ_f2(&self) -> BlkTypF2R {
        BlkTypF2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn blkno_f2(&self) -> BlknoF2R {
        BlknoF2R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn blksz_f2(&self) -> BlkszF2R {
        BlkszF2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn en_f2(&self) -> EnF2R {
        EnF2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn blk_typ_f3(&self) -> BlkTypF3R {
        BlkTypF3R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn blkno_f3(&self) -> BlknoF3R {
        BlknoF3R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn blksz_f3(&self) -> BlkszF3R {
        BlkszF3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn en_f3(&self) -> EnF3R {
        EnF3R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Block Type for FIFO 0"]
    #[inline(always)]
    pub fn blk_typ_f0(&mut self) -> BlkTypF0W<'_, DevFcfgSpec> {
        BlkTypF0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Number of blocks for FIFO 0"]
    #[inline(always)]
    pub fn blkno_f0(&mut self) -> BlknoF0W<'_, DevFcfgSpec> {
        BlknoF0W::new(self, 2)
    }
    #[doc = "Bit 4 - Block size for FIFO 0"]
    #[inline(always)]
    pub fn blksz_f0(&mut self) -> BlkszF0W<'_, DevFcfgSpec> {
        BlkszF0W::new(self, 4)
    }
    #[doc = "Bit 5 - FIFO 0 enable"]
    #[inline(always)]
    pub fn en_f0(&mut self) -> EnF0W<'_, DevFcfgSpec> {
        EnF0W::new(self, 5)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn blk_typ_f1(&mut self) -> BlkTypF1W<'_, DevFcfgSpec> {
        BlkTypF1W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn blkno_f1(&mut self) -> BlknoF1W<'_, DevFcfgSpec> {
        BlknoF1W::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn blksz_f1(&mut self) -> BlkszF1W<'_, DevFcfgSpec> {
        BlkszF1W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn en_f1(&mut self) -> EnF1W<'_, DevFcfgSpec> {
        EnF1W::new(self, 13)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn blk_typ_f2(&mut self) -> BlkTypF2W<'_, DevFcfgSpec> {
        BlkTypF2W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn blkno_f2(&mut self) -> BlknoF2W<'_, DevFcfgSpec> {
        BlknoF2W::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn blksz_f2(&mut self) -> BlkszF2W<'_, DevFcfgSpec> {
        BlkszF2W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn en_f2(&mut self) -> EnF2W<'_, DevFcfgSpec> {
        EnF2W::new(self, 21)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn blk_typ_f3(&mut self) -> BlkTypF3W<'_, DevFcfgSpec> {
        BlkTypF3W::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn blkno_f3(&mut self) -> BlknoF3W<'_, DevFcfgSpec> {
        BlknoF3W::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn blksz_f3(&mut self) -> BlkszF3W<'_, DevFcfgSpec> {
        BlkszF3W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn en_f3(&mut self) -> EnF3W<'_, DevFcfgSpec> {
        EnF3W::new(self, 29)
    }
}
#[doc = "Device FIFO Configuration (FIFO 0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFcfgSpec;
impl crate::RegisterSpec for DevFcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fcfg::R`](R) reader structure"]
impl crate::Readable for DevFcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_fcfg::W`](W) writer structure"]
impl crate::Writable for DevFcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FCFG to value 0"]
impl crate::Resettable for DevFcfgSpec {}
