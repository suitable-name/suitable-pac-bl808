#[doc = "Register `AXI2DVP_STATUS` reader"]
pub type R = crate::R<Axi2dvpStatusSpec>;
#[doc = "Field `st_axi_fifo_cnt_by` reader - AXI to DVP luma (BY) FIFO current count"]
pub type StAxiFifoCntByR = crate::FieldReader;
#[doc = "Field `st_axi_drain_error_by` reader - AXI to DVP luma (BY) FIFO drain error status"]
pub type StAxiDrainErrorByR = crate::BitReader;
#[doc = "Field `st_axi_state_idle_by` reader - AXI2DVP luma (BY) state machine idle status"]
pub type StAxiStateIdleByR = crate::BitReader;
#[doc = "Field `st_axi_state_func_by` reader - AXI2DVP luma (BY) state machine functional status"]
pub type StAxiStateFuncByR = crate::BitReader;
#[doc = "Field `st_axi_state_flsh_by` reader - AXI2DVP luma (BY) state machine flush status"]
pub type StAxiStateFlshByR = crate::BitReader;
#[doc = "Field `st_axi_fifo_cnt_uv` reader - AXI to DVP chroma (UV) FIFO current count"]
pub type StAxiFifoCntUvR = crate::FieldReader;
#[doc = "Field `st_axi_drain_error_uv` reader - AXI to DVP chroma (UV) FIFO drain error status"]
pub type StAxiDrainErrorUvR = crate::BitReader;
#[doc = "Field `st_axi_state_idle_uv` reader - AXI2DVP chroma (UV) state machine idle status"]
pub type StAxiStateIdleUvR = crate::BitReader;
#[doc = "Field `st_axi_state_func_uv` reader - AXI2DVP chroma (UV) state machine functional status"]
pub type StAxiStateFuncUvR = crate::BitReader;
#[doc = "Field `st_axi_state_flsh_uv` reader - AXI2DVP chroma (UV) state machine flush status"]
pub type StAxiStateFlshUvR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - AXI to DVP luma (BY) FIFO current count"]
    #[inline(always)]
    pub fn st_axi_fifo_cnt_by(&self) -> StAxiFifoCntByR {
        StAxiFifoCntByR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - AXI to DVP luma (BY) FIFO drain error status"]
    #[inline(always)]
    pub fn st_axi_drain_error_by(&self) -> StAxiDrainErrorByR {
        StAxiDrainErrorByR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AXI2DVP luma (BY) state machine idle status"]
    #[inline(always)]
    pub fn st_axi_state_idle_by(&self) -> StAxiStateIdleByR {
        StAxiStateIdleByR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AXI2DVP luma (BY) state machine functional status"]
    #[inline(always)]
    pub fn st_axi_state_func_by(&self) -> StAxiStateFuncByR {
        StAxiStateFuncByR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AXI2DVP luma (BY) state machine flush status"]
    #[inline(always)]
    pub fn st_axi_state_flsh_by(&self) -> StAxiStateFlshByR {
        StAxiStateFlshByR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:22 - AXI to DVP chroma (UV) FIFO current count"]
    #[inline(always)]
    pub fn st_axi_fifo_cnt_uv(&self) -> StAxiFifoCntUvR {
        StAxiFifoCntUvR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - AXI to DVP chroma (UV) FIFO drain error status"]
    #[inline(always)]
    pub fn st_axi_drain_error_uv(&self) -> StAxiDrainErrorUvR {
        StAxiDrainErrorUvR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - AXI2DVP chroma (UV) state machine idle status"]
    #[inline(always)]
    pub fn st_axi_state_idle_uv(&self) -> StAxiStateIdleUvR {
        StAxiStateIdleUvR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AXI2DVP chroma (UV) state machine functional status"]
    #[inline(always)]
    pub fn st_axi_state_func_uv(&self) -> StAxiStateFuncUvR {
        StAxiStateFuncUvR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - AXI2DVP chroma (UV) state machine flush status"]
    #[inline(always)]
    pub fn st_axi_state_flsh_uv(&self) -> StAxiStateFlshUvR {
        StAxiStateFlshUvR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "AXI to DVP Status (luma/BY channel bits\\[15:0\\], chroma/UV channel bits\\[31:16\\])\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Axi2dvpStatusSpec;
impl crate::RegisterSpec for Axi2dvpStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi2dvp_status::R`](R) reader structure"]
impl crate::Readable for Axi2dvpStatusSpec {}
#[doc = "`reset()` method sets AXI2DVP_STATUS to value 0"]
impl crate::Resettable for Axi2dvpStatusSpec {}
