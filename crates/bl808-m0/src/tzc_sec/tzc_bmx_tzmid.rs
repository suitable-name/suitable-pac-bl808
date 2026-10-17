#[doc = "Register `TZC_BMX_TZMID` reader"]
pub type R = crate::R<TzcBmxTzmidSpec>;
#[doc = "Register `TZC_BMX_TZMID` writer"]
pub type W = crate::W<TzcBmxTzmidSpec>;
#[doc = "Field `tzc_pico_tzmid` reader - PICO core TZID value"]
pub type TzcPicoTzmidR = crate::BitReader;
#[doc = "Field `tzc_pico_tzmid` writer - PICO core TZID value"]
pub type TzcPicoTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_mm_tzmid` reader - Multimedia core TZID value"]
pub type TzcMmTzmidR = crate::BitReader;
#[doc = "Field `tzc_mm_tzmid` writer - Multimedia core TZID value"]
pub type TzcMmTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_usb_tzmid` reader - USB controller TZID value"]
pub type TzcUsbTzmidR = crate::BitReader;
#[doc = "Field `tzc_usb_tzmid` writer - USB controller TZID value"]
pub type TzcUsbTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wifi_tzmid` reader - WiFi controller TZID value"]
pub type TzcWifiTzmidR = crate::BitReader;
#[doc = "Field `tzc_wifi_tzmid` writer - WiFi controller TZID value"]
pub type TzcWifiTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cci_tzmid` reader - CCI controller TZID value"]
pub type TzcCciTzmidR = crate::BitReader;
#[doc = "Field `tzc_cci_tzmid` writer - CCI controller TZID value"]
pub type TzcCciTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sdhm_tzmid` reader - SD Host Master TZID value"]
pub type TzcSdhmTzmidR = crate::BitReader;
#[doc = "Field `tzc_sdhm_tzmid` writer - SD Host Master TZID value"]
pub type TzcSdhmTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_emacA_tzmid` reader - EMAC A TZID value"]
pub type TzcEmacATzmidR = crate::BitReader;
#[doc = "Field `tzc_emacA_tzmid` writer - EMAC A TZID value"]
pub type TzcEmacATzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cpu_tzmid` reader - M0 CPU TZID value"]
pub type TzcCpuTzmidR = crate::BitReader;
#[doc = "Field `tzc_cpu_tzmid` writer - M0 CPU TZID value"]
pub type TzcCpuTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_dma_tzmid` reader - DMA0/DMA1 TZID value"]
pub type TzcDmaTzmidR = crate::BitReader;
#[doc = "Field `tzc_dma_tzmid` writer - DMA0/DMA1 TZID value"]
pub type TzcDmaTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_dma2_tzmid` reader - DMA2 TZID value"]
pub type TzcDma2TzmidR = crate::BitReader;
#[doc = "Field `tzc_dma2_tzmid` writer - DMA2 TZID value"]
pub type TzcDma2TzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_lz4_tzmid` reader - LZ4 TZID value"]
pub type TzcLz4TzmidR = crate::BitReader;
#[doc = "Field `tzc_lz4_tzmid` writer - LZ4 TZID value"]
pub type TzcLz4TzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_pico_tzmid_sel` reader - PICO core TZID select enable"]
pub type TzcPicoTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_pico_tzmid_sel` writer - PICO core TZID select enable"]
pub type TzcPicoTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_mm_tzmid_sel` reader - Multimedia core TZID select enable"]
pub type TzcMmTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_mm_tzmid_sel` writer - Multimedia core TZID select enable"]
pub type TzcMmTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_usb_tzmid_sel` reader - USB controller TZID select enable"]
pub type TzcUsbTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_usb_tzmid_sel` writer - USB controller TZID select enable"]
pub type TzcUsbTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_wifi_tzmid_sel` reader - WiFi controller TZID select enable"]
pub type TzcWifiTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_wifi_tzmid_sel` writer - WiFi controller TZID select enable"]
pub type TzcWifiTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cci_tzmid_sel` reader - CCI controller TZID select enable"]
pub type TzcCciTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_cci_tzmid_sel` writer - CCI controller TZID select enable"]
pub type TzcCciTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sdhm_tzmid_sel` reader - SD Host Master TZID select enable"]
pub type TzcSdhmTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_sdhm_tzmid_sel` writer - SD Host Master TZID select enable"]
pub type TzcSdhmTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_emacA_tzmid_sel` reader - EMAC A TZID select enable"]
pub type TzcEmacATzmidSelR = crate::BitReader;
#[doc = "Field `tzc_emacA_tzmid_sel` writer - EMAC A TZID select enable"]
pub type TzcEmacATzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_cpu_tzmid_sel` reader - M0 CPU TZID select enable"]
pub type TzcCpuTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_cpu_tzmid_sel` writer - M0 CPU TZID select enable"]
pub type TzcCpuTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_dma_tzmid_sel` reader - DMA0/DMA1 TZID select enable"]
pub type TzcDmaTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_dma_tzmid_sel` writer - DMA0/DMA1 TZID select enable"]
pub type TzcDmaTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_dma2_tzmid_sel` reader - DMA2 TZID select enable"]
pub type TzcDma2TzmidSelR = crate::BitReader;
#[doc = "Field `tzc_dma2_tzmid_sel` writer - DMA2 TZID select enable"]
pub type TzcDma2TzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_lz4_tzmid_sel` reader - LZ4 TZID select enable"]
pub type TzcLz4TzmidSelR = crate::BitReader;
#[doc = "Field `tzc_lz4_tzmid_sel` writer - LZ4 TZID select enable"]
pub type TzcLz4TzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PICO core TZID value"]
    #[inline(always)]
    pub fn tzc_pico_tzmid(&self) -> TzcPicoTzmidR {
        TzcPicoTzmidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multimedia core TZID value"]
    #[inline(always)]
    pub fn tzc_mm_tzmid(&self) -> TzcMmTzmidR {
        TzcMmTzmidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB controller TZID value"]
    #[inline(always)]
    pub fn tzc_usb_tzmid(&self) -> TzcUsbTzmidR {
        TzcUsbTzmidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WiFi controller TZID value"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid(&self) -> TzcWifiTzmidR {
        TzcWifiTzmidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCI controller TZID value"]
    #[inline(always)]
    pub fn tzc_cci_tzmid(&self) -> TzcCciTzmidR {
        TzcCciTzmidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SD Host Master TZID value"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid(&self) -> TzcSdhmTzmidR {
        TzcSdhmTzmidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EMAC A TZID value"]
    #[inline(always)]
    pub fn tzc_emac_a_tzmid(&self) -> TzcEmacATzmidR {
        TzcEmacATzmidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - M0 CPU TZID value"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid(&self) -> TzcCpuTzmidR {
        TzcCpuTzmidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA0/DMA1 TZID value"]
    #[inline(always)]
    pub fn tzc_dma_tzmid(&self) -> TzcDmaTzmidR {
        TzcDmaTzmidR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA2 TZID value"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid(&self) -> TzcDma2TzmidR {
        TzcDma2TzmidR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LZ4 TZID value"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid(&self) -> TzcLz4TzmidR {
        TzcLz4TzmidR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - PICO core TZID select enable"]
    #[inline(always)]
    pub fn tzc_pico_tzmid_sel(&self) -> TzcPicoTzmidSelR {
        TzcPicoTzmidSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Multimedia core TZID select enable"]
    #[inline(always)]
    pub fn tzc_mm_tzmid_sel(&self) -> TzcMmTzmidSelR {
        TzcMmTzmidSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USB controller TZID select enable"]
    #[inline(always)]
    pub fn tzc_usb_tzmid_sel(&self) -> TzcUsbTzmidSelR {
        TzcUsbTzmidSelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - WiFi controller TZID select enable"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid_sel(&self) -> TzcWifiTzmidSelR {
        TzcWifiTzmidSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CCI controller TZID select enable"]
    #[inline(always)]
    pub fn tzc_cci_tzmid_sel(&self) -> TzcCciTzmidSelR {
        TzcCciTzmidSelR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SD Host Master TZID select enable"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid_sel(&self) -> TzcSdhmTzmidSelR {
        TzcSdhmTzmidSelR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EMAC A TZID select enable"]
    #[inline(always)]
    pub fn tzc_emac_a_tzmid_sel(&self) -> TzcEmacATzmidSelR {
        TzcEmacATzmidSelR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - M0 CPU TZID select enable"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid_sel(&self) -> TzcCpuTzmidSelR {
        TzcCpuTzmidSelR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA0/DMA1 TZID select enable"]
    #[inline(always)]
    pub fn tzc_dma_tzmid_sel(&self) -> TzcDmaTzmidSelR {
        TzcDmaTzmidSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA2 TZID select enable"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid_sel(&self) -> TzcDma2TzmidSelR {
        TzcDma2TzmidSelR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LZ4 TZID select enable"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid_sel(&self) -> TzcLz4TzmidSelR {
        TzcLz4TzmidSelR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PICO core TZID value"]
    #[inline(always)]
    pub fn tzc_pico_tzmid(&mut self) -> TzcPicoTzmidW<'_, TzcBmxTzmidSpec> {
        TzcPicoTzmidW::new(self, 0)
    }
    #[doc = "Bit 1 - Multimedia core TZID value"]
    #[inline(always)]
    pub fn tzc_mm_tzmid(&mut self) -> TzcMmTzmidW<'_, TzcBmxTzmidSpec> {
        TzcMmTzmidW::new(self, 1)
    }
    #[doc = "Bit 2 - USB controller TZID value"]
    #[inline(always)]
    pub fn tzc_usb_tzmid(&mut self) -> TzcUsbTzmidW<'_, TzcBmxTzmidSpec> {
        TzcUsbTzmidW::new(self, 2)
    }
    #[doc = "Bit 3 - WiFi controller TZID value"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid(&mut self) -> TzcWifiTzmidW<'_, TzcBmxTzmidSpec> {
        TzcWifiTzmidW::new(self, 3)
    }
    #[doc = "Bit 4 - CCI controller TZID value"]
    #[inline(always)]
    pub fn tzc_cci_tzmid(&mut self) -> TzcCciTzmidW<'_, TzcBmxTzmidSpec> {
        TzcCciTzmidW::new(self, 4)
    }
    #[doc = "Bit 5 - SD Host Master TZID value"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid(&mut self) -> TzcSdhmTzmidW<'_, TzcBmxTzmidSpec> {
        TzcSdhmTzmidW::new(self, 5)
    }
    #[doc = "Bit 6 - EMAC A TZID value"]
    #[inline(always)]
    pub fn tzc_emac_a_tzmid(&mut self) -> TzcEmacATzmidW<'_, TzcBmxTzmidSpec> {
        TzcEmacATzmidW::new(self, 6)
    }
    #[doc = "Bit 7 - M0 CPU TZID value"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid(&mut self) -> TzcCpuTzmidW<'_, TzcBmxTzmidSpec> {
        TzcCpuTzmidW::new(self, 7)
    }
    #[doc = "Bit 8 - DMA0/DMA1 TZID value"]
    #[inline(always)]
    pub fn tzc_dma_tzmid(&mut self) -> TzcDmaTzmidW<'_, TzcBmxTzmidSpec> {
        TzcDmaTzmidW::new(self, 8)
    }
    #[doc = "Bit 9 - DMA2 TZID value"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid(&mut self) -> TzcDma2TzmidW<'_, TzcBmxTzmidSpec> {
        TzcDma2TzmidW::new(self, 9)
    }
    #[doc = "Bit 10 - LZ4 TZID value"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid(&mut self) -> TzcLz4TzmidW<'_, TzcBmxTzmidSpec> {
        TzcLz4TzmidW::new(self, 10)
    }
    #[doc = "Bit 16 - PICO core TZID select enable"]
    #[inline(always)]
    pub fn tzc_pico_tzmid_sel(&mut self) -> TzcPicoTzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcPicoTzmidSelW::new(self, 16)
    }
    #[doc = "Bit 17 - Multimedia core TZID select enable"]
    #[inline(always)]
    pub fn tzc_mm_tzmid_sel(&mut self) -> TzcMmTzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcMmTzmidSelW::new(self, 17)
    }
    #[doc = "Bit 18 - USB controller TZID select enable"]
    #[inline(always)]
    pub fn tzc_usb_tzmid_sel(&mut self) -> TzcUsbTzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcUsbTzmidSelW::new(self, 18)
    }
    #[doc = "Bit 19 - WiFi controller TZID select enable"]
    #[inline(always)]
    pub fn tzc_wifi_tzmid_sel(&mut self) -> TzcWifiTzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcWifiTzmidSelW::new(self, 19)
    }
    #[doc = "Bit 20 - CCI controller TZID select enable"]
    #[inline(always)]
    pub fn tzc_cci_tzmid_sel(&mut self) -> TzcCciTzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcCciTzmidSelW::new(self, 20)
    }
    #[doc = "Bit 21 - SD Host Master TZID select enable"]
    #[inline(always)]
    pub fn tzc_sdhm_tzmid_sel(&mut self) -> TzcSdhmTzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcSdhmTzmidSelW::new(self, 21)
    }
    #[doc = "Bit 22 - EMAC A TZID select enable"]
    #[inline(always)]
    pub fn tzc_emac_a_tzmid_sel(&mut self) -> TzcEmacATzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcEmacATzmidSelW::new(self, 22)
    }
    #[doc = "Bit 23 - M0 CPU TZID select enable"]
    #[inline(always)]
    pub fn tzc_cpu_tzmid_sel(&mut self) -> TzcCpuTzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcCpuTzmidSelW::new(self, 23)
    }
    #[doc = "Bit 24 - DMA0/DMA1 TZID select enable"]
    #[inline(always)]
    pub fn tzc_dma_tzmid_sel(&mut self) -> TzcDmaTzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcDmaTzmidSelW::new(self, 24)
    }
    #[doc = "Bit 25 - DMA2 TZID select enable"]
    #[inline(always)]
    pub fn tzc_dma2_tzmid_sel(&mut self) -> TzcDma2TzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcDma2TzmidSelW::new(self, 25)
    }
    #[doc = "Bit 26 - LZ4 TZID select enable"]
    #[inline(always)]
    pub fn tzc_lz4_tzmid_sel(&mut self) -> TzcLz4TzmidSelW<'_, TzcBmxTzmidSpec> {
        TzcLz4TzmidSelW::new(self, 26)
    }
}
#[doc = "Bus Matrix Master TZID Assignment\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_tzmid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_bmx_tzmid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcBmxTzmidSpec;
impl crate::RegisterSpec for TzcBmxTzmidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_tzmid::R`](R) reader structure"]
impl crate::Readable for TzcBmxTzmidSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_bmx_tzmid::W`](W) writer structure"]
impl crate::Writable for TzcBmxTzmidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_BMX_TZMID to value 0x07ff_0000"]
impl crate::Resettable for TzcBmxTzmidSpec {
    const RESET_VALUE: u32 = 0x07ff_0000;
}
