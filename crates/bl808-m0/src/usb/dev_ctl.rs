#[doc = "Register `DEV_CTL` reader"]
pub type R = crate::R<DevCtlSpec>;
#[doc = "Register `DEV_CTL` writer"]
pub type W = crate::W<DevCtlSpec>;
#[doc = "Field `CAP_RMWAKUP` reader - Capability Remote Wakeup"]
pub type CapRmwakupR = crate::BitReader;
#[doc = "Field `HALF_SPEED_HOV` reader - Half Speed Enable (Host Overwrite)"]
pub type HalfSpeedHovR = crate::BitReader;
#[doc = "Field `HALF_SPEED_HOV` writer - Half Speed Enable (Host Overwrite)"]
pub type HalfSpeedHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLINT_EN_HOV` reader - Global Interrupt Enable (Host Overwrite)"]
pub type GlintEnHovR = crate::BitReader;
#[doc = "Field `GLINT_EN_HOV` writer - Global Interrupt Enable (Host Overwrite)"]
pub type GlintEnHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOSUSP` reader - Global Suspend"]
pub type GosuspR = crate::BitReader;
#[doc = "Field `GOSUSP` writer - Global Suspend"]
pub type GosuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFRST_HOV` reader - Soft Reset (Host Overwrite)"]
pub type SfrstHovR = crate::BitReader;
#[doc = "Field `SFRST_HOV` writer - Soft Reset (Host Overwrite)"]
pub type SfrstHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHIP_EN_HOV` reader - Chip Enable (Host Overwrite)"]
pub type ChipEnHovR = crate::BitReader;
#[doc = "Field `CHIP_EN_HOV` writer - Chip Enable (Host Overwrite)"]
pub type ChipEnHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_EN_HOV` reader - High Speed Enable (Host Overwrite)"]
pub type HsEnHovR = crate::BitReader;
#[doc = "Field `HS_EN_HOV` writer - High Speed Enable (Host Overwrite)"]
pub type HsEnHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSBUS_WIDTH_HOV` reader - System Bus Width (Host Overwrite)"]
pub type SysbusWidthHovR = crate::BitReader;
#[doc = "Field `SYSBUS_WIDTH_HOV` writer - System Bus Width (Host Overwrite)"]
pub type SysbusWidthHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_FS` reader - Force Full Speed Mode"]
pub type ForceFsR = crate::BitReader;
#[doc = "Field `FORCE_FS` writer - Force Full Speed Mode"]
pub type ForceFsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_DEGLITCH_HOV` reader - Idle Deglitch time (Host Overwrite)"]
pub type IdleDeglitchHovR = crate::FieldReader;
#[doc = "Field `IDLE_DEGLITCH_HOV` writer - Idle Deglitch time (Host Overwrite)"]
pub type IdleDeglitchHovW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPM_BESL_MAX` reader - LPM Best Effort Service Latency Max"]
pub type LpmBeslMaxR = crate::FieldReader;
#[doc = "Field `LPM_BESL_MIN` reader - LPM Best Effort Service Latency Min"]
pub type LpmBeslMinR = crate::FieldReader;
#[doc = "Field `LPM_BESL` reader - LPM Best Effort Service Latency"]
pub type LpmBeslR = crate::FieldReader;
#[doc = "Field `LPM_EN` reader - LPM Enable"]
pub type LpmEnR = crate::BitReader;
#[doc = "Field `LPM_EN` writer - LPM Enable"]
pub type LpmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_ACCEPT` reader - LPM Accept"]
pub type LpmAcceptR = crate::BitReader;
#[doc = "Field `LPM_ACCEPT` writer - LPM Accept"]
pub type LpmAcceptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capability Remote Wakeup"]
    #[inline(always)]
    pub fn cap_rmwakup(&self) -> CapRmwakupR {
        CapRmwakupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Half Speed Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn half_speed_hov(&self) -> HalfSpeedHovR {
        HalfSpeedHovR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Interrupt Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn glint_en_hov(&self) -> GlintEnHovR {
        GlintEnHovR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global Suspend"]
    #[inline(always)]
    pub fn gosusp(&self) -> GosuspR {
        GosuspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Soft Reset (Host Overwrite)"]
    #[inline(always)]
    pub fn sfrst_hov(&self) -> SfrstHovR {
        SfrstHovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Chip Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn chip_en_hov(&self) -> ChipEnHovR {
        ChipEnHovR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High Speed Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn hs_en_hov(&self) -> HsEnHovR {
        HsEnHovR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - System Bus Width (Host Overwrite)"]
    #[inline(always)]
    pub fn sysbus_width_hov(&self) -> SysbusWidthHovR {
        SysbusWidthHovR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Force Full Speed Mode"]
    #[inline(always)]
    pub fn force_fs(&self) -> ForceFsR {
        ForceFsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Idle Deglitch time (Host Overwrite)"]
    #[inline(always)]
    pub fn idle_deglitch_hov(&self) -> IdleDeglitchHovR {
        IdleDeglitchHovR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - LPM Best Effort Service Latency Max"]
    #[inline(always)]
    pub fn lpm_besl_max(&self) -> LpmBeslMaxR {
        LpmBeslMaxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - LPM Best Effort Service Latency Min"]
    #[inline(always)]
    pub fn lpm_besl_min(&self) -> LpmBeslMinR {
        LpmBeslMinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - LPM Best Effort Service Latency"]
    #[inline(always)]
    pub fn lpm_besl(&self) -> LpmBeslR {
        LpmBeslR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - LPM Enable"]
    #[inline(always)]
    pub fn lpm_en(&self) -> LpmEnR {
        LpmEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LPM Accept"]
    #[inline(always)]
    pub fn lpm_accept(&self) -> LpmAcceptR {
        LpmAcceptR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Half Speed Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn half_speed_hov(&mut self) -> HalfSpeedHovW<'_, DevCtlSpec> {
        HalfSpeedHovW::new(self, 1)
    }
    #[doc = "Bit 2 - Global Interrupt Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn glint_en_hov(&mut self) -> GlintEnHovW<'_, DevCtlSpec> {
        GlintEnHovW::new(self, 2)
    }
    #[doc = "Bit 3 - Global Suspend"]
    #[inline(always)]
    pub fn gosusp(&mut self) -> GosuspW<'_, DevCtlSpec> {
        GosuspW::new(self, 3)
    }
    #[doc = "Bit 4 - Soft Reset (Host Overwrite)"]
    #[inline(always)]
    pub fn sfrst_hov(&mut self) -> SfrstHovW<'_, DevCtlSpec> {
        SfrstHovW::new(self, 4)
    }
    #[doc = "Bit 5 - Chip Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn chip_en_hov(&mut self) -> ChipEnHovW<'_, DevCtlSpec> {
        ChipEnHovW::new(self, 5)
    }
    #[doc = "Bit 6 - High Speed Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn hs_en_hov(&mut self) -> HsEnHovW<'_, DevCtlSpec> {
        HsEnHovW::new(self, 6)
    }
    #[doc = "Bit 7 - System Bus Width (Host Overwrite)"]
    #[inline(always)]
    pub fn sysbus_width_hov(&mut self) -> SysbusWidthHovW<'_, DevCtlSpec> {
        SysbusWidthHovW::new(self, 7)
    }
    #[doc = "Bit 9 - Force Full Speed Mode"]
    #[inline(always)]
    pub fn force_fs(&mut self) -> ForceFsW<'_, DevCtlSpec> {
        ForceFsW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Idle Deglitch time (Host Overwrite)"]
    #[inline(always)]
    pub fn idle_deglitch_hov(&mut self) -> IdleDeglitchHovW<'_, DevCtlSpec> {
        IdleDeglitchHovW::new(self, 10)
    }
    #[doc = "Bit 25 - LPM Enable"]
    #[inline(always)]
    pub fn lpm_en(&mut self) -> LpmEnW<'_, DevCtlSpec> {
        LpmEnW::new(self, 25)
    }
    #[doc = "Bit 26 - LPM Accept"]
    #[inline(always)]
    pub fn lpm_accept(&mut self) -> LpmAcceptW<'_, DevCtlSpec> {
        LpmAcceptW::new(self, 26)
    }
}
#[doc = "Device Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCtlSpec;
impl crate::RegisterSpec for DevCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_ctl::R`](R) reader structure"]
impl crate::Readable for DevCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_ctl::W`](W) writer structure"]
impl crate::Writable for DevCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_CTL to value 0"]
impl crate::Resettable for DevCtlSpec {}
