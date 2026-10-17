#[doc = "Register `EF_DATA_1_LOCK` reader"]
pub type R = crate::R<EfData1LockSpec>;
#[doc = "Register `EF_DATA_1_LOCK` writer"]
pub type W = crate::W<EfData1LockSpec>;
#[doc = "Field `wr_lock_rsvd_1` reader - Write lock for reserved data bits"]
pub type WrLockRsvd1R = crate::FieldReader<u16>;
#[doc = "Field `wr_lock_rsvd_1` writer - Write lock for reserved data bits"]
pub type WrLockRsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `wr_lock_key_slot_4` reader - Write lock for Key Slot 4"]
pub type WrLockKeySlot4R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_4` writer - Write lock for Key Slot 4"]
pub type WrLockKeySlot4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_5` reader - Write lock for Key Slot 5"]
pub type WrLockKeySlot5R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_5` writer - Write lock for Key Slot 5"]
pub type WrLockKeySlot5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_6` reader - Write lock for Key Slot 6"]
pub type WrLockKeySlot6R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_6` writer - Write lock for Key Slot 6"]
pub type WrLockKeySlot6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_7` reader - Write lock for Key Slot 7"]
pub type WrLockKeySlot7R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_7` writer - Write lock for Key Slot 7"]
pub type WrLockKeySlot7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_8` reader - Write lock for Key Slot 8"]
pub type WrLockKeySlot8R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_8` writer - Write lock for Key Slot 8"]
pub type WrLockKeySlot8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_9` reader - Write lock for Key Slot 9"]
pub type WrLockKeySlot9R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_9` writer - Write lock for Key Slot 9"]
pub type WrLockKeySlot9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_10` reader - Write lock for Key Slot 10"]
pub type WrLockKeySlot10R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_10` writer - Write lock for Key Slot 10"]
pub type WrLockKeySlot10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_dat_1_rsvd_0` reader - Write lock for Reserved Word 0"]
pub type WrLockDat1Rsvd0R = crate::BitReader;
#[doc = "Field `wr_lock_dat_1_rsvd_0` writer - Write lock for Reserved Word 0"]
pub type WrLockDat1Rsvd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_dat_1_rsvd_1` reader - Write lock for Reserved Word 1"]
pub type WrLockDat1Rsvd1R = crate::BitReader;
#[doc = "Field `wr_lock_dat_1_rsvd_1` writer - Write lock for Reserved Word 1"]
pub type WrLockDat1Rsvd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_dat_1_rsvd_2` reader - Write lock for Reserved Word 2"]
pub type WrLockDat1Rsvd2R = crate::BitReader;
#[doc = "Field `wr_lock_dat_1_rsvd_2` writer - Write lock for Reserved Word 2"]
pub type WrLockDat1Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_4` reader - Read lock for Key Slot 4"]
pub type RdLockKeySlot4R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_4` writer - Read lock for Key Slot 4"]
pub type RdLockKeySlot4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_5` reader - Read lock for Key Slot 5"]
pub type RdLockKeySlot5R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_5` writer - Read lock for Key Slot 5"]
pub type RdLockKeySlot5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_6` reader - Read lock for Key Slot 6"]
pub type RdLockKeySlot6R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_6` writer - Read lock for Key Slot 6"]
pub type RdLockKeySlot6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_7` reader - Read lock for Key Slot 7"]
pub type RdLockKeySlot7R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_7` writer - Read lock for Key Slot 7"]
pub type RdLockKeySlot7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_8` reader - Read lock for Key Slot 8"]
pub type RdLockKeySlot8R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_8` writer - Read lock for Key Slot 8"]
pub type RdLockKeySlot8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_9` reader - Read lock for Key Slot 9"]
pub type RdLockKeySlot9R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_9` writer - Read lock for Key Slot 9"]
pub type RdLockKeySlot9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_10` reader - Read lock for Key Slot 10"]
pub type RdLockKeySlot10R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_10` writer - Read lock for Key Slot 10"]
pub type RdLockKeySlot10W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Write lock for reserved data bits"]
    #[inline(always)]
    pub fn wr_lock_rsvd_1(&self) -> WrLockRsvd1R {
        WrLockRsvd1R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Write lock for Key Slot 4"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4(&self) -> WrLockKeySlot4R {
        WrLockKeySlot4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write lock for Key Slot 5"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5(&self) -> WrLockKeySlot5R {
        WrLockKeySlot5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write lock for Key Slot 6"]
    #[inline(always)]
    pub fn wr_lock_key_slot_6(&self) -> WrLockKeySlot6R {
        WrLockKeySlot6R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write lock for Key Slot 7"]
    #[inline(always)]
    pub fn wr_lock_key_slot_7(&self) -> WrLockKeySlot7R {
        WrLockKeySlot7R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write lock for Key Slot 8"]
    #[inline(always)]
    pub fn wr_lock_key_slot_8(&self) -> WrLockKeySlot8R {
        WrLockKeySlot8R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write lock for Key Slot 9"]
    #[inline(always)]
    pub fn wr_lock_key_slot_9(&self) -> WrLockKeySlot9R {
        WrLockKeySlot9R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write lock for Key Slot 10"]
    #[inline(always)]
    pub fn wr_lock_key_slot_10(&self) -> WrLockKeySlot10R {
        WrLockKeySlot10R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write lock for Reserved Word 0"]
    #[inline(always)]
    pub fn wr_lock_dat_1_rsvd_0(&self) -> WrLockDat1Rsvd0R {
        WrLockDat1Rsvd0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write lock for Reserved Word 1"]
    #[inline(always)]
    pub fn wr_lock_dat_1_rsvd_1(&self) -> WrLockDat1Rsvd1R {
        WrLockDat1Rsvd1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Write lock for Reserved Word 2"]
    #[inline(always)]
    pub fn wr_lock_dat_1_rsvd_2(&self) -> WrLockDat1Rsvd2R {
        WrLockDat1Rsvd2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Read lock for Key Slot 4"]
    #[inline(always)]
    pub fn rd_lock_key_slot_4(&self) -> RdLockKeySlot4R {
        RdLockKeySlot4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Read lock for Key Slot 5"]
    #[inline(always)]
    pub fn rd_lock_key_slot_5(&self) -> RdLockKeySlot5R {
        RdLockKeySlot5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Read lock for Key Slot 6"]
    #[inline(always)]
    pub fn rd_lock_key_slot_6(&self) -> RdLockKeySlot6R {
        RdLockKeySlot6R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Read lock for Key Slot 7"]
    #[inline(always)]
    pub fn rd_lock_key_slot_7(&self) -> RdLockKeySlot7R {
        RdLockKeySlot7R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Read lock for Key Slot 8"]
    #[inline(always)]
    pub fn rd_lock_key_slot_8(&self) -> RdLockKeySlot8R {
        RdLockKeySlot8R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Read lock for Key Slot 9"]
    #[inline(always)]
    pub fn rd_lock_key_slot_9(&self) -> RdLockKeySlot9R {
        RdLockKeySlot9R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Read lock for Key Slot 10"]
    #[inline(always)]
    pub fn rd_lock_key_slot_10(&self) -> RdLockKeySlot10R {
        RdLockKeySlot10R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Write lock for reserved data bits"]
    #[inline(always)]
    pub fn wr_lock_rsvd_1(&mut self) -> WrLockRsvd1W<'_, EfData1LockSpec> {
        WrLockRsvd1W::new(self, 0)
    }
    #[doc = "Bit 15 - Write lock for Key Slot 4"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4(&mut self) -> WrLockKeySlot4W<'_, EfData1LockSpec> {
        WrLockKeySlot4W::new(self, 15)
    }
    #[doc = "Bit 16 - Write lock for Key Slot 5"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5(&mut self) -> WrLockKeySlot5W<'_, EfData1LockSpec> {
        WrLockKeySlot5W::new(self, 16)
    }
    #[doc = "Bit 17 - Write lock for Key Slot 6"]
    #[inline(always)]
    pub fn wr_lock_key_slot_6(&mut self) -> WrLockKeySlot6W<'_, EfData1LockSpec> {
        WrLockKeySlot6W::new(self, 17)
    }
    #[doc = "Bit 18 - Write lock for Key Slot 7"]
    #[inline(always)]
    pub fn wr_lock_key_slot_7(&mut self) -> WrLockKeySlot7W<'_, EfData1LockSpec> {
        WrLockKeySlot7W::new(self, 18)
    }
    #[doc = "Bit 19 - Write lock for Key Slot 8"]
    #[inline(always)]
    pub fn wr_lock_key_slot_8(&mut self) -> WrLockKeySlot8W<'_, EfData1LockSpec> {
        WrLockKeySlot8W::new(self, 19)
    }
    #[doc = "Bit 20 - Write lock for Key Slot 9"]
    #[inline(always)]
    pub fn wr_lock_key_slot_9(&mut self) -> WrLockKeySlot9W<'_, EfData1LockSpec> {
        WrLockKeySlot9W::new(self, 20)
    }
    #[doc = "Bit 21 - Write lock for Key Slot 10"]
    #[inline(always)]
    pub fn wr_lock_key_slot_10(&mut self) -> WrLockKeySlot10W<'_, EfData1LockSpec> {
        WrLockKeySlot10W::new(self, 21)
    }
    #[doc = "Bit 22 - Write lock for Reserved Word 0"]
    #[inline(always)]
    pub fn wr_lock_dat_1_rsvd_0(&mut self) -> WrLockDat1Rsvd0W<'_, EfData1LockSpec> {
        WrLockDat1Rsvd0W::new(self, 22)
    }
    #[doc = "Bit 23 - Write lock for Reserved Word 1"]
    #[inline(always)]
    pub fn wr_lock_dat_1_rsvd_1(&mut self) -> WrLockDat1Rsvd1W<'_, EfData1LockSpec> {
        WrLockDat1Rsvd1W::new(self, 23)
    }
    #[doc = "Bit 24 - Write lock for Reserved Word 2"]
    #[inline(always)]
    pub fn wr_lock_dat_1_rsvd_2(&mut self) -> WrLockDat1Rsvd2W<'_, EfData1LockSpec> {
        WrLockDat1Rsvd2W::new(self, 24)
    }
    #[doc = "Bit 25 - Read lock for Key Slot 4"]
    #[inline(always)]
    pub fn rd_lock_key_slot_4(&mut self) -> RdLockKeySlot4W<'_, EfData1LockSpec> {
        RdLockKeySlot4W::new(self, 25)
    }
    #[doc = "Bit 26 - Read lock for Key Slot 5"]
    #[inline(always)]
    pub fn rd_lock_key_slot_5(&mut self) -> RdLockKeySlot5W<'_, EfData1LockSpec> {
        RdLockKeySlot5W::new(self, 26)
    }
    #[doc = "Bit 27 - Read lock for Key Slot 6"]
    #[inline(always)]
    pub fn rd_lock_key_slot_6(&mut self) -> RdLockKeySlot6W<'_, EfData1LockSpec> {
        RdLockKeySlot6W::new(self, 27)
    }
    #[doc = "Bit 28 - Read lock for Key Slot 7"]
    #[inline(always)]
    pub fn rd_lock_key_slot_7(&mut self) -> RdLockKeySlot7W<'_, EfData1LockSpec> {
        RdLockKeySlot7W::new(self, 28)
    }
    #[doc = "Bit 29 - Read lock for Key Slot 8"]
    #[inline(always)]
    pub fn rd_lock_key_slot_8(&mut self) -> RdLockKeySlot8W<'_, EfData1LockSpec> {
        RdLockKeySlot8W::new(self, 29)
    }
    #[doc = "Bit 30 - Read lock for Key Slot 9"]
    #[inline(always)]
    pub fn rd_lock_key_slot_9(&mut self) -> RdLockKeySlot9W<'_, EfData1LockSpec> {
        RdLockKeySlot9W::new(self, 30)
    }
    #[doc = "Bit 31 - Read lock for Key Slot 10"]
    #[inline(always)]
    pub fn rd_lock_key_slot_10(&mut self) -> RdLockKeySlot10W<'_, EfData1LockSpec> {
        RdLockKeySlot10W::new(self, 31)
    }
}
#[doc = "eFuse Data 1 Write/Read Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_data_1_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_data_1_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfData1LockSpec;
impl crate::RegisterSpec for EfData1LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_data_1_lock::R`](R) reader structure"]
impl crate::Readable for EfData1LockSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_data_1_lock::W`](W) writer structure"]
impl crate::Writable for EfData1LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_DATA_1_LOCK to value 0"]
impl crate::Resettable for EfData1LockSpec {}
