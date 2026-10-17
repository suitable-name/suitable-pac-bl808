#[doc = "Register `SF_IF_IAHB_3` reader"]
pub type R = crate::R<SfIfIahb3Spec>;
#[doc = "Register `SF_IF_IAHB_3` writer"]
pub type W = crate::W<SfIfIahb3Spec>;
#[doc = "Field `sf_if_2_dmy_byte` reader - "]
pub type SfIf2DmyByteR = crate::FieldReader;
#[doc = "Field `sf_if_2_dmy_byte` writer - "]
pub type SfIf2DmyByteW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `sf_if_2_adr_byte` reader - "]
pub type SfIf2AdrByteR = crate::FieldReader;
#[doc = "Field `sf_if_2_adr_byte` writer - "]
pub type SfIf2AdrByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_2_cmd_byte` reader - "]
pub type SfIf2CmdByteR = crate::FieldReader;
#[doc = "Field `sf_if_2_cmd_byte` writer - "]
pub type SfIf2CmdByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_2_dat_rw` reader - "]
pub type SfIf2DatRwR = crate::BitReader;
#[doc = "Field `sf_if_2_dat_rw` writer - "]
pub type SfIf2DatRwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_2_dat_en` reader - "]
pub type SfIf2DatEnR = crate::BitReader;
#[doc = "Field `sf_if_2_dat_en` writer - "]
pub type SfIf2DatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_2_dmy_en` reader - "]
pub type SfIf2DmyEnR = crate::BitReader;
#[doc = "Field `sf_if_2_dmy_en` writer - "]
pub type SfIf2DmyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_2_adr_en` reader - "]
pub type SfIf2AdrEnR = crate::BitReader;
#[doc = "Field `sf_if_2_adr_en` writer - "]
pub type SfIf2AdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_2_cmd_en` reader - "]
pub type SfIf2CmdEnR = crate::BitReader;
#[doc = "Field `sf_if_2_cmd_en` writer - "]
pub type SfIf2CmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_2_spi_mode` reader - "]
pub type SfIf2SpiModeR = crate::FieldReader;
#[doc = "Field `sf_if_2_spi_mode` writer - "]
pub type SfIf2SpiModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_2_qpi_mode_en` reader - "]
pub type SfIf2QpiModeEnR = crate::BitReader;
#[doc = "Field `sf_if_2_qpi_mode_en` writer - "]
pub type SfIf2QpiModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_2_dmy_byte(&self) -> SfIf2DmyByteR {
        SfIf2DmyByteR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_2_adr_byte(&self) -> SfIf2AdrByteR {
        SfIf2AdrByteR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_2_cmd_byte(&self) -> SfIf2CmdByteR {
        SfIf2CmdByteR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_2_dat_rw(&self) -> SfIf2DatRwR {
        SfIf2DatRwR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_2_dat_en(&self) -> SfIf2DatEnR {
        SfIf2DatEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_2_dmy_en(&self) -> SfIf2DmyEnR {
        SfIf2DmyEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_2_adr_en(&self) -> SfIf2AdrEnR {
        SfIf2AdrEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_2_cmd_en(&self) -> SfIf2CmdEnR {
        SfIf2CmdEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_2_spi_mode(&self) -> SfIf2SpiModeR {
        SfIf2SpiModeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_2_qpi_mode_en(&self) -> SfIf2QpiModeEnR {
        SfIf2QpiModeEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_2_dmy_byte(&mut self) -> SfIf2DmyByteW<'_, SfIfIahb3Spec> {
        SfIf2DmyByteW::new(self, 12)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_2_adr_byte(&mut self) -> SfIf2AdrByteW<'_, SfIfIahb3Spec> {
        SfIf2AdrByteW::new(self, 17)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_2_cmd_byte(&mut self) -> SfIf2CmdByteW<'_, SfIfIahb3Spec> {
        SfIf2CmdByteW::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_2_dat_rw(&mut self) -> SfIf2DatRwW<'_, SfIfIahb3Spec> {
        SfIf2DatRwW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_2_dat_en(&mut self) -> SfIf2DatEnW<'_, SfIfIahb3Spec> {
        SfIf2DatEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_2_dmy_en(&mut self) -> SfIf2DmyEnW<'_, SfIfIahb3Spec> {
        SfIf2DmyEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_2_adr_en(&mut self) -> SfIf2AdrEnW<'_, SfIfIahb3Spec> {
        SfIf2AdrEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_2_cmd_en(&mut self) -> SfIf2CmdEnW<'_, SfIfIahb3Spec> {
        SfIf2CmdEnW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_2_spi_mode(&mut self) -> SfIf2SpiModeW<'_, SfIfIahb3Spec> {
        SfIf2SpiModeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_2_qpi_mode_en(&mut self) -> SfIf2QpiModeEnW<'_, SfIfIahb3Spec> {
        SfIf2QpiModeEnW::new(self, 31)
    }
}
#[doc = "Instruction AHB Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb3Spec;
impl crate::RegisterSpec for SfIfIahb3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_3::R`](R) reader structure"]
impl crate::Readable for SfIfIahb3Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_3::W`](W) writer structure"]
impl crate::Writable for SfIfIahb3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IAHB_3 to value 0"]
impl crate::Resettable for SfIfIahb3Spec {}
