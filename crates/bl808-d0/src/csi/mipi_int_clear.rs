#[doc = "Register `MIPI_INT_CLEAR` writer"]
pub type W = crate::W<MipiIntClearSpec>;
#[doc = "Field `csi_int_clear` writer - Write 1 to clear interrupt status"]
pub type CsiIntClearW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl W {
    #[doc = "Bits 0:5 - Write 1 to clear interrupt status"]
    #[inline(always)]
    pub fn csi_int_clear(&mut self) -> CsiIntClearW<'_, MipiIntClearSpec> {
        CsiIntClearW::new(self, 0)
    }
}
#[doc = "MIPI CSI Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mipi_int_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MipiIntClearSpec;
impl crate::RegisterSpec for MipiIntClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mipi_int_clear::W`](W) writer structure"]
impl crate::Writable for MipiIntClearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets MIPI_INT_CLEAR to value 0"]
impl crate::Resettable for MipiIntClearSpec {}
