#[doc = "Register `TZC_SE_CTRL_2` reader"]
pub type R = crate::R<TzcSeCtrl2Spec>;
#[doc = "Field `tzc_se_sha_tzsid_lock` reader - "]
pub type TzcSeShaTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_aes_tzsid_lock` reader - "]
pub type TzcSeAesTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_trng_tzsid_lock` reader - "]
pub type TzcSeTrngTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_pka_tzsid_lock` reader - "]
pub type TzcSePkaTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_cdet_tzsid_lock` reader - "]
pub type TzcSeCdetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_gmac_tzsid_lock` reader - "]
pub type TzcSeGmacTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_tzsid_crmd_lock` reader - "]
pub type TzcSeTzsidCrmdLockR = crate::BitReader;
#[doc = "Field `tzc_sf_cr_tzsid_lock` reader - "]
pub type TzcSfCrTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_sf_sec_tzsid_lock` reader - "]
pub type TzcSfSecTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsid_crmd_lock` reader - "]
pub type TzcSfTzsidCrmdLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_se_sha_tzsid_lock(&self) -> TzcSeShaTzsidLockR {
        TzcSeShaTzsidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_se_aes_tzsid_lock(&self) -> TzcSeAesTzsidLockR {
        TzcSeAesTzsidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_se_trng_tzsid_lock(&self) -> TzcSeTrngTzsidLockR {
        TzcSeTrngTzsidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_se_pka_tzsid_lock(&self) -> TzcSePkaTzsidLockR {
        TzcSePkaTzsidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_se_cdet_tzsid_lock(&self) -> TzcSeCdetTzsidLockR {
        TzcSeCdetTzsidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_se_gmac_tzsid_lock(&self) -> TzcSeGmacTzsidLockR {
        TzcSeGmacTzsidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_se_tzsid_crmd_lock(&self) -> TzcSeTzsidCrmdLockR {
        TzcSeTzsidCrmdLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_sf_cr_tzsid_lock(&self) -> TzcSfCrTzsidLockR {
        TzcSfCrTzsidLockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_sf_sec_tzsid_lock(&self) -> TzcSfSecTzsidLockR {
        TzcSfSecTzsidLockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_sf_tzsid_crmd_lock(&self) -> TzcSfTzsidCrmdLockR {
        TzcSfTzsidCrmdLockR::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Security Engine Control 2 (TZID Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSeCtrl2Spec;
impl crate::RegisterSpec for TzcSeCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_se_ctrl_2::R`](R) reader structure"]
impl crate::Readable for TzcSeCtrl2Spec {}
#[doc = "`reset()` method sets TZC_SE_CTRL_2 to value 0"]
impl crate::Resettable for TzcSeCtrl2Spec {}
