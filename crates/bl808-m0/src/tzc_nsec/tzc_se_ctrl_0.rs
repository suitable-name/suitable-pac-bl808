#[doc = "Register `TZC_SE_CTRL_0` reader"]
pub type R = crate::R<TzcSeCtrl0Spec>;
#[doc = "Field `tzc_se_sha_tzsid_en` reader - "]
pub type TzcSeShaTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_aes_tzsid_en` reader - "]
pub type TzcSeAesTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_trng_tzsid_en` reader - "]
pub type TzcSeTrngTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_pka_tzsid_en` reader - "]
pub type TzcSePkaTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_cdet_tzsid_en` reader - "]
pub type TzcSeCdetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_gmac_tzsid_en` reader - "]
pub type TzcSeGmacTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_tzsid_crmd` reader - SE Command Mode"]
pub type TzcSeTzsidCrmdR = crate::BitReader;
#[doc = "Field `tzc_se_wdt_dly` reader - SE Watchdog Delay"]
pub type TzcSeWdtDlyR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_se_sha_tzsid_en(&self) -> TzcSeShaTzsidEnR {
        TzcSeShaTzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_se_aes_tzsid_en(&self) -> TzcSeAesTzsidEnR {
        TzcSeAesTzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_se_trng_tzsid_en(&self) -> TzcSeTrngTzsidEnR {
        TzcSeTrngTzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_se_pka_tzsid_en(&self) -> TzcSePkaTzsidEnR {
        TzcSePkaTzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tzc_se_cdet_tzsid_en(&self) -> TzcSeCdetTzsidEnR {
        TzcSeCdetTzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_se_gmac_tzsid_en(&self) -> TzcSeGmacTzsidEnR {
        TzcSeGmacTzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SE Command Mode"]
    #[inline(always)]
    pub fn tzc_se_tzsid_crmd(&self) -> TzcSeTzsidCrmdR {
        TzcSeTzsidCrmdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - SE Watchdog Delay"]
    #[inline(always)]
    pub fn tzc_se_wdt_dly(&self) -> TzcSeWdtDlyR {
        TzcSeWdtDlyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Security Engine Control 0 (TZID Enable)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSeCtrl0Spec;
impl crate::RegisterSpec for TzcSeCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_se_ctrl_0::R`](R) reader structure"]
impl crate::Readable for TzcSeCtrl0Spec {}
#[doc = "`reset()` method sets TZC_SE_CTRL_0 to value 0"]
impl crate::Resettable for TzcSeCtrl0Spec {}
