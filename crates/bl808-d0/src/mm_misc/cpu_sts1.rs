#[doc = "Register `CPU_STS1` reader"]
pub type R = crate::R<CpuSts1Spec>;
#[doc = "Field `cpu0_lpmd_b` reader - D0 Low Power Mode Status bits from C906 core."]
pub type Cpu0LpmdBR = crate::FieldReader;
#[doc = "Field `cpu0_retire_pc_39_32` reader - Upper bits \\[39:32\\] of the Program Counter at the last retire or exception point."]
pub type Cpu0RetirePc39_32R = crate::FieldReader;
#[doc = "Field `cpu0_retire` reader - D0 Core instruction retire status."]
pub type Cpu0RetireR = crate::BitReader;
#[doc = "Field `cpu0_pad_halted` reader - D0 Core debug halted status."]
pub type Cpu0PadHaltedR = crate::BitReader;
#[doc = "Field `cpu0_ndm_rstn_req` reader - Non-Debug Mode reset request status."]
pub type Cpu0NdmRstnReqR = crate::BitReader;
#[doc = "Field `cpu0_hart_rstn_req` reader - HART reset request status."]
pub type Cpu0HartRstnReqR = crate::BitReader;
impl R {
    #[doc = "Bits 4:5 - D0 Low Power Mode Status bits from C906 core."]
    #[inline(always)]
    pub fn cpu0_lpmd_b(&self) -> Cpu0LpmdBR {
        Cpu0LpmdBR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Upper bits \\[39:32\\] of the Program Counter at the last retire or exception point."]
    #[inline(always)]
    pub fn cpu0_retire_pc_39_32(&self) -> Cpu0RetirePc39_32R {
        Cpu0RetirePc39_32R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - D0 Core instruction retire status."]
    #[inline(always)]
    pub fn cpu0_retire(&self) -> Cpu0RetireR {
        Cpu0RetireR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - D0 Core debug halted status."]
    #[inline(always)]
    pub fn cpu0_pad_halted(&self) -> Cpu0PadHaltedR {
        Cpu0PadHaltedR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Non-Debug Mode reset request status."]
    #[inline(always)]
    pub fn cpu0_ndm_rstn_req(&self) -> Cpu0NdmRstnReqR {
        Cpu0NdmRstnReqR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HART reset request status."]
    #[inline(always)]
    pub fn cpu0_hart_rstn_req(&self) -> Cpu0HartRstnReqR {
        Cpu0HartRstnReqR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "D0 Core Status Register 1 (Halt and LPMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_sts1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuSts1Spec;
impl crate::RegisterSpec for CpuSts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_sts1::R`](R) reader structure"]
impl crate::Readable for CpuSts1Spec {}
#[doc = "`reset()` method sets CPU_STS1 to value 0"]
impl crate::Resettable for CpuSts1Spec {}
