#[doc = "Register `TZC_MM_BMX_S_LOCK1` reader"]
pub type R = crate::R<TzcMmBmxSLock1Spec>;
#[doc = "Field `tzc_codec_s10_tzsid_lock` reader - "]
pub type TzcCodecS10TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s11_tzsid_lock` reader - "]
pub type TzcCodecS11TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s12_tzsid_lock` reader - "]
pub type TzcCodecS12TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s13_tzsid_lock` reader - "]
pub type TzcCodecS13TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s14_tzsid_lock` reader - "]
pub type TzcCodecS14TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s15_tzsid_lock` reader - "]
pub type TzcCodecS15TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s16_tzsid_lock` reader - "]
pub type TzcCodecS16TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s17_tzsid_lock` reader - "]
pub type TzcCodecS17TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s18_tzsid_lock` reader - "]
pub type TzcCodecS18TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s19_tzsid_lock` reader - "]
pub type TzcCodecS19TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s1a_tzsid_lock` reader - "]
pub type TzcCodecS1aTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s1b_tzsid_lock` reader - "]
pub type TzcCodecS1bTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s1c_tzsid_lock` reader - "]
pub type TzcCodecS1cTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s1d_tzsid_lock` reader - "]
pub type TzcCodecS1dTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s1e_tzsid_lock` reader - "]
pub type TzcCodecS1eTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_codec_s1f_tzsid_lock` reader - "]
pub type TzcCodecS1fTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s10_tzsid_lock` reader - "]
pub type TzcS10TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s11_tzsid_lock` reader - "]
pub type TzcS11TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s12_tzsid_lock` reader - "]
pub type TzcS12TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s13_tzsid_lock` reader - "]
pub type TzcS13TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s14_tzsid_lock` reader - "]
pub type TzcS14TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s15_tzsid_lock` reader - "]
pub type TzcS15TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s16_tzsid_lock` reader - "]
pub type TzcS16TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s17_tzsid_lock` reader - "]
pub type TzcS17TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s18_tzsid_lock` reader - "]
pub type TzcS18TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s19_tzsid_lock` reader - "]
pub type TzcS19TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s1a_tzsid_lock` reader - "]
pub type TzcS1aTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s1b_tzsid_lock` reader - "]
pub type TzcS1bTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s1c_tzsid_lock` reader - "]
pub type TzcS1cTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s1d_tzsid_lock` reader - "]
pub type TzcS1dTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s1e_tzsid_lock` reader - "]
pub type TzcS1eTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_s1f_tzsid_lock` reader - "]
pub type TzcS1fTzsidLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_codec_s10_tzsid_lock(&self) -> TzcCodecS10TzsidLockR {
        TzcCodecS10TzsidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_codec_s11_tzsid_lock(&self) -> TzcCodecS11TzsidLockR {
        TzcCodecS11TzsidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_codec_s12_tzsid_lock(&self) -> TzcCodecS12TzsidLockR {
        TzcCodecS12TzsidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_codec_s13_tzsid_lock(&self) -> TzcCodecS13TzsidLockR {
        TzcCodecS13TzsidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_codec_s14_tzsid_lock(&self) -> TzcCodecS14TzsidLockR {
        TzcCodecS14TzsidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_codec_s15_tzsid_lock(&self) -> TzcCodecS15TzsidLockR {
        TzcCodecS15TzsidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_codec_s16_tzsid_lock(&self) -> TzcCodecS16TzsidLockR {
        TzcCodecS16TzsidLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_codec_s17_tzsid_lock(&self) -> TzcCodecS17TzsidLockR {
        TzcCodecS17TzsidLockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_codec_s18_tzsid_lock(&self) -> TzcCodecS18TzsidLockR {
        TzcCodecS18TzsidLockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_codec_s19_tzsid_lock(&self) -> TzcCodecS19TzsidLockR {
        TzcCodecS19TzsidLockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_codec_s1a_tzsid_lock(&self) -> TzcCodecS1aTzsidLockR {
        TzcCodecS1aTzsidLockR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_codec_s1b_tzsid_lock(&self) -> TzcCodecS1bTzsidLockR {
        TzcCodecS1bTzsidLockR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_codec_s1c_tzsid_lock(&self) -> TzcCodecS1cTzsidLockR {
        TzcCodecS1cTzsidLockR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_codec_s1d_tzsid_lock(&self) -> TzcCodecS1dTzsidLockR {
        TzcCodecS1dTzsidLockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_codec_s1e_tzsid_lock(&self) -> TzcCodecS1eTzsidLockR {
        TzcCodecS1eTzsidLockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_codec_s1f_tzsid_lock(&self) -> TzcCodecS1fTzsidLockR {
        TzcCodecS1fTzsidLockR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_s10_tzsid_lock(&self) -> TzcS10TzsidLockR {
        TzcS10TzsidLockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_s11_tzsid_lock(&self) -> TzcS11TzsidLockR {
        TzcS11TzsidLockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_s12_tzsid_lock(&self) -> TzcS12TzsidLockR {
        TzcS12TzsidLockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_s13_tzsid_lock(&self) -> TzcS13TzsidLockR {
        TzcS13TzsidLockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_s14_tzsid_lock(&self) -> TzcS14TzsidLockR {
        TzcS14TzsidLockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_s15_tzsid_lock(&self) -> TzcS15TzsidLockR {
        TzcS15TzsidLockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_s16_tzsid_lock(&self) -> TzcS16TzsidLockR {
        TzcS16TzsidLockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_s17_tzsid_lock(&self) -> TzcS17TzsidLockR {
        TzcS17TzsidLockR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_s18_tzsid_lock(&self) -> TzcS18TzsidLockR {
        TzcS18TzsidLockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_s19_tzsid_lock(&self) -> TzcS19TzsidLockR {
        TzcS19TzsidLockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_s1a_tzsid_lock(&self) -> TzcS1aTzsidLockR {
        TzcS1aTzsidLockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_s1b_tzsid_lock(&self) -> TzcS1bTzsidLockR {
        TzcS1bTzsidLockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_s1c_tzsid_lock(&self) -> TzcS1cTzsidLockR {
        TzcS1cTzsidLockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_s1d_tzsid_lock(&self) -> TzcS1dTzsidLockR {
        TzcS1dTzsidLockR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_s1e_tzsid_lock(&self) -> TzcS1eTzsidLockR {
        TzcS1eTzsidLockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_s1f_tzsid_lock(&self) -> TzcS1fTzsidLockR {
        TzcS1fTzsidLockR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "MM BMX Slave Lock 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s_lock1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmBmxSLock1Spec;
impl crate::RegisterSpec for TzcMmBmxSLock1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_s_lock1::R`](R) reader structure"]
impl crate::Readable for TzcMmBmxSLock1Spec {}
#[doc = "`reset()` method sets TZC_MM_BMX_S_LOCK1 to value 0"]
impl crate::Resettable for TzcMmBmxSLock1Spec {}
