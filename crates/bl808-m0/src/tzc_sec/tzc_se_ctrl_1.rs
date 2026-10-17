#[doc = "Register `TZC_SE_CTRL_1` reader"]
pub type R = crate::R<TzcSeCtrl1Spec>;
#[doc = "Register `TZC_SE_CTRL_1` writer"]
pub type W = crate::W<TzcSeCtrl1Spec>;
#[doc = "Field `tzc_sf_cr_tzsid_en` reader - SF Control Register TZSID enable"]
pub type TzcSfCrTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_cr_tzsid_en` writer - SF Control Register TZSID enable"]
pub type TzcSfCrTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_sf_sec_tzsid_en` reader - SF Security Register TZSID enable"]
pub type TzcSfSecTzsidEnR = crate::FieldReader;
#[doc = "Field `tzc_sf_sec_tzsid_en` writer - SF Security Register TZSID enable"]
pub type TzcSfSecTzsidEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_sf_tzsid_crmd` reader - SF TZID Command Mode"]
pub type TzcSfTzsidCrmdR = crate::BitReader;
#[doc = "Field `tzc_sf_tzsid_crmd` writer - SF TZID Command Mode"]
pub type TzcSfTzsidCrmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SF Control Register TZSID enable"]
    #[inline(always)]
    pub fn tzc_sf_cr_tzsid_en(&self) -> TzcSfCrTzsidEnR {
        TzcSfCrTzsidEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SF Security Register TZSID enable"]
    #[inline(always)]
    pub fn tzc_sf_sec_tzsid_en(&self) -> TzcSfSecTzsidEnR {
        TzcSfSecTzsidEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SF TZID Command Mode"]
    #[inline(always)]
    pub fn tzc_sf_tzsid_crmd(&self) -> TzcSfTzsidCrmdR {
        TzcSfTzsidCrmdR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SF Control Register TZSID enable"]
    #[inline(always)]
    pub fn tzc_sf_cr_tzsid_en(&mut self) -> TzcSfCrTzsidEnW<'_, TzcSeCtrl1Spec> {
        TzcSfCrTzsidEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - SF Security Register TZSID enable"]
    #[inline(always)]
    pub fn tzc_sf_sec_tzsid_en(&mut self) -> TzcSfSecTzsidEnW<'_, TzcSeCtrl1Spec> {
        TzcSfSecTzsidEnW::new(self, 2)
    }
    #[doc = "Bit 4 - SF TZID Command Mode"]
    #[inline(always)]
    pub fn tzc_sf_tzsid_crmd(&mut self) -> TzcSfTzsidCrmdW<'_, TzcSeCtrl1Spec> {
        TzcSfTzsidCrmdW::new(self, 4)
    }
}
#[doc = "Security Engine TrustZone Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_se_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_se_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSeCtrl1Spec;
impl crate::RegisterSpec for TzcSeCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_se_ctrl_1::R`](R) reader structure"]
impl crate::Readable for TzcSeCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_se_ctrl_1::W`](W) writer structure"]
impl crate::Writable for TzcSeCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_SE_CTRL_1 to value 0x0f"]
impl crate::Resettable for TzcSeCtrl1Spec {
    const RESET_VALUE: u32 = 0x0f;
}
