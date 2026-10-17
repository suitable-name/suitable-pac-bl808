#[doc = "Register `CPU_CORE_CFG12` reader"]
pub type R = crate::R<CpuCoreCfg12Spec>;
#[doc = "Register `CPU_CORE_CFG12` writer"]
pub type W = crate::W<CpuCoreCfg12Spec>;
#[doc = "Field `e902_iahbl_base` reader - "]
pub type E902IahblBaseR = crate::FieldReader<u16>;
#[doc = "Field `e902_iahbl_base` writer - "]
pub type E902IahblBaseW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `e902_iahbl_mask` reader - "]
pub type E902IahblMaskR = crate::FieldReader<u16>;
#[doc = "Field `e902_iahbl_mask` writer - "]
pub type E902IahblMaskW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn e902_iahbl_base(&self) -> E902IahblBaseR {
        E902IahblBaseR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn e902_iahbl_mask(&self) -> E902IahblMaskR {
        E902IahblMaskR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn e902_iahbl_base(&mut self) -> E902IahblBaseW<'_, CpuCoreCfg12Spec> {
        E902IahblBaseW::new(self, 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn e902_iahbl_mask(&mut self) -> E902IahblMaskW<'_, CpuCoreCfg12Spec> {
        E902IahblMaskW::new(self, 16)
    }
}
#[doc = "E902 (PICO) I-AHB-Lite bus remapping/masking\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_core_cfg12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_core_cfg12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuCoreCfg12Spec;
impl crate::RegisterSpec for CpuCoreCfg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg12::R`](R) reader structure"]
impl crate::Readable for CpuCoreCfg12Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_core_cfg12::W`](W) writer structure"]
impl crate::Writable for CpuCoreCfg12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_CORE_CFG12 to value 0"]
impl crate::Resettable for CpuCoreCfg12Spec {}
