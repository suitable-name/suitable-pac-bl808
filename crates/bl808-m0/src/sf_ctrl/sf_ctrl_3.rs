#[doc = "Register `SF_CTRL_3` reader"]
pub type R = crate::R<SfCtrl3Spec>;
#[doc = "Register `SF_CTRL_3` writer"]
pub type W = crate::W<SfCtrl3Spec>;
#[doc = "Field `sf_cmds_2_wrap_len` reader - "]
pub type SfCmds2WrapLenR = crate::FieldReader;
#[doc = "Field `sf_cmds_2_wrap_len` writer - "]
pub type SfCmds2WrapLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sf_cmds_2_en` reader - "]
pub type SfCmds2EnR = crate::BitReader;
#[doc = "Field `sf_cmds_2_en` writer - "]
pub type SfCmds2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_cmds_2_bt_dly` reader - "]
pub type SfCmds2BtDlyR = crate::FieldReader;
#[doc = "Field `sf_cmds_2_bt_dly` writer - "]
pub type SfCmds2BtDlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sf_cmds_2_bt_en` reader - "]
pub type SfCmds2BtEnR = crate::BitReader;
#[doc = "Field `sf_cmds_2_bt_en` writer - "]
pub type SfCmds2BtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_cmds_2_wrap_q_ini` reader - "]
pub type SfCmds2WrapQIniR = crate::BitReader;
#[doc = "Field `sf_cmds_2_wrap_q_ini` writer - "]
pub type SfCmds2WrapQIniW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_cmds_2_wrap_mode` reader - "]
pub type SfCmds2WrapModeR = crate::FieldReader;
#[doc = "Field `sf_cmds_2_wrap_mode` writer - "]
pub type SfCmds2WrapModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_cmds_2_wrap_q` reader - "]
pub type SfCmds2WrapQR = crate::BitReader;
#[doc = "Field `sf_cmds_1_wrap_len` reader - "]
pub type SfCmds1WrapLenR = crate::FieldReader;
#[doc = "Field `sf_cmds_1_wrap_len` writer - "]
pub type SfCmds1WrapLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sf_cmds_1_en` reader - "]
pub type SfCmds1EnR = crate::BitReader;
#[doc = "Field `sf_cmds_1_en` writer - "]
pub type SfCmds1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_cmds_1_wrap_mode` reader - "]
pub type SfCmds1WrapModeR = crate::FieldReader;
#[doc = "Field `sf_cmds_1_wrap_mode` writer - "]
pub type SfCmds1WrapModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_cmds_core_en` reader - "]
pub type SfCmdsCoreEnR = crate::BitReader;
#[doc = "Field `sf_cmds_core_en` writer - "]
pub type SfCmdsCoreEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sf_if_1_ack_lat` reader - "]
pub type SfIf1AckLatR = crate::FieldReader;
#[doc = "Field `sf_if_1_ack_lat` writer - "]
pub type SfIf1AckLatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sf_cmds_2_wrap_len(&self) -> SfCmds2WrapLenR {
        SfCmds2WrapLenR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_cmds_2_en(&self) -> SfCmds2EnR {
        SfCmds2EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn sf_cmds_2_bt_dly(&self) -> SfCmds2BtDlyR {
        SfCmds2BtDlyR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sf_cmds_2_bt_en(&self) -> SfCmds2BtEnR {
        SfCmds2BtEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sf_cmds_2_wrap_q_ini(&self) -> SfCmds2WrapQIniR {
        SfCmds2WrapQIniR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sf_cmds_2_wrap_mode(&self) -> SfCmds2WrapModeR {
        SfCmds2WrapModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sf_cmds_2_wrap_q(&self) -> SfCmds2WrapQR {
        SfCmds2WrapQR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16"]
    #[inline(always)]
    pub fn sf_cmds_1_wrap_len(&self) -> SfCmds1WrapLenR {
        SfCmds1WrapLenR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_cmds_1_en(&self) -> SfCmds1EnR {
        SfCmds1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sf_cmds_1_wrap_mode(&self) -> SfCmds1WrapModeR {
        SfCmds1WrapModeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sf_cmds_core_en(&self) -> SfCmdsCoreEnR {
        SfCmdsCoreEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sf_if_1_ack_lat(&self) -> SfIf1AckLatR {
        SfIf1AckLatR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sf_cmds_2_wrap_len(&mut self) -> SfCmds2WrapLenW<'_, SfCtrl3Spec> {
        SfCmds2WrapLenW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_cmds_2_en(&mut self) -> SfCmds2EnW<'_, SfCtrl3Spec> {
        SfCmds2EnW::new(self, 4)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn sf_cmds_2_bt_dly(&mut self) -> SfCmds2BtDlyW<'_, SfCtrl3Spec> {
        SfCmds2BtDlyW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sf_cmds_2_bt_en(&mut self) -> SfCmds2BtEnW<'_, SfCtrl3Spec> {
        SfCmds2BtEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sf_cmds_2_wrap_q_ini(&mut self) -> SfCmds2WrapQIniW<'_, SfCtrl3Spec> {
        SfCmds2WrapQIniW::new(self, 9)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sf_cmds_2_wrap_mode(&mut self) -> SfCmds2WrapModeW<'_, SfCtrl3Spec> {
        SfCmds2WrapModeW::new(self, 10)
    }
    #[doc = "Bits 13:16"]
    #[inline(always)]
    pub fn sf_cmds_1_wrap_len(&mut self) -> SfCmds1WrapLenW<'_, SfCtrl3Spec> {
        SfCmds1WrapLenW::new(self, 13)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_cmds_1_en(&mut self) -> SfCmds1EnW<'_, SfCtrl3Spec> {
        SfCmds1EnW::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn sf_cmds_1_wrap_mode(&mut self) -> SfCmds1WrapModeW<'_, SfCtrl3Spec> {
        SfCmds1WrapModeW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sf_cmds_core_en(&mut self) -> SfCmdsCoreEnW<'_, SfCtrl3Spec> {
        SfCmdsCoreEnW::new(self, 20)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sf_if_1_ack_lat(&mut self) -> SfIf1AckLatW<'_, SfCtrl3Spec> {
        SfIf1AckLatW::new(self, 29)
    }
}
#[doc = "Serial Flash Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_ctrl_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_ctrl_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfCtrl3Spec;
impl crate::RegisterSpec for SfCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_ctrl_3::R`](R) reader structure"]
impl crate::Readable for SfCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_ctrl_3::W`](W) writer structure"]
impl crate::Writable for SfCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_CTRL_3 to value 0"]
impl crate::Resettable for SfCtrl3Spec {}
