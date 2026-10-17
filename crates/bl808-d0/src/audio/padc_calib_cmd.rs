#[doc = "Register `padc_calib_cmd` reader"]
pub type R = crate::R<PadcCalibCmdSpec>;
#[doc = "Register `padc_calib_cmd` writer"]
pub type W = crate::W<PadcCalibCmdSpec>;
#[doc = "Field `padc_calib_average` reader - "]
pub type PadcCalibAverageR = crate::FieldReader;
#[doc = "Field `padc_calib_average` writer - "]
pub type PadcCalibAverageW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `padc_calib_discard` reader - "]
pub type PadcCalibDiscardR = crate::FieldReader;
#[doc = "Field `padc_calib_discard` writer - "]
pub type PadcCalibDiscardW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `padc_calib_run` reader - "]
pub type PadcCalibRunR = crate::BitReader;
#[doc = "Field `padc_calib_run` writer - "]
pub type PadcCalibRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_calib_lvl` reader - "]
pub type PadcCalibLvlR = crate::FieldReader;
#[doc = "Field `padc_calib_lvl` writer - "]
pub type PadcCalibLvlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_calib_gain` reader - "]
pub type PadcCalibGainR = crate::FieldReader;
#[doc = "Field `padc_calib_gain` writer - "]
pub type PadcCalibGainW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `padc_calib_mode` reader - "]
pub type PadcCalibModeR = crate::FieldReader;
#[doc = "Field `padc_calib_mode` writer - "]
pub type PadcCalibModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `padc_calib_start` writer - "]
pub type PadcCalibStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `padc_calib_method` reader - "]
pub type PadcCalibMethodR = crate::BitReader;
#[doc = "Field `padc_calib_method` writer - "]
pub type PadcCalibMethodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn padc_calib_average(&self) -> PadcCalibAverageR {
        PadcCalibAverageR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn padc_calib_discard(&self) -> PadcCalibDiscardR {
        PadcCalibDiscardR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn padc_calib_run(&self) -> PadcCalibRunR {
        PadcCalibRunR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn padc_calib_lvl(&self) -> PadcCalibLvlR {
        PadcCalibLvlR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn padc_calib_gain(&self) -> PadcCalibGainR {
        PadcCalibGainR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn padc_calib_mode(&self) -> PadcCalibModeR {
        PadcCalibModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn padc_calib_method(&self) -> PadcCalibMethodR {
        PadcCalibMethodR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn padc_calib_average(&mut self) -> PadcCalibAverageW<'_, PadcCalibCmdSpec> {
        PadcCalibAverageW::new(self, 16)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn padc_calib_discard(&mut self) -> PadcCalibDiscardW<'_, PadcCalibCmdSpec> {
        PadcCalibDiscardW::new(self, 19)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn padc_calib_run(&mut self) -> PadcCalibRunW<'_, PadcCalibCmdSpec> {
        PadcCalibRunW::new(self, 22)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn padc_calib_lvl(&mut self) -> PadcCalibLvlW<'_, PadcCalibCmdSpec> {
        PadcCalibLvlW::new(self, 23)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn padc_calib_gain(&mut self) -> PadcCalibGainW<'_, PadcCalibCmdSpec> {
        PadcCalibGainW::new(self, 25)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn padc_calib_mode(&mut self) -> PadcCalibModeW<'_, PadcCalibCmdSpec> {
        PadcCalibModeW::new(self, 28)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn padc_calib_start(&mut self) -> PadcCalibStartW<'_, PadcCalibCmdSpec> {
        PadcCalibStartW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn padc_calib_method(&mut self) -> PadcCalibMethodW<'_, PadcCalibCmdSpec> {
        PadcCalibMethodW::new(self, 31)
    }
}
#[doc = "PADC Calibration Command\n\nYou can [`read`](crate::Reg::read) this register and get [`padc_calib_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padc_calib_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadcCalibCmdSpec;
impl crate::RegisterSpec for PadcCalibCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padc_calib_cmd::R`](R) reader structure"]
impl crate::Readable for PadcCalibCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`padc_calib_cmd::W`](W) writer structure"]
impl crate::Writable for PadcCalibCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets padc_calib_cmd to value 0"]
impl crate::Resettable for PadcCalibCmdSpec {}
