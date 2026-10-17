#[doc = "Register `AXI2DVP_SWAP_ADDR_UV` reader"]
pub type R = crate::R<Axi2dvpSwapAddrUvSpec>;
#[doc = "Register `AXI2DVP_SWAP_ADDR_UV` writer"]
pub type W = crate::W<Axi2dvpSwapAddrUvSpec>;
#[doc = "Field `cr_axi_addr_swap_uv` reader - Swap address for chroma (UV) plane continuous streaming"]
pub type CrAxiAddrSwapUvR = crate::FieldReader<u32>;
#[doc = "Field `cr_axi_addr_swap_uv` writer - Swap address for chroma (UV) plane continuous streaming"]
pub type CrAxiAddrSwapUvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Swap address for chroma (UV) plane continuous streaming"]
    #[inline(always)]
    pub fn cr_axi_addr_swap_uv(&self) -> CrAxiAddrSwapUvR {
        CrAxiAddrSwapUvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Swap address for chroma (UV) plane continuous streaming"]
    #[inline(always)]
    pub fn cr_axi_addr_swap_uv(&mut self) -> CrAxiAddrSwapUvW<'_, Axi2dvpSwapAddrUvSpec> {
        CrAxiAddrSwapUvW::new(self, 0)
    }
}
#[doc = "AXI to DVP Swap Address for Chroma (UV) plane (double buffering)\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_swap_addr_uv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_swap_addr_uv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Axi2dvpSwapAddrUvSpec;
impl crate::RegisterSpec for Axi2dvpSwapAddrUvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi2dvp_swap_addr_uv::R`](R) reader structure"]
impl crate::Readable for Axi2dvpSwapAddrUvSpec {}
#[doc = "`write(|w| ..)` method takes [`axi2dvp_swap_addr_uv::W`](W) writer structure"]
impl crate::Writable for Axi2dvpSwapAddrUvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI2DVP_SWAP_ADDR_UV to value 0"]
impl crate::Resettable for Axi2dvpSwapAddrUvSpec {}
