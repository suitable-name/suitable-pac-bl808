#[doc = "Register `AXI2DVP_BURST_CNT` reader"]
pub type R = crate::R<Axi2dvpBurstCntSpec>;
#[doc = "Register `AXI2DVP_BURST_CNT` writer"]
pub type W = crate::W<Axi2dvpBurstCntSpec>;
#[doc = "Field `cr_axi_frame_bc` reader - Total burst count for one frame transfer"]
pub type CrAxiFrameBcR = crate::FieldReader<u32>;
#[doc = "Field `cr_axi_frame_bc` writer - Total burst count for one frame transfer"]
pub type CrAxiFrameBcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Total burst count for one frame transfer"]
    #[inline(always)]
    pub fn cr_axi_frame_bc(&self) -> CrAxiFrameBcR {
        CrAxiFrameBcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total burst count for one frame transfer"]
    #[inline(always)]
    pub fn cr_axi_frame_bc(&mut self) -> CrAxiFrameBcW<'_, Axi2dvpBurstCntSpec> {
        CrAxiFrameBcW::new(self, 0)
    }
}
#[doc = "AXI to DVP Total Burst Count\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_burst_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_burst_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Axi2dvpBurstCntSpec;
impl crate::RegisterSpec for Axi2dvpBurstCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi2dvp_burst_cnt::R`](R) reader structure"]
impl crate::Readable for Axi2dvpBurstCntSpec {}
#[doc = "`write(|w| ..)` method takes [`axi2dvp_burst_cnt::W`](W) writer structure"]
impl crate::Writable for Axi2dvpBurstCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI2DVP_BURST_CNT to value 0"]
impl crate::Resettable for Axi2dvpBurstCntSpec {}
