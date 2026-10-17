#[doc = "Register `DVP2AXI_FRAME_BCNT` reader"]
pub type R = crate::R<Dvp2axiFrameBcntSpec>;
#[doc = "Register `DVP2AXI_FRAME_BCNT` writer"]
pub type W = crate::W<Dvp2axiFrameBcntSpec>;
#[doc = "Field `reg_frame_byte_cnt` reader - "]
pub type RegFrameByteCntR = crate::FieldReader<u32>;
#[doc = "Field `reg_frame_byte_cnt` writer - "]
pub type RegFrameByteCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_frame_byte_cnt(&self) -> RegFrameByteCntR {
        RegFrameByteCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_frame_byte_cnt(&mut self) -> RegFrameByteCntW<'_, Dvp2axiFrameBcntSpec> {
        RegFrameByteCntW::new(self, 0)
    }
}
#[doc = "Single Frame Byte Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_frame_bcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_frame_bcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvp2axiFrameBcntSpec;
impl crate::RegisterSpec for Dvp2axiFrameBcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp2axi_frame_bcnt::R`](R) reader structure"]
impl crate::Readable for Dvp2axiFrameBcntSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp2axi_frame_bcnt::W`](W) writer structure"]
impl crate::Writable for Dvp2axiFrameBcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP2AXI_FRAME_BCNT to value 0x7e90"]
impl crate::Resettable for Dvp2axiFrameBcntSpec {
    const RESET_VALUE: u32 = 0x7e90;
}
