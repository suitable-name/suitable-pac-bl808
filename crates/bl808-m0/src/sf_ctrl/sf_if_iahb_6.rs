#[doc = "Register `SF_IF_IAHB_6` reader"]
pub type R = crate::R<SfIfIahb6Spec>;
#[doc = "Register `SF_IF_IAHB_6` writer"]
pub type W = crate::W<SfIfIahb6Spec>;
#[doc = "Field `sf_if_3_adr_byte` reader - "]
pub type SfIf3AdrByteR = crate::FieldReader;
#[doc = "Field `sf_if_3_adr_byte` writer - "]
pub type SfIf3AdrByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_3_cmd_byte` reader - "]
pub type SfIf3CmdByteR = crate::FieldReader;
#[doc = "Field `sf_if_3_cmd_byte` writer - "]
pub type SfIf3CmdByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_3_adr_en` reader - "]
pub type SfIf3AdrEnR = crate::BitReader;
#[doc = "Field `sf_if_3_adr_en` writer - "]
pub type SfIf3AdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_3_cmd_en` reader - "]
pub type SfIf3CmdEnR = crate::BitReader;
#[doc = "Field `sf_if_3_cmd_en` writer - "]
pub type SfIf3CmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_3_spi_mode` reader - "]
pub type SfIf3SpiModeR = crate::FieldReader;
#[doc = "Field `sf_if_3_spi_mode` writer - "]
pub type SfIf3SpiModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_3_qpi_mode_en` reader - "]
pub type SfIf3QpiModeEnR = crate::BitReader;
#[doc = "Field `sf_if_3_qpi_mode_en` writer - "]
pub type SfIf3QpiModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_3_adr_byte(&self) -> SfIf3AdrByteR {
        SfIf3AdrByteR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_3_cmd_byte(&self) -> SfIf3CmdByteR {
        SfIf3CmdByteR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_3_adr_en(&self) -> SfIf3AdrEnR {
        SfIf3AdrEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_3_cmd_en(&self) -> SfIf3CmdEnR {
        SfIf3CmdEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_3_spi_mode(&self) -> SfIf3SpiModeR {
        SfIf3SpiModeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_3_qpi_mode_en(&self) -> SfIf3QpiModeEnR {
        SfIf3QpiModeEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_3_adr_byte(&mut self) -> SfIf3AdrByteW<'_, SfIfIahb6Spec> {
        SfIf3AdrByteW::new(self, 17)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_3_cmd_byte(&mut self) -> SfIf3CmdByteW<'_, SfIfIahb6Spec> {
        SfIf3CmdByteW::new(self, 20)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_3_adr_en(&mut self) -> SfIf3AdrEnW<'_, SfIfIahb6Spec> {
        SfIf3AdrEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_3_cmd_en(&mut self) -> SfIf3CmdEnW<'_, SfIfIahb6Spec> {
        SfIf3CmdEnW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_3_spi_mode(&mut self) -> SfIf3SpiModeW<'_, SfIfIahb6Spec> {
        SfIf3SpiModeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_3_qpi_mode_en(&mut self) -> SfIf3QpiModeEnW<'_, SfIfIahb6Spec> {
        SfIf3QpiModeEnW::new(self, 31)
    }
}
#[doc = "Instruction AHB Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_iahb_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_iahb_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIahb6Spec;
impl crate::RegisterSpec for SfIfIahb6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_iahb_6::R`](R) reader structure"]
impl crate::Readable for SfIfIahb6Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_iahb_6::W`](W) writer structure"]
impl crate::Writable for SfIfIahb6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IAHB_6 to value 0"]
impl crate::Resettable for SfIfIahb6Spec {}
