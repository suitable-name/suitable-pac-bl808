#[doc = "Register `DEV_FCFG4` reader"]
pub type R = crate::R<DevFcfg4Spec>;
#[doc = "Register `DEV_FCFG4` writer"]
pub type W = crate::W<DevFcfg4Spec>;
#[doc = "Field `BLK_TYP_F12` reader - "]
pub type BlkTypF12R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F12` writer - "]
pub type BlkTypF12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F12` reader - "]
pub type BlknoF12R = crate::FieldReader;
#[doc = "Field `BLKNO_F12` writer - "]
pub type BlknoF12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F12` reader - "]
pub type BlkszF12R = crate::BitReader;
#[doc = "Field `BLKSZ_F12` writer - "]
pub type BlkszF12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F12` reader - "]
pub type EnF12R = crate::BitReader;
#[doc = "Field `EN_F12` writer - "]
pub type EnF12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F13` reader - "]
pub type BlkTypF13R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F13` writer - "]
pub type BlkTypF13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F13` reader - "]
pub type BlknoF13R = crate::FieldReader;
#[doc = "Field `BLKNO_F13` writer - "]
pub type BlknoF13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F13` reader - "]
pub type BlkszF13R = crate::BitReader;
#[doc = "Field `BLKSZ_F13` writer - "]
pub type BlkszF13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F13` reader - "]
pub type EnF13R = crate::BitReader;
#[doc = "Field `EN_F13` writer - "]
pub type EnF13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F14` reader - "]
pub type BlkTypF14R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F14` writer - "]
pub type BlkTypF14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F14` reader - "]
pub type BlknoF14R = crate::FieldReader;
#[doc = "Field `BLKNO_F14` writer - "]
pub type BlknoF14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F14` reader - "]
pub type BlkszF14R = crate::BitReader;
#[doc = "Field `BLKSZ_F14` writer - "]
pub type BlkszF14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F14` reader - "]
pub type EnF14R = crate::BitReader;
#[doc = "Field `EN_F14` writer - "]
pub type EnF14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_TYP_F15` reader - "]
pub type BlkTypF15R = crate::FieldReader;
#[doc = "Field `BLK_TYP_F15` writer - "]
pub type BlkTypF15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKNO_F15` reader - "]
pub type BlknoF15R = crate::FieldReader;
#[doc = "Field `BLKNO_F15` writer - "]
pub type BlknoF15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BLKSZ_F15` reader - "]
pub type BlkszF15R = crate::BitReader;
#[doc = "Field `BLKSZ_F15` writer - "]
pub type BlkszF15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_F15` reader - "]
pub type EnF15R = crate::BitReader;
#[doc = "Field `EN_F15` writer - "]
pub type EnF15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn blk_typ_f12(&self) -> BlkTypF12R {
        BlkTypF12R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn blkno_f12(&self) -> BlknoF12R {
        BlknoF12R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn blksz_f12(&self) -> BlkszF12R {
        BlkszF12R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn en_f12(&self) -> EnF12R {
        EnF12R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn blk_typ_f13(&self) -> BlkTypF13R {
        BlkTypF13R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn blkno_f13(&self) -> BlknoF13R {
        BlknoF13R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn blksz_f13(&self) -> BlkszF13R {
        BlkszF13R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn en_f13(&self) -> EnF13R {
        EnF13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn blk_typ_f14(&self) -> BlkTypF14R {
        BlkTypF14R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn blkno_f14(&self) -> BlknoF14R {
        BlknoF14R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn blksz_f14(&self) -> BlkszF14R {
        BlkszF14R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn en_f14(&self) -> EnF14R {
        EnF14R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn blk_typ_f15(&self) -> BlkTypF15R {
        BlkTypF15R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn blkno_f15(&self) -> BlknoF15R {
        BlknoF15R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn blksz_f15(&self) -> BlkszF15R {
        BlkszF15R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn en_f15(&self) -> EnF15R {
        EnF15R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn blk_typ_f12(&mut self) -> BlkTypF12W<'_, DevFcfg4Spec> {
        BlkTypF12W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn blkno_f12(&mut self) -> BlknoF12W<'_, DevFcfg4Spec> {
        BlknoF12W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn blksz_f12(&mut self) -> BlkszF12W<'_, DevFcfg4Spec> {
        BlkszF12W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn en_f12(&mut self) -> EnF12W<'_, DevFcfg4Spec> {
        EnF12W::new(self, 5)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn blk_typ_f13(&mut self) -> BlkTypF13W<'_, DevFcfg4Spec> {
        BlkTypF13W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn blkno_f13(&mut self) -> BlknoF13W<'_, DevFcfg4Spec> {
        BlknoF13W::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn blksz_f13(&mut self) -> BlkszF13W<'_, DevFcfg4Spec> {
        BlkszF13W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn en_f13(&mut self) -> EnF13W<'_, DevFcfg4Spec> {
        EnF13W::new(self, 13)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn blk_typ_f14(&mut self) -> BlkTypF14W<'_, DevFcfg4Spec> {
        BlkTypF14W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn blkno_f14(&mut self) -> BlknoF14W<'_, DevFcfg4Spec> {
        BlknoF14W::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn blksz_f14(&mut self) -> BlkszF14W<'_, DevFcfg4Spec> {
        BlkszF14W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn en_f14(&mut self) -> EnF14W<'_, DevFcfg4Spec> {
        EnF14W::new(self, 21)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn blk_typ_f15(&mut self) -> BlkTypF15W<'_, DevFcfg4Spec> {
        BlkTypF15W::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn blkno_f15(&mut self) -> BlknoF15W<'_, DevFcfg4Spec> {
        BlknoF15W::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn blksz_f15(&mut self) -> BlkszF15W<'_, DevFcfg4Spec> {
        BlkszF15W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn en_f15(&mut self) -> EnF15W<'_, DevFcfg4Spec> {
        EnF15W::new(self, 29)
    }
}
#[doc = "Device FIFO Configuration 4 (FIFO 12-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_fcfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_fcfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevFcfg4Spec;
impl crate::RegisterSpec for DevFcfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_fcfg4::R`](R) reader structure"]
impl crate::Readable for DevFcfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_fcfg4::W`](W) writer structure"]
impl crate::Writable for DevFcfg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_FCFG4 to value 0"]
impl crate::Resettable for DevFcfg4Spec {}
