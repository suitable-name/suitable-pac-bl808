#[doc = "Register `SD_CE_ATA_2` reader"]
pub type R = crate::R<SdCeAta2Spec>;
#[doc = "Register `SD_CE_ATA_2` writer"]
pub type W = crate::W<SdCeAta2Spec>;
#[doc = "Field `MISC_INT_INT_EN` reader - "]
pub type MiscIntIntEnR = crate::BitReader;
#[doc = "Field `MISC_INT_INT_EN` writer - "]
pub type MiscIntIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISC_INT_EN` reader - "]
pub type MiscIntEnR = crate::BitReader;
#[doc = "Field `MISC_INT_EN` writer - "]
pub type MiscIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISC_INT` reader - "]
pub type MiscIntR = crate::BitReader;
#[doc = "Field `MISC_INT` writer - "]
pub type MiscIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CPL_COMPLETE_INT_EN` reader - "]
pub type CplCompleteIntEnR = crate::BitReader;
#[doc = "Field `CPL_COMPLETE_INT_EN` writer - "]
pub type CplCompleteIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPL_COMPLETE_EN` reader - "]
pub type CplCompleteEnR = crate::BitReader;
#[doc = "Field `CPL_COMPLETE_EN` writer - "]
pub type CplCompleteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPL_COMPLETE` reader - "]
pub type CplCompleteR = crate::BitReader;
#[doc = "Field `CPL_COMPLETE` writer - "]
pub type CplCompleteW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MMC_RESETN` reader - "]
pub type MmcResetnR = crate::BitReader;
#[doc = "Field `MMC_RESETN` writer - "]
pub type MmcResetnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMC_CARD` reader - "]
pub type MmcCardR = crate::BitReader;
#[doc = "Field `MMC_CARD` writer - "]
pub type MmcCardW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEATA_CARD` reader - "]
pub type CeataCardR = crate::BitReader;
#[doc = "Field `CEATA_CARD` writer - "]
pub type CeataCardW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SND_CPL` reader - "]
pub type SndCplR = crate::BitReader;
#[doc = "Field `SND_CPL` writer - "]
pub type SndCplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHK_CPL` reader - "]
pub type ChkCplR = crate::BitReader;
#[doc = "Field `CHK_CPL` writer - "]
pub type ChkCplW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn misc_int_int_en(&self) -> MiscIntIntEnR {
        MiscIntIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn misc_int_en(&self) -> MiscIntEnR {
        MiscIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn misc_int(&self) -> MiscIntR {
        MiscIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cpl_complete_int_en(&self) -> CplCompleteIntEnR {
        CplCompleteIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cpl_complete_en(&self) -> CplCompleteEnR {
        CplCompleteEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cpl_complete(&self) -> CplCompleteR {
        CplCompleteR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mmc_resetn(&self) -> MmcResetnR {
        MmcResetnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mmc_card(&self) -> MmcCardR {
        MmcCardR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ceata_card(&self) -> CeataCardR {
        CeataCardR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn snd_cpl(&self) -> SndCplR {
        SndCplR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn chk_cpl(&self) -> ChkCplR {
        ChkCplR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn misc_int_int_en(&mut self) -> MiscIntIntEnW<'_, SdCeAta2Spec> {
        MiscIntIntEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn misc_int_en(&mut self) -> MiscIntEnW<'_, SdCeAta2Spec> {
        MiscIntEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn misc_int(&mut self) -> MiscIntW<'_, SdCeAta2Spec> {
        MiscIntW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cpl_complete_int_en(&mut self) -> CplCompleteIntEnW<'_, SdCeAta2Spec> {
        CplCompleteIntEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cpl_complete_en(&mut self) -> CplCompleteEnW<'_, SdCeAta2Spec> {
        CplCompleteEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cpl_complete(&mut self) -> CplCompleteW<'_, SdCeAta2Spec> {
        CplCompleteW::new(self, 6)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mmc_resetn(&mut self) -> MmcResetnW<'_, SdCeAta2Spec> {
        MmcResetnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mmc_card(&mut self) -> MmcCardW<'_, SdCeAta2Spec> {
        MmcCardW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ceata_card(&mut self) -> CeataCardW<'_, SdCeAta2Spec> {
        CeataCardW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn snd_cpl(&mut self) -> SndCplW<'_, SdCeAta2Spec> {
        SndCplW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn chk_cpl(&mut self) -> ChkCplW<'_, SdCeAta2Spec> {
        ChkCplW::new(self, 15)
    }
}
#[doc = "CE-ATA specific command completion and interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_ce_ata_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_ce_ata_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdCeAta2Spec;
impl crate::RegisterSpec for SdCeAta2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_ce_ata_2::R`](R) reader structure"]
impl crate::Readable for SdCeAta2Spec {}
#[doc = "`write(|w| ..)` method takes [`sd_ce_ata_2::W`](W) writer structure"]
impl crate::Writable for SdCeAta2Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0x44;
}
#[doc = "`reset()` method sets SD_CE_ATA_2 to value 0"]
impl crate::Resettable for SdCeAta2Spec {}
