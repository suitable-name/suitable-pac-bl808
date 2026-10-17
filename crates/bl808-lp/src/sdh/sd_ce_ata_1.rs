#[doc = "Register `SD_CE_ATA_1` reader"]
pub type R = crate::R<SdCeAta1Spec>;
#[doc = "Register `SD_CE_ATA_1` writer"]
pub type W = crate::W<SdCeAta1Spec>;
#[doc = "Field `CPL_TIMEOUT` reader - "]
pub type CplTimeoutR = crate::FieldReader<u16>;
#[doc = "Field `CPL_TIMEOUT` writer - "]
pub type CplTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn cpl_timeout(&self) -> CplTimeoutR {
        CplTimeoutR::new(self.bits & 0x3fff)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn cpl_timeout(&mut self) -> CplTimeoutW<'_, SdCeAta1Spec> {
        CplTimeoutW::new(self, 0)
    }
}
#[doc = "CE-ATA specific timeout configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_ce_ata_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_ce_ata_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdCeAta1Spec;
impl crate::RegisterSpec for SdCeAta1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_ce_ata_1::R`](R) reader structure"]
impl crate::Readable for SdCeAta1Spec {}
#[doc = "`write(|w| ..)` method takes [`sd_ce_ata_1::W`](W) writer structure"]
impl crate::Writable for SdCeAta1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_CE_ATA_1 to value 0"]
impl crate::Resettable for SdCeAta1Spec {}
