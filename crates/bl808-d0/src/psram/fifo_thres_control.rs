#[doc = "Register `FIFO_THRES_CONTROL` reader"]
pub type R = crate::R<FifoThresControlSpec>;
#[doc = "Register `FIFO_THRES_CONTROL` writer"]
pub type W = crate::W<FifoThresControlSpec>;
#[doc = "Field `reg_mask_w_fifo_cnt` reader - Write FIFO count mask threshold"]
pub type RegMaskWFifoCntR = crate::FieldReader<u16>;
#[doc = "Field `reg_mask_w_fifo_cnt` writer - Write FIFO count mask threshold"]
pub type RegMaskWFifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_mask_r_fifo_rem` reader - Read FIFO remaining threshold"]
pub type RegMaskRFifoRemR = crate::FieldReader<u16>;
#[doc = "Field `reg_mask_r_fifo_rem` writer - Read FIFO remaining threshold"]
pub type RegMaskRFifoRemW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write FIFO count mask threshold"]
    #[inline(always)]
    pub fn reg_mask_w_fifo_cnt(&self) -> RegMaskWFifoCntR {
        RegMaskWFifoCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read FIFO remaining threshold"]
    #[inline(always)]
    pub fn reg_mask_r_fifo_rem(&self) -> RegMaskRFifoRemR {
        RegMaskRFifoRemR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write FIFO count mask threshold"]
    #[inline(always)]
    pub fn reg_mask_w_fifo_cnt(&mut self) -> RegMaskWFifoCntW<'_, FifoThresControlSpec> {
        RegMaskWFifoCntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read FIFO remaining threshold"]
    #[inline(always)]
    pub fn reg_mask_r_fifo_rem(&mut self) -> RegMaskRFifoRemW<'_, FifoThresControlSpec> {
        RegMaskRFifoRemW::new(self, 16)
    }
}
#[doc = "FIFO Threshold Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_thres_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_thres_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoThresControlSpec;
impl crate::RegisterSpec for FifoThresControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_thres_control::R`](R) reader structure"]
impl crate::Readable for FifoThresControlSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_thres_control::W`](W) writer structure"]
impl crate::Writable for FifoThresControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO_THRES_CONTROL to value 0"]
impl crate::Resettable for FifoThresControlSpec {}
