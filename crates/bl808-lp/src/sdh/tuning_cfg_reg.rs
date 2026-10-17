#[doc = "Register `TUNING_CFG_REG` reader"]
pub type R = crate::R<TuningCfgRegSpec>;
#[doc = "Register `TUNING_CFG_REG` writer"]
pub type W = crate::W<TuningCfgRegSpec>;
#[doc = "Field `TUNING_TT_CNT` reader - "]
pub type TuningTtCntR = crate::FieldReader;
#[doc = "Field `TUNING_TT_CNT` writer - "]
pub type TuningTtCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TUNING_WD_CNT` reader - "]
pub type TuningWdCntR = crate::FieldReader;
#[doc = "Field `TUNING_WD_CNT` writer - "]
pub type TuningWdCntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TUNING_CLK_DLY` reader - "]
pub type TuningClkDlyR = crate::FieldReader<u16>;
#[doc = "Field `TUNING_SUCCESS_CNT` reader - "]
pub type TuningSuccessCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tuning_tt_cnt(&self) -> TuningTtCntR {
        TuningTtCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tuning_wd_cnt(&self) -> TuningWdCntR {
        TuningWdCntR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:23"]
    #[inline(always)]
    pub fn tuning_clk_dly(&self) -> TuningClkDlyR {
        TuningClkDlyR::new(((self.bits >> 14) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn tuning_success_cnt(&self) -> TuningSuccessCntR {
        TuningSuccessCntR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tuning_tt_cnt(&mut self) -> TuningTtCntW<'_, TuningCfgRegSpec> {
        TuningTtCntW::new(self, 0)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn tuning_wd_cnt(&mut self) -> TuningWdCntW<'_, TuningCfgRegSpec> {
        TuningWdCntW::new(self, 8)
    }
}
#[doc = "Vendor specific hardware auto-tuning results and clock delay status\n\nYou can [`read`](crate::Reg::read) this register and get [`tuning_cfg_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tuning_cfg_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TuningCfgRegSpec;
impl crate::RegisterSpec for TuningCfgRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tuning_cfg_reg::R`](R) reader structure"]
impl crate::Readable for TuningCfgRegSpec {}
#[doc = "`write(|w| ..)` method takes [`tuning_cfg_reg::W`](W) writer structure"]
impl crate::Writable for TuningCfgRegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TUNING_CFG_REG to value 0"]
impl crate::Resettable for TuningCfgRegSpec {}
