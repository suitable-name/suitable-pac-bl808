#[doc = "Register `DSI_INT_ENABLE` reader"]
pub type R = crate::R<DsiIntEnableSpec>;
#[doc = "Register `DSI_INT_ENABLE` writer"]
pub type W = crate::W<DsiIntEnableSpec>;
#[doc = "Field `dsi_int_enable` reader - "]
pub type DsiIntEnableR = crate::FieldReader<u16>;
#[doc = "Field `dsi_int_enable` writer - "]
pub type DsiIntEnableW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn dsi_int_enable(&self) -> DsiIntEnableR {
        DsiIntEnableR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn dsi_int_enable(&mut self) -> DsiIntEnableW<'_, DsiIntEnableSpec> {
        DsiIntEnableW::new(self, 0)
    }
}
#[doc = "DSI Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_int_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_int_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiIntEnableSpec;
impl crate::RegisterSpec for DsiIntEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_int_enable::R`](R) reader structure"]
impl crate::Readable for DsiIntEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_int_enable::W`](W) writer structure"]
impl crate::Writable for DsiIntEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_INT_ENABLE to value 0x3fff"]
impl crate::Resettable for DsiIntEnableSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
