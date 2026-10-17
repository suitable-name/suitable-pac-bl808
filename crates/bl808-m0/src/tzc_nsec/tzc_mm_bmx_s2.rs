#[doc = "Register `TZC_MM_BMX_S2` reader"]
pub type R = crate::R<TzcMmBmxS2Spec>;
#[doc = "Field `tzc_codec_s10_tzsid_en` reader - "]
pub type TzcCodecS10TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s11_tzsid_en` reader - "]
pub type TzcCodecS11TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s12_tzsid_en` reader - "]
pub type TzcCodecS12TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s13_tzsid_en` reader - "]
pub type TzcCodecS13TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s14_tzsid_en` reader - "]
pub type TzcCodecS14TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s15_tzsid_en` reader - "]
pub type TzcCodecS15TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s16_tzsid_en` reader - "]
pub type TzcCodecS16TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s17_tzsid_en` reader - "]
pub type TzcCodecS17TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s18_tzsid_en` reader - "]
pub type TzcCodecS18TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s19_tzsid_en` reader - "]
pub type TzcCodecS19TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s1a_tzsid_en` reader - "]
pub type TzcCodecS1aTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s1b_tzsid_en` reader - "]
pub type TzcCodecS1bTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s1c_tzsid_en` reader - "]
pub type TzcCodecS1cTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s1d_tzsid_en` reader - "]
pub type TzcCodecS1dTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s1e_tzsid_en` reader - "]
pub type TzcCodecS1eTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_codec_s1f_tzsid_en` reader - "]
pub type TzcCodecS1fTzsidEnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_codec_s10_tzsid_en(&self) -> TzcCodecS10TzsidEnR {
        TzcCodecS10TzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_codec_s11_tzsid_en(&self) -> TzcCodecS11TzsidEnR {
        TzcCodecS11TzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_codec_s12_tzsid_en(&self) -> TzcCodecS12TzsidEnR {
        TzcCodecS12TzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_codec_s13_tzsid_en(&self) -> TzcCodecS13TzsidEnR {
        TzcCodecS13TzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tzc_codec_s14_tzsid_en(&self) -> TzcCodecS14TzsidEnR {
        TzcCodecS14TzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_codec_s15_tzsid_en(&self) -> TzcCodecS15TzsidEnR {
        TzcCodecS15TzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tzc_codec_s16_tzsid_en(&self) -> TzcCodecS16TzsidEnR {
        TzcCodecS16TzsidEnR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tzc_codec_s17_tzsid_en(&self) -> TzcCodecS17TzsidEnR {
        TzcCodecS17TzsidEnR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tzc_codec_s18_tzsid_en(&self) -> TzcCodecS18TzsidEnR {
        TzcCodecS18TzsidEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn tzc_codec_s19_tzsid_en(&self) -> TzcCodecS19TzsidEnR {
        TzcCodecS19TzsidEnR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tzc_codec_s1a_tzsid_en(&self) -> TzcCodecS1aTzsidEnR {
        TzcCodecS1aTzsidEnR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tzc_codec_s1b_tzsid_en(&self) -> TzcCodecS1bTzsidEnR {
        TzcCodecS1bTzsidEnR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn tzc_codec_s1c_tzsid_en(&self) -> TzcCodecS1cTzsidEnR {
        TzcCodecS1cTzsidEnR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn tzc_codec_s1d_tzsid_en(&self) -> TzcCodecS1dTzsidEnR {
        TzcCodecS1dTzsidEnR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tzc_codec_s1e_tzsid_en(&self) -> TzcCodecS1eTzsidEnR {
        TzcCodecS1eTzsidEnR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn tzc_codec_s1f_tzsid_en(&self) -> TzcCodecS1fTzsidEnR {
        TzcCodecS1fTzsidEnR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "MM BMX Slave 2 Security ID Enable (CODEC Subsystem Slaves)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmBmxS2Spec;
impl crate::RegisterSpec for TzcMmBmxS2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_s2::R`](R) reader structure"]
impl crate::Readable for TzcMmBmxS2Spec {}
#[doc = "`reset()` method sets TZC_MM_BMX_S2 to value 0"]
impl crate::Resettable for TzcMmBmxS2Spec {}
