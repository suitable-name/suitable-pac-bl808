#[doc = "Register `DSI_CONFIG` reader"]
pub type R = crate::R<DsiConfigSpec>;
#[doc = "Register `DSI_CONFIG` writer"]
pub type W = crate::W<DsiConfigSpec>;
#[doc = "Field `cr_hstx_en` reader - HSTX function enable signal"]
pub type CrHstxEnR = crate::BitReader;
#[doc = "Field `cr_hstx_en` writer - HSTX function enable signal"]
pub type CrHstxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_lane_num` reader - 0:1-lane, 1:2-lane, 2:4-lane"]
pub type CrLaneNumR = crate::FieldReader;
#[doc = "Field `cr_lane_num` writer - 0:1-lane, 1:2-lane, 2:4-lane"]
pub type CrLaneNumW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_hstx_mode` reader - 0:Sync Event, 1:Sync Pulse"]
pub type CrHstxModeR = crate::BitReader;
#[doc = "Field `cr_hstx_mode` writer - 0:Sync Event, 1:Sync Pulse"]
pub type CrHstxModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_lse_pkt_en` reader - LSE packet enable"]
pub type CrLsePktEnR = crate::BitReader;
#[doc = "Field `cr_lse_pkt_en` writer - LSE packet enable"]
pub type CrLsePktEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_lane_mux_sel` reader - Lane multiplexer selection"]
pub type CrLaneMuxSelR = crate::FieldReader;
#[doc = "Field `cr_lane_mux_sel` writer - Lane multiplexer selection"]
pub type CrLaneMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_dt` reader - Data Type (YUV422, RGB565, RGB888, etc.)"]
pub type CrDtR = crate::FieldReader;
#[doc = "Field `cr_dt` writer - Data Type (YUV422, RGB565, RGB888, etc.)"]
pub type CrDtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `cr_vc` reader - Virtual Channel number"]
pub type CrVcR = crate::FieldReader;
#[doc = "Field `cr_vc` writer - Virtual Channel number"]
pub type CrVcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_hstx_vfp` reader - HSTX Vertical Front Porch width"]
pub type CrHstxVfpR = crate::FieldReader;
#[doc = "Field `cr_hstx_vfp` writer - HSTX Vertical Front Porch width"]
pub type CrHstxVfpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cr_hstx_vsa` reader - HSTX Vertical Sync Active width"]
pub type CrHstxVsaR = crate::FieldReader;
#[doc = "Field `cr_hstx_vsa` writer - HSTX Vertical Sync Active width"]
pub type CrHstxVsaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - HSTX function enable signal"]
    #[inline(always)]
    pub fn cr_hstx_en(&self) -> CrHstxEnR {
        CrHstxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 0:1-lane, 1:2-lane, 2:4-lane"]
    #[inline(always)]
    pub fn cr_lane_num(&self) -> CrLaneNumR {
        CrLaneNumR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - 0:Sync Event, 1:Sync Pulse"]
    #[inline(always)]
    pub fn cr_hstx_mode(&self) -> CrHstxModeR {
        CrHstxModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LSE packet enable"]
    #[inline(always)]
    pub fn cr_lse_pkt_en(&self) -> CrLsePktEnR {
        CrLsePktEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Lane multiplexer selection"]
    #[inline(always)]
    pub fn cr_lane_mux_sel(&self) -> CrLaneMuxSelR {
        CrLaneMuxSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Data Type (YUV422, RGB565, RGB888, etc.)"]
    #[inline(always)]
    pub fn cr_dt(&self) -> CrDtR {
        CrDtR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Virtual Channel number"]
    #[inline(always)]
    pub fn cr_vc(&self) -> CrVcR {
        CrVcR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - HSTX Vertical Front Porch width"]
    #[inline(always)]
    pub fn cr_hstx_vfp(&self) -> CrHstxVfpR {
        CrHstxVfpR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - HSTX Vertical Sync Active width"]
    #[inline(always)]
    pub fn cr_hstx_vsa(&self) -> CrHstxVsaR {
        CrHstxVsaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HSTX function enable signal"]
    #[inline(always)]
    pub fn cr_hstx_en(&mut self) -> CrHstxEnW<'_, DsiConfigSpec> {
        CrHstxEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 0:1-lane, 1:2-lane, 2:4-lane"]
    #[inline(always)]
    pub fn cr_lane_num(&mut self) -> CrLaneNumW<'_, DsiConfigSpec> {
        CrLaneNumW::new(self, 1)
    }
    #[doc = "Bit 4 - 0:Sync Event, 1:Sync Pulse"]
    #[inline(always)]
    pub fn cr_hstx_mode(&mut self) -> CrHstxModeW<'_, DsiConfigSpec> {
        CrHstxModeW::new(self, 4)
    }
    #[doc = "Bit 5 - LSE packet enable"]
    #[inline(always)]
    pub fn cr_lse_pkt_en(&mut self) -> CrLsePktEnW<'_, DsiConfigSpec> {
        CrLsePktEnW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Lane multiplexer selection"]
    #[inline(always)]
    pub fn cr_lane_mux_sel(&mut self) -> CrLaneMuxSelW<'_, DsiConfigSpec> {
        CrLaneMuxSelW::new(self, 6)
    }
    #[doc = "Bits 8:13 - Data Type (YUV422, RGB565, RGB888, etc.)"]
    #[inline(always)]
    pub fn cr_dt(&mut self) -> CrDtW<'_, DsiConfigSpec> {
        CrDtW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Virtual Channel number"]
    #[inline(always)]
    pub fn cr_vc(&mut self) -> CrVcW<'_, DsiConfigSpec> {
        CrVcW::new(self, 14)
    }
    #[doc = "Bits 16:23 - HSTX Vertical Front Porch width"]
    #[inline(always)]
    pub fn cr_hstx_vfp(&mut self) -> CrHstxVfpW<'_, DsiConfigSpec> {
        CrHstxVfpW::new(self, 16)
    }
    #[doc = "Bits 24:31 - HSTX Vertical Sync Active width"]
    #[inline(always)]
    pub fn cr_hstx_vsa(&mut self) -> CrHstxVsaW<'_, DsiConfigSpec> {
        CrHstxVsaW::new(self, 24)
    }
}
#[doc = "DSI Main Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiConfigSpec;
impl crate::RegisterSpec for DsiConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_config::R`](R) reader structure"]
impl crate::Readable for DsiConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_config::W`](W) writer structure"]
impl crate::Writable for DsiConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_CONFIG to value 0x0202_3e10"]
impl crate::Resettable for DsiConfigSpec {
    const RESET_VALUE: u32 = 0x0202_3e10;
}
