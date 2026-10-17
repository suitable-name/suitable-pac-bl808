#[doc = "Register `SD_DBG_RESERVED` reader"]
pub type R = crate::R<SdDbgReservedSpec>;
#[doc = "Register `SD_DBG_RESERVED` writer"]
pub type W = crate::W<SdDbgReservedSpec>;
#[doc = "Field `sd_dbg_reserved` reader - "]
pub type SdDbgReservedR = crate::FieldReader<u32>;
#[doc = "Field `sd_dbg_reserved` writer - "]
pub type SdDbgReservedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_dbg_reserved(&self) -> SdDbgReservedR {
        SdDbgReservedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_dbg_reserved(&mut self) -> SdDbgReservedW<'_, SdDbgReservedSpec> {
        SdDbgReservedW::new(self, 0)
    }
}
#[doc = "Debug Reserved Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_dbg_reserved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_dbg_reserved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdDbgReservedSpec;
impl crate::RegisterSpec for SdDbgReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_dbg_reserved::R`](R) reader structure"]
impl crate::Readable for SdDbgReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_dbg_reserved::W`](W) writer structure"]
impl crate::Writable for SdDbgReservedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_DBG_RESERVED to value 0xffff"]
impl crate::Resettable for SdDbgReservedSpec {
    const RESET_VALUE: u32 = 0xffff;
}
