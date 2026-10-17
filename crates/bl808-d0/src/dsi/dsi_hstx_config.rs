#[doc = "Register `DSI_HSTX_CONFIG` reader"]
pub type R = crate::R<DsiHstxConfigSpec>;
#[doc = "Register `DSI_HSTX_CONFIG` writer"]
pub type W = crate::W<DsiHstxConfigSpec>;
#[doc = "Field `cr_hstx_pc` reader - HSTX packet count"]
pub type CrHstxPcR = crate::FieldReader<u16>;
#[doc = "Field `cr_hstx_pc` writer - HSTX packet count"]
pub type CrHstxPcW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `cr_hstx_out_th` reader - HSTX output threshold"]
pub type CrHstxOutThR = crate::FieldReader<u16>;
#[doc = "Field `cr_hstx_out_th` writer - HSTX output threshold"]
pub type CrHstxOutThW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - HSTX packet count"]
    #[inline(always)]
    pub fn cr_hstx_pc(&self) -> CrHstxPcR {
        CrHstxPcR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - HSTX output threshold"]
    #[inline(always)]
    pub fn cr_hstx_out_th(&self) -> CrHstxOutThR {
        CrHstxOutThR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - HSTX packet count"]
    #[inline(always)]
    pub fn cr_hstx_pc(&mut self) -> CrHstxPcW<'_, DsiHstxConfigSpec> {
        CrHstxPcW::new(self, 0)
    }
    #[doc = "Bits 16:26 - HSTX output threshold"]
    #[inline(always)]
    pub fn cr_hstx_out_th(&mut self) -> CrHstxOutThW<'_, DsiHstxConfigSpec> {
        CrHstxOutThW::new(self, 16)
    }
}
#[doc = "High Speed Transmission Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_hstx_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_hstx_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiHstxConfigSpec;
impl crate::RegisterSpec for DsiHstxConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_hstx_config::R`](R) reader structure"]
impl crate::Readable for DsiHstxConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_hstx_config::W`](W) writer structure"]
impl crate::Writable for DsiHstxConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_HSTX_CONFIG to value 0x0348_0500"]
impl crate::Resettable for DsiHstxConfigSpec {
    const RESET_VALUE: u32 = 0x0348_0500;
}
