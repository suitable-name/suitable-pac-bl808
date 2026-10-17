#[doc = "Register `TZC_SE_CTRL_0` reader"]
pub type R = crate::R<TzcSeCtrl0Spec>;
#[doc = "Register `TZC_SE_CTRL_0` writer"]
pub type W = crate::W<TzcSeCtrl0Spec>;
#[doc = "Field `tzc_se_sha_tzsid_en` reader - SHA TZSID enable"]
pub type TzcSeShaTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_sha_tzsid_en` writer - SHA TZSID enable"]
pub type TzcSeShaTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_aes_tzsid_en` reader - AES TZSID enable"]
pub type TzcSeAesTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_aes_tzsid_en` writer - AES TZSID enable"]
pub type TzcSeAesTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_trng_tzsid_en` reader - TRNG TZSID enable"]
pub type TzcSeTrngTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_trng_tzsid_en` writer - TRNG TZSID enable"]
pub type TzcSeTrngTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_pka_tzsid_en` reader - PKA TZSID enable"]
pub type TzcSePkaTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_pka_tzsid_en` writer - PKA TZSID enable"]
pub type TzcSePkaTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_cdet_tzsid_en` reader - CDET TZSID enable"]
pub type TzcSeCdetTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_cdet_tzsid_en` writer - CDET TZSID enable"]
pub type TzcSeCdetTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_gmac_tzsid_en` reader - GMAC TZSID enable"]
pub type TzcSeGmacTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_se_gmac_tzsid_en` writer - GMAC TZSID enable"]
pub type TzcSeGmacTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_tzsid_crmd` reader - Security Engine TZID Command Mode"]
pub type TzcSeTzsidCrmdR = crate::BitReader;
#[doc = "Field `tzc_se_tzsid_crmd` writer - Security Engine TZID Command Mode"]
pub type TzcSeTzsidCrmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_wdt_dly` reader - Security Engine Watchdog Delay Cycles"]
pub type TzcSeWdtDlyR = crate::FieldReader<u16>;
#[doc = "Field `tzc_se_wdt_dly` writer - Security Engine Watchdog Delay Cycles"]
pub type TzcSeWdtDlyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - SHA TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_sha_tzsid_en(&self) -> TzcSeShaTzsidEnR {
        TzcSeShaTzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - AES TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_aes_tzsid_en(&self) -> TzcSeAesTzsidEnR {
        TzcSeAesTzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TRNG TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_trng_tzsid_en(&self) -> TzcSeTrngTzsidEnR {
        TzcSeTrngTzsidEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PKA TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_pka_tzsid_en(&self) -> TzcSePkaTzsidEnR {
        TzcSePkaTzsidEnR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - CDET TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_cdet_tzsid_en(&self) -> TzcSeCdetTzsidEnR {
        TzcSeCdetTzsidEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GMAC TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_gmac_tzsid_en(&self) -> TzcSeGmacTzsidEnR {
        TzcSeGmacTzsidEnR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Security Engine TZID Command Mode"]
    #[inline(always)]
    pub fn tzc_se_tzsid_crmd(&self) -> TzcSeTzsidCrmdR {
        TzcSeTzsidCrmdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Security Engine Watchdog Delay Cycles"]
    #[inline(always)]
    pub fn tzc_se_wdt_dly(&self) -> TzcSeWdtDlyR {
        TzcSeWdtDlyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - SHA TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_sha_tzsid_en(&mut self) -> TzcSeShaTzsidEnW<'_, TzcSeCtrl0Spec> {
        TzcSeShaTzsidEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - AES TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_aes_tzsid_en(&mut self) -> TzcSeAesTzsidEnW<'_, TzcSeCtrl0Spec> {
        TzcSeAesTzsidEnW::new(self, 2)
    }
    #[doc = "Bits 4:5 - TRNG TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_trng_tzsid_en(&mut self) -> TzcSeTrngTzsidEnW<'_, TzcSeCtrl0Spec> {
        TzcSeTrngTzsidEnW::new(self, 4)
    }
    #[doc = "Bits 6:7 - PKA TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_pka_tzsid_en(&mut self) -> TzcSePkaTzsidEnW<'_, TzcSeCtrl0Spec> {
        TzcSePkaTzsidEnW::new(self, 6)
    }
    #[doc = "Bits 8:9 - CDET TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_cdet_tzsid_en(&mut self) -> TzcSeCdetTzsidEnW<'_, TzcSeCtrl0Spec> {
        TzcSeCdetTzsidEnW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GMAC TZSID enable"]
    #[inline(always)]
    pub fn tzc_se_gmac_tzsid_en(&mut self) -> TzcSeGmacTzsidEnW<'_, TzcSeCtrl0Spec> {
        TzcSeGmacTzsidEnW::new(self, 10)
    }
    #[doc = "Bit 12 - Security Engine TZID Command Mode"]
    #[inline(always)]
    pub fn tzc_se_tzsid_crmd(&mut self) -> TzcSeTzsidCrmdW<'_, TzcSeCtrl0Spec> {
        TzcSeTzsidCrmdW::new(self, 12)
    }
    #[doc = "Bits 16:31 - Security Engine Watchdog Delay Cycles"]
    #[inline(always)]
    pub fn tzc_se_wdt_dly(&mut self) -> TzcSeWdtDlyW<'_, TzcSeCtrl0Spec> {
        TzcSeWdtDlyW::new(self, 16)
    }
}
#[doc = "Security Engine TrustZone Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_se_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSeCtrl0Spec;
impl crate::RegisterSpec for TzcSeCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_se_ctrl_0::R`](R) reader structure"]
impl crate::Readable for TzcSeCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_se_ctrl_0::W`](W) writer structure"]
impl crate::Writable for TzcSeCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_SE_CTRL_0 to value 0x0005_0fff"]
impl crate::Resettable for TzcSeCtrl0Spec {
    const RESET_VALUE: u32 = 0x0005_0fff;
}
