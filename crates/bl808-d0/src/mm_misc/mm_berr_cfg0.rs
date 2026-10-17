#[doc = "Register `MM_BERR_CFG0` reader"]
pub type R = crate::R<MmBerrCfg0Spec>;
#[doc = "Register `MM_BERR_CFG0` writer"]
pub type W = crate::W<MmBerrCfg0Spec>;
#[doc = "Field `reg_berr_en` reader - Enable main bus error monitoring."]
pub type RegBerrEnR = crate::FieldReader;
#[doc = "Field `reg_berr_en` writer - Enable main bus error monitoring."]
pub type RegBerrEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_codec_berr_en` reader - Enable CODEC bus error monitoring."]
pub type RegCodecBerrEnR = crate::FieldReader;
#[doc = "Field `reg_codec_berr_en` writer - Enable CODEC bus error monitoring."]
pub type RegCodecBerrEnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_mmcpu_berr_en` reader - Enable MM CPU bus error monitoring."]
pub type RegMmcpuBerrEnR = crate::BitReader;
#[doc = "Field `reg_mmcpu_berr_en` writer - Enable MM CPU bus error monitoring."]
pub type RegMmcpuBerrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mminfra_berr_en` reader - Enable MM infrastructure bus error monitoring."]
pub type RegMminfraBerrEnR = crate::FieldReader;
#[doc = "Field `reg_mminfra_berr_en` writer - Enable MM infrastructure bus error monitoring."]
pub type RegMminfraBerrEnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - Enable main bus error monitoring."]
    #[inline(always)]
    pub fn reg_berr_en(&self) -> RegBerrEnR {
        RegBerrEnR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Enable CODEC bus error monitoring."]
    #[inline(always)]
    pub fn reg_codec_berr_en(&self) -> RegCodecBerrEnR {
        RegCodecBerrEnR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable MM CPU bus error monitoring."]
    #[inline(always)]
    pub fn reg_mmcpu_berr_en(&self) -> RegMmcpuBerrEnR {
        RegMmcpuBerrEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Enable MM infrastructure bus error monitoring."]
    #[inline(always)]
    pub fn reg_mminfra_berr_en(&self) -> RegMminfraBerrEnR {
        RegMminfraBerrEnR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Enable main bus error monitoring."]
    #[inline(always)]
    pub fn reg_berr_en(&mut self) -> RegBerrEnW<'_, MmBerrCfg0Spec> {
        RegBerrEnW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Enable CODEC bus error monitoring."]
    #[inline(always)]
    pub fn reg_codec_berr_en(&mut self) -> RegCodecBerrEnW<'_, MmBerrCfg0Spec> {
        RegCodecBerrEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable MM CPU bus error monitoring."]
    #[inline(always)]
    pub fn reg_mmcpu_berr_en(&mut self) -> RegMmcpuBerrEnW<'_, MmBerrCfg0Spec> {
        RegMmcpuBerrEnW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Enable MM infrastructure bus error monitoring."]
    #[inline(always)]
    pub fn reg_mminfra_berr_en(&mut self) -> RegMminfraBerrEnW<'_, MmBerrCfg0Spec> {
        RegMminfraBerrEnW::new(self, 24)
    }
}
#[doc = "Multimedia Bus Error Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_berr_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmBerrCfg0Spec;
impl crate::RegisterSpec for MmBerrCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_berr_cfg0::R`](R) reader structure"]
impl crate::Readable for MmBerrCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`mm_berr_cfg0::W`](W) writer structure"]
impl crate::Writable for MmBerrCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_BERR_CFG0 to value 0x1f01_0707"]
impl crate::Resettable for MmBerrCfg0Spec {
    const RESET_VALUE: u32 = 0x1f01_0707;
}
