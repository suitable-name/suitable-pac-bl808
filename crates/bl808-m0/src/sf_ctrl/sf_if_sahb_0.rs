#[doc = "Register `SF_IF_SAHB_0` reader"]
pub type R = crate::R<SfIfSahb0Spec>;
#[doc = "Register `SF_IF_SAHB_0` writer"]
pub type W = crate::W<SfIfSahb0Spec>;
#[doc = "Field `sf_if_busy` reader - "]
pub type SfIfBusyR = crate::BitReader;
#[doc = "Field `sf_if_0_trig` reader - "]
pub type SfIf0TrigR = crate::BitReader;
#[doc = "Field `sf_if_0_trig` writer - "]
pub type SfIf0TrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_0_dat_byte` reader - "]
pub type SfIf0DatByteR = crate::FieldReader<u16>;
#[doc = "Field `sf_if_0_dat_byte` writer - "]
pub type SfIf0DatByteW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `sf_if_0_dmy_byte` reader - "]
pub type SfIf0DmyByteR = crate::FieldReader;
#[doc = "Field `sf_if_0_dmy_byte` writer - "]
pub type SfIf0DmyByteW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `sf_if_0_adr_byte` reader - "]
pub type SfIf0AdrByteR = crate::FieldReader;
#[doc = "Field `sf_if_0_adr_byte` writer - "]
pub type SfIf0AdrByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_0_cmd_byte` reader - "]
pub type SfIf0CmdByteR = crate::FieldReader;
#[doc = "Field `sf_if_0_cmd_byte` writer - "]
pub type SfIf0CmdByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_0_dat_rw` reader - "]
pub type SfIf0DatRwR = crate::BitReader;
#[doc = "Field `sf_if_0_dat_rw` writer - "]
pub type SfIf0DatRwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_0_dat_en` reader - "]
pub type SfIf0DatEnR = crate::BitReader;
#[doc = "Field `sf_if_0_dat_en` writer - "]
pub type SfIf0DatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_0_dmy_en` reader - "]
pub type SfIf0DmyEnR = crate::BitReader;
#[doc = "Field `sf_if_0_dmy_en` writer - "]
pub type SfIf0DmyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_0_adr_en` reader - "]
pub type SfIf0AdrEnR = crate::BitReader;
#[doc = "Field `sf_if_0_adr_en` writer - "]
pub type SfIf0AdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_0_cmd_en` reader - "]
pub type SfIf0CmdEnR = crate::BitReader;
#[doc = "Field `sf_if_0_cmd_en` writer - "]
pub type SfIf0CmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_0_spi_mode` reader - "]
pub type SfIf0SpiModeR = crate::FieldReader;
#[doc = "Field `sf_if_0_spi_mode` writer - "]
pub type SfIf0SpiModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if_0_qpi_mode_en` reader - "]
pub type SfIf0QpiModeEnR = crate::BitReader;
#[doc = "Field `sf_if_0_qpi_mode_en` writer - "]
pub type SfIf0QpiModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_if_busy(&self) -> SfIfBusyR {
        SfIfBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_if_0_trig(&self) -> SfIf0TrigR {
        SfIf0TrigR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn sf_if_0_dat_byte(&self) -> SfIf0DatByteR {
        SfIf0DatByteR::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_0_dmy_byte(&self) -> SfIf0DmyByteR {
        SfIf0DmyByteR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_0_adr_byte(&self) -> SfIf0AdrByteR {
        SfIf0AdrByteR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_cmd_byte(&self) -> SfIf0CmdByteR {
        SfIf0CmdByteR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_0_dat_rw(&self) -> SfIf0DatRwR {
        SfIf0DatRwR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_0_dat_en(&self) -> SfIf0DatEnR {
        SfIf0DatEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_0_dmy_en(&self) -> SfIf0DmyEnR {
        SfIf0DmyEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_0_adr_en(&self) -> SfIf0AdrEnR {
        SfIf0AdrEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_0_cmd_en(&self) -> SfIf0CmdEnR {
        SfIf0CmdEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_0_spi_mode(&self) -> SfIf0SpiModeR {
        SfIf0SpiModeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_0_qpi_mode_en(&self) -> SfIf0QpiModeEnR {
        SfIf0QpiModeEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_if_0_trig(&mut self) -> SfIf0TrigW<'_, SfIfSahb0Spec> {
        SfIf0TrigW::new(self, 1)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn sf_if_0_dat_byte(&mut self) -> SfIf0DatByteW<'_, SfIfSahb0Spec> {
        SfIf0DatByteW::new(self, 2)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_0_dmy_byte(&mut self) -> SfIf0DmyByteW<'_, SfIfSahb0Spec> {
        SfIf0DmyByteW::new(self, 12)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_0_adr_byte(&mut self) -> SfIf0AdrByteW<'_, SfIfSahb0Spec> {
        SfIf0AdrByteW::new(self, 17)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_cmd_byte(&mut self) -> SfIf0CmdByteW<'_, SfIfSahb0Spec> {
        SfIf0CmdByteW::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_0_dat_rw(&mut self) -> SfIf0DatRwW<'_, SfIfSahb0Spec> {
        SfIf0DatRwW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_0_dat_en(&mut self) -> SfIf0DatEnW<'_, SfIfSahb0Spec> {
        SfIf0DatEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_0_dmy_en(&mut self) -> SfIf0DmyEnW<'_, SfIfSahb0Spec> {
        SfIf0DmyEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_0_adr_en(&mut self) -> SfIf0AdrEnW<'_, SfIfSahb0Spec> {
        SfIf0AdrEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_0_cmd_en(&mut self) -> SfIf0CmdEnW<'_, SfIfSahb0Spec> {
        SfIf0CmdEnW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_0_spi_mode(&mut self) -> SfIf0SpiModeW<'_, SfIfSahb0Spec> {
        SfIf0SpiModeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_0_qpi_mode_en(&mut self) -> SfIf0QpiModeEnW<'_, SfIfSahb0Spec> {
        SfIf0QpiModeEnW::new(self, 31)
    }
}
#[doc = "System AHB Interface Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_sahb_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_sahb_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfSahb0Spec;
impl crate::RegisterSpec for SfIfSahb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_sahb_0::R`](R) reader structure"]
impl crate::Readable for SfIfSahb0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_sahb_0::W`](W) writer structure"]
impl crate::Writable for SfIfSahb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_SAHB_0 to value 0"]
impl crate::Resettable for SfIfSahb0Spec {}
