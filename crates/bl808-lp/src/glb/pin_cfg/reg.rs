#[doc = "Register `REG` reader"]
pub type R = crate::R<RegSpec>;
#[doc = "Register `REG` writer"]
pub type W = crate::W<RegSpec>;
#[doc = "Field `ie` reader - Input Enable"]
pub type IeR = crate::BitReader;
#[doc = "Field `ie` writer - Input Enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `smt` reader - Schmitt Trigger Enable"]
pub type SmtR = crate::BitReader;
#[doc = "Field `smt` writer - Schmitt Trigger Enable"]
pub type SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `drv` reader - Drive Strength (0-3)"]
pub type DrvR = crate::FieldReader;
#[doc = "Field `drv` writer - Drive Strength (0-3)"]
pub type DrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pu` reader - Pull-Up Enable"]
pub type PuR = crate::BitReader;
#[doc = "Field `pu` writer - Pull-Up Enable"]
pub type PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pd` reader - Pull-Down Enable"]
pub type PdR = crate::BitReader;
#[doc = "Field `pd` writer - Pull-Down Enable"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `oe` reader - Output Enable"]
pub type OeR = crate::BitReader;
#[doc = "Field `oe` writer - Output Enable"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pin Function Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Alternate {
    #[doc = "0: `0`"]
    Sdh = 0,
    #[doc = "1: `1`"]
    Spi0 = 1,
    #[doc = "2: `10`"]
    Flash = 2,
    #[doc = "3: `11`"]
    I2s = 3,
    #[doc = "4: `100`"]
    Pdm = 4,
    #[doc = "5: `101`"]
    I2c0 = 5,
    #[doc = "6: `110`"]
    I2c1 = 6,
    #[doc = "7: `111`"]
    Uart = 7,
    #[doc = "8: `1000`"]
    Emac = 8,
    #[doc = "9: `1001`"]
    Cam = 9,
    #[doc = "10: `1010`"]
    Analog = 10,
    #[doc = "11: `1011`"]
    Gpio = 11,
    #[doc = "16: `10000`"]
    Pwm0 = 16,
    #[doc = "17: `10001`"]
    Pwm1 = 17,
    #[doc = "18: `10010`"]
    Spi1 = 18,
    #[doc = "19: `10011`"]
    I2c2 = 19,
    #[doc = "20: `10100`"]
    I2c3 = 20,
}
impl From<Alternate> for u8 {
    #[inline(always)]
    fn from(variant: Alternate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Alternate {
    type Ux = u8;
}
impl crate::IsEnum for Alternate {}
#[doc = "Field `func_sel` reader - Pin Function Selection"]
pub type FuncSelR = crate::FieldReader<Alternate>;
impl FuncSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Alternate> {
        match self.bits {
            0 => Some(Alternate::Sdh),
            1 => Some(Alternate::Spi0),
            2 => Some(Alternate::Flash),
            3 => Some(Alternate::I2s),
            4 => Some(Alternate::Pdm),
            5 => Some(Alternate::I2c0),
            6 => Some(Alternate::I2c1),
            7 => Some(Alternate::Uart),
            8 => Some(Alternate::Emac),
            9 => Some(Alternate::Cam),
            10 => Some(Alternate::Analog),
            11 => Some(Alternate::Gpio),
            16 => Some(Alternate::Pwm0),
            17 => Some(Alternate::Pwm1),
            18 => Some(Alternate::Spi1),
            19 => Some(Alternate::I2c2),
            20 => Some(Alternate::I2c3),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_sdh(&self) -> bool {
        *self == Alternate::Sdh
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_spi0(&self) -> bool {
        *self == Alternate::Spi0
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == Alternate::Flash
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == Alternate::I2s
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_pdm(&self) -> bool {
        *self == Alternate::Pdm
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == Alternate::I2c0
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == Alternate::I2c1
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == Alternate::Uart
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_emac(&self) -> bool {
        *self == Alternate::Emac
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_cam(&self) -> bool {
        *self == Alternate::Cam
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == Alternate::Analog
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Alternate::Gpio
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == Alternate::Pwm0
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == Alternate::Pwm1
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_spi1(&self) -> bool {
        *self == Alternate::Spi1
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_i2c2(&self) -> bool {
        *self == Alternate::I2c2
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn is_i2c3(&self) -> bool {
        *self == Alternate::I2c3
    }
}
#[doc = "Field `func_sel` writer - Pin Function Selection"]
pub type FuncSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, Alternate>;
impl<'a, REG> FuncSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn sdh(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Sdh)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn spi0(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Spi0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Flash)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::I2s)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pdm(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Pdm)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::I2c0)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c1(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::I2c1)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Uart)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn emac(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Emac)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn cam(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Cam)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Analog)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Gpio)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Pwm0)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Pwm1)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn spi1(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::Spi1)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn i2c2(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::I2c2)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn i2c3(self) -> &'a mut crate::W<REG> {
        self.variant(Alternate::I2c3)
    }
}
#[doc = "Field `int_mode_set` reader - Interrupt Mode (0:Sync Low, 1:Sync High, 2:Sync Falling, 3:Sync Rising...)"]
pub type IntModeSetR = crate::FieldReader;
#[doc = "Field `int_mode_set` writer - Interrupt Mode (0:Sync Low, 1:Sync High, 2:Sync Falling, 3:Sync Rising...)"]
pub type IntModeSetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `int_clr` writer - Interrupt Clear"]
pub type IntClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `int_sts` reader - Interrupt Status"]
pub type IntStsR = crate::BitReader;
#[doc = "Field `int_mask` reader - Interrupt Mask (1=Masked)"]
pub type IntMaskR = crate::BitReader;
#[doc = "Field `int_mask` writer - Interrupt Mask (1=Masked)"]
pub type IntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_o` reader - Output Data Control and Readback"]
pub type RegOR = crate::BitReader;
#[doc = "Field `reg_o` writer - Output Data Control and Readback"]
pub type RegOW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_set` writer - Atomic Set"]
pub type RegSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_clr` writer - Atomic Clear"]
pub type RegClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_i` reader - Input Data Readback"]
pub type RegIR = crate::BitReader;
#[doc = "Field `mode` reader - Output Mode (0:Reg, 1:Set/Clr, 2:DMA, 3:DMA Set/Clr)"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `mode` writer - Output Mode (0:Reg, 1:Set/Clr, 2:DMA, 3:DMA Set/Clr)"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Input Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt Trigger Enable"]
    #[inline(always)]
    pub fn smt(&self) -> SmtR {
        SmtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Drive Strength (0-3)"]
    #[inline(always)]
    pub fn drv(&self) -> DrvR {
        DrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Pull-Up Enable"]
    #[inline(always)]
    pub fn pu(&self) -> PuR {
        PuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pull-Down Enable"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Enable"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Pin Function Selection"]
    #[inline(always)]
    pub fn func_sel(&self) -> FuncSelR {
        FuncSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Interrupt Mode (0:Sync Low, 1:Sync High, 2:Sync Falling, 3:Sync Rising...)"]
    #[inline(always)]
    pub fn int_mode_set(&self) -> IntModeSetR {
        IntModeSetR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Interrupt Status"]
    #[inline(always)]
    pub fn int_sts(&self) -> IntStsR {
        IntStsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Mask (1=Masked)"]
    #[inline(always)]
    pub fn int_mask(&self) -> IntMaskR {
        IntMaskR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Output Data Control and Readback"]
    #[inline(always)]
    pub fn reg_o(&self) -> RegOR {
        RegOR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Input Data Readback"]
    #[inline(always)]
    pub fn reg_i(&self) -> RegIR {
        RegIR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Output Mode (0:Reg, 1:Set/Clr, 2:DMA, 3:DMA Set/Clr)"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Input Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, RegSpec> {
        IeW::new(self, 0)
    }
    #[doc = "Bit 1 - Schmitt Trigger Enable"]
    #[inline(always)]
    pub fn smt(&mut self) -> SmtW<'_, RegSpec> {
        SmtW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Drive Strength (0-3)"]
    #[inline(always)]
    pub fn drv(&mut self) -> DrvW<'_, RegSpec> {
        DrvW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull-Up Enable"]
    #[inline(always)]
    pub fn pu(&mut self) -> PuW<'_, RegSpec> {
        PuW::new(self, 4)
    }
    #[doc = "Bit 5 - Pull-Down Enable"]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<'_, RegSpec> {
        PdW::new(self, 5)
    }
    #[doc = "Bit 6 - Output Enable"]
    #[inline(always)]
    pub fn oe(&mut self) -> OeW<'_, RegSpec> {
        OeW::new(self, 6)
    }
    #[doc = "Bits 8:12 - Pin Function Selection"]
    #[inline(always)]
    pub fn func_sel(&mut self) -> FuncSelW<'_, RegSpec> {
        FuncSelW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Interrupt Mode (0:Sync Low, 1:Sync High, 2:Sync Falling, 3:Sync Rising...)"]
    #[inline(always)]
    pub fn int_mode_set(&mut self) -> IntModeSetW<'_, RegSpec> {
        IntModeSetW::new(self, 16)
    }
    #[doc = "Bit 20 - Interrupt Clear"]
    #[inline(always)]
    pub fn int_clr(&mut self) -> IntClrW<'_, RegSpec> {
        IntClrW::new(self, 20)
    }
    #[doc = "Bit 22 - Interrupt Mask (1=Masked)"]
    #[inline(always)]
    pub fn int_mask(&mut self) -> IntMaskW<'_, RegSpec> {
        IntMaskW::new(self, 22)
    }
    #[doc = "Bit 24 - Output Data Control and Readback"]
    #[inline(always)]
    pub fn reg_o(&mut self) -> RegOW<'_, RegSpec> {
        RegOW::new(self, 24)
    }
    #[doc = "Bit 25 - Atomic Set"]
    #[inline(always)]
    pub fn reg_set(&mut self) -> RegSetW<'_, RegSpec> {
        RegSetW::new(self, 25)
    }
    #[doc = "Bit 26 - Atomic Clear"]
    #[inline(always)]
    pub fn reg_clr(&mut self) -> RegClrW<'_, RegSpec> {
        RegClrW::new(self, 26)
    }
    #[doc = "Bits 30:31 - Output Mode (0:Reg, 1:Set/Clr, 2:DMA, 3:DMA Set/Clr)"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, RegSpec> {
        ModeW::new(self, 30)
    }
}
#[doc = "GPIO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpec;
impl crate::RegisterSpec for RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg::R`](R) reader structure"]
impl crate::Readable for RegSpec {}
#[doc = "`write(|w| ..)` method takes [`reg::W`](W) writer structure"]
impl crate::Writable for RegSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0010_0000;
}
#[doc = "`reset()` method sets REG to value 0"]
impl crate::Resettable for RegSpec {}
