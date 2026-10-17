#[doc = "Register `DSI_INT_MASK` reader"]
pub type R = crate::R<DsiIntMaskSpec>;
#[doc = "Register `DSI_INT_MASK` writer"]
pub type W = crate::W<DsiIntMaskSpec>;
#[doc = "Field `dsi_int_mask` reader - "]
pub type DsiIntMaskR = crate::FieldReader<u16>;
#[doc = "Field `dsi_int_mask` writer - "]
pub type DsiIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn dsi_int_mask(&self) -> DsiIntMaskR {
        DsiIntMaskR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn dsi_int_mask(&mut self) -> DsiIntMaskW<'_, DsiIntMaskSpec> {
        DsiIntMaskW::new(self, 0)
    }
}
#[doc = "DSI Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiIntMaskSpec;
impl crate::RegisterSpec for DsiIntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_int_mask::R`](R) reader structure"]
impl crate::Readable for DsiIntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_int_mask::W`](W) writer structure"]
impl crate::Writable for DsiIntMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_INT_MASK to value 0x3fff"]
impl crate::Resettable for DsiIntMaskSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
