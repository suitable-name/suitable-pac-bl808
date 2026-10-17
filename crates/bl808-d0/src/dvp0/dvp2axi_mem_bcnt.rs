#[doc = "Register `DVP2AXI_MEM_BCNT` reader"]
pub type R = crate::R<Dvp2axiMemBcntSpec>;
#[doc = "Register `DVP2AXI_MEM_BCNT` writer"]
pub type W = crate::W<Dvp2axiMemBcntSpec>;
#[doc = "Field `reg_mem_burst_cnt` reader - "]
pub type RegMemBurstCntR = crate::FieldReader<u32>;
#[doc = "Field `reg_mem_burst_cnt` writer - "]
pub type RegMemBurstCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_mem_burst_cnt(&self) -> RegMemBurstCntR {
        RegMemBurstCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_mem_burst_cnt(&mut self) -> RegMemBurstCntW<'_, Dvp2axiMemBcntSpec> {
        RegMemBurstCntW::new(self, 0)
    }
}
#[doc = "AXI Burst Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_mem_bcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_mem_bcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvp2axiMemBcntSpec;
impl crate::RegisterSpec for Dvp2axiMemBcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp2axi_mem_bcnt::R`](R) reader structure"]
impl crate::Readable for Dvp2axiMemBcntSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp2axi_mem_bcnt::W`](W) writer structure"]
impl crate::Writable for Dvp2axiMemBcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP2AXI_MEM_BCNT to value 0xc000"]
impl crate::Resettable for Dvp2axiMemBcntSpec {
    const RESET_VALUE: u32 = 0xc000;
}
