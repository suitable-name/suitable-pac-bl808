#[doc = "Register `BLAI_INT_CFG` reader"]
pub type R = crate::R<BlaiIntCfgSpec>;
#[doc = "Register `BLAI_INT_CFG` writer"]
pub type W = crate::W<BlaiIntCfgSpec>;
#[doc = "Field `reg_blai_start` writer - Start the BLAI execution (Write 1 pulse)"]
pub type RegBlaiStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_blai_stop` writer - Stop the BLAI execution (Write 1 pulse)"]
pub type RegBlaiStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_blai_resume` writer - Resume BLAI execution from suspend (Write 1 pulse)"]
pub type RegBlaiResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_int_clr` writer - Clear BLAI interrupt status (Write 1 pulse)"]
pub type RegIntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_blai_int` reader - BLAI interrupt status (End of task)"]
pub type StsBlaiIntR = crate::BitReader;
#[doc = "Field `reg_relu_n` reader - Negative value output setting for ReLU function"]
pub type RegReluNR = crate::FieldReader;
#[doc = "Field `reg_relu_n` writer - Negative value output setting for ReLU function"]
pub type RegReluNW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 9 - BLAI interrupt status (End of task)"]
    #[inline(always)]
    pub fn sts_blai_int(&self) -> StsBlaiIntR {
        StsBlaiIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Negative value output setting for ReLU function"]
    #[inline(always)]
    pub fn reg_relu_n(&self) -> RegReluNR {
        RegReluNR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start the BLAI execution (Write 1 pulse)"]
    #[inline(always)]
    pub fn reg_blai_start(&mut self) -> RegBlaiStartW<'_, BlaiIntCfgSpec> {
        RegBlaiStartW::new(self, 0)
    }
    #[doc = "Bit 1 - Stop the BLAI execution (Write 1 pulse)"]
    #[inline(always)]
    pub fn reg_blai_stop(&mut self) -> RegBlaiStopW<'_, BlaiIntCfgSpec> {
        RegBlaiStopW::new(self, 1)
    }
    #[doc = "Bit 2 - Resume BLAI execution from suspend (Write 1 pulse)"]
    #[inline(always)]
    pub fn reg_blai_resume(&mut self) -> RegBlaiResumeW<'_, BlaiIntCfgSpec> {
        RegBlaiResumeW::new(self, 2)
    }
    #[doc = "Bit 8 - Clear BLAI interrupt status (Write 1 pulse)"]
    #[inline(always)]
    pub fn reg_int_clr(&mut self) -> RegIntClrW<'_, BlaiIntCfgSpec> {
        RegIntClrW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Negative value output setting for ReLU function"]
    #[inline(always)]
    pub fn reg_relu_n(&mut self) -> RegReluNW<'_, BlaiIntCfgSpec> {
        RegReluNW::new(self, 16)
    }
}
#[doc = "BLAI Interrupt and Control Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_int_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_int_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiIntCfgSpec;
impl crate::RegisterSpec for BlaiIntCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_int_cfg::R`](R) reader structure"]
impl crate::Readable for BlaiIntCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`blai_int_cfg::W`](W) writer structure"]
impl crate::Writable for BlaiIntCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_INT_CFG to value 0"]
impl crate::Resettable for BlaiIntCfgSpec {}
