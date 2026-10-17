#[doc = "Register `VDO_CORE_REG3` reader"]
pub type R = crate::R<VdoCoreReg3Spec>;
#[doc = "Register `VDO_CORE_REG3` writer"]
pub type W = crate::W<VdoCoreReg3Spec>;
#[doc = "Field `target_i_qp` reader - Main-stream target I-frame QP (0-51)"]
pub type TargetIQpR = crate::FieldReader;
#[doc = "Field `target_i_qp` writer - Main-stream target I-frame QP (0-51)"]
pub type TargetIQpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `s_target_i_qp` reader - Secondary-stream target I-frame QP (0-51)"]
pub type STargetIQpR = crate::FieldReader;
#[doc = "Field `s_target_i_qp` writer - Secondary-stream target I-frame QP (0-51)"]
pub type STargetIQpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Main-stream target I-frame QP (0-51)"]
    #[inline(always)]
    pub fn target_i_qp(&self) -> TargetIQpR {
        TargetIQpR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Secondary-stream target I-frame QP (0-51)"]
    #[inline(always)]
    pub fn s_target_i_qp(&self) -> STargetIQpR {
        STargetIQpR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Main-stream target I-frame QP (0-51)"]
    #[inline(always)]
    pub fn target_i_qp(&mut self) -> TargetIQpW<'_, VdoCoreReg3Spec> {
        TargetIQpW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Secondary-stream target I-frame QP (0-51)"]
    #[inline(always)]
    pub fn s_target_i_qp(&mut self) -> STargetIQpW<'_, VdoCoreReg3Spec> {
        STargetIQpW::new(self, 16)
    }
}
#[doc = "Target I-frame QP for both streams\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_core_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_core_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoCoreReg3Spec;
impl crate::RegisterSpec for VdoCoreReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_core_reg3::R`](R) reader structure"]
impl crate::Readable for VdoCoreReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`vdo_core_reg3::W`](W) writer structure"]
impl crate::Writable for VdoCoreReg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_CORE_REG3 to value 0"]
impl crate::Resettable for VdoCoreReg3Spec {}
