#[doc = "Register `HOST_CTRL_2` reader"]
pub type R = crate::R<HostCtrl2Spec>;
#[doc = "Register `HOST_CTRL_2` writer"]
pub type W = crate::W<HostCtrl2Spec>;
#[doc = "Field `UHS_MODE_SEL` reader - "]
pub type UhsModeSelR = crate::FieldReader;
#[doc = "Field `UHS_MODE_SEL` writer - "]
pub type UhsModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SDH_V18_EN` reader - "]
pub type SdhV18EnR = crate::BitReader;
#[doc = "Field `SDH_V18_EN` writer - "]
pub type SdhV18EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV_STRENGTH_SEL` reader - "]
pub type DrvStrengthSelR = crate::FieldReader;
#[doc = "Field `DRV_STRENGTH_SEL` writer - "]
pub type DrvStrengthSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXE_TUNING` reader - "]
pub type ExeTuningR = crate::BitReader;
#[doc = "Field `EXE_TUNING` writer - "]
pub type ExeTuningW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SAMPLING_CLK_SEL` reader - "]
pub type SamplingClkSelR = crate::BitReader;
#[doc = "Field `SAMPLING_CLK_SEL` writer - "]
pub type SamplingClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNC_INT_EN` reader - "]
pub type AsyncIntEnR = crate::BitReader;
#[doc = "Field `ASYNC_INT_EN` writer - "]
pub type AsyncIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_VAL_EN` reader - "]
pub type PreValEnR = crate::BitReader;
#[doc = "Field `PRE_VAL_EN` writer - "]
pub type PreValEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhs_mode_sel(&self) -> UhsModeSelR {
        UhsModeSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sdh_v18_en(&self) -> SdhV18EnR {
        SdhV18EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn drv_strength_sel(&self) -> DrvStrengthSelR {
        DrvStrengthSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn exe_tuning(&self) -> ExeTuningR {
        ExeTuningR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sampling_clk_sel(&self) -> SamplingClkSelR {
        SamplingClkSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn async_int_en(&self) -> AsyncIntEnR {
        AsyncIntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pre_val_en(&self) -> PreValEnR {
        PreValEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn uhs_mode_sel(&mut self) -> UhsModeSelW<'_, HostCtrl2Spec> {
        UhsModeSelW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sdh_v18_en(&mut self) -> SdhV18EnW<'_, HostCtrl2Spec> {
        SdhV18EnW::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn drv_strength_sel(&mut self) -> DrvStrengthSelW<'_, HostCtrl2Spec> {
        DrvStrengthSelW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn exe_tuning(&mut self) -> ExeTuningW<'_, HostCtrl2Spec> {
        ExeTuningW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sampling_clk_sel(&mut self) -> SamplingClkSelW<'_, HostCtrl2Spec> {
        SamplingClkSelW::new(self, 7)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn async_int_en(&mut self) -> AsyncIntEnW<'_, HostCtrl2Spec> {
        AsyncIntEnW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pre_val_en(&mut self) -> PreValEnW<'_, HostCtrl2Spec> {
        PreValEnW::new(self, 15)
    }
}
#[doc = "Host Control 2 Register (UHS-I and Sampling Control)\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCtrl2Spec;
impl crate::RegisterSpec for HostCtrl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`host_ctrl_2::R`](R) reader structure"]
impl crate::Readable for HostCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`host_ctrl_2::W`](W) writer structure"]
impl crate::Writable for HostCtrl2Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0x40;
}
#[doc = "`reset()` method sets HOST_CTRL_2 to value 0"]
impl crate::Resettable for HostCtrl2Spec {}
