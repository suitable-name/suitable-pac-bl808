#[doc = "Register `CGEN_M` reader"]
pub type R = crate::R<CgenMSpec>;
#[doc = "Register `CGEN_M` writer"]
pub type W = crate::W<CgenMSpec>;
#[doc = "Field `cgen_m_cpu` reader - "]
pub type CgenMCpuR = crate::BitReader;
#[doc = "Field `cgen_m_cpu` writer - "]
pub type CgenMCpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_m_sdu` reader - "]
pub type CgenMSduR = crate::BitReader;
#[doc = "Field `cgen_m_sdu` writer - "]
pub type CgenMSduW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_m_sec` reader - "]
pub type CgenMSecR = crate::BitReader;
#[doc = "Field `cgen_m_sec` writer - "]
pub type CgenMSecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_m_dma` reader - "]
pub type CgenMDmaR = crate::BitReader;
#[doc = "Field `cgen_m_dma` writer - "]
pub type CgenMDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_m_cci` reader - "]
pub type CgenMCciR = crate::BitReader;
#[doc = "Field `cgen_m_cci` writer - "]
pub type CgenMCciW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_m_cpu(&self) -> CgenMCpuR {
        CgenMCpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cgen_m_sdu(&self) -> CgenMSduR {
        CgenMSduR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_m_sec(&self) -> CgenMSecR {
        CgenMSecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_m_dma(&self) -> CgenMDmaR {
        CgenMDmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_m_cci(&self) -> CgenMCciR {
        CgenMCciR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_m_cpu(&mut self) -> CgenMCpuW<'_, CgenMSpec> {
        CgenMCpuW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cgen_m_sdu(&mut self) -> CgenMSduW<'_, CgenMSpec> {
        CgenMSduW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_m_sec(&mut self) -> CgenMSecW<'_, CgenMSpec> {
        CgenMSecW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_m_dma(&mut self) -> CgenMDmaW<'_, CgenMSpec> {
        CgenMDmaW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_m_cci(&mut self) -> CgenMCciW<'_, CgenMSpec> {
        CgenMCciW::new(self, 4)
    }
}
#[doc = "Clock Gate Config 0 (Core)\n\nYou can [`read`](crate::Reg::read) this register and get [`cgen_m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgen_m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgenMSpec;
impl crate::RegisterSpec for CgenMSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_m::R`](R) reader structure"]
impl crate::Readable for CgenMSpec {}
#[doc = "`write(|w| ..)` method takes [`cgen_m::W`](W) writer structure"]
impl crate::Writable for CgenMSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CGEN_M to value 0"]
impl crate::Resettable for CgenMSpec {}
