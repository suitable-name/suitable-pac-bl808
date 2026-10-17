#[doc = "Register `CCI_ADDR` reader"]
pub type R = crate::R<CciAddrSpec>;
#[doc = "Register `CCI_ADDR` writer"]
pub type W = crate::W<CciAddrSpec>;
#[doc = "Field `apb_cci_addr` reader - Target memory address for CCI access"]
pub type ApbCciAddrR = crate::FieldReader<u32>;
#[doc = "Field `apb_cci_addr` writer - Target memory address for CCI access"]
pub type ApbCciAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target memory address for CCI access"]
    #[inline(always)]
    pub fn apb_cci_addr(&self) -> ApbCciAddrR {
        ApbCciAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target memory address for CCI access"]
    #[inline(always)]
    pub fn apb_cci_addr(&mut self) -> ApbCciAddrW<'_, CciAddrSpec> {
        ApbCciAddrW::new(self, 0)
    }
}
#[doc = "CCI Access Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cci_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cci_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CciAddrSpec;
impl crate::RegisterSpec for CciAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci_addr::R`](R) reader structure"]
impl crate::Readable for CciAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`cci_addr::W`](W) writer structure"]
impl crate::Writable for CciAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCI_ADDR to value 0"]
impl crate::Resettable for CciAddrSpec {}
