#[doc = "Register `SF_IF2_SAHB_0` reader"]
pub type R = crate::R<SfIf2Sahb0Spec>;
#[doc = "Register `SF_IF2_SAHB_0` writer"]
pub type W = crate::W<SfIf2Sahb0Spec>;
#[doc = "Field `sf_if2_busy` reader - "]
pub type SfIf2BusyR = crate::BitReader;
#[doc = "Field `sf_if2_0_trig` reader - "]
pub type SfIf2_0TrigR = crate::BitReader;
#[doc = "Field `sf_if2_0_trig` writer - "]
pub type SfIf2_0TrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_0_dat_byte` reader - "]
pub type SfIf2_0DatByteR = crate::FieldReader<u16>;
#[doc = "Field `sf_if2_0_dat_byte` writer - "]
pub type SfIf2_0DatByteW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `sf_if2_0_dmy_byte` reader - "]
pub type SfIf2_0DmyByteR = crate::FieldReader;
#[doc = "Field `sf_if2_0_dmy_byte` writer - "]
pub type SfIf2_0DmyByteW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `sf_if2_0_adr_byte` reader - "]
pub type SfIf2_0AdrByteR = crate::FieldReader;
#[doc = "Field `sf_if2_0_adr_byte` writer - "]
pub type SfIf2_0AdrByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if2_0_cmd_byte` reader - "]
pub type SfIf2_0CmdByteR = crate::FieldReader;
#[doc = "Field `sf_if2_0_cmd_byte` writer - "]
pub type SfIf2_0CmdByteW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if2_0_dat_rw` reader - "]
pub type SfIf2_0DatRwR = crate::BitReader;
#[doc = "Field `sf_if2_0_dat_rw` writer - "]
pub type SfIf2_0DatRwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_0_dat_en` reader - "]
pub type SfIf2_0DatEnR = crate::BitReader;
#[doc = "Field `sf_if2_0_dat_en` writer - "]
pub type SfIf2_0DatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_0_dmy_en` reader - "]
pub type SfIf2_0DmyEnR = crate::BitReader;
#[doc = "Field `sf_if2_0_dmy_en` writer - "]
pub type SfIf2_0DmyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_0_adr_en` reader - "]
pub type SfIf2_0AdrEnR = crate::BitReader;
#[doc = "Field `sf_if2_0_adr_en` writer - "]
pub type SfIf2_0AdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_0_cmd_en` reader - "]
pub type SfIf2_0CmdEnR = crate::BitReader;
#[doc = "Field `sf_if2_0_cmd_en` writer - "]
pub type SfIf2_0CmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if2_0_spi_mode` reader - "]
pub type SfIf2_0SpiModeR = crate::FieldReader;
#[doc = "Field `sf_if2_0_spi_mode` writer - "]
pub type SfIf2_0SpiModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_if2_0_qpi_mode_en` reader - "]
pub type SfIf2_0QpiModeEnR = crate::BitReader;
#[doc = "Field `sf_if2_0_qpi_mode_en` writer - "]
pub type SfIf2_0QpiModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_if2_busy(&self) -> SfIf2BusyR {
        SfIf2BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_if2_0_trig(&self) -> SfIf2_0TrigR {
        SfIf2_0TrigR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn sf_if2_0_dat_byte(&self) -> SfIf2_0DatByteR {
        SfIf2_0DatByteR::new(((self.bits >> 2) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if2_0_dmy_byte(&self) -> SfIf2_0DmyByteR {
        SfIf2_0DmyByteR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if2_0_adr_byte(&self) -> SfIf2_0AdrByteR {
        SfIf2_0AdrByteR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if2_0_cmd_byte(&self) -> SfIf2_0CmdByteR {
        SfIf2_0CmdByteR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if2_0_dat_rw(&self) -> SfIf2_0DatRwR {
        SfIf2_0DatRwR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if2_0_dat_en(&self) -> SfIf2_0DatEnR {
        SfIf2_0DatEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if2_0_dmy_en(&self) -> SfIf2_0DmyEnR {
        SfIf2_0DmyEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if2_0_adr_en(&self) -> SfIf2_0AdrEnR {
        SfIf2_0AdrEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if2_0_cmd_en(&self) -> SfIf2_0CmdEnR {
        SfIf2_0CmdEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if2_0_spi_mode(&self) -> SfIf2_0SpiModeR {
        SfIf2_0SpiModeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if2_0_qpi_mode_en(&self) -> SfIf2_0QpiModeEnR {
        SfIf2_0QpiModeEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_if2_0_trig(&mut self) -> SfIf2_0TrigW<'_, SfIf2Sahb0Spec> {
        SfIf2_0TrigW::new(self, 1)
    }
    #[doc = "Bits 2:11"]
    #[inline(always)]
    pub fn sf_if2_0_dat_byte(&mut self) -> SfIf2_0DatByteW<'_, SfIf2Sahb0Spec> {
        SfIf2_0DatByteW::new(self, 2)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if2_0_dmy_byte(&mut self) -> SfIf2_0DmyByteW<'_, SfIf2Sahb0Spec> {
        SfIf2_0DmyByteW::new(self, 12)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if2_0_adr_byte(&mut self) -> SfIf2_0AdrByteW<'_, SfIf2Sahb0Spec> {
        SfIf2_0AdrByteW::new(self, 17)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if2_0_cmd_byte(&mut self) -> SfIf2_0CmdByteW<'_, SfIf2Sahb0Spec> {
        SfIf2_0CmdByteW::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if2_0_dat_rw(&mut self) -> SfIf2_0DatRwW<'_, SfIf2Sahb0Spec> {
        SfIf2_0DatRwW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if2_0_dat_en(&mut self) -> SfIf2_0DatEnW<'_, SfIf2Sahb0Spec> {
        SfIf2_0DatEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if2_0_dmy_en(&mut self) -> SfIf2_0DmyEnW<'_, SfIf2Sahb0Spec> {
        SfIf2_0DmyEnW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if2_0_adr_en(&mut self) -> SfIf2_0AdrEnW<'_, SfIf2Sahb0Spec> {
        SfIf2_0AdrEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if2_0_cmd_en(&mut self) -> SfIf2_0CmdEnW<'_, SfIf2Sahb0Spec> {
        SfIf2_0CmdEnW::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if2_0_spi_mode(&mut self) -> SfIf2_0SpiModeW<'_, SfIf2Sahb0Spec> {
        SfIf2_0SpiModeW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if2_0_qpi_mode_en(&mut self) -> SfIf2_0QpiModeEnW<'_, SfIf2Sahb0Spec> {
        SfIf2_0QpiModeEnW::new(self, 31)
    }
}
#[doc = "SF2 System AHB Interface Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if2_sahb_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if2_sahb_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIf2Sahb0Spec;
impl crate::RegisterSpec for SfIf2Sahb0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if2_sahb_0::R`](R) reader structure"]
impl crate::Readable for SfIf2Sahb0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if2_sahb_0::W`](W) writer structure"]
impl crate::Writable for SfIf2Sahb0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF2_SAHB_0 to value 0"]
impl crate::Resettable for SfIf2Sahb0Spec {}
