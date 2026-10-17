#[doc = "Register `MIPI_INT_ENABLE` reader"]
pub type R = crate::R<MipiIntEnableSpec>;
#[doc = "Register `MIPI_INT_ENABLE` writer"]
pub type W = crate::W<MipiIntEnableSpec>;
#[doc = "Field `csi_int_enable` reader - Interrupt enable bits for CSI events"]
pub type CsiIntEnableR = crate::FieldReader;
#[doc = "Field `csi_int_enable` writer - Interrupt enable bits for CSI events"]
pub type CsiIntEnableW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Interrupt enable bits for CSI events"]
    #[inline(always)]
    pub fn csi_int_enable(&self) -> CsiIntEnableR {
        CsiIntEnableR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Interrupt enable bits for CSI events"]
    #[inline(always)]
    pub fn csi_int_enable(&mut self) -> CsiIntEnableW<'_, MipiIntEnableSpec> {
        CsiIntEnableW::new(self, 0)
    }
}
#[doc = "MIPI CSI Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_int_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_int_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiIntEnableSpec;
impl crate::RegisterSpec for MipiIntEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_int_enable::R`](R) reader structure"]
impl crate::Readable for MipiIntEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`mipi_int_enable::W`](W) writer structure"]
impl crate::Writable for MipiIntEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_INT_ENABLE to value 0x3f"]
impl crate::Resettable for MipiIntEnableSpec {
    const RESET_VALUE: u32 = 0x3f;
}
