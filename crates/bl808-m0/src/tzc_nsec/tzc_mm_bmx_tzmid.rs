#[doc = "Register `TZC_MM_BMX_TZMID` reader"]
pub type R = crate::R<TzcMmBmxTzmidSpec>;
#[doc = "Field `tzc_mmcpu_tzmid` reader - "]
pub type TzcMmcpuTzmidR = crate::BitReader;
#[doc = "Field `tzc_blai_tzmid` reader - BLAI (NPU) TZID Status"]
pub type TzcBlaiTzmidR = crate::BitReader;
#[doc = "Field `tzc_codec_tzmid` reader - "]
pub type TzcCodecTzmidR = crate::BitReader;
#[doc = "Field `tzc_subsys_tzmid` reader - MM Subsystem TZID Status"]
pub type TzcSubsysTzmidR = crate::BitReader;
#[doc = "Field `tzc_x2ddma_tzmid` reader - DMA2D TZID Status"]
pub type TzcX2ddmaTzmidR = crate::BitReader;
#[doc = "Field `tzc_xdma_tzmid` reader - DMA2 TZID Status"]
pub type TzcXdmaTzmidR = crate::BitReader;
#[doc = "Field `tzc_mmcpu_tzmid_sel` reader - "]
pub type TzcMmcpuTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_blai_tzmid_sel` reader - "]
pub type TzcBlaiTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_codec_tzmid_sel` reader - "]
pub type TzcCodecTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_subsys_tzmid_sel` reader - "]
pub type TzcSubsysTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_x2ddma_tzmid_sel` reader - "]
pub type TzcX2ddmaTzmidSelR = crate::BitReader;
#[doc = "Field `tzc_xdma_tzmid_sel` reader - "]
pub type TzcXdmaTzmidSelR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid(&self) -> TzcMmcpuTzmidR {
        TzcMmcpuTzmidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BLAI (NPU) TZID Status"]
    #[inline(always)]
    pub fn tzc_blai_tzmid(&self) -> TzcBlaiTzmidR {
        TzcBlaiTzmidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_codec_tzmid(&self) -> TzcCodecTzmidR {
        TzcCodecTzmidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MM Subsystem TZID Status"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid(&self) -> TzcSubsysTzmidR {
        TzcSubsysTzmidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D TZID Status"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid(&self) -> TzcX2ddmaTzmidR {
        TzcX2ddmaTzmidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA2 TZID Status"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid(&self) -> TzcXdmaTzmidR {
        TzcXdmaTzmidR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid_sel(&self) -> TzcMmcpuTzmidSelR {
        TzcMmcpuTzmidSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_blai_tzmid_sel(&self) -> TzcBlaiTzmidSelR {
        TzcBlaiTzmidSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_codec_tzmid_sel(&self) -> TzcCodecTzmidSelR {
        TzcCodecTzmidSelR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid_sel(&self) -> TzcSubsysTzmidSelR {
        TzcSubsysTzmidSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid_sel(&self) -> TzcX2ddmaTzmidSelR {
        TzcX2ddmaTzmidSelR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid_sel(&self) -> TzcXdmaTzmidSelR {
        TzcXdmaTzmidSelR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "MM BMX Bus Master ID Security State\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_tzmid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmBmxTzmidSpec;
impl crate::RegisterSpec for TzcMmBmxTzmidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_tzmid::R`](R) reader structure"]
impl crate::Readable for TzcMmBmxTzmidSpec {}
#[doc = "`reset()` method sets TZC_MM_BMX_TZMID to value 0"]
impl crate::Resettable for TzcMmBmxTzmidSpec {}
