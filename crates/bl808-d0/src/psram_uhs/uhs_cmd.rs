#[doc = "Register `UHS_CMD` reader"]
pub type R = crate::R<UhsCmdSpec>;
#[doc = "Register `UHS_CMD` writer"]
pub type W = crate::W<UhsCmdSpec>;
#[doc = "Field `reg_glbr_pulse` writer - Trigger Global Reset pulse"]
pub type RegGlbrPulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_srfi_pulse` writer - Trigger Self-Refresh Entry pulse"]
pub type RegSrfiPulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_srfo_pulse` writer - Trigger Self-Refresh Exit pulse"]
pub type RegSrfoPulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_regw_pulse` writer - Trigger Register Write pulse"]
pub type RegRegwPulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_regr_pulse` writer - Trigger Register Read pulse"]
pub type RegRegrPulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_glbr_done` reader - Global Reset done status"]
pub type StsGlbrDoneR = crate::BitReader;
#[doc = "Field `sts_srfi_done` reader - Self-Refresh Entry done status"]
pub type StsSrfiDoneR = crate::BitReader;
#[doc = "Field `sts_srfo_done` reader - Self-Refresh Exit done status"]
pub type StsSrfoDoneR = crate::BitReader;
#[doc = "Field `sts_regw_done` reader - Register Write done status"]
pub type StsRegwDoneR = crate::BitReader;
#[doc = "Field `sts_regr_done` reader - Register Read done status"]
pub type StsRegrDoneR = crate::BitReader;
#[doc = "Field `sts_init_done` reader - Initialization done status"]
pub type StsInitDoneR = crate::BitReader;
#[doc = "Field `sts_config_read` reader - Configuration read-back status"]
pub type StsConfigReadR = crate::FieldReader;
impl R {
    #[doc = "Bit 8 - Global Reset done status"]
    #[inline(always)]
    pub fn sts_glbr_done(&self) -> StsGlbrDoneR {
        StsGlbrDoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Self-Refresh Entry done status"]
    #[inline(always)]
    pub fn sts_srfi_done(&self) -> StsSrfiDoneR {
        StsSrfiDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Self-Refresh Exit done status"]
    #[inline(always)]
    pub fn sts_srfo_done(&self) -> StsSrfoDoneR {
        StsSrfoDoneR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Register Write done status"]
    #[inline(always)]
    pub fn sts_regw_done(&self) -> StsRegwDoneR {
        StsRegwDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Register Read done status"]
    #[inline(always)]
    pub fn sts_regr_done(&self) -> StsRegrDoneR {
        StsRegrDoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Initialization done status"]
    #[inline(always)]
    pub fn sts_init_done(&self) -> StsInitDoneR {
        StsInitDoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Configuration read-back status"]
    #[inline(always)]
    pub fn sts_config_read(&self) -> StsConfigReadR {
        StsConfigReadR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Global Reset pulse"]
    #[inline(always)]
    pub fn reg_glbr_pulse(&mut self) -> RegGlbrPulseW<'_, UhsCmdSpec> {
        RegGlbrPulseW::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger Self-Refresh Entry pulse"]
    #[inline(always)]
    pub fn reg_srfi_pulse(&mut self) -> RegSrfiPulseW<'_, UhsCmdSpec> {
        RegSrfiPulseW::new(self, 1)
    }
    #[doc = "Bit 2 - Trigger Self-Refresh Exit pulse"]
    #[inline(always)]
    pub fn reg_srfo_pulse(&mut self) -> RegSrfoPulseW<'_, UhsCmdSpec> {
        RegSrfoPulseW::new(self, 2)
    }
    #[doc = "Bit 3 - Trigger Register Write pulse"]
    #[inline(always)]
    pub fn reg_regw_pulse(&mut self) -> RegRegwPulseW<'_, UhsCmdSpec> {
        RegRegwPulseW::new(self, 3)
    }
    #[doc = "Bit 4 - Trigger Register Read pulse"]
    #[inline(always)]
    pub fn reg_regr_pulse(&mut self) -> RegRegrPulseW<'_, UhsCmdSpec> {
        RegRegrPulseW::new(self, 4)
    }
}
#[doc = "UHS Command and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsCmdSpec;
impl crate::RegisterSpec for UhsCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_cmd::R`](R) reader structure"]
impl crate::Readable for UhsCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_cmd::W`](W) writer structure"]
impl crate::Writable for UhsCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_CMD to value 0"]
impl crate::Resettable for UhsCmdSpec {}
