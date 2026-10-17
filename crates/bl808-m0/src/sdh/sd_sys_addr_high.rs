#[doc = "Register `SD_SYS_ADDR_HIGH` reader"]
pub type R = crate::R<SdSysAddrHighSpec>;
#[doc = "Register `SD_SYS_ADDR_HIGH` writer"]
pub type W = crate::W<SdSysAddrHighSpec>;
#[doc = "Field `DMA_ADDR_H` reader - "]
pub type DmaAddrHR = crate::FieldReader<u16>;
#[doc = "Field `DMA_ADDR_H` writer - "]
pub type DmaAddrHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dma_addr_h(&self) -> DmaAddrHR {
        DmaAddrHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dma_addr_h(&mut self) -> DmaAddrHW<'_, SdSysAddrHighSpec> {
        DmaAddrHW::new(self, 0)
    }
}
#[doc = "System Address High Register (16-bit part of SDMA/ADMA address)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_sys_addr_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_sys_addr_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdSysAddrHighSpec;
impl crate::RegisterSpec for SdSysAddrHighSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_sys_addr_high::R`](R) reader structure"]
impl crate::Readable for SdSysAddrHighSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_sys_addr_high::W`](W) writer structure"]
impl crate::Writable for SdSysAddrHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_SYS_ADDR_HIGH to value 0"]
impl crate::Resettable for SdSysAddrHighSpec {}
