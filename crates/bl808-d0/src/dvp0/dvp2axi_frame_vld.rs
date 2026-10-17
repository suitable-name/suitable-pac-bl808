#[doc = "Register `DVP2AXI_FRAME_VLD` reader"]
pub type R = crate::R<Dvp2axiFrameVldSpec>;
#[doc = "Register `DVP2AXI_FRAME_VLD` writer"]
pub type W = crate::W<Dvp2axiFrameVldSpec>;
#[doc = "Field `reg_frame_n_vld` reader - "]
pub type RegFrameNVldR = crate::FieldReader<u32>;
#[doc = "Field `reg_frame_n_vld` writer - "]
pub type RegFrameNVldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_frame_n_vld(&self) -> RegFrameNVldR {
        RegFrameNVldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_frame_n_vld(&mut self) -> RegFrameNVldW<'_, Dvp2axiFrameVldSpec> {
        RegFrameNVldW::new(self, 0)
    }
}
#[doc = "Frame Valid Period\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_frame_vld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_frame_vld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvp2axiFrameVldSpec;
impl crate::RegisterSpec for Dvp2axiFrameVldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp2axi_frame_vld::R`](R) reader structure"]
impl crate::Readable for Dvp2axiFrameVldSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp2axi_frame_vld::W`](W) writer structure"]
impl crate::Writable for Dvp2axiFrameVldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP2AXI_FRAME_VLD to value 0xffff_ffff"]
impl crate::Resettable for Dvp2axiFrameVldSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
