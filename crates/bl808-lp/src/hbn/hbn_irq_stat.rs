#[doc = "Register `HBN_IRQ_STAT` reader"]
pub type R = crate::R<HbnIrqStatSpec>;
#[doc = "Field `irq_stat` reader - "]
pub type IrqStatR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_stat(&self) -> IrqStatR {
        IrqStatR::new(self.bits)
    }
}
#[doc = "HBN Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_irq_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnIrqStatSpec;
impl crate::RegisterSpec for HbnIrqStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_irq_stat::R`](R) reader structure"]
impl crate::Readable for HbnIrqStatSpec {}
#[doc = "`reset()` method sets HBN_IRQ_STAT to value 0"]
impl crate::Resettable for HbnIrqStatSpec {}
