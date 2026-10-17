#[doc = "Register `TZC_MM_BMX_TZMID_LOCK` reader"]
pub type R = crate::R<TzcMmBmxTzmidLockSpec>;
#[doc = "Field `tzc_mmcpu_tzmid_lock` reader - "]
pub type TzcMmcpuTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_blai_tzmid_lock` reader - "]
pub type TzcBlaiTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_tzmid_lock` reader - "]
pub type TzcCodecTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_subsys_tzmid_lock` reader - "]
pub type TzcSubsysTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_x2ddma_tzmid_lock` reader - "]
pub type TzcX2ddmaTzmidLockR = crate::BitReader;
#[doc = "Field `tzc_xdma_tzmid_lock` reader - "]
pub type TzcXdmaTzmidLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_mmcpu_tzmid_lock(&self) -> TzcMmcpuTzmidLockR {
        TzcMmcpuTzmidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_blai_tzmid_lock(&self) -> TzcBlaiTzmidLockR {
        TzcBlaiTzmidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_codec_tzmid_lock(&self) -> TzcCodecTzmidLockR {
        TzcCodecTzmidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_subsys_tzmid_lock(&self) -> TzcSubsysTzmidLockR {
        TzcSubsysTzmidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_x2ddma_tzmid_lock(&self) -> TzcX2ddmaTzmidLockR {
        TzcX2ddmaTzmidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_xdma_tzmid_lock(&self) -> TzcXdmaTzmidLockR {
        TzcXdmaTzmidLockR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "MM BMX Bus Master ID Security State Lock\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_tzmid_lock::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmBmxTzmidLockSpec;
impl crate::RegisterSpec for TzcMmBmxTzmidLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_tzmid_lock::R`](R) reader structure"]
impl crate::Readable for TzcMmBmxTzmidLockSpec {}
#[doc = "`reset()` method sets TZC_MM_BMX_TZMID_LOCK to value 0"]
impl crate::Resettable for TzcMmBmxTzmidLockSpec {}
