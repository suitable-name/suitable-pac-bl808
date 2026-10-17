#[doc = "Register `AXI_CR` reader"]
pub type R = crate::R<AxiCrSpec>;
#[doc = "Register `AXI_CR` writer"]
pub type W = crate::W<AxiCrSpec>;
#[doc = "Field `AXI_SGLBST` reader - AXI Single Burst Enable"]
pub type AxiSglbstR = crate::BitReader;
#[doc = "Field `AXI_SGLBST` writer - AXI Single Burst Enable"]
pub type AxiSglbstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AXI Single Burst Enable"]
    #[inline(always)]
    pub fn axi_sglbst(&self) -> AxiSglbstR {
        AxiSglbstR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AXI Single Burst Enable"]
    #[inline(always)]
    pub fn axi_sglbst(&mut self) -> AxiSglbstW<'_, AxiCrSpec> {
        AxiSglbstW::new(self, 0)
    }
}
#[doc = "AXI Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiCrSpec;
impl crate::RegisterSpec for AxiCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_cr::R`](R) reader structure"]
impl crate::Readable for AxiCrSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_cr::W`](W) writer structure"]
impl crate::Writable for AxiCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_CR to value 0"]
impl crate::Resettable for AxiCrSpec {}
