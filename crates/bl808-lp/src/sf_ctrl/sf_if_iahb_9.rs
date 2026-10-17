#[doc = "Register `SF_IF_IAHB_9` reader"]
pub type R = crate::R<SfIfIahb9Spec>;
#[doc = "Register `SF_IF_IAHB_9` writer"]
pub type W = crate::W<SfIfIahb9Spec>;
#[doc = "Field `sf_if_4_dmy_byte` reader - "]
pub type SfIf4DmyByteR = crate::FieldReader;
#[doc = "Field `sf_if_4_dmy_byte` writer - "]
pub type SfIf4DmyByteW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `sf_if_4_adr_byte` reader - "]
pub type SfIf4AdrByteR = crate::FieldReader;
#[doc = "Field `sf_if_4_adr_byte` writer - "]
pub type SfIf4AdrByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_4_cmd_byte` reader - "]
pub type SfIf4CmdByteR = crate::FieldReader;
#[doc = "Field `sf_if_4_cmd_byte` writer - "]
pub type SfIf4CmdByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_4_dat_rw` reader - "]
pub type SfIf4DatRwR = crate::BitReader;
#[doc = "Field `sf_if_4_dat_rw` writer - "]
pub type SfIf4DatRwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_4_dat_en` reader - "]
pub type SfIf4DatEnR = crate::BitReader;
#[doc = "Field `sf_if_4_dat_en` writer - "]
pub type SfIf4DatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_4_dmy_en` reader - "]
pub type SfIf4DmyEnR = crate::BitReader;
#[doc = "Field `sf_if_4_dmy_en` writer - "]
pub type SfIf4DmyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_4_adr_en` reader - "]
pub type SfIf4AdrEnR = crate::BitReader;
#[doc = "Field `sf_if_4_adr_en` writer - "]
pub type SfIf4AdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_4_cmd_en` reader - "]
pub type SfIf4CmdEnR = crate::BitReader;
#[doc = "Field `sf_if_4_cmd_en` writer - "]
pub type SfIf4CmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_4_spi_mode` reader - "]
pub type SfIf4SpiModeR = crate::FieldReader;
#[doc = "Field `sf_if_4_spi_mode` writer - "]
pub type SfIf4SpiModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_4_qpi_mode_en` reader - "]
pub type SfIf4QpiModeEnR = crate::BitReader;
#[doc = "Field `sf_if_4_qpi_mode_en` writer - "]
pub type SfIf4QpiModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_4_dmy_byte(&self) -> SfIf4DmyByteR {
        SfIf4DmyByteR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_4_adr_byte(&self) -> SfIf4AdrByteR {
        SfIf4AdrByteR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_4_cmd_byte(&self) -> SfIf4CmdByteR {
        SfIf4CmdByteR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_4_dat_rw(&self) -> SfIf4DatRwR {
        SfIf4DatRwR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_4_dat_en(&self) -> SfIf4DatEnR {
        SfIf4DatEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_4_dmy_en(&self) -> SfIf4DmyEnR {
        SfIf4DmyEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_4_adr_en(&self) -> SfIf4AdrEnR {
        SfIf4AdrEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_4_cmd_en(&self) -> SfIf4CmdEnR {
        SfIf4CmdEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_4_spi_mode(&self) -> SfIf4SpiModeR {
        SfIf4SpiModeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_4_qpi_mode_en(&self) -> SfIf4QpiModeEnR {
        SfIf4QpiModeEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_4_dmy_byte(&mut self) -> SfIf4DmyByteW<'_, SfIfIahb9Spec> {
        SfIf4DmyByteW::new(self, 12)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_4_adr_byte(&mut self) -> SfIf4AdrByteW<'_, SfIfIahb9Spec> {
        SfIf4AdrByteW::new(self, 17)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_4_cmd_byte(&mut self) -> SfIf4CmdByteW<'_, SfIfIahb9Spec> {
        SfIf4CmdByteW::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_4_dat_rw(&mut self) -> SfIf4DatRwW<'_, SfIfIahb9Spec> {
        SfIf4DatRwW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_4_dat_en(&mut self) -> SfIf4DatEnW<'_, SfIfIahb9Spec> {
        SfIf4DatEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_4_dmy_en(&mut self) -> SfIf4DmyEnW<'_, SfIfIahb9Spec> {
        SfIf4DmyEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_4_adr_en(&mut self) -> SfIf4AdrEnW<'_, SfIfIahb9Spec> {
        SfIf4AdrEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_4_cmd_en(&mut self) -> SfIf4CmdEnW<'_, SfIfIahb9Spec> {
        SfIf4CmdEnW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_4_spi_mode(&mut self) -> SfIf4SpiModeW<'_, SfIfIahb9Spec> {
        SfIf4SpiModeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_4_qpi_mode_en(&mut self) -> SfIf4QpiModeEnW<'_, SfIfIahb9Spec> {
        SfIf4QpiModeEnW::new(self, 31)
    }
}
#[doc = "Instruction AHB Control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb9Spec;
impl crate::RegisterSpec for SfIfIahb9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_9::R`](R) reader structure"]
impl crate::Readable for SfIfIahb9Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_9::W`](W) writer structure"]
impl crate::Writable for SfIfIahb9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IAHB_9 to value 0"]
impl crate::Resettable for SfIfIahb9Spec {}
