#[doc = "Register `SPI_PRD_0` reader"]
pub type R = crate::R<SpiPrd0Spec>;
#[doc = "Register `SPI_PRD_0` writer"]
pub type W = crate::W<SpiPrd0Spec>;
#[doc = "Field `cr_spi_prd_s` reader - Start condition duration (cycles)"]
pub type CrSpiPrdSR = crate::FieldReader;
#[doc = "Field `cr_spi_prd_s` writer - Start condition duration (cycles)"]
pub type CrSpiPrdSW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_spi_prd_p` reader - Stop condition duration (cycles)"]
pub type CrSpiPrdPR = crate::FieldReader;
#[doc = "Field `cr_spi_prd_p` writer - Stop condition duration (cycles)"]
pub type CrSpiPrdPW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_spi_prd_d_ph_0` reader - Data phase 0 duration (cycles)"]
pub type CrSpiPrdDPh0R = crate::FieldReader;
#[doc = "Field `cr_spi_prd_d_ph_0` writer - Data phase 0 duration (cycles)"]
pub type CrSpiPrdDPh0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_spi_prd_d_ph_1` reader - Data phase 1 duration (cycles)"]
pub type CrSpiPrdDPh1R = crate::FieldReader;
#[doc = "Field `cr_spi_prd_d_ph_1` writer - Data phase 1 duration (cycles)"]
pub type CrSpiPrdDPh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start condition duration (cycles)"]
    #[inline(always)]
    pub fn cr_spi_prd_s(&self) -> CrSpiPrdSR {
        CrSpiPrdSR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Stop condition duration (cycles)"]
    #[inline(always)]
    pub fn cr_spi_prd_p(&self) -> CrSpiPrdPR {
        CrSpiPrdPR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data phase 0 duration (cycles)"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_0(&self) -> CrSpiPrdDPh0R {
        CrSpiPrdDPh0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data phase 1 duration (cycles)"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_1(&self) -> CrSpiPrdDPh1R {
        CrSpiPrdDPh1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start condition duration (cycles)"]
    #[inline(always)]
    pub fn cr_spi_prd_s(&mut self) -> CrSpiPrdSW<'_, SpiPrd0Spec> {
        CrSpiPrdSW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Stop condition duration (cycles)"]
    #[inline(always)]
    pub fn cr_spi_prd_p(&mut self) -> CrSpiPrdPW<'_, SpiPrd0Spec> {
        CrSpiPrdPW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data phase 0 duration (cycles)"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_0(&mut self) -> CrSpiPrdDPh0W<'_, SpiPrd0Spec> {
        CrSpiPrdDPh0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data phase 1 duration (cycles)"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_1(&mut self) -> CrSpiPrdDPh1W<'_, SpiPrd0Spec> {
        CrSpiPrdDPh1W::new(self, 24)
    }
}
#[doc = "SPI Phase Duration Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_prd_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_prd_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiPrd0Spec;
impl crate::RegisterSpec for SpiPrd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_prd_0::R`](R) reader structure"]
impl crate::Readable for SpiPrd0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi_prd_0::W`](W) writer structure"]
impl crate::Writable for SpiPrd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_PRD_0 to value 0x0f0f_0f0f"]
impl crate::Resettable for SpiPrd0Spec {
    const RESET_VALUE: u32 = 0x0f0f_0f0f;
}
