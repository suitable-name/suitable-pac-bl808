#[doc = "Register `VDO_CORE_REG12` reader"]
pub type R = crate::R<VdoCoreReg12Spec>;
#[doc = "Register `VDO_CORE_REG12` writer"]
pub type W = crate::W<VdoCoreReg12Spec>;
#[doc = "Field `profile_idc` reader - Main-stream profile IDC"]
pub type ProfileIdcR = crate::BitReader;
#[doc = "Field `profile_idc` writer - Main-stream profile IDC"]
pub type ProfileIdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_profile_idc` reader - Secondary-stream profile IDC"]
pub type SProfileIdcR = crate::BitReader;
#[doc = "Field `s_profile_idc` writer - Secondary-stream profile IDC"]
pub type SProfileIdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Main-stream profile IDC"]
    #[inline(always)]
    pub fn profile_idc(&self) -> ProfileIdcR {
        ProfileIdcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Secondary-stream profile IDC"]
    #[inline(always)]
    pub fn s_profile_idc(&self) -> SProfileIdcR {
        SProfileIdcR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main-stream profile IDC"]
    #[inline(always)]
    pub fn profile_idc(&mut self) -> ProfileIdcW<'_, VdoCoreReg12Spec> {
        ProfileIdcW::new(self, 0)
    }
    #[doc = "Bit 16 - Secondary-stream profile IDC"]
    #[inline(always)]
    pub fn s_profile_idc(&mut self) -> SProfileIdcW<'_, VdoCoreReg12Spec> {
        SProfileIdcW::new(self, 16)
    }
}
#[doc = "H.264 profile IDC for both streams (0=Baseline, 1=Main)\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_core_reg12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_core_reg12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoCoreReg12Spec;
impl crate::RegisterSpec for VdoCoreReg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_core_reg12::R`](R) reader structure"]
impl crate::Readable for VdoCoreReg12Spec {}
#[doc = "`write(|w| ..)` method takes [`vdo_core_reg12::W`](W) writer structure"]
impl crate::Writable for VdoCoreReg12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_CORE_REG12 to value 0"]
impl crate::Resettable for VdoCoreReg12Spec {}
