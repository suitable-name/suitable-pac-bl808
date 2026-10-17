#[doc = "Register `TZC_BMX_S0` reader"]
pub type R = crate::R<TzcBmxS0Spec>;
#[doc = "Field `tzc_bmx_mm_tzsid_en` reader - "]
pub type TzcBmxMmTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_dma_tzsid_en` reader - "]
pub type TzcBmxDmaTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_dma2_tzsid_en` reader - "]
pub type TzcBmxDma2TzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_pwr_tzsid_en` reader - "]
pub type TzcBmxPwrTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_sdh_tzsid_en` reader - "]
pub type TzcBmxSdhTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_emac_tzsid_en` reader - "]
pub type TzcBmxEmacTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_bmx_mm_tzsid_lock` reader - "]
pub type TzcBmxMmTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_dma_tzsid_lock` reader - "]
pub type TzcBmxDmaTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_dma2_tzsid_lock` reader - "]
pub type TzcBmxDma2TzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_pwr_tzsid_lock` reader - "]
pub type TzcBmxPwrTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_sdh_tzsid_lock` reader - "]
pub type TzcBmxSdhTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_bmx_emac_tzsid_lock` reader - "]
pub type TzcBmxEmacTzsidLockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tzc_bmx_mm_tzsid_en(&self) -> TzcBmxMmTzsidEnR {
        TzcBmxMmTzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tzc_bmx_dma_tzsid_en(&self) -> TzcBmxDmaTzsidEnR {
        TzcBmxDmaTzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn tzc_bmx_dma2_tzsid_en(&self) -> TzcBmxDma2TzsidEnR {
        TzcBmxDma2TzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn tzc_bmx_pwr_tzsid_en(&self) -> TzcBmxPwrTzsidEnR {
        TzcBmxPwrTzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tzc_bmx_sdh_tzsid_en(&self) -> TzcBmxSdhTzsidEnR {
        TzcBmxSdhTzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn tzc_bmx_emac_tzsid_en(&self) -> TzcBmxEmacTzsidEnR {
        TzcBmxEmacTzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_bmx_mm_tzsid_lock(&self) -> TzcBmxMmTzsidLockR {
        TzcBmxMmTzsidLockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_bmx_dma_tzsid_lock(&self) -> TzcBmxDmaTzsidLockR {
        TzcBmxDmaTzsidLockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_bmx_dma2_tzsid_lock(&self) -> TzcBmxDma2TzsidLockR {
        TzcBmxDma2TzsidLockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_bmx_pwr_tzsid_lock(&self) -> TzcBmxPwrTzsidLockR {
        TzcBmxPwrTzsidLockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_bmx_sdh_tzsid_lock(&self) -> TzcBmxSdhTzsidLockR {
        TzcBmxSdhTzsidLockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_bmx_emac_tzsid_lock(&self) -> TzcBmxEmacTzsidLockR {
        TzcBmxEmacTzsidLockR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "BMX Slave 0 Security ID Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_bmx_s0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcBmxS0Spec;
impl crate::RegisterSpec for TzcBmxS0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_s0::R`](R) reader structure"]
impl crate::Readable for TzcBmxS0Spec {}
#[doc = "`reset()` method sets TZC_BMX_S0 to value 0"]
impl crate::Resettable for TzcBmxS0Spec {}
