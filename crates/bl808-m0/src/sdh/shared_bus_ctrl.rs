#[doc = "Register `SHARED_BUS_CTRL` reader"]
pub type R = crate::R<SharedBusCtrlSpec>;
#[doc = "Register `SHARED_BUS_CTRL` writer"]
pub type W = crate::W<SharedBusCtrlSpec>;
#[doc = "Field `NUM_CLK_PINS` reader - "]
pub type NumClkPinsR = crate::FieldReader;
#[doc = "Field `NUM_INT_PINS` reader - "]
pub type NumIntPinsR = crate::FieldReader;
#[doc = "Field `BUS_WIDTH_PRESET` reader - "]
pub type BusWidthPresetR = crate::FieldReader;
#[doc = "Field `CLK_PIN_SEL` reader - "]
pub type ClkPinSelR = crate::FieldReader;
#[doc = "Field `CLK_PIN_SEL` writer - "]
pub type ClkPinSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INT_PIN_SEL` reader - "]
pub type IntPinSelR = crate::FieldReader;
#[doc = "Field `INT_PIN_SEL` writer - "]
pub type IntPinSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BEND_PWR_CTRL` reader - "]
pub type BendPwrCtrlR = crate::FieldReader;
#[doc = "Field `BEND_PWR_CTRL` writer - "]
pub type BendPwrCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn num_clk_pins(&self) -> NumClkPinsR {
        NumClkPinsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn num_int_pins(&self) -> NumIntPinsR {
        NumIntPinsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn bus_width_preset(&self) -> BusWidthPresetR {
        BusWidthPresetR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn clk_pin_sel(&self) -> ClkPinSelR {
        ClkPinSelR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn int_pin_sel(&self) -> IntPinSelR {
        IntPinSelR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn bend_pwr_ctrl(&self) -> BendPwrCtrlR {
        BendPwrCtrlR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn clk_pin_sel(&mut self) -> ClkPinSelW<'_, SharedBusCtrlSpec> {
        ClkPinSelW::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn int_pin_sel(&mut self) -> IntPinSelW<'_, SharedBusCtrlSpec> {
        IntPinSelW::new(self, 20)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn bend_pwr_ctrl(&mut self) -> BendPwrCtrlW<'_, SharedBusCtrlSpec> {
        BendPwrCtrlW::new(self, 24)
    }
}
#[doc = "Shared Bus Control Register (Vendor Specific)\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_bus_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_bus_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharedBusCtrlSpec;
impl crate::RegisterSpec for SharedBusCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shared_bus_ctrl::R`](R) reader structure"]
impl crate::Readable for SharedBusCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`shared_bus_ctrl::W`](W) writer structure"]
impl crate::Writable for SharedBusCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHARED_BUS_CTRL to value 0"]
impl crate::Resettable for SharedBusCtrlSpec {}
