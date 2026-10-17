#[doc = "Register `DPHY_CONFIG_8` reader"]
pub type R = crate::R<DphyConfig8Spec>;
#[doc = "Register `DPHY_CONFIG_8` writer"]
pub type W = crate::W<DphyConfig8Spec>;
#[doc = "Field `reg_ana_lptxp_data` reader - "]
pub type RegAnaLptxpDataR = crate::FieldReader;
#[doc = "Field `reg_ana_lptxp_data` writer - "]
pub type RegAnaLptxpDataW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_ana_lptxn_data` reader - "]
pub type RegAnaLptxnDataR = crate::FieldReader;
#[doc = "Field `reg_ana_lptxn_data` writer - "]
pub type RegAnaLptxnDataW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_ana_lptxen` reader - "]
pub type RegAnaLptxenR = crate::FieldReader;
#[doc = "Field `reg_ana_lptxen` writer - "]
pub type RegAnaLptxenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_ana_lprxen` reader - "]
pub type RegAnaLprxenR = crate::FieldReader;
#[doc = "Field `reg_ana_lprxen` writer - "]
pub type RegAnaLprxenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_ana_hstxen` reader - "]
pub type RegAnaHstxenR = crate::FieldReader;
#[doc = "Field `reg_ana_hstxen` writer - "]
pub type RegAnaHstxenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_ana_hs_trail_byte` reader - "]
pub type RegAnaHsTrailByteR = crate::FieldReader;
#[doc = "Field `reg_ana_hs_trail_byte` writer - "]
pub type RegAnaHsTrailByteW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_ana_hs_sync_ld_byte` reader - "]
pub type RegAnaHsSyncLdByteR = crate::FieldReader;
#[doc = "Field `reg_ana_hs_sync_ld_byte` writer - "]
pub type RegAnaHsSyncLdByteW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_ana_hs_p2s_sel_byte` reader - "]
pub type RegAnaHsP2sSelByteR = crate::FieldReader;
#[doc = "Field `reg_ana_hs_p2s_sel_byte` writer - "]
pub type RegAnaHsP2sSelByteW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_ana_lptxp_data(&self) -> RegAnaLptxpDataR {
        RegAnaLptxpDataR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_ana_lptxn_data(&self) -> RegAnaLptxnDataR {
        RegAnaLptxnDataR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_ana_lptxen(&self) -> RegAnaLptxenR {
        RegAnaLptxenR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reg_ana_lprxen(&self) -> RegAnaLprxenR {
        RegAnaLprxenR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reg_ana_hstxen(&self) -> RegAnaHstxenR {
        RegAnaHstxenR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_ana_hs_trail_byte(&self) -> RegAnaHsTrailByteR {
        RegAnaHsTrailByteR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_ana_hs_sync_ld_byte(&self) -> RegAnaHsSyncLdByteR {
        RegAnaHsSyncLdByteR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reg_ana_hs_p2s_sel_byte(&self) -> RegAnaHsP2sSelByteR {
        RegAnaHsP2sSelByteR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_ana_lptxp_data(&mut self) -> RegAnaLptxpDataW<'_, DphyConfig8Spec> {
        RegAnaLptxpDataW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_ana_lptxn_data(&mut self) -> RegAnaLptxnDataW<'_, DphyConfig8Spec> {
        RegAnaLptxnDataW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn reg_ana_lptxen(&mut self) -> RegAnaLptxenW<'_, DphyConfig8Spec> {
        RegAnaLptxenW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn reg_ana_lprxen(&mut self) -> RegAnaLprxenW<'_, DphyConfig8Spec> {
        RegAnaLprxenW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn reg_ana_hstxen(&mut self) -> RegAnaHstxenW<'_, DphyConfig8Spec> {
        RegAnaHstxenW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_ana_hs_trail_byte(&mut self) -> RegAnaHsTrailByteW<'_, DphyConfig8Spec> {
        RegAnaHsTrailByteW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reg_ana_hs_sync_ld_byte(&mut self) -> RegAnaHsSyncLdByteW<'_, DphyConfig8Spec> {
        RegAnaHsSyncLdByteW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reg_ana_hs_p2s_sel_byte(&mut self) -> RegAnaHsP2sSelByteW<'_, DphyConfig8Spec> {
        RegAnaHsP2sSelByteW::new(self, 28)
    }
}
#[doc = "DPHY Analog TX/RX Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_config_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_config_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyConfig8Spec;
impl crate::RegisterSpec for DphyConfig8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_config_8::R`](R) reader structure"]
impl crate::Readable for DphyConfig8Spec {}
#[doc = "`write(|w| ..)` method takes [`dphy_config_8::W`](W) writer structure"]
impl crate::Writable for DphyConfig8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_CONFIG_8 to value 0"]
impl crate::Resettable for DphyConfig8Spec {}
