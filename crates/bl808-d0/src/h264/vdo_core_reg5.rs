#[doc = "Register `VDO_CORE_REG5` reader"]
pub type R = crate::R<VdoCoreReg5Spec>;
#[doc = "Register `VDO_CORE_REG5` writer"]
pub type W = crate::W<VdoCoreReg5Spec>;
#[doc = "Field `target_p_qp` reader - Main-stream target P-frame QP (0-51)"]
pub type TargetPQpR = crate::FieldReader;
#[doc = "Field `target_p_qp` writer - Main-stream target P-frame QP (0-51)"]
pub type TargetPQpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `s_target_p_qp` reader - Secondary-stream target P-frame QP (0-51)"]
pub type STargetPQpR = crate::FieldReader;
#[doc = "Field `s_target_p_qp` writer - Secondary-stream target P-frame QP (0-51)"]
pub type STargetPQpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Main-stream target P-frame QP (0-51)"]
    #[inline(always)]
    pub fn target_p_qp(&self) -> TargetPQpR {
        TargetPQpR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Secondary-stream target P-frame QP (0-51)"]
    #[inline(always)]
    pub fn s_target_p_qp(&self) -> STargetPQpR {
        STargetPQpR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Main-stream target P-frame QP (0-51)"]
    #[inline(always)]
    pub fn target_p_qp(&mut self) -> TargetPQpW<'_, VdoCoreReg5Spec> {
        TargetPQpW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Secondary-stream target P-frame QP (0-51)"]
    #[inline(always)]
    pub fn s_target_p_qp(&mut self) -> STargetPQpW<'_, VdoCoreReg5Spec> {
        STargetPQpW::new(self, 16)
    }
}
#[doc = "Target P-frame QP for both streams\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_core_reg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_core_reg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoCoreReg5Spec;
impl crate::RegisterSpec for VdoCoreReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_core_reg5::R`](R) reader structure"]
impl crate::Readable for VdoCoreReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`vdo_core_reg5::W`](W) writer structure"]
impl crate::Writable for VdoCoreReg5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_CORE_REG5 to value 0"]
impl crate::Resettable for VdoCoreReg5Spec {}
