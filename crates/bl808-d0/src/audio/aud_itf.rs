#[doc = "Register `aud_itf` reader"]
pub type R = crate::R<AudItfSpec>;
#[doc = "Register `aud_itf` writer"]
pub type W = crate::W<AudItfSpec>;
#[doc = "Field `adc_0_en` reader - "]
pub type Adc0EnR = crate::BitReader;
#[doc = "Field `adc_0_en` writer - "]
pub type Adc0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_en` reader - "]
pub type Adc1EnR = crate::BitReader;
#[doc = "Field `adc_1_en` writer - "]
pub type Adc1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_2_en` reader - "]
pub type Adc2EnR = crate::BitReader;
#[doc = "Field `adc_2_en` writer - "]
pub type Adc2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_0_en` reader - "]
pub type Dac0EnR = crate::BitReader;
#[doc = "Field `dac_0_en` writer - "]
pub type Dac0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_1_en` reader - "]
pub type Dac1EnR = crate::BitReader;
#[doc = "Field `dac_1_en` writer - "]
pub type Dac1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aec_0_en` reader - "]
pub type Aec0EnR = crate::BitReader;
#[doc = "Field `aec_0_en` writer - "]
pub type Aec0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aec_1_en` reader - "]
pub type Aec1EnR = crate::BitReader;
#[doc = "Field `aec_1_en` writer - "]
pub type Aec1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aud_rx0_sel` reader - RX Channel 0 Source Select"]
pub type AudRx0SelR = crate::FieldReader;
#[doc = "Field `aud_rx0_sel` writer - RX Channel 0 Source Select"]
pub type AudRx0SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aud_rx1_sel` reader - RX Channel 1 Source Select"]
pub type AudRx1SelR = crate::FieldReader;
#[doc = "Field `aud_rx1_sel` writer - RX Channel 1 Source Select"]
pub type AudRx1SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aud_rx2_sel` reader - RX Channel 2 Source Select"]
pub type AudRx2SelR = crate::FieldReader;
#[doc = "Field `aud_rx2_sel` writer - RX Channel 2 Source Select"]
pub type AudRx2SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aud_rx3_sel` reader - RX Channel 3 Source Select"]
pub type AudRx3SelR = crate::FieldReader;
#[doc = "Field `aud_rx3_sel` writer - RX Channel 3 Source Select"]
pub type AudRx3SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aud_rx4_sel` reader - RX Channel 4 Source Select"]
pub type AudRx4SelR = crate::FieldReader;
#[doc = "Field `aud_rx4_sel` writer - RX Channel 4 Source Select"]
pub type AudRx4SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `aud_tx0_sel` reader - TX Channel 0 Source Select"]
pub type AudTx0SelR = crate::BitReader;
#[doc = "Field `aud_tx0_sel` writer - TX Channel 0 Source Select"]
pub type AudTx0SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aud_tx1_sel` reader - TX Channel 1 Source Select"]
pub type AudTx1SelR = crate::BitReader;
#[doc = "Field `aud_tx1_sel` writer - TX Channel 1 Source Select"]
pub type AudTx1SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_itf_en` reader - ADC Interface Enable"]
pub type AdcItfEnR = crate::BitReader;
#[doc = "Field `adc_itf_en` writer - ADC Interface Enable"]
pub type AdcItfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dac_itf_en` reader - DAC Interface Enable"]
pub type DacItfEnR = crate::BitReader;
#[doc = "Field `dac_itf_en` writer - DAC Interface Enable"]
pub type DacItfEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_0_en(&self) -> Adc0EnR {
        Adc0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_1_en(&self) -> Adc1EnR {
        Adc1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adc_2_en(&self) -> Adc2EnR {
        Adc2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dac_0_en(&self) -> Dac0EnR {
        Dac0EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dac_1_en(&self) -> Dac1EnR {
        Dac1EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn aec_0_en(&self) -> Aec0EnR {
        Aec0EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn aec_1_en(&self) -> Aec1EnR {
        Aec1EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 13:15 - RX Channel 0 Source Select"]
    #[inline(always)]
    pub fn aud_rx0_sel(&self) -> AudRx0SelR {
        AudRx0SelR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - RX Channel 1 Source Select"]
    #[inline(always)]
    pub fn aud_rx1_sel(&self) -> AudRx1SelR {
        AudRx1SelR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - RX Channel 2 Source Select"]
    #[inline(always)]
    pub fn aud_rx2_sel(&self) -> AudRx2SelR {
        AudRx2SelR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - RX Channel 3 Source Select"]
    #[inline(always)]
    pub fn aud_rx3_sel(&self) -> AudRx3SelR {
        AudRx3SelR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - RX Channel 4 Source Select"]
    #[inline(always)]
    pub fn aud_rx4_sel(&self) -> AudRx4SelR {
        AudRx4SelR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - TX Channel 0 Source Select"]
    #[inline(always)]
    pub fn aud_tx0_sel(&self) -> AudTx0SelR {
        AudTx0SelR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TX Channel 1 Source Select"]
    #[inline(always)]
    pub fn aud_tx1_sel(&self) -> AudTx1SelR {
        AudTx1SelR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ADC Interface Enable"]
    #[inline(always)]
    pub fn adc_itf_en(&self) -> AdcItfEnR {
        AdcItfEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DAC Interface Enable"]
    #[inline(always)]
    pub fn dac_itf_en(&self) -> DacItfEnR {
        DacItfEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc_0_en(&mut self) -> Adc0EnW<'_, AudItfSpec> {
        Adc0EnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc_1_en(&mut self) -> Adc1EnW<'_, AudItfSpec> {
        Adc1EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adc_2_en(&mut self) -> Adc2EnW<'_, AudItfSpec> {
        Adc2EnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dac_0_en(&mut self) -> Dac0EnW<'_, AudItfSpec> {
        Dac0EnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dac_1_en(&mut self) -> Dac1EnW<'_, AudItfSpec> {
        Dac1EnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn aec_0_en(&mut self) -> Aec0EnW<'_, AudItfSpec> {
        Aec0EnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn aec_1_en(&mut self) -> Aec1EnW<'_, AudItfSpec> {
        Aec1EnW::new(self, 6)
    }
    #[doc = "Bits 13:15 - RX Channel 0 Source Select"]
    #[inline(always)]
    pub fn aud_rx0_sel(&mut self) -> AudRx0SelW<'_, AudItfSpec> {
        AudRx0SelW::new(self, 13)
    }
    #[doc = "Bits 16:18 - RX Channel 1 Source Select"]
    #[inline(always)]
    pub fn aud_rx1_sel(&mut self) -> AudRx1SelW<'_, AudItfSpec> {
        AudRx1SelW::new(self, 16)
    }
    #[doc = "Bits 19:21 - RX Channel 2 Source Select"]
    #[inline(always)]
    pub fn aud_rx2_sel(&mut self) -> AudRx2SelW<'_, AudItfSpec> {
        AudRx2SelW::new(self, 19)
    }
    #[doc = "Bits 22:24 - RX Channel 3 Source Select"]
    #[inline(always)]
    pub fn aud_rx3_sel(&mut self) -> AudRx3SelW<'_, AudItfSpec> {
        AudRx3SelW::new(self, 22)
    }
    #[doc = "Bits 25:27 - RX Channel 4 Source Select"]
    #[inline(always)]
    pub fn aud_rx4_sel(&mut self) -> AudRx4SelW<'_, AudItfSpec> {
        AudRx4SelW::new(self, 25)
    }
    #[doc = "Bit 28 - TX Channel 0 Source Select"]
    #[inline(always)]
    pub fn aud_tx0_sel(&mut self) -> AudTx0SelW<'_, AudItfSpec> {
        AudTx0SelW::new(self, 28)
    }
    #[doc = "Bit 29 - TX Channel 1 Source Select"]
    #[inline(always)]
    pub fn aud_tx1_sel(&mut self) -> AudTx1SelW<'_, AudItfSpec> {
        AudTx1SelW::new(self, 29)
    }
    #[doc = "Bit 30 - ADC Interface Enable"]
    #[inline(always)]
    pub fn adc_itf_en(&mut self) -> AdcItfEnW<'_, AudItfSpec> {
        AdcItfEnW::new(self, 30)
    }
    #[doc = "Bit 31 - DAC Interface Enable"]
    #[inline(always)]
    pub fn dac_itf_en(&mut self) -> DacItfEnW<'_, AudItfSpec> {
        DacItfEnW::new(self, 31)
    }
}
#[doc = "Audio Interface Configuration: Channel routing and enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`aud_itf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aud_itf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AudItfSpec;
impl crate::RegisterSpec for AudItfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aud_itf::R`](R) reader structure"]
impl crate::Readable for AudItfSpec {}
#[doc = "`write(|w| ..)` method takes [`aud_itf::W`](W) writer structure"]
impl crate::Writable for AudItfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets aud_itf to value 0"]
impl crate::Resettable for AudItfSpec {}
