#[doc = "Register `EF_CTRL_EF_RESERVED` reader"]
pub type R = crate::R<EfCtrlEfReservedSpec>;
#[doc = "Register `EF_CTRL_EF_RESERVED` writer"]
pub type W = crate::W<EfCtrlEfReservedSpec>;
#[doc = "Field `ef_reserved` reader - "]
pub type EfReservedR = crate::FieldReader<u32>;
#[doc = "Field `ef_reserved` writer - "]
pub type EfReservedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_reserved(&self) -> EfReservedR {
        EfReservedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_reserved(&mut self) -> EfReservedW<'_, EfCtrlEfReservedSpec> {
        EfReservedW::new(self, 0)
    }
}
#[doc = "eFuse Reserved Register (Read-Write)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_ef_reserved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_ef_reserved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlEfReservedSpec;
impl crate::RegisterSpec for EfCtrlEfReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_ef_reserved::R`](R) reader structure"]
impl crate::Readable for EfCtrlEfReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_ef_reserved::W`](W) writer structure"]
impl crate::Writable for EfCtrlEfReservedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_EF_RESERVED to value 0xffff"]
impl crate::Resettable for EfCtrlEfReservedSpec {
    const RESET_VALUE: u32 = 0xffff;
}
