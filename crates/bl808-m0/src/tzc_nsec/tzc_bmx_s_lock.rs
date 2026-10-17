#[doc = "Register `TZC_BMX_S_LOCK` reader"]
pub type R = crate::R<TzcBmxSLockSpec>;
#[doc = "Field `tzc_bmx_s10_tzsid_lock` reader - "]
pub type TzcBmxS10TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s11_tzsid_lock` reader - "]
pub type TzcBmxS11TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s12_tzsid_lock` reader - "]
pub type TzcBmxS12TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s13_tzsid_lock` reader - "]
pub type TzcBmxS13TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s14_tzsid_lock` reader - "]
pub type TzcBmxS14TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s15_tzsid_lock` reader - "]
pub type TzcBmxS15TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s16_tzsid_lock` reader - "]
pub type TzcBmxS16TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s17_tzsid_lock` reader - "]
pub type TzcBmxS17TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s18_tzsid_lock` reader - "]
pub type TzcBmxS18TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s19_tzsid_lock` reader - "]
pub type TzcBmxS19TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s1a_tzsid_lock` reader - "]
pub type TzcBmxS1aTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s1b_tzsid_lock` reader - "]
pub type TzcBmxS1bTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s1c_tzsid_lock` reader - "]
pub type TzcBmxS1cTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s1d_tzsid_lock` reader - "]
pub type TzcBmxS1dTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s1e_tzsid_lock` reader - "]
pub type TzcBmxS1eTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s1f_tzsid_lock` reader - "]
pub type TzcBmxS1fTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s20_tzsid_lock` reader - "]
pub type TzcBmxS20TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s21_tzsid_lock` reader - "]
pub type TzcBmxS21TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s22_tzsid_lock` reader - "]
pub type TzcBmxS22TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s23_tzsid_lock` reader - "]
pub type TzcBmxS23TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s24_tzsid_lock` reader - "]
pub type TzcBmxS24TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s25_tzsid_lock` reader - "]
pub type TzcBmxS25TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s26_tzsid_lock` reader - "]
pub type TzcBmxS26TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s27_tzsid_lock` reader - "]
pub type TzcBmxS27TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s28_tzsid_lock` reader - "]
pub type TzcBmxS28TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s29_tzsid_lock` reader - "]
pub type TzcBmxS29TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s2a_tzsid_lock` reader - "]
pub type TzcBmxS2aTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s2b_tzsid_lock` reader - "]
pub type TzcBmxS2bTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s2c_tzsid_lock` reader - "]
pub type TzcBmxS2cTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s2d_tzsid_lock` reader - "]
pub type TzcBmxS2dTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s2e_tzsid_lock` reader - "]
pub type TzcBmxS2eTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_s2f_tzsid_lock` reader - "]
pub type TzcBmxS2fTzsidLockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_bmx_s10_tzsid_lock(&self) -> TzcBmxS10TzsidLockR {
        TzcBmxS10TzsidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_bmx_s11_tzsid_lock(&self) -> TzcBmxS11TzsidLockR {
        TzcBmxS11TzsidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_bmx_s12_tzsid_lock(&self) -> TzcBmxS12TzsidLockR {
        TzcBmxS12TzsidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_bmx_s13_tzsid_lock(&self) -> TzcBmxS13TzsidLockR {
        TzcBmxS13TzsidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_bmx_s14_tzsid_lock(&self) -> TzcBmxS14TzsidLockR {
        TzcBmxS14TzsidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_bmx_s15_tzsid_lock(&self) -> TzcBmxS15TzsidLockR {
        TzcBmxS15TzsidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_bmx_s16_tzsid_lock(&self) -> TzcBmxS16TzsidLockR {
        TzcBmxS16TzsidLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_bmx_s17_tzsid_lock(&self) -> TzcBmxS17TzsidLockR {
        TzcBmxS17TzsidLockR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_bmx_s18_tzsid_lock(&self) -> TzcBmxS18TzsidLockR {
        TzcBmxS18TzsidLockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_bmx_s19_tzsid_lock(&self) -> TzcBmxS19TzsidLockR {
        TzcBmxS19TzsidLockR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_bmx_s1a_tzsid_lock(&self) -> TzcBmxS1aTzsidLockR {
        TzcBmxS1aTzsidLockR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_bmx_s1b_tzsid_lock(&self) -> TzcBmxS1bTzsidLockR {
        TzcBmxS1bTzsidLockR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_bmx_s1c_tzsid_lock(&self) -> TzcBmxS1cTzsidLockR {
        TzcBmxS1cTzsidLockR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_bmx_s1d_tzsid_lock(&self) -> TzcBmxS1dTzsidLockR {
        TzcBmxS1dTzsidLockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_bmx_s1e_tzsid_lock(&self) -> TzcBmxS1eTzsidLockR {
        TzcBmxS1eTzsidLockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_bmx_s1f_tzsid_lock(&self) -> TzcBmxS1fTzsidLockR {
        TzcBmxS1fTzsidLockR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_bmx_s20_tzsid_lock(&self) -> TzcBmxS20TzsidLockR {
        TzcBmxS20TzsidLockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_bmx_s21_tzsid_lock(&self) -> TzcBmxS21TzsidLockR {
        TzcBmxS21TzsidLockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_bmx_s22_tzsid_lock(&self) -> TzcBmxS22TzsidLockR {
        TzcBmxS22TzsidLockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_bmx_s23_tzsid_lock(&self) -> TzcBmxS23TzsidLockR {
        TzcBmxS23TzsidLockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_bmx_s24_tzsid_lock(&self) -> TzcBmxS24TzsidLockR {
        TzcBmxS24TzsidLockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_bmx_s25_tzsid_lock(&self) -> TzcBmxS25TzsidLockR {
        TzcBmxS25TzsidLockR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_bmx_s26_tzsid_lock(&self) -> TzcBmxS26TzsidLockR {
        TzcBmxS26TzsidLockR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_bmx_s27_tzsid_lock(&self) -> TzcBmxS27TzsidLockR {
        TzcBmxS27TzsidLockR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_bmx_s28_tzsid_lock(&self) -> TzcBmxS28TzsidLockR {
        TzcBmxS28TzsidLockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_bmx_s29_tzsid_lock(&self) -> TzcBmxS29TzsidLockR {
        TzcBmxS29TzsidLockR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_bmx_s2a_tzsid_lock(&self) -> TzcBmxS2aTzsidLockR {
        TzcBmxS2aTzsidLockR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_bmx_s2b_tzsid_lock(&self) -> TzcBmxS2bTzsidLockR {
        TzcBmxS2bTzsidLockR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_bmx_s2c_tzsid_lock(&self) -> TzcBmxS2cTzsidLockR {
        TzcBmxS2cTzsidLockR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_bmx_s2d_tzsid_lock(&self) -> TzcBmxS2dTzsidLockR {
        TzcBmxS2dTzsidLockR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_bmx_s2e_tzsid_lock(&self) -> TzcBmxS2eTzsidLockR {
        TzcBmxS2eTzsidLockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_bmx_s2f_tzsid_lock(&self) -> TzcBmxS2fTzsidLockR {
        TzcBmxS2fTzsidLockR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "BMX Slave TZSID Lock\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s_lock::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcBmxSLockSpec;
impl crate::RegisterSpec for TzcBmxSLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_s_lock::R`](R) reader structure"]
impl crate::Readable for TzcBmxSLockSpec {}
#[doc = "`reset()` method sets TZC_BMX_S_LOCK to value 0"]
impl crate::Resettable for TzcBmxSLockSpec {}
