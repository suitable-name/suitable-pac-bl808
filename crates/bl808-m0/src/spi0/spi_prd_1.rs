#[doc = "Register `SPI_PRD_1` reader"]
pub type R = crate::R<SpiPrd1Spec>;
#[doc = "Register `SPI_PRD_1` writer"]
pub type W = crate::W<SpiPrd1Spec>;
#[doc = "Field `cr_spi_prd_i` reader - Inter-frame interval duration (cycles)"]
pub type CrSpiPrdIR = crate::FieldReader;
#[doc = "Field `cr_spi_prd_i` writer - Inter-frame interval duration (cycles)"]
pub type CrSpiPrdIW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Inter-frame interval duration (cycles)"]
    #[inline(always)]
    pub fn cr_spi_prd_i(&self) -> CrSpiPrdIR {
        CrSpiPrdIR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Inter-frame interval duration (cycles)"]
    #[inline(always)]
    pub fn cr_spi_prd_i(&mut self) -> CrSpiPrdIW<'_, SpiPrd1Spec> {
        CrSpiPrdIW::new(self, 0)
    }
}
#[doc = "SPI Phase Duration Configuration 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_prd_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_prd_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiPrd1Spec;
impl crate::RegisterSpec for SpiPrd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_prd_1::R`](R) reader structure"]
impl crate::Readable for SpiPrd1Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_prd_1::W`](W) writer structure"]
impl crate::Writable for SpiPrd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_PRD_1 to value 0x0f"]
impl crate::Resettable for SpiPrd1Spec {
    const RESET_VALUE: u32 = 0x0f;
}
