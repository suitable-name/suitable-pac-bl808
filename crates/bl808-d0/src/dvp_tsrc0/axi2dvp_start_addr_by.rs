#[doc = "Register `AXI2DVP_START_ADDR_BY` reader"]
pub type R = crate::R<Axi2dvpStartAddrBySpec>;
#[doc = "Register `AXI2DVP_START_ADDR_BY` writer"]
pub type W = crate::W<Axi2dvpStartAddrBySpec>;
#[doc = "Field `cr_axi_addr_start_by` reader - Start address for AXI read transfers (luma/BY channel)"]
pub type CrAxiAddrStartByR = crate::FieldReader<u32>;
#[doc = "Field `cr_axi_addr_start_by` writer - Start address for AXI read transfers (luma/BY channel)"]
pub type CrAxiAddrStartByW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address for AXI read transfers (luma/BY channel)"]
    #[inline(always)]
    pub fn cr_axi_addr_start_by(&self) -> CrAxiAddrStartByR {
        CrAxiAddrStartByR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address for AXI read transfers (luma/BY channel)"]
    #[inline(always)]
    pub fn cr_axi_addr_start_by(&mut self) -> CrAxiAddrStartByW<'_, Axi2dvpStartAddrBySpec> {
        CrAxiAddrStartByW::new(self, 0)
    }
}
#[doc = "AXI to DVP Start Address (luma/BY plane)\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_start_addr_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_start_addr_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Axi2dvpStartAddrBySpec;
impl crate::RegisterSpec for Axi2dvpStartAddrBySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi2dvp_start_addr_by::R`](R) reader structure"]
impl crate::Readable for Axi2dvpStartAddrBySpec {}
#[doc = "`write(|w| ..)` method takes [`axi2dvp_start_addr_by::W`](W) writer structure"]
impl crate::Writable for Axi2dvpStartAddrBySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI2DVP_START_ADDR_BY to value 0"]
impl crate::Resettable for Axi2dvpStartAddrBySpec {}
