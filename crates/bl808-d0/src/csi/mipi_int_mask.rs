#[doc = "Register `MIPI_INT_MASK` reader"]
pub type R = crate::R<MipiIntMaskSpec>;
#[doc = "Register `MIPI_INT_MASK` writer"]
pub type W = crate::W<MipiIntMaskSpec>;
#[doc = "Field `csi_int_mask` reader - "]
pub type CsiIntMaskR = crate::FieldReader;
#[doc = "Field `csi_int_mask` writer - "]
pub type CsiIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn csi_int_mask(&self) -> CsiIntMaskR {
        CsiIntMaskR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn csi_int_mask(&mut self) -> CsiIntMaskW<'_, MipiIntMaskSpec> {
        CsiIntMaskW::new(self, 0)
    }
}
#[doc = "MIPI CSI Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mipi_int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiIntMaskSpec;
impl crate::RegisterSpec for MipiIntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mipi_int_mask::R`](R) reader structure"]
impl crate::Readable for MipiIntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mipi_int_mask::W`](W) writer structure"]
impl crate::Writable for MipiIntMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIPI_INT_MASK to value 0x3f"]
impl crate::Resettable for MipiIntMaskSpec {
    const RESET_VALUE: u32 = 0x3f;
}
