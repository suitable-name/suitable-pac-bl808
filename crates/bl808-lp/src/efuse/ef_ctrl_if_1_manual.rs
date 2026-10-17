#[doc = "Register `EF_CTRL_IF_1_MANUAL` reader"]
pub type R = crate::R<EfCtrlIf1ManualSpec>;
#[doc = "Register `EF_CTRL_IF_1_MANUAL` writer"]
pub type W = crate::W<EfCtrlIf1ManualSpec>;
#[doc = "Field `ef_if_1_q` reader - Manual read data output"]
pub type EfIf1QR = crate::FieldReader;
impl R {
    #[doc = "Bits 16:23 - Manual read data output"]
    #[inline(always)]
    pub fn ef_if_1_q(&self) -> EfIf1QR {
        EfIf1QR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {}
#[doc = "eFuse Interface 1 Manual Operation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_1_manual::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_1_manual::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIf1ManualSpec;
impl crate::RegisterSpec for EfCtrlIf1ManualSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_1_manual::R`](R) reader structure"]
impl crate::Readable for EfCtrlIf1ManualSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_1_manual::W`](W) writer structure"]
impl crate::Writable for EfCtrlIf1ManualSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_1_MANUAL to value 0"]
impl crate::Resettable for EfCtrlIf1ManualSpec {}
