#[doc = "Register `TZC_MM_BMX_S_LOCK0` reader"]
pub type R = crate::R<TzcMmBmxSLock0Spec>;
#[doc = "Field `tzc_mmperi_s10_tzsid_lock` reader - "]
pub type TzcMmperiS10TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s11_tzsid_lock` reader - "]
pub type TzcMmperiS11TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s12_tzsid_lock` reader - "]
pub type TzcMmperiS12TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s13_tzsid_lock` reader - "]
pub type TzcMmperiS13TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s14_tzsid_lock` reader - "]
pub type TzcMmperiS14TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s15_tzsid_lock` reader - "]
pub type TzcMmperiS15TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s16_tzsid_lock` reader - "]
pub type TzcMmperiS16TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s17_tzsid_lock` reader - "]
pub type TzcMmperiS17TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s18_tzsid_lock` reader - "]
pub type TzcMmperiS18TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s19_tzsid_lock` reader - "]
pub type TzcMmperiS19TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s1a_tzsid_lock` reader - "]
pub type TzcMmperiS1aTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s1b_tzsid_lock` reader - "]
pub type TzcMmperiS1bTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s1c_tzsid_lock` reader - "]
pub type TzcMmperiS1cTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s1d_tzsid_lock` reader - "]
pub type TzcMmperiS1dTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s1e_tzsid_lock` reader - "]
pub type TzcMmperiS1eTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_mmperi_s1f_tzsid_lock` reader - "]
pub type TzcMmperiS1fTzsidLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_mmperi_s10_tzsid_lock(&self) -> TzcMmperiS10TzsidLockR {
        TzcMmperiS10TzsidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_mmperi_s11_tzsid_lock(&self) -> TzcMmperiS11TzsidLockR {
        TzcMmperiS11TzsidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_mmperi_s12_tzsid_lock(&self) -> TzcMmperiS12TzsidLockR {
        TzcMmperiS12TzsidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_mmperi_s13_tzsid_lock(&self) -> TzcMmperiS13TzsidLockR {
        TzcMmperiS13TzsidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_mmperi_s14_tzsid_lock(&self) -> TzcMmperiS14TzsidLockR {
        TzcMmperiS14TzsidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_mmperi_s15_tzsid_lock(&self) -> TzcMmperiS15TzsidLockR {
        TzcMmperiS15TzsidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_mmperi_s16_tzsid_lock(&self) -> TzcMmperiS16TzsidLockR {
        TzcMmperiS16TzsidLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_mmperi_s17_tzsid_lock(&self) -> TzcMmperiS17TzsidLockR {
        TzcMmperiS17TzsidLockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_mmperi_s18_tzsid_lock(&self) -> TzcMmperiS18TzsidLockR {
        TzcMmperiS18TzsidLockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_mmperi_s19_tzsid_lock(&self) -> TzcMmperiS19TzsidLockR {
        TzcMmperiS19TzsidLockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_mmperi_s1a_tzsid_lock(&self) -> TzcMmperiS1aTzsidLockR {
        TzcMmperiS1aTzsidLockR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_mmperi_s1b_tzsid_lock(&self) -> TzcMmperiS1bTzsidLockR {
        TzcMmperiS1bTzsidLockR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_mmperi_s1c_tzsid_lock(&self) -> TzcMmperiS1cTzsidLockR {
        TzcMmperiS1cTzsidLockR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_mmperi_s1d_tzsid_lock(&self) -> TzcMmperiS1dTzsidLockR {
        TzcMmperiS1dTzsidLockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_mmperi_s1e_tzsid_lock(&self) -> TzcMmperiS1eTzsidLockR {
        TzcMmperiS1eTzsidLockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_mmperi_s1f_tzsid_lock(&self) -> TzcMmperiS1fTzsidLockR {
        TzcMmperiS1fTzsidLockR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "MM BMX Slave Lock 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s_lock0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmBmxSLock0Spec;
impl crate::RegisterSpec for TzcMmBmxSLock0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_s_lock0::R`](R) reader structure"]
impl crate::Readable for TzcMmBmxSLock0Spec {}
#[doc = "`reset()` method sets TZC_MM_BMX_S_LOCK0 to value 0"]
impl crate::Resettable for TzcMmBmxSLock0Spec {}
