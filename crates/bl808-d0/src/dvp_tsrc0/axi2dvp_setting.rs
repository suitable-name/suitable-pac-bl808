#[doc = "Register `AXI2DVP_SETTING` reader"]
pub type R = crate::R<Axi2dvpSettingSpec>;
#[doc = "Register `AXI2DVP_SETTING` writer"]
pub type W = crate::W<Axi2dvpSettingSpec>;
#[doc = "Field `cr_axi_xlen` reader - AXI burst length control: 0:Single, 1:INCR4, 2:INCR8, 3:INCR16, etc."]
pub type CrAxiXlenR = crate::FieldReader;
#[doc = "Field `cr_axi_xlen` writer - AXI burst length control: 0:Single, 1:INCR4, 2:INCR8, 3:INCR16, etc."]
pub type CrAxiXlenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `cr_axi_drain_err_clr` writer - Clear drain FIFO error (Write 1 pulse)"]
pub type CrAxiDrainErrClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_axi_420_mode` reader - YUV 4:2:0 mode enable for AXI to DVP path"]
pub type CrAxi420ModeR = crate::BitReader;
#[doc = "Field `cr_axi_420_mode` writer - YUV 4:2:0 mode enable for AXI to DVP path"]
pub type CrAxi420ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_axi_420_ud_sel` reader - YUV 4:2:0 U/D (top/bottom) field selection"]
pub type CrAxi420UdSelR = crate::BitReader;
#[doc = "Field `cr_axi_420_ud_sel` writer - YUV 4:2:0 U/D (top/bottom) field selection"]
pub type CrAxi420UdSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_qos_sw_mode` reader - AXI QoS software override mode enable"]
pub type CrQosSwModeR = crate::BitReader;
#[doc = "Field `cr_qos_sw_mode` writer - AXI QoS software override mode enable"]
pub type CrQosSwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_qos_sw` reader - AXI QoS software override value"]
pub type CrQosSwR = crate::BitReader;
#[doc = "Field `cr_qos_sw` writer - AXI QoS software override value"]
pub type CrQosSwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - AXI burst length control: 0:Single, 1:INCR4, 2:INCR8, 3:INCR16, etc."]
    #[inline(always)]
    pub fn cr_axi_xlen(&self) -> CrAxiXlenR {
        CrAxiXlenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - YUV 4:2:0 mode enable for AXI to DVP path"]
    #[inline(always)]
    pub fn cr_axi_420_mode(&self) -> CrAxi420ModeR {
        CrAxi420ModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - YUV 4:2:0 U/D (top/bottom) field selection"]
    #[inline(always)]
    pub fn cr_axi_420_ud_sel(&self) -> CrAxi420UdSelR {
        CrAxi420UdSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AXI QoS software override mode enable"]
    #[inline(always)]
    pub fn cr_qos_sw_mode(&self) -> CrQosSwModeR {
        CrQosSwModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AXI QoS software override value"]
    #[inline(always)]
    pub fn cr_qos_sw(&self) -> CrQosSwR {
        CrQosSwR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXI burst length control: 0:Single, 1:INCR4, 2:INCR8, 3:INCR16, etc."]
    #[inline(always)]
    pub fn cr_axi_xlen(&mut self) -> CrAxiXlenW<'_, Axi2dvpSettingSpec> {
        CrAxiXlenW::new(self, 0)
    }
    #[doc = "Bit 4 - Clear drain FIFO error (Write 1 pulse)"]
    #[inline(always)]
    pub fn cr_axi_drain_err_clr(&mut self) -> CrAxiDrainErrClrW<'_, Axi2dvpSettingSpec> {
        CrAxiDrainErrClrW::new(self, 4)
    }
    #[doc = "Bit 8 - YUV 4:2:0 mode enable for AXI to DVP path"]
    #[inline(always)]
    pub fn cr_axi_420_mode(&mut self) -> CrAxi420ModeW<'_, Axi2dvpSettingSpec> {
        CrAxi420ModeW::new(self, 8)
    }
    #[doc = "Bit 9 - YUV 4:2:0 U/D (top/bottom) field selection"]
    #[inline(always)]
    pub fn cr_axi_420_ud_sel(&mut self) -> CrAxi420UdSelW<'_, Axi2dvpSettingSpec> {
        CrAxi420UdSelW::new(self, 9)
    }
    #[doc = "Bit 10 - AXI QoS software override mode enable"]
    #[inline(always)]
    pub fn cr_qos_sw_mode(&mut self) -> CrQosSwModeW<'_, Axi2dvpSettingSpec> {
        CrQosSwModeW::new(self, 10)
    }
    #[doc = "Bit 11 - AXI QoS software override value"]
    #[inline(always)]
    pub fn cr_qos_sw(&mut self) -> CrQosSwW<'_, Axi2dvpSettingSpec> {
        CrQosSwW::new(self, 11)
    }
}
#[doc = "AXI to DVP Interface Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`axi2dvp_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi2dvp_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Axi2dvpSettingSpec;
impl crate::RegisterSpec for Axi2dvpSettingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi2dvp_setting::R`](R) reader structure"]
impl crate::Readable for Axi2dvpSettingSpec {}
#[doc = "`write(|w| ..)` method takes [`axi2dvp_setting::W`](W) writer structure"]
impl crate::Writable for Axi2dvpSettingSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x10;
}
#[doc = "`reset()` method sets AXI2DVP_SETTING to value 0x03"]
impl crate::Resettable for Axi2dvpSettingSpec {
    const RESET_VALUE: u32 = 0x03;
}
