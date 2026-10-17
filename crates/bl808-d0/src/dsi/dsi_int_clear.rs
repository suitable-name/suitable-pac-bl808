#[doc = "Register `DSI_INT_CLEAR` writer"]
pub type W = crate::W<DsiIntClearSpec>;
#[doc = "Field `dsi_int_clear` writer - "]
pub type DsiIntClearW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn dsi_int_clear(&mut self) -> DsiIntClearW<'_, DsiIntClearSpec> {
        DsiIntClearW::new(self, 0)
    }
}
#[doc = "DSI Interrupt Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_int_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiIntClearSpec;
impl crate::RegisterSpec for DsiIntClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dsi_int_clear::W`](W) writer structure"]
impl crate::Writable for DsiIntClearSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3fff;
}
#[doc = "`reset()` method sets DSI_INT_CLEAR to value 0"]
impl crate::Resettable for DsiIntClearSpec {}
