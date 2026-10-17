#[doc = "Register `TZC_BMX_TZMID_LOCK` reader"]
pub type R = crate::R<TzcBmxTzmidLockSpec>;
#[doc = "Field `tzc_pico_tzmid_lock` reader - "]
pub type TzcPicoTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_tzmid_lock` reader - "]
pub type TzcMmTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_usb_tzmid_lock` reader - "]
pub type TzcUsbTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_wifi_tzmid_lock` reader - "]
pub type TzcWifiTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_cci_tzmid_lock` reader - "]
pub type TzcCciTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_sdhm_tzmid_lock` reader - "]
pub type TzcSdhmTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_emaca_tzmid_lock` reader - "]
pub type TzcEmacaTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_cpu_tzmid_lock` reader - "]
pub type TzcCpuTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_dma_tzmid_lock` reader - "]
pub type TzcDmaTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_dma2_tzmid_lock` reader - "]
pub type TzcDma2TzmidLockR = crate::BitReader;
#[doc = "Field `tzc_lz4_tzmid_lock` reader - "]
pub type TzcLz4TzmidLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_pico_tzmid_lock(&self) -> TzcPicoTzmidLockR {
        TzcPicoTzmidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_mm_tzmid_lock(&self) -> TzcMmTzmidLockR {
        TzcMmTzmidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_usb_tzmid_lock(&self) -> TzcUsbTzmidLockR {
        TzcUsbTzmidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid_lock(&self) -> TzcWifiTzmidLockR {
        TzcWifiTzmidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_cci_tzmid_lock(&self) -> TzcCciTzmidLockR {
        TzcCciTzmidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid_lock(&self) -> TzcSdhmTzmidLockR {
        TzcSdhmTzmidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_emaca_tzmid_lock(&self) -> TzcEmacaTzmidLockR {
        TzcEmacaTzmidLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid_lock(&self) -> TzcCpuTzmidLockR {
        TzcCpuTzmidLockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_dma_tzmid_lock(&self) -> TzcDmaTzmidLockR {
        TzcDmaTzmidLockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid_lock(&self) -> TzcDma2TzmidLockR {
        TzcDma2TzmidLockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid_lock(&self) -> TzcLz4TzmidLockR {
        TzcLz4TzmidLockR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Bus Master ID Security State Lock (TZMID_LOCK)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_tzmid_lock::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcBmxTzmidLockSpec;
impl crate::RegisterSpec for TzcBmxTzmidLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_tzmid_lock::R`](R) reader structure"]
impl crate::Readable for TzcBmxTzmidLockSpec {}
#[doc = "`reset()` method sets TZC_BMX_TZMID_LOCK to value 0"]
impl crate::Resettable for TzcBmxTzmidLockSpec {}
