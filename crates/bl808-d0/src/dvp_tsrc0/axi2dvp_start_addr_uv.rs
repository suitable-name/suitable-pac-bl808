#[doc = "Register `AXI2DVP_START_ADDR_UV` reader"]
pub type R = crate::R<Axi2dvpStartAddrUvSpec>;
#[doc = "Register `AXI2DVP_START_ADDR_UV` writer"]
pub type W = crate::W<Axi2dvpStartAddrUvSpec>;
#[doc = "Field `cr_axi_addr_start_uv` reader - Start address for AXI read transfers of chroma (UV) plane"]
pub type CrAxiAddrStartUvR = crate::FieldReader<u32>;
#[doc = "Field `cr_axi_addr_start_uv` writer - Start address for AXI read transfers of chroma (UV) plane"]
pub type CrAxiAddrStartUvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address for AXI read transfers of chroma (UV) plane"]
    #[inline(always)]
    pub fn cr_axi_addr_start_uv(&self) -> CrAxiAddrStartUvR {
        CrAxiAddrStartUvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address for AXI read transfers of chroma (UV) plane"]
    #[inline(always)]
    pub fn cr_axi_addr_start_uv(&mut self) -> CrAxiAddrStartUvW<'_, Axi2dvpStartAddrUvSpec> {
        CrAxiAddrStartUvW::new(self, 0)
    }
}
#[doc = "AXI to DVP Start Address for Chroma (UV) plane\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_start_addr_uv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_start_addr_uv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Axi2dvpStartAddrUvSpec;
impl crate::RegisterSpec for Axi2dvpStartAddrUvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi2dvp_start_addr_uv::R`](R) reader structure"]
impl crate::Readable for Axi2dvpStartAddrUvSpec {}
#[doc = "`write(|w| ..)` method takes [`axi2dvp_start_addr_uv::W`](W) writer structure"]
impl crate::Writable for Axi2dvpStartAddrUvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI2DVP_START_ADDR_UV to value 0"]
impl crate::Resettable for Axi2dvpStartAddrUvSpec {}
