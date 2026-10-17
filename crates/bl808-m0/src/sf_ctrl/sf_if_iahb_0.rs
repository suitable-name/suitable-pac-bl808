#[doc = "Register `SF_IF_IAHB_0` reader"]
pub type R = crate::R<SfIfIahb0Spec>;
#[doc = "Register `SF_IF_IAHB_0` writer"]
pub type W = crate::W<SfIfIahb0Spec>;
#[doc = "Field `sf_if_1_dmy_byte` reader - "]
pub type SfIf1DmyByteR = crate::FieldReader;
#[doc = "Field `sf_if_1_dmy_byte` writer - "]
pub type SfIf1DmyByteW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `sf_if_1_adr_byte` reader - "]
pub type SfIf1AdrByteR = crate::FieldReader;
#[doc = "Field `sf_if_1_adr_byte` writer - "]
pub type SfIf1AdrByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_1_cmd_byte` reader - "]
pub type SfIf1CmdByteR = crate::FieldReader;
#[doc = "Field `sf_if_1_cmd_byte` writer - "]
pub type SfIf1CmdByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_1_dat_rw` reader - "]
pub type SfIf1DatRwR = crate::BitReader;
#[doc = "Field `sf_if_1_dat_rw` writer - "]
pub type SfIf1DatRwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_1_dat_en` reader - "]
pub type SfIf1DatEnR = crate::BitReader;
#[doc = "Field `sf_if_1_dat_en` writer - "]
pub type SfIf1DatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_1_dmy_en` reader - "]
pub type SfIf1DmyEnR = crate::BitReader;
#[doc = "Field `sf_if_1_dmy_en` writer - "]
pub type SfIf1DmyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_1_adr_en` reader - "]
pub type SfIf1AdrEnR = crate::BitReader;
#[doc = "Field `sf_if_1_adr_en` writer - "]
pub type SfIf1AdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_1_cmd_en` reader - "]
pub type SfIf1CmdEnR = crate::BitReader;
#[doc = "Field `sf_if_1_cmd_en` writer - "]
pub type SfIf1CmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_1_spi_mode` reader - "]
pub type SfIf1SpiModeR = crate::FieldReader;
#[doc = "Field `sf_if_1_spi_mode` writer - "]
pub type SfIf1SpiModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_1_qpi_mode_en` reader - "]
pub type SfIf1QpiModeEnR = crate::BitReader;
#[doc = "Field `sf_if_1_qpi_mode_en` writer - "]
pub type SfIf1QpiModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_1_dmy_byte(&self) -> SfIf1DmyByteR {
        SfIf1DmyByteR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_1_adr_byte(&self) -> SfIf1AdrByteR {
        SfIf1AdrByteR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_1_cmd_byte(&self) -> SfIf1CmdByteR {
        SfIf1CmdByteR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_1_dat_rw(&self) -> SfIf1DatRwR {
        SfIf1DatRwR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_1_dat_en(&self) -> SfIf1DatEnR {
        SfIf1DatEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_1_dmy_en(&self) -> SfIf1DmyEnR {
        SfIf1DmyEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_1_adr_en(&self) -> SfIf1AdrEnR {
        SfIf1AdrEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_1_cmd_en(&self) -> SfIf1CmdEnR {
        SfIf1CmdEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_1_spi_mode(&self) -> SfIf1SpiModeR {
        SfIf1SpiModeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_1_qpi_mode_en(&self) -> SfIf1QpiModeEnR {
        SfIf1QpiModeEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_1_dmy_byte(&mut self) -> SfIf1DmyByteW<'_, SfIfIahb0Spec> {
        SfIf1DmyByteW::new(self, 12)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_1_adr_byte(&mut self) -> SfIf1AdrByteW<'_, SfIfIahb0Spec> {
        SfIf1AdrByteW::new(self, 17)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_1_cmd_byte(&mut self) -> SfIf1CmdByteW<'_, SfIfIahb0Spec> {
        SfIf1CmdByteW::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_1_dat_rw(&mut self) -> SfIf1DatRwW<'_, SfIfIahb0Spec> {
        SfIf1DatRwW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_1_dat_en(&mut self) -> SfIf1DatEnW<'_, SfIfIahb0Spec> {
        SfIf1DatEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_1_dmy_en(&mut self) -> SfIf1DmyEnW<'_, SfIfIahb0Spec> {
        SfIf1DmyEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_1_adr_en(&mut self) -> SfIf1AdrEnW<'_, SfIfIahb0Spec> {
        SfIf1AdrEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_1_cmd_en(&mut self) -> SfIf1CmdEnW<'_, SfIfIahb0Spec> {
        SfIf1CmdEnW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_1_spi_mode(&mut self) -> SfIf1SpiModeW<'_, SfIfIahb0Spec> {
        SfIf1SpiModeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_1_qpi_mode_en(&mut self) -> SfIf1QpiModeEnW<'_, SfIfIahb0Spec> {
        SfIf1QpiModeEnW::new(self, 31)
    }
}
#[doc = "Instruction AHB Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb0Spec;
impl crate::RegisterSpec for SfIfIahb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_0::R`](R) reader structure"]
impl crate::Readable for SfIfIahb0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_0::W`](W) writer structure"]
impl crate::Writable for SfIfIahb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IAHB_0 to value 0"]
impl crate::Resettable for SfIfIahb0Spec {}
