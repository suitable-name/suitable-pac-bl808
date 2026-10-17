#[doc = "Register `SD_TIMEOUT_CTRL_SW_RESET` reader"]
pub type R = crate::R<SdTimeoutCtrlSwResetSpec>;
#[doc = "Register `SD_TIMEOUT_CTRL_SW_RESET` writer"]
pub type W = crate::W<SdTimeoutCtrlSwResetSpec>;
#[doc = "Field `TIMEOUT_VALUE` reader - "]
pub type TimeoutValueR = crate::FieldReader;
#[doc = "Field `TIMEOUT_VALUE` writer - "]
pub type TimeoutValueW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SW_RST_ALL` reader - "]
pub type SwRstAllR = crate::BitReader;
#[doc = "Field `SW_RST_ALL` writer - "]
pub type SwRstAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RST_CMD` reader - "]
pub type SwRstCmdR = crate::BitReader;
#[doc = "Field `SW_RST_CMD` writer - "]
pub type SwRstCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RST_DAT` reader - "]
pub type SwRstDatR = crate::BitReader;
#[doc = "Field `SW_RST_DAT` writer - "]
pub type SwRstDatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn timeout_value(&self) -> TimeoutValueR {
        TimeoutValueR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SwRstAllR {
        SwRstAllR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sw_rst_cmd(&self) -> SwRstCmdR {
        SwRstCmdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sw_rst_dat(&self) -> SwRstDatR {
        SwRstDatR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn timeout_value(&mut self) -> TimeoutValueW<'_, SdTimeoutCtrlSwResetSpec> {
        TimeoutValueW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sw_rst_all(&mut self) -> SwRstAllW<'_, SdTimeoutCtrlSwResetSpec> {
        SwRstAllW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sw_rst_cmd(&mut self) -> SwRstCmdW<'_, SdTimeoutCtrlSwResetSpec> {
        SwRstCmdW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sw_rst_dat(&mut self) -> SwRstDatW<'_, SdTimeoutCtrlSwResetSpec> {
        SwRstDatW::new(self, 10)
    }
}
#[doc = "Timeout Control / Software Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_timeout_ctrl_sw_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_timeout_ctrl_sw_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdTimeoutCtrlSwResetSpec;
impl crate::RegisterSpec for SdTimeoutCtrlSwResetSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_timeout_ctrl_sw_reset::R`](R) reader structure"]
impl crate::Readable for SdTimeoutCtrlSwResetSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_timeout_ctrl_sw_reset::W`](W) writer structure"]
impl crate::Writable for SdTimeoutCtrlSwResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_TIMEOUT_CTRL_SW_RESET to value 0"]
impl crate::Resettable for SdTimeoutCtrlSwResetSpec {}
