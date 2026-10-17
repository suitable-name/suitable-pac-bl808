#[doc = "Register `DSI_LPDT_TX_CONFIG` reader"]
pub type R = crate::R<DsiLpdtTxConfigSpec>;
#[doc = "Register `DSI_LPDT_TX_CONFIG` writer"]
pub type W = crate::W<DsiLpdtTxConfigSpec>;
#[doc = "Field `cr_lpdt_pdlen` reader - "]
pub type CrLpdtPdlenR = crate::FieldReader;
#[doc = "Field `cr_lpdt_pdlen` writer - "]
pub type CrLpdtPdlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_lpdt_vc` reader - "]
pub type CrLpdtVcR = crate::FieldReader;
#[doc = "Field `cr_lpdt_vc` writer - "]
pub type CrLpdtVcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_lpdt_di` reader - "]
pub type CrLpdtDiR = crate::FieldReader;
#[doc = "Field `cr_lpdt_di` writer - "]
pub type CrLpdtDiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `cr_lpdt_word0` reader - "]
pub type CrLpdtWord0R = crate::FieldReader;
#[doc = "Field `cr_lpdt_word0` writer - "]
pub type CrLpdtWord0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_lpdt_word1` reader - "]
pub type CrLpdtWord1R = crate::FieldReader;
#[doc = "Field `cr_lpdt_word1` writer - "]
pub type CrLpdtWord1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_lpdt_pdlen(&self) -> CrLpdtPdlenR {
        CrLpdtPdlenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cr_lpdt_vc(&self) -> CrLpdtVcR {
        CrLpdtVcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn cr_lpdt_di(&self) -> CrLpdtDiR {
        CrLpdtDiR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_lpdt_word0(&self) -> CrLpdtWord0R {
        CrLpdtWord0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_lpdt_word1(&self) -> CrLpdtWord1R {
        CrLpdtWord1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_lpdt_pdlen(&mut self) -> CrLpdtPdlenW<'_, DsiLpdtTxConfigSpec> {
        CrLpdtPdlenW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cr_lpdt_vc(&mut self) -> CrLpdtVcW<'_, DsiLpdtTxConfigSpec> {
        CrLpdtVcW::new(self, 8)
    }
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn cr_lpdt_di(&mut self) -> CrLpdtDiW<'_, DsiLpdtTxConfigSpec> {
        CrLpdtDiW::new(self, 10)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_lpdt_word0(&mut self) -> CrLpdtWord0W<'_, DsiLpdtTxConfigSpec> {
        CrLpdtWord0W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_lpdt_word1(&mut self) -> CrLpdtWord1W<'_, DsiLpdtTxConfigSpec> {
        CrLpdtWord1W::new(self, 24)
    }
}
#[doc = "Low Power Data Transmission Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lpdt_tx_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lpdt_tx_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiLpdtTxConfigSpec;
impl crate::RegisterSpec for DsiLpdtTxConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lpdt_tx_config::R`](R) reader structure"]
impl crate::Readable for DsiLpdtTxConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_lpdt_tx_config::W`](W) writer structure"]
impl crate::Writable for DsiLpdtTxConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_LPDT_TX_CONFIG to value 0"]
impl crate::Resettable for DsiLpdtTxConfigSpec {}
