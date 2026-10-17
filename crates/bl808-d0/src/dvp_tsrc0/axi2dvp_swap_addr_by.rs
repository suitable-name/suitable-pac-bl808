#[doc = "Register `AXI2DVP_SWAP_ADDR_BY` reader"]
pub type R = crate::R<Axi2dvpSwapAddrBySpec>;
#[doc = "Register `AXI2DVP_SWAP_ADDR_BY` writer"]
pub type W = crate::W<Axi2dvpSwapAddrBySpec>;
#[doc = "Field `cr_axi_addr_swap_by` reader - Swap address for luma (BY) continuous streaming"]
pub type CrAxiAddrSwapByR = crate::FieldReader<u32>;
#[doc = "Field `cr_axi_addr_swap_by` writer - Swap address for luma (BY) continuous streaming"]
pub type CrAxiAddrSwapByW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Swap address for luma (BY) continuous streaming"]
    #[inline(always)]
    pub fn cr_axi_addr_swap_by(&self) -> CrAxiAddrSwapByR {
        CrAxiAddrSwapByR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Swap address for luma (BY) continuous streaming"]
    #[inline(always)]
    pub fn cr_axi_addr_swap_by(&mut self) -> CrAxiAddrSwapByW<'_, Axi2dvpSwapAddrBySpec> {
        CrAxiAddrSwapByW::new(self, 0)
    }
}
#[doc = "AXI to DVP Swap Address (luma/BY plane, for double buffering)\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_swap_addr_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_swap_addr_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Axi2dvpSwapAddrBySpec;
impl crate::RegisterSpec for Axi2dvpSwapAddrBySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi2dvp_swap_addr_by::R`](R) reader structure"]
impl crate::Readable for Axi2dvpSwapAddrBySpec {}
#[doc = "`write(|w| ..)` method takes [`axi2dvp_swap_addr_by::W`](W) writer structure"]
impl crate::Writable for Axi2dvpSwapAddrBySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI2DVP_SWAP_ADDR_BY to value 0"]
impl crate::Resettable for Axi2dvpSwapAddrBySpec {}
