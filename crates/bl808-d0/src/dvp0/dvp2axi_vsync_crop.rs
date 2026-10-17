#[doc = "Register `DVP2AXI_VSYNC_CROP` reader"]
pub type R = crate::R<Dvp2axiVsyncCropSpec>;
#[doc = "Register `DVP2AXI_VSYNC_CROP` writer"]
pub type W = crate::W<Dvp2axiVsyncCropSpec>;
#[doc = "Field `reg_vsync_act_end` reader - VSYNC active end position"]
pub type RegVsyncActEndR = crate::FieldReader<u16>;
#[doc = "Field `reg_vsync_act_end` writer - VSYNC active end position"]
pub type RegVsyncActEndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_vsync_act_start` reader - VSYNC active start position"]
pub type RegVsyncActStartR = crate::FieldReader<u16>;
#[doc = "Field `reg_vsync_act_start` writer - VSYNC active start position"]
pub type RegVsyncActStartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VSYNC active end position"]
    #[inline(always)]
    pub fn reg_vsync_act_end(&self) -> RegVsyncActEndR {
        RegVsyncActEndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VSYNC active start position"]
    #[inline(always)]
    pub fn reg_vsync_act_start(&self) -> RegVsyncActStartR {
        RegVsyncActStartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VSYNC active end position"]
    #[inline(always)]
    pub fn reg_vsync_act_end(&mut self) -> RegVsyncActEndW<'_, Dvp2axiVsyncCropSpec> {
        RegVsyncActEndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - VSYNC active start position"]
    #[inline(always)]
    pub fn reg_vsync_act_start(&mut self) -> RegVsyncActStartW<'_, Dvp2axiVsyncCropSpec> {
        RegVsyncActStartW::new(self, 16)
    }
}
#[doc = "VSYNC Active Region Crop\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_vsync_crop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_vsync_crop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvp2axiVsyncCropSpec;
impl crate::RegisterSpec for Dvp2axiVsyncCropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp2axi_vsync_crop::R`](R) reader structure"]
impl crate::Readable for Dvp2axiVsyncCropSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp2axi_vsync_crop::W`](W) writer structure"]
impl crate::Writable for Dvp2axiVsyncCropSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP2AXI_VSYNC_CROP to value 0xffff"]
impl crate::Resettable for Dvp2axiVsyncCropSpec {
    const RESET_VALUE: u32 = 0xffff;
}
