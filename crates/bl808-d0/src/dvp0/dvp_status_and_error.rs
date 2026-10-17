#[doc = "Register `DVP_STATUS_AND_ERROR` reader"]
pub type R = crate::R<DvpStatusAndErrorSpec>;
#[doc = "Register `DVP_STATUS_AND_ERROR` writer"]
pub type W = crate::W<DvpStatusAndErrorSpec>;
#[doc = "Field `reg_frame_cnt_trgr_int` reader - Frame count to trigger interrupt"]
pub type RegFrameCntTrgrIntR = crate::FieldReader;
#[doc = "Field `reg_frame_cnt_trgr_int` writer - Frame count to trigger interrupt"]
pub type RegFrameCntTrgrIntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `reg_int_hcnt_en` reader - HSYNC count interrupt enable"]
pub type RegIntHcntEnR = crate::BitReader;
#[doc = "Field `reg_int_hcnt_en` writer - HSYNC count interrupt enable"]
pub type RegIntHcntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_vcnt_en` reader - VSYNC count interrupt enable"]
pub type RegIntVcntEnR = crate::BitReader;
#[doc = "Field `reg_int_vcnt_en` writer - VSYNC count interrupt enable"]
pub type RegIntVcntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_normal_en` reader - Normal interrupt enable"]
pub type RegIntNormalEnR = crate::BitReader;
#[doc = "Field `reg_int_normal_en` writer - Normal interrupt enable"]
pub type RegIntNormalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_mem_en` reader - Memory overrun interrupt enable"]
pub type RegIntMemEnR = crate::BitReader;
#[doc = "Field `reg_int_mem_en` writer - Memory overrun interrupt enable"]
pub type RegIntMemEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_frame_en` reader - Frame complete interrupt enable"]
pub type RegIntFrameEnR = crate::BitReader;
#[doc = "Field `reg_int_frame_en` writer - Frame complete interrupt enable"]
pub type RegIntFrameEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_fifo_en` reader - FIFO overrun interrupt enable"]
pub type RegIntFifoEnR = crate::BitReader;
#[doc = "Field `reg_int_fifo_en` writer - FIFO overrun interrupt enable"]
pub type RegIntFifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_normal_int` reader - Normal interrupt status"]
pub type StsNormalIntR = crate::BitReader;
#[doc = "Field `sts_mem_int` reader - Memory overrun interrupt status"]
pub type StsMemIntR = crate::BitReader;
#[doc = "Field `sts_frame_int` reader - Frame complete interrupt status"]
pub type StsFrameIntR = crate::BitReader;
#[doc = "Field `sts_fifo_int` reader - FIFO overrun interrupt status"]
pub type StsFifoIntR = crate::BitReader;
#[doc = "Field `frame_valid_cnt` reader - Number of valid frames received"]
pub type FrameValidCntR = crate::FieldReader;
#[doc = "Field `sts_hcnt_int` reader - HSYNC count interrupt status"]
pub type StsHcntIntR = crate::BitReader;
#[doc = "Field `sts_vcnt_int` reader - VSYNC count interrupt status"]
pub type StsVcntIntR = crate::BitReader;
#[doc = "Field `st_bus_idle` reader - Bus idle status"]
pub type StBusIdleR = crate::BitReader;
#[doc = "Field `st_bus_func` reader - Bus functional status"]
pub type StBusFuncR = crate::BitReader;
#[doc = "Field `st_bus_wait` reader - Bus wait status"]
pub type StBusWaitR = crate::BitReader;
#[doc = "Field `st_bus_flsh` reader - Bus flush status"]
pub type StBusFlshR = crate::BitReader;
#[doc = "Field `axi_idle` reader - AXI idle status"]
pub type AxiIdleR = crate::BitReader;
#[doc = "Field `st_dvp_idle` reader - DVP idle status"]
pub type StDvpIdleR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Frame count to trigger interrupt"]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&self) -> RegFrameCntTrgrIntR {
        RegFrameCntTrgrIntR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - HSYNC count interrupt enable"]
    #[inline(always)]
    pub fn reg_int_hcnt_en(&self) -> RegIntHcntEnR {
        RegIntHcntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VSYNC count interrupt enable"]
    #[inline(always)]
    pub fn reg_int_vcnt_en(&self) -> RegIntVcntEnR {
        RegIntVcntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal interrupt enable"]
    #[inline(always)]
    pub fn reg_int_normal_en(&self) -> RegIntNormalEnR {
        RegIntNormalEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Memory overrun interrupt enable"]
    #[inline(always)]
    pub fn reg_int_mem_en(&self) -> RegIntMemEnR {
        RegIntMemEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Frame complete interrupt enable"]
    #[inline(always)]
    pub fn reg_int_frame_en(&self) -> RegIntFrameEnR {
        RegIntFrameEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn reg_int_fifo_en(&self) -> RegIntFifoEnR {
        RegIntFifoEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Normal interrupt status"]
    #[inline(always)]
    pub fn sts_normal_int(&self) -> StsNormalIntR {
        StsNormalIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Memory overrun interrupt status"]
    #[inline(always)]
    pub fn sts_mem_int(&self) -> StsMemIntR {
        StsMemIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Frame complete interrupt status"]
    #[inline(always)]
    pub fn sts_frame_int(&self) -> StsFrameIntR {
        StsFrameIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FIFO overrun interrupt status"]
    #[inline(always)]
    pub fn sts_fifo_int(&self) -> StsFifoIntR {
        StsFifoIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Number of valid frames received"]
    #[inline(always)]
    pub fn frame_valid_cnt(&self) -> FrameValidCntR {
        FrameValidCntR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - HSYNC count interrupt status"]
    #[inline(always)]
    pub fn sts_hcnt_int(&self) -> StsHcntIntR {
        StsHcntIntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - VSYNC count interrupt status"]
    #[inline(always)]
    pub fn sts_vcnt_int(&self) -> StsVcntIntR {
        StsVcntIntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Bus idle status"]
    #[inline(always)]
    pub fn st_bus_idle(&self) -> StBusIdleR {
        StBusIdleR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus functional status"]
    #[inline(always)]
    pub fn st_bus_func(&self) -> StBusFuncR {
        StBusFuncR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bus wait status"]
    #[inline(always)]
    pub fn st_bus_wait(&self) -> StBusWaitR {
        StBusWaitR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bus flush status"]
    #[inline(always)]
    pub fn st_bus_flsh(&self) -> StBusFlshR {
        StBusFlshR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - AXI idle status"]
    #[inline(always)]
    pub fn axi_idle(&self) -> AxiIdleR {
        AxiIdleR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DVP idle status"]
    #[inline(always)]
    pub fn st_dvp_idle(&self) -> StDvpIdleR {
        StDvpIdleR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Frame count to trigger interrupt"]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&mut self) -> RegFrameCntTrgrIntW<'_, DvpStatusAndErrorSpec> {
        RegFrameCntTrgrIntW::new(self, 0)
    }
    #[doc = "Bit 6 - HSYNC count interrupt enable"]
    #[inline(always)]
    pub fn reg_int_hcnt_en(&mut self) -> RegIntHcntEnW<'_, DvpStatusAndErrorSpec> {
        RegIntHcntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - VSYNC count interrupt enable"]
    #[inline(always)]
    pub fn reg_int_vcnt_en(&mut self) -> RegIntVcntEnW<'_, DvpStatusAndErrorSpec> {
        RegIntVcntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Normal interrupt enable"]
    #[inline(always)]
    pub fn reg_int_normal_en(&mut self) -> RegIntNormalEnW<'_, DvpStatusAndErrorSpec> {
        RegIntNormalEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Memory overrun interrupt enable"]
    #[inline(always)]
    pub fn reg_int_mem_en(&mut self) -> RegIntMemEnW<'_, DvpStatusAndErrorSpec> {
        RegIntMemEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Frame complete interrupt enable"]
    #[inline(always)]
    pub fn reg_int_frame_en(&mut self) -> RegIntFrameEnW<'_, DvpStatusAndErrorSpec> {
        RegIntFrameEnW::new(self, 10)
    }
    #[doc = "Bit 11 - FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn reg_int_fifo_en(&mut self) -> RegIntFifoEnW<'_, DvpStatusAndErrorSpec> {
        RegIntFifoEnW::new(self, 11)
    }
}
#[doc = "DVP Status and Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp_status_and_error::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp_status_and_error::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DvpStatusAndErrorSpec;
impl crate::RegisterSpec for DvpStatusAndErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp_status_and_error::R`](R) reader structure"]
impl crate::Readable for DvpStatusAndErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp_status_and_error::W`](W) writer structure"]
impl crate::Writable for DvpStatusAndErrorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP_STATUS_AND_ERROR to value 0x3100_0800"]
impl crate::Resettable for DvpStatusAndErrorSpec {
    const RESET_VALUE: u32 = 0x3100_0800;
}
