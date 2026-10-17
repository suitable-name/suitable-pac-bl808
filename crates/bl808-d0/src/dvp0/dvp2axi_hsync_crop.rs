#[doc = "Register `DVP2AXI_HSYNC_CROP` reader"]
pub type R = crate::R<Dvp2axiHsyncCropSpec>;
#[doc = "Register `DVP2AXI_HSYNC_CROP` writer"]
pub type W = crate::W<Dvp2axiHsyncCropSpec>;
#[doc = "Field `reg_hsync_act_end` reader - HSYNC active end position"]
pub type RegHsyncActEndR = crate::FieldReader<u16>;
#[doc = "Field `reg_hsync_act_end` writer - HSYNC active end position"]
pub type RegHsyncActEndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_hsync_act_start` reader - HSYNC active start position"]
pub type RegHsyncActStartR = crate::FieldReader<u16>;
#[doc = "Field `reg_hsync_act_start` writer - HSYNC active start position"]
pub type RegHsyncActStartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - HSYNC active end position"]
    #[inline(always)]
    pub fn reg_hsync_act_end(&self) -> RegHsyncActEndR {
        RegHsyncActEndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - HSYNC active start position"]
    #[inline(always)]
    pub fn reg_hsync_act_start(&self) -> RegHsyncActStartR {
        RegHsyncActStartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HSYNC active end position"]
    #[inline(always)]
    pub fn reg_hsync_act_end(&mut self) -> RegHsyncActEndW<'_, Dvp2axiHsyncCropSpec> {
        RegHsyncActEndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - HSYNC active start position"]
    #[inline(always)]
    pub fn reg_hsync_act_start(&mut self) -> RegHsyncActStartW<'_, Dvp2axiHsyncCropSpec> {
        RegHsyncActStartW::new(self, 16)
    }
}
#[doc = "HSYNC Active Region Crop\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_hsync_crop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_hsync_crop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvp2axiHsyncCropSpec;
impl crate::RegisterSpec for Dvp2axiHsyncCropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp2axi_hsync_crop::R`](R) reader structure"]
impl crate::Readable for Dvp2axiHsyncCropSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp2axi_hsync_crop::W`](W) writer structure"]
impl crate::Writable for Dvp2axiHsyncCropSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP2AXI_HSYNC_CROP to value 0xffff"]
impl crate::Resettable for Dvp2axiHsyncCropSpec {
    const RESET_VALUE: u32 = 0xffff;
}
