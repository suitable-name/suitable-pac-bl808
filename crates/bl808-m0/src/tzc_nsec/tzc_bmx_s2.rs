#[doc = "Register `TZC_BMX_S2` reader"]
pub type R = crate::R<TzcBmxS2Spec>;
#[doc = "Field `tzc_bmx_s20_tzsid_en` reader - "]
pub type TzcBmxS20TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s21_tzsid_en` reader - "]
pub type TzcBmxS21TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s22_tzsid_en` reader - "]
pub type TzcBmxS22TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s23_tzsid_en` reader - "]
pub type TzcBmxS23TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s24_tzsid_en` reader - "]
pub type TzcBmxS24TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s25_tzsid_en` reader - "]
pub type TzcBmxS25TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s26_tzsid_en` reader - "]
pub type TzcBmxS26TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s27_tzsid_en` reader - "]
pub type TzcBmxS27TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s28_tzsid_en` reader - "]
pub type TzcBmxS28TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s29_tzsid_en` reader - "]
pub type TzcBmxS29TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s2a_tzsid_en` reader - "]
pub type TzcBmxS2aTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s2b_tzsid_en` reader - "]
pub type TzcBmxS2bTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s2c_tzsid_en` reader - "]
pub type TzcBmxS2cTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s2d_tzsid_en` reader - "]
pub type TzcBmxS2dTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s2e_tzsid_en` reader - "]
pub type TzcBmxS2eTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_s2f_tzsid_en` reader - "]
pub type TzcBmxS2fTzsidEnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_bmx_s20_tzsid_en(&self) -> TzcBmxS20TzsidEnR {
        TzcBmxS20TzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_bmx_s21_tzsid_en(&self) -> TzcBmxS21TzsidEnR {
        TzcBmxS21TzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_bmx_s22_tzsid_en(&self) -> TzcBmxS22TzsidEnR {
        TzcBmxS22TzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_bmx_s23_tzsid_en(&self) -> TzcBmxS23TzsidEnR {
        TzcBmxS23TzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tzc_bmx_s24_tzsid_en(&self) -> TzcBmxS24TzsidEnR {
        TzcBmxS24TzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_bmx_s25_tzsid_en(&self) -> TzcBmxS25TzsidEnR {
        TzcBmxS25TzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tzc_bmx_s26_tzsid_en(&self) -> TzcBmxS26TzsidEnR {
        TzcBmxS26TzsidEnR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tzc_bmx_s27_tzsid_en(&self) -> TzcBmxS27TzsidEnR {
        TzcBmxS27TzsidEnR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tzc_bmx_s28_tzsid_en(&self) -> TzcBmxS28TzsidEnR {
        TzcBmxS28TzsidEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn tzc_bmx_s29_tzsid_en(&self) -> TzcBmxS29TzsidEnR {
        TzcBmxS29TzsidEnR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tzc_bmx_s2a_tzsid_en(&self) -> TzcBmxS2aTzsidEnR {
        TzcBmxS2aTzsidEnR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tzc_bmx_s2b_tzsid_en(&self) -> TzcBmxS2bTzsidEnR {
        TzcBmxS2bTzsidEnR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn tzc_bmx_s2c_tzsid_en(&self) -> TzcBmxS2cTzsidEnR {
        TzcBmxS2cTzsidEnR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn tzc_bmx_s2d_tzsid_en(&self) -> TzcBmxS2dTzsidEnR {
        TzcBmxS2dTzsidEnR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tzc_bmx_s2e_tzsid_en(&self) -> TzcBmxS2eTzsidEnR {
        TzcBmxS2eTzsidEnR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn tzc_bmx_s2f_tzsid_en(&self) -> TzcBmxS2fTzsidEnR {
        TzcBmxS2fTzsidEnR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "BMX Slave 2 Security ID Enable (TZSID for AHB Slaves 0x20 - 0x2F)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcBmxS2Spec;
impl crate::RegisterSpec for TzcBmxS2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_s2::R`](R) reader structure"]
impl crate::Readable for TzcBmxS2Spec {}
#[doc = "`reset()` method sets TZC_BMX_S2 to value 0"]
impl crate::Resettable for TzcBmxS2Spec {}
