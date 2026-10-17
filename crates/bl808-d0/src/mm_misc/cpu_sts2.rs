#[doc = "Register `CPU_STS2` reader"]
pub type R = crate::R<CpuSts2Spec>;
#[doc = "Field `cpu0_retire_pc_31_0` reader - Lower bits \\[31:0\\] of the Program Counter at the last retire or exception point."]
pub type Cpu0RetirePc31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Lower bits \\[31:0\\] of the Program Counter at the last retire or exception point."]
    #[inline(always)]
    pub fn cpu0_retire_pc_31_0(&self) -> Cpu0RetirePc31_0R {
        Cpu0RetirePc31_0R::new(self.bits)
    }
}
#[doc = "D0 Core Status Register 2 (Program Counter Low Word)\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_sts2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuSts2Spec;
impl crate::RegisterSpec for CpuSts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_sts2::R`](R) reader structure"]
impl crate::Readable for CpuSts2Spec {}
#[doc = "`reset()` method sets CPU_STS2 to value 0"]
impl crate::Resettable for CpuSts2Spec {}
