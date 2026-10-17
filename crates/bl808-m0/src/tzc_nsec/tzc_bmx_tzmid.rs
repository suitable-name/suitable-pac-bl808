#[doc = "Register `TZC_BMX_TZMID` reader"]
pub type R = crate::R<TzcBmxTzmidSpec>;
#[doc = "Field `tzc_pico_tzmid` reader - PICO core TZID status (0:Secure, 1:Non-Secure)"]
pub type TzcPicoTzmidR = crate::BitReader;
#[doc = "Field `tzc_mm_tzmid` reader - MM Subsystem TZID status"]
pub type TzcMmTzmidR = crate::BitReader;
#[doc = "Field `tzc_usb_tzmid` reader - USB TZID status"]
pub type TzcUsbTzmidR = crate::BitReader;
#[doc = "Field `tzc_wifi_tzmid` reader - WiFi TZID status"]
pub type TzcWifiTzmidR = crate::BitReader;
#[doc = "Field `tzc_cci_tzmid` reader - CCI TZID status"]
pub type TzcCciTzmidR = crate::BitReader;
#[doc = "Field `tzc_sdhm_tzmid` reader - SD Host Master TZID status"]
pub type TzcSdhmTzmidR = crate::BitReader;
#[doc = "Field `tzc_emaca_tzmid` reader - EMAC A TZID status"]
pub type TzcEmacaTzmidR = crate::BitReader;
#[doc = "Field `tzc_cpu_tzmid` reader - CPU (M0/E907) TZID status"]
pub type TzcCpuTzmidR = crate::BitReader;
#[doc = "Field `tzc_dma_tzmid` reader - DMA0/DMA1 TZID status"]
pub type TzcDmaTzmidR = crate::BitReader;
#[doc = "Field `tzc_dma2_tzmid` reader - DMA2 TZID status"]
pub type TzcDma2TzmidR = crate::BitReader;
#[doc = "Field `tzc_lz4_tzmid` reader - LZ4 TZID status"]
pub type TzcLz4TzmidR = crate::BitReader;
#[doc = "Field `tzc_pico_tzmid_sel` reader - PICO Core TZID Select"]
pub type TzcPicoTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_mm_tzmid_sel` reader - MM Subsystem TZID Select"]
pub type TzcMmTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_usb_tzmid_sel` reader - USB TZID Select"]
pub type TzcUsbTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_wifi_tzmid_sel` reader - WIFI TZID Select"]
pub type TzcWifiTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_cci_tzmid_sel` reader - CCI TZID Select"]
pub type TzcCciTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_sdhm_tzmid_sel` reader - SD Host Master TZID Select"]
pub type TzcSdhmTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_emaca_tzmid_sel` reader - EMAC A TZID Select"]
pub type TzcEmacaTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_cpu_tzmid_sel` reader - CPU (M0/E907) TZID Select"]
pub type TzcCpuTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_dma_tzmid_sel` reader - DMA0/DMA1 TZID Select"]
pub type TzcDmaTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_dma2_tzmid_sel` reader - DMA2 TZID Select"]
pub type TzcDma2TzmidSelR = crate::BitReader;
#[doc = "Field `tzc_lz4_tzmid_sel` reader - LZ4 TZID Select"]
pub type TzcLz4TzmidSelR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PICO core TZID status (0:Secure, 1:Non-Secure)"]
    #[inline(always)]
    pub fn tzc_pico_tzmid(&self) -> TzcPicoTzmidR {
        TzcPicoTzmidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MM Subsystem TZID status"]
    #[inline(always)]
    pub fn tzc_mm_tzmid(&self) -> TzcMmTzmidR {
        TzcMmTzmidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB TZID status"]
    #[inline(always)]
    pub fn tzc_usb_tzmid(&self) -> TzcUsbTzmidR {
        TzcUsbTzmidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WiFi TZID status"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid(&self) -> TzcWifiTzmidR {
        TzcWifiTzmidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCI TZID status"]
    #[inline(always)]
    pub fn tzc_cci_tzmid(&self) -> TzcCciTzmidR {
        TzcCciTzmidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SD Host Master TZID status"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid(&self) -> TzcSdhmTzmidR {
        TzcSdhmTzmidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EMAC A TZID status"]
    #[inline(always)]
    pub fn tzc_emaca_tzmid(&self) -> TzcEmacaTzmidR {
        TzcEmacaTzmidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU (M0/E907) TZID status"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid(&self) -> TzcCpuTzmidR {
        TzcCpuTzmidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA0/DMA1 TZID status"]
    #[inline(always)]
    pub fn tzc_dma_tzmid(&self) -> TzcDmaTzmidR {
        TzcDmaTzmidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA2 TZID status"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid(&self) -> TzcDma2TzmidR {
        TzcDma2TzmidR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LZ4 TZID status"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid(&self) -> TzcLz4TzmidR {
        TzcLz4TzmidR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - PICO Core TZID Select"]
    #[inline(always)]
    pub fn tzc_pico_tzmid_sel(&self) -> TzcPicoTzmidSelR {
        TzcPicoTzmidSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MM Subsystem TZID Select"]
    #[inline(always)]
    pub fn tzc_mm_tzmid_sel(&self) -> TzcMmTzmidSelR {
        TzcMmTzmidSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USB TZID Select"]
    #[inline(always)]
    pub fn tzc_usb_tzmid_sel(&self) -> TzcUsbTzmidSelR {
        TzcUsbTzmidSelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - WIFI TZID Select"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid_sel(&self) -> TzcWifiTzmidSelR {
        TzcWifiTzmidSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CCI TZID Select"]
    #[inline(always)]
    pub fn tzc_cci_tzmid_sel(&self) -> TzcCciTzmidSelR {
        TzcCciTzmidSelR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SD Host Master TZID Select"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid_sel(&self) -> TzcSdhmTzmidSelR {
        TzcSdhmTzmidSelR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EMAC A TZID Select"]
    #[inline(always)]
    pub fn tzc_emaca_tzmid_sel(&self) -> TzcEmacaTzmidSelR {
        TzcEmacaTzmidSelR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU (M0/E907) TZID Select"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid_sel(&self) -> TzcCpuTzmidSelR {
        TzcCpuTzmidSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA0/DMA1 TZID Select"]
    #[inline(always)]
    pub fn tzc_dma_tzmid_sel(&self) -> TzcDmaTzmidSelR {
        TzcDmaTzmidSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA2 TZID Select"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid_sel(&self) -> TzcDma2TzmidSelR {
        TzcDma2TzmidSelR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LZ4 TZID Select"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid_sel(&self) -> TzcLz4TzmidSelR {
        TzcLz4TzmidSelR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Bus Master ID Security State (TZMID) Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_tzmid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcBmxTzmidSpec;
impl crate::RegisterSpec for TzcBmxTzmidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_tzmid::R`](R) reader structure"]
impl crate::Readable for TzcBmxTzmidSpec {}
#[doc = "`reset()` method sets TZC_BMX_TZMID to value 0"]
impl crate::Resettable for TzcBmxTzmidSpec {}
