#[doc = "Register `TZC_MM_BMX_TZMID_LOCK` reader"]
pub type R = crate::R<TzcMmBmxTzmidLockSpec>;
#[doc = "Register `TZC_MM_BMX_TZMID_LOCK` writer"]
pub type W = crate::W<TzcMmBmxTzmidLockSpec>;
#[doc = "Field `tzc_mmcpu_tzmid_lock` reader - MM CPU (D0) TZID lock"]
pub type TzcMmcpuTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_mmcpu_tzmid_lock` writer - MM CPU (D0) TZID lock"]
pub type TzcMmcpuTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_blai_tzmid_lock` reader - BLAI (NPU) TZID lock"]
pub type TzcBlaiTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_blai_tzmid_lock` writer - BLAI (NPU) TZID lock"]
pub type TzcBlaiTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_codec_tzmid_lock` reader - CODEC TZID lock"]
pub type TzcCodecTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_tzmid_lock` writer - CODEC TZID lock"]
pub type TzcCodecTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_subsys_tzmid_lock` reader - Multimedia Subsystem TZID lock"]
pub type TzcSubsysTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_subsys_tzmid_lock` writer - Multimedia Subsystem TZID lock"]
pub type TzcSubsysTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_x2ddma_tzmid_lock` reader - 2D DMA TZID lock"]
pub type TzcX2ddmaTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_x2ddma_tzmid_lock` writer - 2D DMA TZID lock"]
pub type TzcX2ddmaTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_xdma_tzmid_lock` reader - DMA2 TZID lock"]
pub type TzcXdmaTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_xdma_tzmid_lock` writer - DMA2 TZID lock"]
pub type TzcXdmaTzmidLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MM CPU (D0) TZID lock"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid_lock(&self) -> TzcMmcpuTzmidLockR {
        TzcMmcpuTzmidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BLAI (NPU) TZID lock"]
    #[inline(always)]
    pub fn tzc_blai_tzmid_lock(&self) -> TzcBlaiTzmidLockR {
        TzcBlaiTzmidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CODEC TZID lock"]
    #[inline(always)]
    pub fn tzc_codec_tzmid_lock(&self) -> TzcCodecTzmidLockR {
        TzcCodecTzmidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multimedia Subsystem TZID lock"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid_lock(&self) -> TzcSubsysTzmidLockR {
        TzcSubsysTzmidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 2D DMA TZID lock"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid_lock(&self) -> TzcX2ddmaTzmidLockR {
        TzcX2ddmaTzmidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA2 TZID lock"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid_lock(&self) -> TzcXdmaTzmidLockR {
        TzcXdmaTzmidLockR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MM CPU (D0) TZID lock"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid_lock(&mut self) -> TzcMmcpuTzmidLockW<'_, TzcMmBmxTzmidLockSpec> {
        TzcMmcpuTzmidLockW::new(self, 0)
    }
    #[doc = "Bit 1 - BLAI (NPU) TZID lock"]
    #[inline(always)]
    pub fn tzc_blai_tzmid_lock(&mut self) -> TzcBlaiTzmidLockW<'_, TzcMmBmxTzmidLockSpec> {
        TzcBlaiTzmidLockW::new(self, 1)
    }
    #[doc = "Bit 2 - CODEC TZID lock"]
    #[inline(always)]
    pub fn tzc_codec_tzmid_lock(&mut self) -> TzcCodecTzmidLockW<'_, TzcMmBmxTzmidLockSpec> {
        TzcCodecTzmidLockW::new(self, 2)
    }
    #[doc = "Bit 3 - Multimedia Subsystem TZID lock"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid_lock(&mut self) -> TzcSubsysTzmidLockW<'_, TzcMmBmxTzmidLockSpec> {
        TzcSubsysTzmidLockW::new(self, 3)
    }
    #[doc = "Bit 4 - 2D DMA TZID lock"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid_lock(&mut self) -> TzcX2ddmaTzmidLockW<'_, TzcMmBmxTzmidLockSpec> {
        TzcX2ddmaTzmidLockW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA2 TZID lock"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid_lock(&mut self) -> TzcXdmaTzmidLockW<'_, TzcMmBmxTzmidLockSpec> {
        TzcXdmaTzmidLockW::new(self, 5)
    }
}
#[doc = "Multimedia Bus Matrix Master TZID Lock (Write-Once)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_tzmid_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_mm_bmx_tzmid_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmBmxTzmidLockSpec;
impl crate::RegisterSpec for TzcMmBmxTzmidLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_tzmid_lock::R`](R) reader structure"]
impl crate::Readable for TzcMmBmxTzmidLockSpec {}
#[doc = "`write(|w| ..)` method takes [`tzc_mm_bmx_tzmid_lock::W`](W) writer structure"]
impl crate::Writable for TzcMmBmxTzmidLockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_MM_BMX_TZMID_LOCK to value 0"]
impl crate::Resettable for TzcMmBmxTzmidLockSpec {}
