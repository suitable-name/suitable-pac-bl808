#[doc = "Register `TZC_MM_BMX_S0` reader"]
pub type R = crate::R<TzcMmBmxS0Spec>;
#[doc = "Field `tzc_mmperi_s10_tzsid_en` reader - "]
pub type TzcMmperiS10TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s11_tzsid_en` reader - "]
pub type TzcMmperiS11TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s12_tzsid_en` reader - "]
pub type TzcMmperiS12TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s13_tzsid_en` reader - "]
pub type TzcMmperiS13TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s14_tzsid_en` reader - "]
pub type TzcMmperiS14TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s15_tzsid_en` reader - "]
pub type TzcMmperiS15TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s16_tzsid_en` reader - "]
pub type TzcMmperiS16TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s17_tzsid_en` reader - "]
pub type TzcMmperiS17TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s18_tzsid_en` reader - "]
pub type TzcMmperiS18TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s19_tzsid_en` reader - "]
pub type TzcMmperiS19TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s1a_tzsid_en` reader - "]
pub type TzcMmperiS1aTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s1b_tzsid_en` reader - "]
pub type TzcMmperiS1bTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s1c_tzsid_en` reader - "]
pub type TzcMmperiS1cTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s1d_tzsid_en` reader - "]
pub type TzcMmperiS1dTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s1e_tzsid_en` reader - "]
pub type TzcMmperiS1eTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_mmperi_s1f_tzsid_en` reader - "]
pub type TzcMmperiS1fTzsidEnR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_mmperi_s10_tzsid_en(&self) -> TzcMmperiS10TzsidEnR {
        TzcMmperiS10TzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_mmperi_s11_tzsid_en(&self) -> TzcMmperiS11TzsidEnR {
        TzcMmperiS11TzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_mmperi_s12_tzsid_en(&self) -> TzcMmperiS12TzsidEnR {
        TzcMmperiS12TzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_mmperi_s13_tzsid_en(&self) -> TzcMmperiS13TzsidEnR {
        TzcMmperiS13TzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tzc_mmperi_s14_tzsid_en(&self) -> TzcMmperiS14TzsidEnR {
        TzcMmperiS14TzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_mmperi_s15_tzsid_en(&self) -> TzcMmperiS15TzsidEnR {
        TzcMmperiS15TzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn tzc_mmperi_s16_tzsid_en(&self) -> TzcMmperiS16TzsidEnR {
        TzcMmperiS16TzsidEnR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn tzc_mmperi_s17_tzsid_en(&self) -> TzcMmperiS17TzsidEnR {
        TzcMmperiS17TzsidEnR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tzc_mmperi_s18_tzsid_en(&self) -> TzcMmperiS18TzsidEnR {
        TzcMmperiS18TzsidEnR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn tzc_mmperi_s19_tzsid_en(&self) -> TzcMmperiS19TzsidEnR {
        TzcMmperiS19TzsidEnR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn tzc_mmperi_s1a_tzsid_en(&self) -> TzcMmperiS1aTzsidEnR {
        TzcMmperiS1aTzsidEnR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn tzc_mmperi_s1b_tzsid_en(&self) -> TzcMmperiS1bTzsidEnR {
        TzcMmperiS1bTzsidEnR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn tzc_mmperi_s1c_tzsid_en(&self) -> TzcMmperiS1cTzsidEnR {
        TzcMmperiS1cTzsidEnR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn tzc_mmperi_s1d_tzsid_en(&self) -> TzcMmperiS1dTzsidEnR {
        TzcMmperiS1dTzsidEnR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn tzc_mmperi_s1e_tzsid_en(&self) -> TzcMmperiS1eTzsidEnR {
        TzcMmperiS1eTzsidEnR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn tzc_mmperi_s1f_tzsid_en(&self) -> TzcMmperiS1fTzsidEnR {
        TzcMmperiS1fTzsidEnR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "MM BMX Slave 0 Security ID Enable (MM Peripherals)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_mm_bmx_s0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcMmBmxS0Spec;
impl crate::RegisterSpec for TzcMmBmxS0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_s0::R`](R) reader structure"]
impl crate::Readable for TzcMmBmxS0Spec {}
#[doc = "`reset()` method sets TZC_MM_BMX_S0 to value 0"]
impl crate::Resettable for TzcMmBmxS0Spec {}
