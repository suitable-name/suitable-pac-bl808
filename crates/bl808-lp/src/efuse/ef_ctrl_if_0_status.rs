#[doc = "Register `EF_CTRL_IF_0_STATUS` reader"]
pub type R = crate::R<EfCtrlIf0StatusSpec>;
#[doc = "Register `EF_CTRL_IF_0_STATUS` writer"]
pub type W = crate::W<EfCtrlIf0StatusSpec>;
#[doc = "Field `ef_if_0_status` reader - Overall IF0 status"]
pub type EfIf0StatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Overall IF0 status"]
    #[inline(always)]
    pub fn ef_if_0_status(&self) -> EfIf0StatusR {
        EfIf0StatusR::new(self.bits)
    }
}
impl W {}
#[doc = "eFuse Interface 0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_0_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_0_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIf0StatusSpec;
impl crate::RegisterSpec for EfCtrlIf0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_0_status::R`](R) reader structure"]
impl crate::Readable for EfCtrlIf0StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_0_status::W`](W) writer structure"]
impl crate::Writable for EfCtrlIf0StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_0_STATUS to value 0xe400"]
impl crate::Resettable for EfCtrlIf0StatusSpec {
    const RESET_VALUE: u32 = 0xe400;
}
