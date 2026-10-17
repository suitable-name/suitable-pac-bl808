#[doc = "Register `EF_DATA_0_LOCK` reader"]
pub type R = crate::R<EfData0LockSpec>;
#[doc = "Register `EF_DATA_0_LOCK` writer"]
pub type W = crate::W<EfData0LockSpec>;
#[doc = "Field `ef_sec_lifecycle` reader - Security Life Cycle (SBLC)"]
pub type EfSecLifecycleR = crate::FieldReader;
#[doc = "Field `ef_sec_lifecycle` writer - Security Life Cycle (SBLC)"]
pub type EfSecLifecycleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wr_lock_rsvd_0` reader - Write lock for reserved data bits"]
pub type WrLockRsvd0R = crate::FieldReader<u16>;
#[doc = "Field `wr_lock_rsvd_0` writer - Write lock for reserved data bits"]
pub type WrLockRsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `wr_lock_boot_mode` reader - Write lock for boot mode/SF AES fields"]
pub type WrLockBootModeR = crate::BitReader;
#[doc = "Field `wr_lock_boot_mode` writer - Write lock for boot mode/SF AES fields"]
pub type WrLockBootModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_dbg_pwd` reader - Write lock for debug password slots 1 & 2"]
pub type WrLockDbgPwdR = crate::BitReader;
#[doc = "Field `wr_lock_dbg_pwd` writer - Write lock for debug password slots 1 & 2"]
pub type WrLockDbgPwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_wifi_mac` reader - Write lock for Wi-Fi MAC address"]
pub type WrLockWifiMacR = crate::BitReader;
#[doc = "Field `wr_lock_wifi_mac` writer - Write lock for Wi-Fi MAC address"]
pub type WrLockWifiMacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_0` reader - Write lock for Key Slot 0"]
pub type WrLockKeySlot0R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_0` writer - Write lock for Key Slot 0"]
pub type WrLockKeySlot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_1` reader - Write lock for Key Slot 1"]
pub type WrLockKeySlot1R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_1` writer - Write lock for Key Slot 1"]
pub type WrLockKeySlot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_2` reader - Write lock for Key Slot 2"]
pub type WrLockKeySlot2R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_2` writer - Write lock for Key Slot 2"]
pub type WrLockKeySlot2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_3` reader - Write lock for Key Slot 3"]
pub type WrLockKeySlot3R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_3` writer - Write lock for Key Slot 3"]
pub type WrLockKeySlot3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_sw_usage_0` reader - Write lock for Software Usage 0"]
pub type WrLockSwUsage0R = crate::BitReader;
#[doc = "Field `wr_lock_sw_usage_0` writer - Write lock for Software Usage 0"]
pub type WrLockSwUsage0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_sw_usage_1` reader - Write lock for Software Usage 1"]
pub type WrLockSwUsage1R = crate::BitReader;
#[doc = "Field `wr_lock_sw_usage_1` writer - Write lock for Software Usage 1"]
pub type WrLockSwUsage1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_sw_usage_2` reader - Write lock for Software Usage 2"]
pub type WrLockSwUsage2R = crate::BitReader;
#[doc = "Field `wr_lock_sw_usage_2` writer - Write lock for Software Usage 2"]
pub type WrLockSwUsage2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_sw_usage_3` reader - Write lock for Software Usage 3"]
pub type WrLockSwUsage3R = crate::BitReader;
#[doc = "Field `wr_lock_sw_usage_3` writer - Write lock for Software Usage 3"]
pub type WrLockSwUsage3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_lock_key_slot_11` reader - Write lock for Key Slot 11"]
pub type WrLockKeySlot11R = crate::BitReader;
#[doc = "Field `wr_lock_key_slot_11` writer - Write lock for Key Slot 11"]
pub type WrLockKeySlot11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_dbg_pwd` reader - Read lock for debug password"]
pub type RdLockDbgPwdR = crate::BitReader;
#[doc = "Field `rd_lock_dbg_pwd` writer - Read lock for debug password"]
pub type RdLockDbgPwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_0` reader - Read lock for Key Slot 0"]
pub type RdLockKeySlot0R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_0` writer - Read lock for Key Slot 0"]
pub type RdLockKeySlot0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_1` reader - Read lock for Key Slot 1"]
pub type RdLockKeySlot1R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_1` writer - Read lock for Key Slot 1"]
pub type RdLockKeySlot1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_2` reader - Read lock for Key Slot 2"]
pub type RdLockKeySlot2R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_2` writer - Read lock for Key Slot 2"]
pub type RdLockKeySlot2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_3` reader - Read lock for Key Slot 3"]
pub type RdLockKeySlot3R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_3` writer - Read lock for Key Slot 3"]
pub type RdLockKeySlot3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_lock_key_slot_11` reader - Read lock for Key Slot 11"]
pub type RdLockKeySlot11R = crate::BitReader;
#[doc = "Field `rd_lock_key_slot_11` writer - Read lock for Key Slot 11"]
pub type RdLockKeySlot11W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Security Life Cycle (SBLC)"]
    #[inline(always)]
    pub fn ef_sec_lifecycle(&self) -> EfSecLifecycleR {
        EfSecLifecycleR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:13 - Write lock for reserved data bits"]
    #[inline(always)]
    pub fn wr_lock_rsvd_0(&self) -> WrLockRsvd0R {
        WrLockRsvd0R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - Write lock for boot mode/SF AES fields"]
    #[inline(always)]
    pub fn wr_lock_boot_mode(&self) -> WrLockBootModeR {
        WrLockBootModeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write lock for debug password slots 1 & 2"]
    #[inline(always)]
    pub fn wr_lock_dbg_pwd(&self) -> WrLockDbgPwdR {
        WrLockDbgPwdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write lock for Wi-Fi MAC address"]
    #[inline(always)]
    pub fn wr_lock_wifi_mac(&self) -> WrLockWifiMacR {
        WrLockWifiMacR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write lock for Key Slot 0"]
    #[inline(always)]
    pub fn wr_lock_key_slot_0(&self) -> WrLockKeySlot0R {
        WrLockKeySlot0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write lock for Key Slot 1"]
    #[inline(always)]
    pub fn wr_lock_key_slot_1(&self) -> WrLockKeySlot1R {
        WrLockKeySlot1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write lock for Key Slot 2"]
    #[inline(always)]
    pub fn wr_lock_key_slot_2(&self) -> WrLockKeySlot2R {
        WrLockKeySlot2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write lock for Key Slot 3"]
    #[inline(always)]
    pub fn wr_lock_key_slot_3(&self) -> WrLockKeySlot3R {
        WrLockKeySlot3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write lock for Software Usage 0"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_0(&self) -> WrLockSwUsage0R {
        WrLockSwUsage0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Write lock for Software Usage 1"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_1(&self) -> WrLockSwUsage1R {
        WrLockSwUsage1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write lock for Software Usage 2"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_2(&self) -> WrLockSwUsage2R {
        WrLockSwUsage2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Write lock for Software Usage 3"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_3(&self) -> WrLockSwUsage3R {
        WrLockSwUsage3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write lock for Key Slot 11"]
    #[inline(always)]
    pub fn wr_lock_key_slot_11(&self) -> WrLockKeySlot11R {
        WrLockKeySlot11R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Read lock for debug password"]
    #[inline(always)]
    pub fn rd_lock_dbg_pwd(&self) -> RdLockDbgPwdR {
        RdLockDbgPwdR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Read lock for Key Slot 0"]
    #[inline(always)]
    pub fn rd_lock_key_slot_0(&self) -> RdLockKeySlot0R {
        RdLockKeySlot0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Read lock for Key Slot 1"]
    #[inline(always)]
    pub fn rd_lock_key_slot_1(&self) -> RdLockKeySlot1R {
        RdLockKeySlot1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Read lock for Key Slot 2"]
    #[inline(always)]
    pub fn rd_lock_key_slot_2(&self) -> RdLockKeySlot2R {
        RdLockKeySlot2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Read lock for Key Slot 3"]
    #[inline(always)]
    pub fn rd_lock_key_slot_3(&self) -> RdLockKeySlot3R {
        RdLockKeySlot3R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Read lock for Key Slot 11"]
    #[inline(always)]
    pub fn rd_lock_key_slot_11(&self) -> RdLockKeySlot11R {
        RdLockKeySlot11R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Security Life Cycle (SBLC)"]
    #[inline(always)]
    pub fn ef_sec_lifecycle(&mut self) -> EfSecLifecycleW<'_, EfData0LockSpec> {
        EfSecLifecycleW::new(self, 0)
    }
    #[doc = "Bits 4:13 - Write lock for reserved data bits"]
    #[inline(always)]
    pub fn wr_lock_rsvd_0(&mut self) -> WrLockRsvd0W<'_, EfData0LockSpec> {
        WrLockRsvd0W::new(self, 4)
    }
    #[doc = "Bit 14 - Write lock for boot mode/SF AES fields"]
    #[inline(always)]
    pub fn wr_lock_boot_mode(&mut self) -> WrLockBootModeW<'_, EfData0LockSpec> {
        WrLockBootModeW::new(self, 14)
    }
    #[doc = "Bit 15 - Write lock for debug password slots 1 & 2"]
    #[inline(always)]
    pub fn wr_lock_dbg_pwd(&mut self) -> WrLockDbgPwdW<'_, EfData0LockSpec> {
        WrLockDbgPwdW::new(self, 15)
    }
    #[doc = "Bit 16 - Write lock for Wi-Fi MAC address"]
    #[inline(always)]
    pub fn wr_lock_wifi_mac(&mut self) -> WrLockWifiMacW<'_, EfData0LockSpec> {
        WrLockWifiMacW::new(self, 16)
    }
    #[doc = "Bit 17 - Write lock for Key Slot 0"]
    #[inline(always)]
    pub fn wr_lock_key_slot_0(&mut self) -> WrLockKeySlot0W<'_, EfData0LockSpec> {
        WrLockKeySlot0W::new(self, 17)
    }
    #[doc = "Bit 18 - Write lock for Key Slot 1"]
    #[inline(always)]
    pub fn wr_lock_key_slot_1(&mut self) -> WrLockKeySlot1W<'_, EfData0LockSpec> {
        WrLockKeySlot1W::new(self, 18)
    }
    #[doc = "Bit 19 - Write lock for Key Slot 2"]
    #[inline(always)]
    pub fn wr_lock_key_slot_2(&mut self) -> WrLockKeySlot2W<'_, EfData0LockSpec> {
        WrLockKeySlot2W::new(self, 19)
    }
    #[doc = "Bit 20 - Write lock for Key Slot 3"]
    #[inline(always)]
    pub fn wr_lock_key_slot_3(&mut self) -> WrLockKeySlot3W<'_, EfData0LockSpec> {
        WrLockKeySlot3W::new(self, 20)
    }
    #[doc = "Bit 21 - Write lock for Software Usage 0"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_0(&mut self) -> WrLockSwUsage0W<'_, EfData0LockSpec> {
        WrLockSwUsage0W::new(self, 21)
    }
    #[doc = "Bit 22 - Write lock for Software Usage 1"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_1(&mut self) -> WrLockSwUsage1W<'_, EfData0LockSpec> {
        WrLockSwUsage1W::new(self, 22)
    }
    #[doc = "Bit 23 - Write lock for Software Usage 2"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_2(&mut self) -> WrLockSwUsage2W<'_, EfData0LockSpec> {
        WrLockSwUsage2W::new(self, 23)
    }
    #[doc = "Bit 24 - Write lock for Software Usage 3"]
    #[inline(always)]
    pub fn wr_lock_sw_usage_3(&mut self) -> WrLockSwUsage3W<'_, EfData0LockSpec> {
        WrLockSwUsage3W::new(self, 24)
    }
    #[doc = "Bit 25 - Write lock for Key Slot 11"]
    #[inline(always)]
    pub fn wr_lock_key_slot_11(&mut self) -> WrLockKeySlot11W<'_, EfData0LockSpec> {
        WrLockKeySlot11W::new(self, 25)
    }
    #[doc = "Bit 26 - Read lock for debug password"]
    #[inline(always)]
    pub fn rd_lock_dbg_pwd(&mut self) -> RdLockDbgPwdW<'_, EfData0LockSpec> {
        RdLockDbgPwdW::new(self, 26)
    }
    #[doc = "Bit 27 - Read lock for Key Slot 0"]
    #[inline(always)]
    pub fn rd_lock_key_slot_0(&mut self) -> RdLockKeySlot0W<'_, EfData0LockSpec> {
        RdLockKeySlot0W::new(self, 27)
    }
    #[doc = "Bit 28 - Read lock for Key Slot 1"]
    #[inline(always)]
    pub fn rd_lock_key_slot_1(&mut self) -> RdLockKeySlot1W<'_, EfData0LockSpec> {
        RdLockKeySlot1W::new(self, 28)
    }
    #[doc = "Bit 29 - Read lock for Key Slot 2"]
    #[inline(always)]
    pub fn rd_lock_key_slot_2(&mut self) -> RdLockKeySlot2W<'_, EfData0LockSpec> {
        RdLockKeySlot2W::new(self, 29)
    }
    #[doc = "Bit 30 - Read lock for Key Slot 3"]
    #[inline(always)]
    pub fn rd_lock_key_slot_3(&mut self) -> RdLockKeySlot3W<'_, EfData0LockSpec> {
        RdLockKeySlot3W::new(self, 30)
    }
    #[doc = "Bit 31 - Read lock for Key Slot 11"]
    #[inline(always)]
    pub fn rd_lock_key_slot_11(&mut self) -> RdLockKeySlot11W<'_, EfData0LockSpec> {
        RdLockKeySlot11W::new(self, 31)
    }
}
#[doc = "eFuse Data 0 Write/Read Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_data_0_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_data_0_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfData0LockSpec;
impl crate::RegisterSpec for EfData0LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_data_0_lock::R`](R) reader structure"]
impl crate::Readable for EfData0LockSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_data_0_lock::W`](W) writer structure"]
impl crate::Writable for EfData0LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_DATA_0_LOCK to value 0"]
impl crate::Resettable for EfData0LockSpec {}
