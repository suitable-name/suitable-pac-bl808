#[doc = "Register `MMSYS_MISC_DUMMY` reader"]
pub type R = crate::R<MmsysMiscDummySpec>;
#[doc = "Register `MMSYS_MISC_DUMMY` writer"]
pub type W = crate::W<MmsysMiscDummySpec>;
#[doc = "Field `PIR_ctrl_o` reader - Passive Infra-Red (PIR) sensor control output signal."]
pub type PirCtrlOR = crate::BitReader;
#[doc = "Field `PIR_ctrl_o` writer - Passive Infra-Red (PIR) sensor control output signal."]
pub type PirCtrlOW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Light_sensor_ctrl_o` reader - Ambient light sensor control output signal."]
pub type LightSensorCtrlOR = crate::BitReader;
#[doc = "Field `Light_sensor_ctrl_o` writer - Ambient light sensor control output signal."]
pub type LightSensorCtrlOW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_cut_ctrl_o` reader - Infrared (IR) Cut filter mechanical control signal."]
pub type IrCutCtrlOR = crate::BitReader;
#[doc = "Field `IR_cut_ctrl_o` writer - Infrared (IR) Cut filter mechanical control signal."]
pub type IrCutCtrlOW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dvp_sensor_pwdn` reader - DVP Image Sensor Power-Down control output."]
pub type DvpSensorPwdnR = crate::BitReader;
#[doc = "Field `dvp_sensor_pwdn` writer - DVP Image Sensor Power-Down control output."]
pub type DvpSensorPwdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy_reg` reader - General purpose dummy bits for software/hardware handshaking."]
pub type DummyRegR = crate::FieldReader<u32>;
#[doc = "Field `dummy_reg` writer - General purpose dummy bits for software/hardware handshaking."]
pub type DummyRegW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - Passive Infra-Red (PIR) sensor control output signal."]
    #[inline(always)]
    pub fn pir_ctrl_o(&self) -> PirCtrlOR {
        PirCtrlOR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ambient light sensor control output signal."]
    #[inline(always)]
    pub fn light_sensor_ctrl_o(&self) -> LightSensorCtrlOR {
        LightSensorCtrlOR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Infrared (IR) Cut filter mechanical control signal."]
    #[inline(always)]
    pub fn ir_cut_ctrl_o(&self) -> IrCutCtrlOR {
        IrCutCtrlOR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DVP Image Sensor Power-Down control output."]
    #[inline(always)]
    pub fn dvp_sensor_pwdn(&self) -> DvpSensorPwdnR {
        DvpSensorPwdnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - General purpose dummy bits for software/hardware handshaking."]
    #[inline(always)]
    pub fn dummy_reg(&self) -> DummyRegR {
        DummyRegR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Passive Infra-Red (PIR) sensor control output signal."]
    #[inline(always)]
    pub fn pir_ctrl_o(&mut self) -> PirCtrlOW<'_, MmsysMiscDummySpec> {
        PirCtrlOW::new(self, 0)
    }
    #[doc = "Bit 1 - Ambient light sensor control output signal."]
    #[inline(always)]
    pub fn light_sensor_ctrl_o(&mut self) -> LightSensorCtrlOW<'_, MmsysMiscDummySpec> {
        LightSensorCtrlOW::new(self, 1)
    }
    #[doc = "Bit 2 - Infrared (IR) Cut filter mechanical control signal."]
    #[inline(always)]
    pub fn ir_cut_ctrl_o(&mut self) -> IrCutCtrlOW<'_, MmsysMiscDummySpec> {
        IrCutCtrlOW::new(self, 2)
    }
    #[doc = "Bit 3 - DVP Image Sensor Power-Down control output."]
    #[inline(always)]
    pub fn dvp_sensor_pwdn(&mut self) -> DvpSensorPwdnW<'_, MmsysMiscDummySpec> {
        DvpSensorPwdnW::new(self, 3)
    }
    #[doc = "Bits 4:31 - General purpose dummy bits for software/hardware handshaking."]
    #[inline(always)]
    pub fn dummy_reg(&mut self) -> DummyRegW<'_, MmsysMiscDummySpec> {
        DummyRegW::new(self, 4)
    }
}
#[doc = "Multimedia System Miscellaneous IO and Debug Dummy Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mmsys_misc_dummy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmsys_misc_dummy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmsysMiscDummySpec;
impl crate::RegisterSpec for MmsysMiscDummySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmsys_misc_dummy::R`](R) reader structure"]
impl crate::Readable for MmsysMiscDummySpec {}
#[doc = "`write(|w| ..)` method takes [`mmsys_misc_dummy::W`](W) writer structure"]
impl crate::Writable for MmsysMiscDummySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMSYS_MISC_DUMMY to value 0x0fff_0000"]
impl crate::Resettable for MmsysMiscDummySpec {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
