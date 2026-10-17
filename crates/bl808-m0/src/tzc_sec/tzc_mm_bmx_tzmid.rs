#[doc = "Register `TZC_MM_BMX_TZMID` reader"]
pub type R = crate::R<TzcMmBmxTzmidSpec>;
#[doc = "Register `TZC_MM_BMX_TZMID` writer"]
pub type W = crate::W<TzcMmBmxTzmidSpec>;
#[doc = "Field `tzc_mmcpu_tzmid` reader - MM CPU (D0) TZID value"]
pub type TzcMmcpuTzmidR = crate::BitReader;
#[doc = "Field `tzc_mmcpu_tzmid` writer - MM CPU (D0) TZID value"]
pub type TzcMmcpuTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_blai_tzmid` reader - BLAI (NPU) TZID value"]
pub type TzcBlaiTzmidR = crate::BitReader;
#[doc = "Field `tzc_blai_tzmid` writer - BLAI (NPU) TZID value"]
pub type TzcBlaiTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_codec_tzmid` reader - CODEC TZID value"]
pub type TzcCodecTzmidR = crate::BitReader;
#[doc = "Field `tzc_codec_tzmid` writer - CODEC TZID value"]
pub type TzcCodecTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_subsys_tzmid` reader - Multimedia Subsystem TZID value"]
pub type TzcSubsysTzmidR = crate::BitReader;
#[doc = "Field `tzc_subsys_tzmid` writer - Multimedia Subsystem TZID value"]
pub type TzcSubsysTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_x2ddma_tzmid` reader - 2D DMA TZID value"]
pub type TzcX2ddmaTzmidR = crate::BitReader;
#[doc = "Field `tzc_x2ddma_tzmid` writer - 2D DMA TZID value"]
pub type TzcX2ddmaTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_xdma_tzmid` reader - DMA2 TZID value"]
pub type TzcXdmaTzmidR = crate::BitReader;
#[doc = "Field `tzc_xdma_tzmid` writer - DMA2 TZID value"]
pub type TzcXdmaTzmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_mmcpu_tzmid_sel` reader - MM CPU (D0) TZID select enable"]
pub type TzcMmcpuTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_mmcpu_tzmid_sel` writer - MM CPU (D0) TZID select enable"]
pub type TzcMmcpuTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_blai_tzmid_sel` reader - BLAI (NPU) TZID select enable"]
pub type TzcBlaiTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_blai_tzmid_sel` writer - BLAI (NPU) TZID select enable"]
pub type TzcBlaiTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_codec_tzmid_sel` reader - CODEC TZID select enable"]
pub type TzcCodecTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_codec_tzmid_sel` writer - CODEC TZID select enable"]
pub type TzcCodecTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_subsys_tzmid_sel` reader - Multimedia Subsystem TZID select enable"]
pub type TzcSubsysTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_subsys_tzmid_sel` writer - Multimedia Subsystem TZID select enable"]
pub type TzcSubsysTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_x2ddma_tzmid_sel` reader - 2D DMA TZID select enable"]
pub type TzcX2ddmaTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_x2ddma_tzmid_sel` writer - 2D DMA TZID select enable"]
pub type TzcX2ddmaTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_xdma_tzmid_sel` reader - DMA2 TZID select enable"]
pub type TzcXdmaTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_xdma_tzmid_sel` writer - DMA2 TZID select enable"]
pub type TzcXdmaTzmidSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MM CPU (D0) TZID value"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid(&self) -> TzcMmcpuTzmidR {
        TzcMmcpuTzmidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BLAI (NPU) TZID value"]
    #[inline(always)]
    pub fn tzc_blai_tzmid(&self) -> TzcBlaiTzmidR {
        TzcBlaiTzmidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CODEC TZID value"]
    #[inline(always)]
    pub fn tzc_codec_tzmid(&self) -> TzcCodecTzmidR {
        TzcCodecTzmidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multimedia Subsystem TZID value"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid(&self) -> TzcSubsysTzmidR {
        TzcSubsysTzmidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 2D DMA TZID value"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid(&self) -> TzcX2ddmaTzmidR {
        TzcX2ddmaTzmidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA2 TZID value"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid(&self) -> TzcXdmaTzmidR {
        TzcXdmaTzmidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - MM CPU (D0) TZID select enable"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid_sel(&self) -> TzcMmcpuTzmidSelR {
        TzcMmcpuTzmidSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BLAI (NPU) TZID select enable"]
    #[inline(always)]
    pub fn tzc_blai_tzmid_sel(&self) -> TzcBlaiTzmidSelR {
        TzcBlaiTzmidSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CODEC TZID select enable"]
    #[inline(always)]
    pub fn tzc_codec_tzmid_sel(&self) -> TzcCodecTzmidSelR {
        TzcCodecTzmidSelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Multimedia Subsystem TZID select enable"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid_sel(&self) -> TzcSubsysTzmidSelR {
        TzcSubsysTzmidSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 2D DMA TZID select enable"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid_sel(&self) -> TzcX2ddmaTzmidSelR {
        TzcX2ddmaTzmidSelR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA2 TZID select enable"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid_sel(&self) -> TzcXdmaTzmidSelR {
        TzcXdmaTzmidSelR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MM CPU (D0) TZID value"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid(&mut self) -> TzcMmcpuTzmidW<'_, TzcMmBmxTzmidSpec> {
        TzcMmcpuTzmidW::new(self, 0)
    }
    #[doc = "Bit 1 - BLAI (NPU) TZID value"]
    #[inline(always)]
    pub fn tzc_blai_tzmid(&mut self) -> TzcBlaiTzmidW<'_, TzcMmBmxTzmidSpec> {
        TzcBlaiTzmidW::new(self, 1)
    }
    #[doc = "Bit 2 - CODEC TZID value"]
    #[inline(always)]
    pub fn tzc_codec_tzmid(&mut self) -> TzcCodecTzmidW<'_, TzcMmBmxTzmidSpec> {
        TzcCodecTzmidW::new(self, 2)
    }
    #[doc = "Bit 3 - Multimedia Subsystem TZID value"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid(&mut self) -> TzcSubsysTzmidW<'_, TzcMmBmxTzmidSpec> {
        TzcSubsysTzmidW::new(self, 3)
    }
    #[doc = "Bit 4 - 2D DMA TZID value"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid(&mut self) -> TzcX2ddmaTzmidW<'_, TzcMmBmxTzmidSpec> {
        TzcX2ddmaTzmidW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA2 TZID value"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid(&mut self) -> TzcXdmaTzmidW<'_, TzcMmBmxTzmidSpec> {
        TzcXdmaTzmidW::new(self, 5)
    }
    #[doc = "Bit 16 - MM CPU (D0) TZID select enable"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid_sel(&mut self) -> TzcMmcpuTzmidSelW<'_, TzcMmBmxTzmidSpec> {
        TzcMmcpuTzmidSelW::new(self, 16)
    }
    #[doc = "Bit 17 - BLAI (NPU) TZID select enable"]
    #[inline(always)]
    pub fn tzc_blai_tzmid_sel(&mut self) -> TzcBlaiTzmidSelW<'_, TzcMmBmxTzmidSpec> {
        TzcBlaiTzmidSelW::new(self, 17)
    }
    #[doc = "Bit 18 - CODEC TZID select enable"]
    #[inline(always)]
    pub fn tzc_codec_tzmid_sel(&mut self) -> TzcCodecTzmidSelW<'_, TzcMmBmxTzmidSpec> {
        TzcCodecTzmidSelW::new(self, 18)
    }
    #[doc = "Bit 19 - Multimedia Subsystem TZID select enable"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid_sel(&mut self) -> TzcSubsysTzmidSelW<'_, TzcMmBmxTzmidSpec> {
        TzcSubsysTzmidSelW::new(self, 19)
    }
    #[doc = "Bit 20 - 2D DMA TZID select enable"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid_sel(&mut self) -> TzcX2ddmaTzmidSelW<'_, TzcMmBmxTzmidSpec> {
        TzcX2ddmaTzmidSelW::new(self, 20)
    }
    #[doc = "Bit 21 - DMA2 TZID select enable"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid_sel(&mut self) -> TzcXdmaTzmidSelW<'_, TzcMmBmxTzmidSpec> {
        TzcXdmaTzmidSelW::new(self, 21)
    }
}
#[doc = "Multimedia Bus Matrix Master TZID Assignment\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_tzmid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_bmx_tzmid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmBmxTzmidSpec;
impl crate::RegisterSpec for TzcMmBmxTzmidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_tzmid::R`](R) reader structure"]
impl crate::Readable for TzcMmBmxTzmidSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_mm_bmx_tzmid::W`](W) writer structure"]
impl crate::Writable for TzcMmBmxTzmidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_MM_BMX_TZMID to value 0x003f_0000"]
impl crate::Resettable for TzcMmBmxTzmidSpec {
    const RESET_VALUE: u32 = 0x003f_0000;
}
