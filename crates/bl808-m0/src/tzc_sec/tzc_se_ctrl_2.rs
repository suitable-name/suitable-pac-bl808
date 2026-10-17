#[doc = "Register `TZC_SE_CTRL_2` reader"]
pub type R = crate::R<TzcSeCtrl2Spec>;
#[doc = "Register `TZC_SE_CTRL_2` writer"]
pub type W = crate::W<TzcSeCtrl2Spec>;
#[doc = "Field `tzc_se_sha_tzsid_lock` reader - SHA TZSID lock"]
pub type TzcSeShaTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_sha_tzsid_lock` writer - SHA TZSID lock"]
pub type TzcSeShaTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_aes_tzsid_lock` reader - AES TZSID lock"]
pub type TzcSeAesTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_aes_tzsid_lock` writer - AES TZSID lock"]
pub type TzcSeAesTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_trng_tzsid_lock` reader - TRNG TZSID lock"]
pub type TzcSeTrngTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_trng_tzsid_lock` writer - TRNG TZSID lock"]
pub type TzcSeTrngTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_pka_tzsid_lock` reader - PKA TZSID lock"]
pub type TzcSePkaTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_pka_tzsid_lock` writer - PKA TZSID lock"]
pub type TzcSePkaTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_cdet_tzsid_lock` reader - CDET TZSID lock"]
pub type TzcSeCdetTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_cdet_tzsid_lock` writer - CDET TZSID lock"]
pub type TzcSeCdetTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_gmac_tzsid_lock` reader - GMAC TZSID lock"]
pub type TzcSeGmacTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_se_gmac_tzsid_lock` writer - GMAC TZSID lock"]
pub type TzcSeGmacTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_tzsid_crmd_lock` reader - Security Engine TZID Command Mode lock"]
pub type TzcSeTzsidCrmdLockR = crate::BitReader;
#[doc = "Field `tzc_se_tzsid_crmd_lock` writer - Security Engine TZID Command Mode lock"]
pub type TzcSeTzsidCrmdLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_cr_tzsid_lock` reader - SF Control Register TZSID lock"]
pub type TzcSfCrTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_sf_cr_tzsid_lock` writer - SF Control Register TZSID lock"]
pub type TzcSfCrTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_sec_tzsid_lock` reader - SF Security Register TZSID lock"]
pub type TzcSfSecTzsidLockR = crate::BitReader;
#[doc = "Field `tzc_sf_sec_tzsid_lock` writer - SF Security Register TZSID lock"]
pub type TzcSfSecTzsidLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsid_crmd_lock` reader - SF TZID Command Mode lock"]
pub type TzcSfTzsidCrmdLockR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsid_crmd_lock` writer - SF TZID Command Mode lock"]
pub type TzcSfTzsidCrmdLockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SHA TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_sha_tzsid_lock(&self) -> TzcSeShaTzsidLockR {
        TzcSeShaTzsidLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_aes_tzsid_lock(&self) -> TzcSeAesTzsidLockR {
        TzcSeAesTzsidLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TRNG TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_trng_tzsid_lock(&self) -> TzcSeTrngTzsidLockR {
        TzcSeTrngTzsidLockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PKA TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_pka_tzsid_lock(&self) -> TzcSePkaTzsidLockR {
        TzcSePkaTzsidLockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CDET TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_cdet_tzsid_lock(&self) -> TzcSeCdetTzsidLockR {
        TzcSeCdetTzsidLockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GMAC TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_gmac_tzsid_lock(&self) -> TzcSeGmacTzsidLockR {
        TzcSeGmacTzsidLockR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Security Engine TZID Command Mode lock"]
    #[inline(always)]
    pub fn tzc_se_tzsid_crmd_lock(&self) -> TzcSeTzsidCrmdLockR {
        TzcSeTzsidCrmdLockR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - SF Control Register TZSID lock"]
    #[inline(always)]
    pub fn tzc_sf_cr_tzsid_lock(&self) -> TzcSfCrTzsidLockR {
        TzcSfCrTzsidLockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SF Security Register TZSID lock"]
    #[inline(always)]
    pub fn tzc_sf_sec_tzsid_lock(&self) -> TzcSfSecTzsidLockR {
        TzcSfSecTzsidLockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SF TZID Command Mode lock"]
    #[inline(always)]
    pub fn tzc_sf_tzsid_crmd_lock(&self) -> TzcSfTzsidCrmdLockR {
        TzcSfTzsidCrmdLockR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHA TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_sha_tzsid_lock(&mut self) -> TzcSeShaTzsidLockW<'_, TzcSeCtrl2Spec> {
        TzcSeShaTzsidLockW::new(self, 0)
    }
    #[doc = "Bit 1 - AES TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_aes_tzsid_lock(&mut self) -> TzcSeAesTzsidLockW<'_, TzcSeCtrl2Spec> {
        TzcSeAesTzsidLockW::new(self, 1)
    }
    #[doc = "Bit 2 - TRNG TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_trng_tzsid_lock(&mut self) -> TzcSeTrngTzsidLockW<'_, TzcSeCtrl2Spec> {
        TzcSeTrngTzsidLockW::new(self, 2)
    }
    #[doc = "Bit 3 - PKA TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_pka_tzsid_lock(&mut self) -> TzcSePkaTzsidLockW<'_, TzcSeCtrl2Spec> {
        TzcSePkaTzsidLockW::new(self, 3)
    }
    #[doc = "Bit 4 - CDET TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_cdet_tzsid_lock(&mut self) -> TzcSeCdetTzsidLockW<'_, TzcSeCtrl2Spec> {
        TzcSeCdetTzsidLockW::new(self, 4)
    }
    #[doc = "Bit 5 - GMAC TZSID lock"]
    #[inline(always)]
    pub fn tzc_se_gmac_tzsid_lock(&mut self) -> TzcSeGmacTzsidLockW<'_, TzcSeCtrl2Spec> {
        TzcSeGmacTzsidLockW::new(self, 5)
    }
    #[doc = "Bit 6 - Security Engine TZID Command Mode lock"]
    #[inline(always)]
    pub fn tzc_se_tzsid_crmd_lock(&mut self) -> TzcSeTzsidCrmdLockW<'_, TzcSeCtrl2Spec> {
        TzcSeTzsidCrmdLockW::new(self, 6)
    }
    #[doc = "Bit 16 - SF Control Register TZSID lock"]
    #[inline(always)]
    pub fn tzc_sf_cr_tzsid_lock(&mut self) -> TzcSfCrTzsidLockW<'_, TzcSeCtrl2Spec> {
        TzcSfCrTzsidLockW::new(self, 16)
    }
    #[doc = "Bit 17 - SF Security Register TZSID lock"]
    #[inline(always)]
    pub fn tzc_sf_sec_tzsid_lock(&mut self) -> TzcSfSecTzsidLockW<'_, TzcSeCtrl2Spec> {
        TzcSfSecTzsidLockW::new(self, 17)
    }
    #[doc = "Bit 18 - SF TZID Command Mode lock"]
    #[inline(always)]
    pub fn tzc_sf_tzsid_crmd_lock(&mut self) -> TzcSfTzsidCrmdLockW<'_, TzcSeCtrl2Spec> {
        TzcSfTzsidCrmdLockW::new(self, 18)
    }
}
#[doc = "Security Engine TrustZone Control Register 2 (Lock)\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_se_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSeCtrl2Spec;
impl crate::RegisterSpec for TzcSeCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_se_ctrl_2::R`](R) reader structure"]
impl crate::Readable for TzcSeCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_se_ctrl_2::W`](W) writer structure"]
impl crate::Writable for TzcSeCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_SE_CTRL_2 to value 0"]
impl crate::Resettable for TzcSeCtrl2Spec {}
