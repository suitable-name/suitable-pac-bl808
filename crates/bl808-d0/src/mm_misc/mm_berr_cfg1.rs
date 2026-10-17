#[doc = "Register `MM_BERR_CFG1` reader"]
pub type R = crate::R<MmBerrCfg1Spec>;
#[doc = "Register `MM_BERR_CFG1` writer"]
pub type W = crate::W<MmBerrCfg1Spec>;
#[doc = "Field `reg_berr_clr` reader - Clear main bus error state."]
pub type RegBerrClrR = crate::BitReader;
#[doc = "Field `reg_berr_clr` writer - Clear main bus error state."]
pub type RegBerrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_codec_berr_clr` reader - Clear CODEC bus error state."]
pub type RegCodecBerrClrR = crate::BitReader;
#[doc = "Field `reg_codec_berr_clr` writer - Clear CODEC bus error state."]
pub type RegCodecBerrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mmcpu_berr_clr` reader - Clear MM CPU bus error state."]
pub type RegMmcpuBerrClrR = crate::BitReader;
#[doc = "Field `reg_mmcpu_berr_clr` writer - Clear MM CPU bus error state."]
pub type RegMmcpuBerrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mminfra_berr_clr` reader - Clear MM infrastructure bus error state."]
pub type RegMminfraBerrClrR = crate::BitReader;
#[doc = "Field `reg_mminfra_berr_clr` writer - Clear MM infrastructure bus error state."]
pub type RegMminfraBerrClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_berr_last` reader - Select last-error capture for main bus error."]
pub type RegBerrLastR = crate::BitReader;
#[doc = "Field `reg_berr_last` writer - Select last-error capture for main bus error."]
pub type RegBerrLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_codec_berr_last` reader - Select last-error capture for CODEC bus error."]
pub type RegCodecBerrLastR = crate::BitReader;
#[doc = "Field `reg_codec_berr_last` writer - Select last-error capture for CODEC bus error."]
pub type RegCodecBerrLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mmcpu_berr_last` reader - Select last-error capture for MM CPU bus error."]
pub type RegMmcpuBerrLastR = crate::BitReader;
#[doc = "Field `reg_mmcpu_berr_last` writer - Select last-error capture for MM CPU bus error."]
pub type RegMmcpuBerrLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mminfra_berr_last` reader - Select last-error capture for MM infrastructure bus error."]
pub type RegMminfraBerrLastR = crate::BitReader;
#[doc = "Field `reg_mminfra_berr_last` writer - Select last-error capture for MM infrastructure bus error."]
pub type RegMminfraBerrLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_berr` reader - Status of main bus error."]
pub type StsBerrR = crate::BitReader;
#[doc = "Field `sts_codec_berr` reader - Status of CODEC bus error."]
pub type StsCodecBerrR = crate::BitReader;
#[doc = "Field `sts_mmcpu_berr` reader - Status of MM CPU bus error."]
pub type StsMmcpuBerrR = crate::BitReader;
#[doc = "Field `sts_mminfra_berr` reader - Status of MM infrastructure bus error."]
pub type StsMminfraBerrR = crate::BitReader;
#[doc = "Field `sts_berr_write` reader - Write direction status of main bus error transaction."]
pub type StsBerrWriteR = crate::BitReader;
#[doc = "Field `sts_codec_berr_write` reader - Write direction status of CODEC bus error transaction."]
pub type StsCodecBerrWriteR = crate::BitReader;
#[doc = "Field `sts_mmcpu_berr_write` reader - Write direction status of MM CPU bus error transaction."]
pub type StsMmcpuBerrWriteR = crate::BitReader;
#[doc = "Field `sts_mminfra_berr_write` reader - Write direction status of MM infrastructure bus error transaction."]
pub type StsMminfraBerrWriteR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clear main bus error state."]
    #[inline(always)]
    pub fn reg_berr_clr(&self) -> RegBerrClrR {
        RegBerrClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear CODEC bus error state."]
    #[inline(always)]
    pub fn reg_codec_berr_clr(&self) -> RegCodecBerrClrR {
        RegCodecBerrClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear MM CPU bus error state."]
    #[inline(always)]
    pub fn reg_mmcpu_berr_clr(&self) -> RegMmcpuBerrClrR {
        RegMmcpuBerrClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear MM infrastructure bus error state."]
    #[inline(always)]
    pub fn reg_mminfra_berr_clr(&self) -> RegMminfraBerrClrR {
        RegMminfraBerrClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Select last-error capture for main bus error."]
    #[inline(always)]
    pub fn reg_berr_last(&self) -> RegBerrLastR {
        RegBerrLastR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select last-error capture for CODEC bus error."]
    #[inline(always)]
    pub fn reg_codec_berr_last(&self) -> RegCodecBerrLastR {
        RegCodecBerrLastR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select last-error capture for MM CPU bus error."]
    #[inline(always)]
    pub fn reg_mmcpu_berr_last(&self) -> RegMmcpuBerrLastR {
        RegMmcpuBerrLastR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select last-error capture for MM infrastructure bus error."]
    #[inline(always)]
    pub fn reg_mminfra_berr_last(&self) -> RegMminfraBerrLastR {
        RegMminfraBerrLastR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Status of main bus error."]
    #[inline(always)]
    pub fn sts_berr(&self) -> StsBerrR {
        StsBerrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Status of CODEC bus error."]
    #[inline(always)]
    pub fn sts_codec_berr(&self) -> StsCodecBerrR {
        StsCodecBerrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Status of MM CPU bus error."]
    #[inline(always)]
    pub fn sts_mmcpu_berr(&self) -> StsMmcpuBerrR {
        StsMmcpuBerrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Status of MM infrastructure bus error."]
    #[inline(always)]
    pub fn sts_mminfra_berr(&self) -> StsMminfraBerrR {
        StsMminfraBerrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Write direction status of main bus error transaction."]
    #[inline(always)]
    pub fn sts_berr_write(&self) -> StsBerrWriteR {
        StsBerrWriteR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write direction status of CODEC bus error transaction."]
    #[inline(always)]
    pub fn sts_codec_berr_write(&self) -> StsCodecBerrWriteR {
        StsCodecBerrWriteR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write direction status of MM CPU bus error transaction."]
    #[inline(always)]
    pub fn sts_mmcpu_berr_write(&self) -> StsMmcpuBerrWriteR {
        StsMmcpuBerrWriteR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write direction status of MM infrastructure bus error transaction."]
    #[inline(always)]
    pub fn sts_mminfra_berr_write(&self) -> StsMminfraBerrWriteR {
        StsMminfraBerrWriteR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear main bus error state."]
    #[inline(always)]
    pub fn reg_berr_clr(&mut self) -> RegBerrClrW<'_, MmBerrCfg1Spec> {
        RegBerrClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear CODEC bus error state."]
    #[inline(always)]
    pub fn reg_codec_berr_clr(&mut self) -> RegCodecBerrClrW<'_, MmBerrCfg1Spec> {
        RegCodecBerrClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear MM CPU bus error state."]
    #[inline(always)]
    pub fn reg_mmcpu_berr_clr(&mut self) -> RegMmcpuBerrClrW<'_, MmBerrCfg1Spec> {
        RegMmcpuBerrClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear MM infrastructure bus error state."]
    #[inline(always)]
    pub fn reg_mminfra_berr_clr(&mut self) -> RegMminfraBerrClrW<'_, MmBerrCfg1Spec> {
        RegMminfraBerrClrW::new(self, 3)
    }
    #[doc = "Bit 8 - Select last-error capture for main bus error."]
    #[inline(always)]
    pub fn reg_berr_last(&mut self) -> RegBerrLastW<'_, MmBerrCfg1Spec> {
        RegBerrLastW::new(self, 8)
    }
    #[doc = "Bit 9 - Select last-error capture for CODEC bus error."]
    #[inline(always)]
    pub fn reg_codec_berr_last(&mut self) -> RegCodecBerrLastW<'_, MmBerrCfg1Spec> {
        RegCodecBerrLastW::new(self, 9)
    }
    #[doc = "Bit 10 - Select last-error capture for MM CPU bus error."]
    #[inline(always)]
    pub fn reg_mmcpu_berr_last(&mut self) -> RegMmcpuBerrLastW<'_, MmBerrCfg1Spec> {
        RegMmcpuBerrLastW::new(self, 10)
    }
    #[doc = "Bit 11 - Select last-error capture for MM infrastructure bus error."]
    #[inline(always)]
    pub fn reg_mminfra_berr_last(&mut self) -> RegMminfraBerrLastW<'_, MmBerrCfg1Spec> {
        RegMminfraBerrLastW::new(self, 11)
    }
}
#[doc = "Multimedia Bus Error Clear, Last-Error Select, and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mm_berr_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mm_berr_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmBerrCfg1Spec;
impl crate::RegisterSpec for MmBerrCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mm_berr_cfg1::R`](R) reader structure"]
impl crate::Readable for MmBerrCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`mm_berr_cfg1::W`](W) writer structure"]
impl crate::Writable for MmBerrCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MM_BERR_CFG1 to value 0"]
impl crate::Resettable for MmBerrCfg1Spec {}
