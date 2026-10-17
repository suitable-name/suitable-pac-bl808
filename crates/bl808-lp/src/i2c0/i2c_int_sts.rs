#[doc = "Register `I2C_INT_STS` reader"]
pub type R = crate::R<I2cIntStsSpec>;
#[doc = "Register `I2C_INT_STS` writer"]
pub type W = crate::W<I2cIntStsSpec>;
#[doc = "Field `i2c_end_int` reader - Transfer end interrupt status"]
pub type I2cEndIntR = crate::BitReader;
#[doc = "Field `i2c_txf_int` reader - TX FIFO ready interrupt status"]
pub type I2cTxfIntR = crate::BitReader;
#[doc = "Field `i2c_rxf_int` reader - RX FIFO ready interrupt status"]
pub type I2cRxfIntR = crate::BitReader;
#[doc = "Field `i2c_nak_int` reader - NAK received interrupt status"]
pub type I2cNakIntR = crate::BitReader;
#[doc = "Field `i2c_arb_int` reader - Arbitration lost interrupt status"]
pub type I2cArbIntR = crate::BitReader;
#[doc = "Field `i2c_fer_int` reader - FIFO error interrupt status"]
pub type I2cFerIntR = crate::BitReader;
#[doc = "Field `cr_i2c_end_mask` reader - End interrupt mask"]
pub type CrI2cEndMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_end_mask` writer - End interrupt mask"]
pub type CrI2cEndMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_txf_mask` reader - TX FIFO interrupt mask"]
pub type CrI2cTxfMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_txf_mask` writer - TX FIFO interrupt mask"]
pub type CrI2cTxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_rxf_mask` reader - RX FIFO interrupt mask"]
pub type CrI2cRxfMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_rxf_mask` writer - RX FIFO interrupt mask"]
pub type CrI2cRxfMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_nak_mask` reader - NAK interrupt mask"]
pub type CrI2cNakMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_nak_mask` writer - NAK interrupt mask"]
pub type CrI2cNakMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_arb_mask` reader - Arbitration lost interrupt mask"]
pub type CrI2cArbMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_arb_mask` writer - Arbitration lost interrupt mask"]
pub type CrI2cArbMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_fer_mask` reader - FIFO error interrupt mask"]
pub type CrI2cFerMaskR = crate::BitReader;
#[doc = "Field `cr_i2c_fer_mask` writer - FIFO error interrupt mask"]
pub type CrI2cFerMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_end_clr` writer - Clear end interrupt"]
pub type CrI2cEndClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_i2c_nak_clr` writer - Clear NAK interrupt"]
pub type CrI2cNakClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_i2c_arb_clr` writer - Clear arbitration lost interrupt"]
pub type CrI2cArbClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `cr_i2c_end_en` reader - End interrupt enable"]
pub type CrI2cEndEnR = crate::BitReader;
#[doc = "Field `cr_i2c_end_en` writer - End interrupt enable"]
pub type CrI2cEndEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_txf_en` reader - TX FIFO interrupt enable"]
pub type CrI2cTxfEnR = crate::BitReader;
#[doc = "Field `cr_i2c_txf_en` writer - TX FIFO interrupt enable"]
pub type CrI2cTxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_rxf_en` reader - RX FIFO interrupt enable"]
pub type CrI2cRxfEnR = crate::BitReader;
#[doc = "Field `cr_i2c_rxf_en` writer - RX FIFO interrupt enable"]
pub type CrI2cRxfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_nak_en` reader - NAK interrupt enable"]
pub type CrI2cNakEnR = crate::BitReader;
#[doc = "Field `cr_i2c_nak_en` writer - NAK interrupt enable"]
pub type CrI2cNakEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_arb_en` reader - Arbitration lost interrupt enable"]
pub type CrI2cArbEnR = crate::BitReader;
#[doc = "Field `cr_i2c_arb_en` writer - Arbitration lost interrupt enable"]
pub type CrI2cArbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_i2c_fer_en` reader - FIFO error interrupt enable"]
pub type CrI2cFerEnR = crate::BitReader;
#[doc = "Field `cr_i2c_fer_en` writer - FIFO error interrupt enable"]
pub type CrI2cFerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer end interrupt status"]
    #[inline(always)]
    pub fn i2c_end_int(&self) -> I2cEndIntR {
        I2cEndIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO ready interrupt status"]
    #[inline(always)]
    pub fn i2c_txf_int(&self) -> I2cTxfIntR {
        I2cTxfIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO ready interrupt status"]
    #[inline(always)]
    pub fn i2c_rxf_int(&self) -> I2cRxfIntR {
        I2cRxfIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAK received interrupt status"]
    #[inline(always)]
    pub fn i2c_nak_int(&self) -> I2cNakIntR {
        I2cNakIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt status"]
    #[inline(always)]
    pub fn i2c_arb_int(&self) -> I2cArbIntR {
        I2cArbIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO error interrupt status"]
    #[inline(always)]
    pub fn i2c_fer_int(&self) -> I2cFerIntR {
        I2cFerIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - End interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_end_mask(&self) -> CrI2cEndMaskR {
        CrI2cEndMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX FIFO interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_txf_mask(&self) -> CrI2cTxfMaskR {
        CrI2cTxfMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RX FIFO interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_rxf_mask(&self) -> CrI2cRxfMaskR {
        CrI2cRxfMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NAK interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_nak_mask(&self) -> CrI2cNakMaskR {
        CrI2cNakMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Arbitration lost interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_arb_mask(&self) -> CrI2cArbMaskR {
        CrI2cArbMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FIFO error interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_fer_mask(&self) -> CrI2cFerMaskR {
        CrI2cFerMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - End interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_end_en(&self) -> CrI2cEndEnR {
        CrI2cEndEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TX FIFO interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_txf_en(&self) -> CrI2cTxfEnR {
        CrI2cTxfEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RX FIFO interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_rxf_en(&self) -> CrI2cRxfEnR {
        CrI2cRxfEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - NAK interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_nak_en(&self) -> CrI2cNakEnR {
        CrI2cNakEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_arb_en(&self) -> CrI2cArbEnR {
        CrI2cArbEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_fer_en(&self) -> CrI2cFerEnR {
        CrI2cFerEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - End interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_end_mask(&mut self) -> CrI2cEndMaskW<'_, I2cIntStsSpec> {
        CrI2cEndMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - TX FIFO interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_txf_mask(&mut self) -> CrI2cTxfMaskW<'_, I2cIntStsSpec> {
        CrI2cTxfMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - RX FIFO interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_rxf_mask(&mut self) -> CrI2cRxfMaskW<'_, I2cIntStsSpec> {
        CrI2cRxfMaskW::new(self, 10)
    }
    #[doc = "Bit 11 - NAK interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_nak_mask(&mut self) -> CrI2cNakMaskW<'_, I2cIntStsSpec> {
        CrI2cNakMaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Arbitration lost interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_arb_mask(&mut self) -> CrI2cArbMaskW<'_, I2cIntStsSpec> {
        CrI2cArbMaskW::new(self, 12)
    }
    #[doc = "Bit 13 - FIFO error interrupt mask"]
    #[inline(always)]
    pub fn cr_i2c_fer_mask(&mut self) -> CrI2cFerMaskW<'_, I2cIntStsSpec> {
        CrI2cFerMaskW::new(self, 13)
    }
    #[doc = "Bit 16 - Clear end interrupt"]
    #[inline(always)]
    pub fn cr_i2c_end_clr(&mut self) -> CrI2cEndClrW<'_, I2cIntStsSpec> {
        CrI2cEndClrW::new(self, 16)
    }
    #[doc = "Bit 19 - Clear NAK interrupt"]
    #[inline(always)]
    pub fn cr_i2c_nak_clr(&mut self) -> CrI2cNakClrW<'_, I2cIntStsSpec> {
        CrI2cNakClrW::new(self, 19)
    }
    #[doc = "Bit 20 - Clear arbitration lost interrupt"]
    #[inline(always)]
    pub fn cr_i2c_arb_clr(&mut self) -> CrI2cArbClrW<'_, I2cIntStsSpec> {
        CrI2cArbClrW::new(self, 20)
    }
    #[doc = "Bit 24 - End interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_end_en(&mut self) -> CrI2cEndEnW<'_, I2cIntStsSpec> {
        CrI2cEndEnW::new(self, 24)
    }
    #[doc = "Bit 25 - TX FIFO interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_txf_en(&mut self) -> CrI2cTxfEnW<'_, I2cIntStsSpec> {
        CrI2cTxfEnW::new(self, 25)
    }
    #[doc = "Bit 26 - RX FIFO interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_rxf_en(&mut self) -> CrI2cRxfEnW<'_, I2cIntStsSpec> {
        CrI2cRxfEnW::new(self, 26)
    }
    #[doc = "Bit 27 - NAK interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_nak_en(&mut self) -> CrI2cNakEnW<'_, I2cIntStsSpec> {
        CrI2cNakEnW::new(self, 27)
    }
    #[doc = "Bit 28 - Arbitration lost interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_arb_en(&mut self) -> CrI2cArbEnW<'_, I2cIntStsSpec> {
        CrI2cArbEnW::new(self, 28)
    }
    #[doc = "Bit 29 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn cr_i2c_fer_en(&mut self) -> CrI2cFerEnW<'_, I2cIntStsSpec> {
        CrI2cFerEnW::new(self, 29)
    }
}
#[doc = "Interrupt Status, Mask, and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_int_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_int_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cIntStsSpec;
impl crate::RegisterSpec for I2cIntStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_int_sts::R`](R) reader structure"]
impl crate::Readable for I2cIntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_int_sts::W`](W) writer structure"]
impl crate::Writable for I2cIntStsSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0019_0000;
}
#[doc = "`reset()` method sets I2C_INT_STS to value 0x3f00_3f02"]
impl crate::Resettable for I2cIntStsSpec {
    const RESET_VALUE: u32 = 0x3f00_3f02;
}
