#[doc = "Register `TZC_MM_BMX_S1` reader"]
pub type R = crate::R<TzcMmBmxS1Spec>;
#[doc = "Field `tzc_s10_tzsid_en` reader - "]
pub type TzcS10TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s11_tzsid_en` reader - "]
pub type TzcS11TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s12_tzsid_en` reader - "]
pub type TzcS12TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s13_tzsid_en` reader - "]
pub type TzcS13TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s14_tzsid_en` reader - "]
pub type TzcS14TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s15_tzsid_en` reader - "]
pub type TzcS15TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s16_tzsid_en` reader - "]
pub type TzcS16TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s17_tzsid_en` reader - "]
pub type TzcS17TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s18_tzsid_en` reader - "]
pub type TzcS18TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s19_tzsid_en` reader - "]
pub type TzcS19TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s1a_tzsid_en` reader - "]
pub type TzcS1aTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s1b_tzsid_en` reader - "]
pub type TzcS1bTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s1c_tzsid_en` reader - "]
pub type TzcS1cTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s1d_tzsid_en` reader - "]
pub type TzcS1dTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s1e_tzsid_en` reader - "]
pub type TzcS1eTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_s1f_tzsid_en` reader - "]
pub type TzcS1fTzsidEnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_s10_tzsid_en(&self) -> TzcS10TzsidEnR {
        TzcS10TzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_s11_tzsid_en(&self) -> TzcS11TzsidEnR {
        TzcS11TzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_s12_tzsid_en(&self) -> TzcS12TzsidEnR {
        TzcS12TzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_s13_tzsid_en(&self) -> TzcS13TzsidEnR {
        TzcS13TzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tzc_s14_tzsid_en(&self) -> TzcS14TzsidEnR {
        TzcS14TzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_s15_tzsid_en(&self) -> TzcS15TzsidEnR {
        TzcS15TzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tzc_s16_tzsid_en(&self) -> TzcS16TzsidEnR {
        TzcS16TzsidEnR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tzc_s17_tzsid_en(&self) -> TzcS17TzsidEnR {
        TzcS17TzsidEnR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tzc_s18_tzsid_en(&self) -> TzcS18TzsidEnR {
        TzcS18TzsidEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn tzc_s19_tzsid_en(&self) -> TzcS19TzsidEnR {
        TzcS19TzsidEnR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tzc_s1a_tzsid_en(&self) -> TzcS1aTzsidEnR {
        TzcS1aTzsidEnR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tzc_s1b_tzsid_en(&self) -> TzcS1bTzsidEnR {
        TzcS1bTzsidEnR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn tzc_s1c_tzsid_en(&self) -> TzcS1cTzsidEnR {
        TzcS1cTzsidEnR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn tzc_s1d_tzsid_en(&self) -> TzcS1dTzsidEnR {
        TzcS1dTzsidEnR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tzc_s1e_tzsid_en(&self) -> TzcS1eTzsidEnR {
        TzcS1eTzsidEnR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn tzc_s1f_tzsid_en(&self) -> TzcS1fTzsidEnR {
        TzcS1fTzsidEnR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "MM BMX Slave 1 Security ID Enable (CODEC Subsystem)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmBmxS1Spec;
impl crate::RegisterSpec for TzcMmBmxS1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_s1::R`](R) reader structure"]
impl crate::Readable for TzcMmBmxS1Spec {}
#[doc = "`reset()` method sets TZC_MM_BMX_S1 to value 0"]
impl crate::Resettable for TzcMmBmxS1Spec {}
