#[doc = "Register `SD_BLOCK_SIZE` reader"]
pub type R = crate::R<SdBlockSizeSpec>;
#[doc = "Register `SD_BLOCK_SIZE` writer"]
pub type W = crate::W<SdBlockSizeSpec>;
#[doc = "Field `BLOCK_SIZE` reader - Transfer block size (in bytes)"]
pub type BlockSizeR = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_SIZE` writer - Transfer block size (in bytes)"]
pub type BlockSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HOST_DMA_BDRY` reader - Host DMA buffer boundary (2^N bytes, 0=4k, 7=128M)"]
pub type HostDmaBdryR = crate::FieldReader;
#[doc = "Field `HOST_DMA_BDRY` writer - Host DMA buffer boundary (2^N bytes, 0=4k, 7=128M)"]
pub type HostDmaBdryW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - Transfer block size (in bytes)"]
    #[inline(always)]
    pub fn block_size(&self) -> BlockSizeR {
        BlockSizeR::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:14 - Host DMA buffer boundary (2^N bytes, 0=4k, 7=128M)"]
    #[inline(always)]
    pub fn host_dma_bdry(&self) -> HostDmaBdryR {
        HostDmaBdryR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer block size (in bytes)"]
    #[inline(always)]
    pub fn block_size(&mut self) -> BlockSizeW<'_, SdBlockSizeSpec> {
        BlockSizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Host DMA buffer boundary (2^N bytes, 0=4k, 7=128M)"]
    #[inline(always)]
    pub fn host_dma_bdry(&mut self) -> HostDmaBdryW<'_, SdBlockSizeSpec> {
        HostDmaBdryW::new(self, 12)
    }
}
#[doc = "Block Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_block_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_block_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdBlockSizeSpec;
impl crate::RegisterSpec for SdBlockSizeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_block_size::R`](R) reader structure"]
impl crate::Readable for SdBlockSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_block_size::W`](W) writer structure"]
impl crate::Writable for SdBlockSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_BLOCK_SIZE to value 0"]
impl crate::Resettable for SdBlockSizeSpec {}
