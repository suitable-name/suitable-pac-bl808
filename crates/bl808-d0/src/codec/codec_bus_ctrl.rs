#[doc = "Register `CODEC_BUS_CTRL` reader"]
pub type R = crate::R<CodecBusCtrlSpec>;
#[doc = "Register `CODEC_BUS_CTRL` writer"]
pub type W = crate::W<CodecBusCtrlSpec>;
#[doc = "Field `RG_PCLK_FORCE_ON` reader - Force peripheral clock on for codec subsystem components"]
pub type RgPclkForceOnR = crate::FieldReader<u16>;
#[doc = "Field `RG_PCLK_FORCE_ON` writer - Force peripheral clock on for codec subsystem components"]
pub type RgPclkForceOnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Force peripheral clock on for codec subsystem components"]
    #[inline(always)]
    pub fn rg_pclk_force_on(&self) -> RgPclkForceOnR {
        RgPclkForceOnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Force peripheral clock on for codec subsystem components"]
    #[inline(always)]
    pub fn rg_pclk_force_on(&mut self) -> RgPclkForceOnW<'_, CodecBusCtrlSpec> {
        RgPclkForceOnW::new(self, 0)
    }
}
#[doc = "Codec Bus Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_bus_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_bus_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecBusCtrlSpec;
impl crate::RegisterSpec for CodecBusCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_bus_ctrl::R`](R) reader structure"]
impl crate::Readable for CodecBusCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`codec_bus_ctrl::W`](W) writer structure"]
impl crate::Writable for CodecBusCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CODEC_BUS_CTRL to value 0xffff"]
impl crate::Resettable for CodecBusCtrlSpec {
    const RESET_VALUE: u32 = 0xffff;
}
