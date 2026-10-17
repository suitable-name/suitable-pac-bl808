#[doc = "Register `TZC_MMSYS_MISC` reader"]
pub type R = crate::R<TzcMmsysMiscSpec>;
#[doc = "Field `tzc_mm_cpu0_lock` reader - TrustZone lock status for the D0 core domain."]
pub type TzcMmCpu0LockR = crate::BitReader;
#[doc = "Field `tzc_mm_sram_lock` reader - TrustZone lock status for the Multimedia SRAM regions."]
pub type TzcMmSramLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TrustZone lock status for the D0 core domain."]
    #[inline(always)]
    pub fn tzc_mm_cpu0_lock(&self) -> TzcMmCpu0LockR {
        TzcMmCpu0LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TrustZone lock status for the Multimedia SRAM regions."]
    #[inline(always)]
    pub fn tzc_mm_sram_lock(&self) -> TzcMmSramLockR {
        TzcMmSramLockR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Multimedia TrustZone and SRAM Protection Status\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mmsys_misc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmsysMiscSpec;
impl crate::RegisterSpec for TzcMmsysMiscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mmsys_misc::R`](R) reader structure"]
impl crate::Readable for TzcMmsysMiscSpec {}
#[doc = "`reset()` method sets TZC_MMSYS_MISC to value 0"]
impl crate::Resettable for TzcMmsysMiscSpec {}
