#[doc = "Register `CPU_PLL_CFG10` reader"]
pub type R = crate::R<CpuPllCfg10Spec>;
#[doc = "Register `CPU_PLL_CFG10` writer"]
pub type W = crate::W<CpuPllCfg10Spec>;
#[doc = "Field `cpupll_ssc_en` reader - "]
pub type CpupllSscEnR = crate::BitReader;
#[doc = "Field `cpupll_ssc_en` writer - "]
pub type CpupllSscEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_ssc_cnt` reader - "]
pub type CpupllSscCntR = crate::FieldReader;
#[doc = "Field `cpupll_ssc_cnt` writer - "]
pub type CpupllSscCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `cpupll_ssc_gain` reader - "]
pub type CpupllSscGainR = crate::FieldReader;
#[doc = "Field `cpupll_ssc_gain` writer - "]
pub type CpupllSscGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cpupll_ssc_start_gate_en` reader - "]
pub type CpupllSscStartGateEnR = crate::BitReader;
#[doc = "Field `cpupll_ssc_start_gate_en` writer - "]
pub type CpupllSscStartGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpupll_ssc_start` reader - "]
pub type CpupllSscStartR = crate::BitReader;
#[doc = "Field `cpupll_ssc_start` writer - "]
pub type CpupllSscStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_ssc_en(&self) -> CpupllSscEnR {
        CpupllSscEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn cpupll_ssc_cnt(&self) -> CpupllSscCntR {
        CpupllSscCntR::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn cpupll_ssc_gain(&self) -> CpupllSscGainR {
        CpupllSscGainR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cpupll_ssc_start_gate_en(&self) -> CpupllSscStartGateEnR {
        CpupllSscStartGateEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cpupll_ssc_start(&self) -> CpupllSscStartR {
        CpupllSscStartR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpupll_ssc_en(&mut self) -> CpupllSscEnW<'_, CpuPllCfg10Spec> {
        CpupllSscEnW::new(self, 0)
    }
    #[doc = "Bits 4:11"]
    #[inline(always)]
    pub fn cpupll_ssc_cnt(&mut self) -> CpupllSscCntW<'_, CpuPllCfg10Spec> {
        CpupllSscCntW::new(self, 4)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn cpupll_ssc_gain(&mut self) -> CpupllSscGainW<'_, CpuPllCfg10Spec> {
        CpupllSscGainW::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cpupll_ssc_start_gate_en(&mut self) -> CpupllSscStartGateEnW<'_, CpuPllCfg10Spec> {
        CpupllSscStartGateEnW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cpupll_ssc_start(&mut self) -> CpupllSscStartW<'_, CpuPllCfg10Spec> {
        CpupllSscStartW::new(self, 20)
    }
}
#[doc = "CPU PLL Configuration 10 - SSC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_pll_cfg10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_pll_cfg10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuPllCfg10Spec;
impl crate::RegisterSpec for CpuPllCfg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_pll_cfg10::R`](R) reader structure"]
impl crate::Readable for CpuPllCfg10Spec {}
#[doc = "`write(|w| ..)` method takes [`cpu_pll_cfg10::W`](W) writer structure"]
impl crate::Writable for CpuPllCfg10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PLL_CFG10 to value 0x0010_4640"]
impl crate::Resettable for CpuPllCfg10Spec {
    const RESET_VALUE: u32 = 0x0010_4640;
}
