#[doc = "Register `TZC_BMX_TZMID_LOCK` reader"]
pub type R = crate::R<TzcBmxTzmidLockSpec>;
#[doc = "Register `TZC_BMX_TZMID_LOCK` writer"]
pub type W = crate::W<TzcBmxTzmidLockSpec>;
#[doc = "Field `tzc_pico_tzmid_lock` reader - PICO core TZID lock"]
pub type TzcPicoTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_pico_tzmid_lock` writer - PICO core TZID lock"]
pub type TzcPicoTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_mm_tzmid_lock` reader - Multimedia core TZID lock"]
pub type TzcMmTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_mm_tzmid_lock` writer - Multimedia core TZID lock"]
pub type TzcMmTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_usb_tzmid_lock` reader - USB controller TZID lock"]
pub type TzcUsbTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_usb_tzmid_lock` writer - USB controller TZID lock"]
pub type TzcUsbTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wifi_tzmid_lock` reader - WiFi controller TZID lock"]
pub type TzcWifiTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_wifi_tzmid_lock` writer - WiFi controller TZID lock"]
pub type TzcWifiTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cci_tzmid_lock` reader - CCI controller TZID lock"]
pub type TzcCciTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_cci_tzmid_lock` writer - CCI controller TZID lock"]
pub type TzcCciTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sdhm_tzmid_lock` reader - SD Host Master TZID lock"]
pub type TzcSdhmTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_sdhm_tzmid_lock` writer - SD Host Master TZID lock"]
pub type TzcSdhmTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_emacA_tzmid_lock` reader - EMAC A TZID lock"]
pub type TzcEmacATzmidLockR = crate::BitReader;
#[doc = "Field `tzc_emacA_tzmid_lock` writer - EMAC A TZID lock"]
pub type TzcEmacATzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cpu_tzmid_lock` reader - M0 CPU TZID lock"]
pub type TzcCpuTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_cpu_tzmid_lock` writer - M0 CPU TZID lock"]
pub type TzcCpuTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_dma_tzmid_lock` reader - DMA0/DMA1 TZID lock"]
pub type TzcDmaTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_dma_tzmid_lock` writer - DMA0/DMA1 TZID lock"]
pub type TzcDmaTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_dma2_tzmid_lock` reader - DMA2 TZID lock"]
pub type TzcDma2TzmidLockR = crate::BitReader;
#[doc = "Field `tzc_dma2_tzmid_lock` writer - DMA2 TZID lock"]
pub type TzcDma2TzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_lz4_tzmid_lock` reader - LZ4 TZID lock"]
pub type TzcLz4TzmidLockR = crate::BitReader;
#[doc = "Field `tzc_lz4_tzmid_lock` writer - LZ4 TZID lock"]
pub type TzcLz4TzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PICO core TZID lock"]
    #[inline(always)]
    pub fn tzc_pico_tzmid_lock(&self) -> TzcPicoTzmidLockR {
        TzcPicoTzmidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multimedia core TZID lock"]
    #[inline(always)]
    pub fn tzc_mm_tzmid_lock(&self) -> TzcMmTzmidLockR {
        TzcMmTzmidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB controller TZID lock"]
    #[inline(always)]
    pub fn tzc_usb_tzmid_lock(&self) -> TzcUsbTzmidLockR {
        TzcUsbTzmidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WiFi controller TZID lock"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid_lock(&self) -> TzcWifiTzmidLockR {
        TzcWifiTzmidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCI controller TZID lock"]
    #[inline(always)]
    pub fn tzc_cci_tzmid_lock(&self) -> TzcCciTzmidLockR {
        TzcCciTzmidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SD Host Master TZID lock"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid_lock(&self) -> TzcSdhmTzmidLockR {
        TzcSdhmTzmidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EMAC A TZID lock"]
    #[inline(always)]
    pub fn tzc_emac_a_tzmid_lock(&self) -> TzcEmacATzmidLockR {
        TzcEmacATzmidLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - M0 CPU TZID lock"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid_lock(&self) -> TzcCpuTzmidLockR {
        TzcCpuTzmidLockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA0/DMA1 TZID lock"]
    #[inline(always)]
    pub fn tzc_dma_tzmid_lock(&self) -> TzcDmaTzmidLockR {
        TzcDmaTzmidLockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA2 TZID lock"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid_lock(&self) -> TzcDma2TzmidLockR {
        TzcDma2TzmidLockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LZ4 TZID lock"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid_lock(&self) -> TzcLz4TzmidLockR {
        TzcLz4TzmidLockR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PICO core TZID lock"]
    #[inline(always)]
    pub fn tzc_pico_tzmid_lock(&mut self) -> TzcPicoTzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcPicoTzmidLockW::new(self, 0)
    }
    #[doc = "Bit 1 - Multimedia core TZID lock"]
    #[inline(always)]
    pub fn tzc_mm_tzmid_lock(&mut self) -> TzcMmTzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcMmTzmidLockW::new(self, 1)
    }
    #[doc = "Bit 2 - USB controller TZID lock"]
    #[inline(always)]
    pub fn tzc_usb_tzmid_lock(&mut self) -> TzcUsbTzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcUsbTzmidLockW::new(self, 2)
    }
    #[doc = "Bit 3 - WiFi controller TZID lock"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid_lock(&mut self) -> TzcWifiTzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcWifiTzmidLockW::new(self, 3)
    }
    #[doc = "Bit 4 - CCI controller TZID lock"]
    #[inline(always)]
    pub fn tzc_cci_tzmid_lock(&mut self) -> TzcCciTzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcCciTzmidLockW::new(self, 4)
    }
    #[doc = "Bit 5 - SD Host Master TZID lock"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid_lock(&mut self) -> TzcSdhmTzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcSdhmTzmidLockW::new(self, 5)
    }
    #[doc = "Bit 6 - EMAC A TZID lock"]
    #[inline(always)]
    pub fn tzc_emac_a_tzmid_lock(&mut self) -> TzcEmacATzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcEmacATzmidLockW::new(self, 6)
    }
    #[doc = "Bit 7 - M0 CPU TZID lock"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid_lock(&mut self) -> TzcCpuTzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcCpuTzmidLockW::new(self, 7)
    }
    #[doc = "Bit 8 - DMA0/DMA1 TZID lock"]
    #[inline(always)]
    pub fn tzc_dma_tzmid_lock(&mut self) -> TzcDmaTzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcDmaTzmidLockW::new(self, 8)
    }
    #[doc = "Bit 9 - DMA2 TZID lock"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid_lock(&mut self) -> TzcDma2TzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcDma2TzmidLockW::new(self, 9)
    }
    #[doc = "Bit 10 - LZ4 TZID lock"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid_lock(&mut self) -> TzcLz4TzmidLockW<'_, TzcBmxTzmidLockSpec> {
        TzcLz4TzmidLockW::new(self, 10)
    }
}
#[doc = "Bus Matrix Master TZID Lock (Write-Once)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_tzmid_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_bmx_tzmid_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcBmxTzmidLockSpec;
impl crate::RegisterSpec for TzcBmxTzmidLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_tzmid_lock::R`](R) reader structure"]
impl crate::Readable for TzcBmxTzmidLockSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_bmx_tzmid_lock::W`](W) writer structure"]
impl crate::Writable for TzcBmxTzmidLockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_BMX_TZMID_LOCK to value 0"]
impl crate::Resettable for TzcBmxTzmidLockSpec {}
