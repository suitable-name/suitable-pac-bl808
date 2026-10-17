#[doc = "Register `HBN_IRQ_MODE` reader"]
pub type R = crate::R<HbnIrqModeSpec>;
#[doc = "Register `HBN_IRQ_MODE` writer"]
pub type W = crate::W<HbnIrqModeSpec>;
#[doc = "Field `hbn_pin_wakeup_mode` reader - "]
pub type HbnPinWakeupModeR = crate::FieldReader;
#[doc = "Field `hbn_pin_wakeup_mode` writer - "]
pub type HbnPinWakeupModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hbn_pin_wakeup_mask` reader - "]
pub type HbnPinWakeupMaskR = crate::FieldReader<u16>;
#[doc = "Field `hbn_pin_wakeup_mask` writer - "]
pub type HbnPinWakeupMaskW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `reg_en_hw_pu_pd` reader - "]
pub type RegEnHwPuPdR = crate::BitReader;
#[doc = "Field `reg_en_hw_pu_pd` writer - "]
pub type RegEnHwPuPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `irq_bor_en` reader - "]
pub type IrqBorEnR = crate::BitReader;
#[doc = "Field `irq_bor_en` writer - "]
pub type IrqBorEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `irq_acomp0_en` reader - "]
pub type IrqAcomp0EnR = crate::FieldReader;
#[doc = "Field `irq_acomp0_en` writer - "]
pub type IrqAcomp0EnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `irq_acomp1_en` reader - "]
pub type IrqAcomp1EnR = crate::FieldReader;
#[doc = "Field `irq_acomp1_en` writer - "]
pub type IrqAcomp1EnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_wakeup_sel` reader - "]
pub type PinWakeupSelR = crate::FieldReader;
#[doc = "Field `pin_wakeup_sel` writer - "]
pub type PinWakeupSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pin_wakeup_en` reader - "]
pub type PinWakeupEnR = crate::BitReader;
#[doc = "Field `pin_wakeup_en` writer - "]
pub type PinWakeupEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mode(&self) -> HbnPinWakeupModeR {
        HbnPinWakeupModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:12"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mask(&self) -> HbnPinWakeupMaskR {
        HbnPinWakeupMaskR::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_en_hw_pu_pd(&self) -> RegEnHwPuPdR {
        RegEnHwPuPdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn irq_bor_en(&self) -> IrqBorEnR {
        IrqBorEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn irq_acomp0_en(&self) -> IrqAcomp0EnR {
        IrqAcomp0EnR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn irq_acomp1_en(&self) -> IrqAcomp1EnR {
        IrqAcomp1EnR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pin_wakeup_sel(&self) -> PinWakeupSelR {
        PinWakeupSelR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin_wakeup_en(&self) -> PinWakeupEnR {
        PinWakeupEnR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mode(&mut self) -> HbnPinWakeupModeW<'_, HbnIrqModeSpec> {
        HbnPinWakeupModeW::new(self, 0)
    }
    #[doc = "Bits 4:12"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mask(&mut self) -> HbnPinWakeupMaskW<'_, HbnIrqModeSpec> {
        HbnPinWakeupMaskW::new(self, 4)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_en_hw_pu_pd(&mut self) -> RegEnHwPuPdW<'_, HbnIrqModeSpec> {
        RegEnHwPuPdW::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn irq_bor_en(&mut self) -> IrqBorEnW<'_, HbnIrqModeSpec> {
        IrqBorEnW::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn irq_acomp0_en(&mut self) -> IrqAcomp0EnW<'_, HbnIrqModeSpec> {
        IrqAcomp0EnW::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn irq_acomp1_en(&mut self) -> IrqAcomp1EnW<'_, HbnIrqModeSpec> {
        IrqAcomp1EnW::new(self, 22)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pin_wakeup_sel(&mut self) -> PinWakeupSelW<'_, HbnIrqModeSpec> {
        PinWakeupSelW::new(self, 24)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin_wakeup_en(&mut self) -> PinWakeupEnW<'_, HbnIrqModeSpec> {
        PinWakeupEnW::new(self, 27)
    }
}
#[doc = "HBN Interrupt Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_irq_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_irq_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnIrqModeSpec;
impl crate::RegisterSpec for HbnIrqModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_irq_mode::R`](R) reader structure"]
impl crate::Readable for HbnIrqModeSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_irq_mode::W`](W) writer structure"]
impl crate::Writable for HbnIrqModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_IRQ_MODE to value 0x0301_0005"]
impl crate::Resettable for HbnIrqModeSpec {
    const RESET_VALUE: u32 = 0x0301_0005;
}
