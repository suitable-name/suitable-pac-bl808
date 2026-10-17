#[doc = "Register `JPEG_STORE_MEMORY` reader"]
pub type R = crate::R<JpegStoreMemorySpec>;
#[doc = "Register `JPEG_STORE_MEMORY` writer"]
pub type W = crate::W<JpegStoreMemorySpec>;
#[doc = "Field `reg_w_burst_cnt` reader - Write burst count value."]
pub type RegWBurstCntR = crate::FieldReader<u32>;
#[doc = "Field `reg_w_burst_cnt` writer - Write burst count value."]
pub type RegWBurstCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write burst count value."]
    #[inline(always)]
    pub fn reg_w_burst_cnt(&self) -> RegWBurstCntR {
        RegWBurstCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write burst count value."]
    #[inline(always)]
    pub fn reg_w_burst_cnt(&mut self) -> RegWBurstCntW<'_, JpegStoreMemorySpec> {
        RegWBurstCntW::new(self, 0)
    }
}
#[doc = "JPEG Output Burst Count Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`jpeg_store_memory::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jpeg_store_memory::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JpegStoreMemorySpec;
impl crate::RegisterSpec for JpegStoreMemorySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jpeg_store_memory::R`](R) reader structure"]
impl crate::Readable for JpegStoreMemorySpec {}
#[doc = "`write(|w| ..)` method takes [`jpeg_store_memory::W`](W) writer structure"]
impl crate::Writable for JpegStoreMemorySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JPEG_STORE_MEMORY to value 0x4000"]
impl crate::Resettable for JpegStoreMemorySpec {
    const RESET_VALUE: u32 = 0x4000;
}
