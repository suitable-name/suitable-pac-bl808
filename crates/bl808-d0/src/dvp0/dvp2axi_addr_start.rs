#[doc = "Register `DVP2AXI_ADDR_START` reader"]
pub type R = crate::R<Dvp2axiAddrStartSpec>;
#[doc = "Register `DVP2AXI_ADDR_START` writer"]
pub type W = crate::W<Dvp2axiAddrStartSpec>;
#[doc = "Field `reg_addr_start` reader - "]
pub type RegAddrStartR = crate::FieldReader<u32>;
#[doc = "Field `reg_addr_start` writer - "]
pub type RegAddrStartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_addr_start(&self) -> RegAddrStartR {
        RegAddrStartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_addr_start(&mut self) -> RegAddrStartW<'_, Dvp2axiAddrStartSpec> {
        RegAddrStartW::new(self, 0)
    }
}
#[doc = "AXI Start Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_addr_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_addr_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvp2axiAddrStartSpec;
impl crate::RegisterSpec for Dvp2axiAddrStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp2axi_addr_start::R`](R) reader structure"]
impl crate::Readable for Dvp2axiAddrStartSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp2axi_addr_start::W`](W) writer structure"]
impl crate::Writable for Dvp2axiAddrStartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP2AXI_ADDR_START to value 0x8000_0000"]
impl crate::Resettable for Dvp2axiAddrStartSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
