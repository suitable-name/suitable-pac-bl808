#[doc = "Register `CAM_CFG0` reader"]
pub type R = crate::R<CamCfg0Spec>;
#[doc = "Register `CAM_CFG0` writer"]
pub type W = crate::W<CamCfg0Spec>;
#[doc = "Field `reg_cam_ref_clk_en` reader - "]
pub type RegCamRefClkEnR = crate::BitReader;
#[doc = "Field `reg_cam_ref_clk_en` writer - "]
pub type RegCamRefClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cam_ref_clk_src_sel` reader - "]
pub type RegCamRefClkSrcSelR = crate::FieldReader;
#[doc = "Field `reg_cam_ref_clk_src_sel` writer - "]
pub type RegCamRefClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_cam_ref_clk_div` reader - "]
pub type RegCamRefClkDivR = crate::FieldReader;
#[doc = "Field `reg_cam_ref_clk_div` writer - "]
pub type RegCamRefClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_en(&self) -> RegCamRefClkEnR {
        RegCamRefClkEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_src_sel(&self) -> RegCamRefClkSrcSelR {
        RegCamRefClkSrcSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_div(&self) -> RegCamRefClkDivR {
        RegCamRefClkDivR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_en(&mut self) -> RegCamRefClkEnW<'_, CamCfg0Spec> {
        RegCamRefClkEnW::new(self, 27)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_src_sel(&mut self) -> RegCamRefClkSrcSelW<'_, CamCfg0Spec> {
        RegCamRefClkSrcSelW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_div(&mut self) -> RegCamRefClkDivW<'_, CamCfg0Spec> {
        RegCamRefClkDivW::new(self, 30)
    }
}
#[doc = "Camera Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`cam_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cam_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CamCfg0Spec;
impl crate::RegisterSpec for CamCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_cfg0::R`](R) reader structure"]
impl crate::Readable for CamCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`cam_cfg0::W`](W) writer structure"]
impl crate::Writable for CamCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAM_CFG0 to value 0"]
impl crate::Resettable for CamCfg0Spec {}
