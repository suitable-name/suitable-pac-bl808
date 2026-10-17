#[doc = "Register `SD_BLOCK_COUNT` reader"]
pub type R = crate::R<SdBlockCountSpec>;
#[doc = "Register `SD_BLOCK_COUNT` writer"]
pub type W = crate::W<SdBlockCountSpec>;
#[doc = "Field `BLOCK_COUNT` reader - Number of blocks to be transferred"]
pub type BlockCountR = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_COUNT` writer - Number of blocks to be transferred"]
pub type BlockCountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of blocks to be transferred"]
    #[inline(always)]
    pub fn block_count(&self) -> BlockCountR {
        BlockCountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of blocks to be transferred"]
    #[inline(always)]
    pub fn block_count(&mut self) -> BlockCountW<'_, SdBlockCountSpec> {
        BlockCountW::new(self, 0)
    }
}
#[doc = "Block Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_block_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_block_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdBlockCountSpec;
impl crate::RegisterSpec for SdBlockCountSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_block_count::R`](R) reader structure"]
impl crate::Readable for SdBlockCountSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_block_count::W`](W) writer structure"]
impl crate::Writable for SdBlockCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_BLOCK_COUNT to value 0"]
impl crate::Resettable for SdBlockCountSpec {}
