#[doc = "Register `SD_SYS_ADDR_LOW` reader"]
pub type R = crate::R<SdSysAddrLowSpec>;
#[doc = "Register `SD_SYS_ADDR_LOW` writer"]
pub type W = crate::W<SdSysAddrLowSpec>;
#[doc = "Field `DMA_ADDR_L` reader - "]
pub type DmaAddrLR = crate::FieldReader<u16>;
#[doc = "Field `DMA_ADDR_L` writer - "]
pub type DmaAddrLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dma_addr_l(&self) -> DmaAddrLR {
        DmaAddrLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dma_addr_l(&mut self) -> DmaAddrLW<'_, SdSysAddrLowSpec> {
        DmaAddrLW::new(self, 0)
    }
}
#[doc = "System Address Low Register (16-bit part of SDMA/ADMA address)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_sys_addr_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_sys_addr_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdSysAddrLowSpec;
impl crate::RegisterSpec for SdSysAddrLowSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_sys_addr_low::R`](R) reader structure"]
impl crate::Readable for SdSysAddrLowSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_sys_addr_low::W`](W) writer structure"]
impl crate::Writable for SdSysAddrLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_SYS_ADDR_LOW to value 0"]
impl crate::Resettable for SdSysAddrLowSpec {}
