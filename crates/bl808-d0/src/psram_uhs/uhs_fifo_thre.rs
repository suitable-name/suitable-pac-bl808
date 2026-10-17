#[doc = "Register `UHS_FIFO_THRE` reader"]
pub type R = crate::R<UhsFifoThreSpec>;
#[doc = "Register `UHS_FIFO_THRE` writer"]
pub type W = crate::W<UhsFifoThreSpec>;
#[doc = "Field `reg_mask_w_fifo_cnt` reader - Write FIFO count mask"]
pub type RegMaskWFifoCntR = crate::FieldReader<u16>;
#[doc = "Field `reg_mask_w_fifo_cnt` writer - Write FIFO count mask"]
pub type RegMaskWFifoCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_mask_r_fifo_rem` reader - Read FIFO remaining count mask"]
pub type RegMaskRFifoRemR = crate::FieldReader<u16>;
#[doc = "Field `reg_mask_r_fifo_rem` writer - Read FIFO remaining count mask"]
pub type RegMaskRFifoRemW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write FIFO count mask"]
    #[inline(always)]
    pub fn reg_mask_w_fifo_cnt(&self) -> RegMaskWFifoCntR {
        RegMaskWFifoCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read FIFO remaining count mask"]
    #[inline(always)]
    pub fn reg_mask_r_fifo_rem(&self) -> RegMaskRFifoRemR {
        RegMaskRFifoRemR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write FIFO count mask"]
    #[inline(always)]
    pub fn reg_mask_w_fifo_cnt(&mut self) -> RegMaskWFifoCntW<'_, UhsFifoThreSpec> {
        RegMaskWFifoCntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read FIFO remaining count mask"]
    #[inline(always)]
    pub fn reg_mask_r_fifo_rem(&mut self) -> RegMaskRFifoRemW<'_, UhsFifoThreSpec> {
        RegMaskRFifoRemW::new(self, 16)
    }
}
#[doc = "UHS FIFO Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_fifo_thre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_fifo_thre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsFifoThreSpec;
impl crate::RegisterSpec for UhsFifoThreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_fifo_thre::R`](R) reader structure"]
impl crate::Readable for UhsFifoThreSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_fifo_thre::W`](W) writer structure"]
impl crate::Writable for UhsFifoThreSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_FIFO_THRE to value 0"]
impl crate::Resettable for UhsFifoThreSpec {}
