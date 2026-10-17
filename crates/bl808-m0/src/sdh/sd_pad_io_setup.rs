#[doc = "Register `SD_PAD_IO_SETUP` reader"]
pub type R = crate::R<SdPadIoSetupSpec>;
#[doc = "Register `SD_PAD_IO_SETUP` writer"]
pub type W = crate::W<SdPadIoSetupSpec>;
#[doc = "Field `ASYNC_IO_EN` reader - "]
pub type AsyncIoEnR = crate::BitReader;
#[doc = "Field `ASYNC_IO_EN` writer - "]
pub type AsyncIoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INAND_SEL` reader - "]
pub type InandSelR = crate::BitReader;
#[doc = "Field `INAND_SEL` writer - "]
pub type InandSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECO_REG` reader - "]
pub type EcoRegR = crate::FieldReader;
#[doc = "Field `ECO_REG` writer - "]
pub type EcoRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn async_io_en(&self) -> AsyncIoEnR {
        AsyncIoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn inand_sel(&self) -> InandSelR {
        InandSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn eco_reg(&self) -> EcoRegR {
        EcoRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn async_io_en(&mut self) -> AsyncIoEnW<'_, SdPadIoSetupSpec> {
        AsyncIoEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn inand_sel(&mut self) -> InandSelW<'_, SdPadIoSetupSpec> {
        InandSelW::new(self, 1)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn eco_reg(&mut self) -> EcoRegW<'_, SdPadIoSetupSpec> {
        EcoRegW::new(self, 16)
    }
}
#[doc = "Vendor specific IO pad and ECO register configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_pad_io_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_pad_io_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdPadIoSetupSpec;
impl crate::RegisterSpec for SdPadIoSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_pad_io_setup::R`](R) reader structure"]
impl crate::Readable for SdPadIoSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_pad_io_setup::W`](W) writer structure"]
impl crate::Writable for SdPadIoSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_PAD_IO_SETUP to value 0"]
impl crate::Resettable for SdPadIoSetupSpec {}
