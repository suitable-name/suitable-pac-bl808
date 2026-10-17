#[doc = "Register `dac_0` reader"]
pub type R = crate::R<Dac0Spec>;
#[doc = "Register `dac_0` writer"]
pub type W = crate::W<Dac0Spec>;
#[doc = "Field `adc_pdm_h` reader - "]
pub type AdcPdmHR = crate::FieldReader;
#[doc = "Field `adc_pdm_h` writer - "]
pub type AdcPdmHW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `adc_pdm_l` reader - "]
pub type AdcPdmLR = crate::FieldReader;
#[doc = "Field `adc_pdm_l` writer - "]
pub type AdcPdmLW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `adc_0_src` reader - "]
pub type Adc0SrcR = crate::BitReader;
#[doc = "Field `adc_0_src` writer - "]
pub type Adc0SrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_pdm_lvl_swap` reader - "]
pub type Adc0PdmLvlSwapR = crate::BitReader;
#[doc = "Field `adc_0_pdm_lvl_swap` writer - "]
pub type Adc0PdmLvlSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_0_mash_bit_swap` reader - "]
pub type Adc0MashBitSwapR = crate::BitReader;
#[doc = "Field `adc_0_mash_bit_swap` writer - "]
pub type Adc0MashBitSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_src` reader - "]
pub type Adc1SrcR = crate::BitReader;
#[doc = "Field `adc_1_src` writer - "]
pub type Adc1SrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_pdm_lvl_swap` reader - "]
pub type Adc1PdmLvlSwapR = crate::BitReader;
#[doc = "Field `adc_1_pdm_lvl_swap` writer - "]
pub type Adc1PdmLvlSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_1_mash_bit_swap` reader - "]
pub type Adc1MashBitSwapR = crate::BitReader;
#[doc = "Field `adc_1_mash_bit_swap` writer - "]
pub type Adc1MashBitSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_2_src` reader - "]
pub type Adc2SrcR = crate::BitReader;
#[doc = "Field `adc_2_src` writer - "]
pub type Adc2SrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_2_pdm_lvl_swap` reader - "]
pub type Adc2PdmLvlSwapR = crate::BitReader;
#[doc = "Field `adc_2_pdm_lvl_swap` writer - "]
pub type Adc2PdmLvlSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_2_mash_bit_swap` reader - "]
pub type Adc2MashBitSwapR = crate::BitReader;
#[doc = "Field `adc_2_mash_bit_swap` writer - "]
pub type Adc2MashBitSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mix_0_sel` reader - "]
pub type Mix0SelR = crate::FieldReader;
#[doc = "Field `mix_0_sel` writer - "]
pub type Mix0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mix_0_mode` reader - "]
pub type Mix0ModeR = crate::FieldReader;
#[doc = "Field `mix_0_mode` writer - "]
pub type Mix0ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mix_0_att_mode1` reader - "]
pub type Mix0AttMode1R = crate::FieldReader;
#[doc = "Field `mix_0_att_mode1` writer - "]
pub type Mix0AttMode1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `mix_0_att_mode2` reader - "]
pub type Mix0AttMode2R = crate::FieldReader;
#[doc = "Field `mix_0_att_mode2` writer - "]
pub type Mix0AttMode2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn adc_pdm_h(&self) -> AdcPdmHR {
        AdcPdmHR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn adc_pdm_l(&self) -> AdcPdmLR {
        AdcPdmLR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_0_src(&self) -> Adc0SrcR {
        Adc0SrcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_0_pdm_lvl_swap(&self) -> Adc0PdmLvlSwapR {
        Adc0PdmLvlSwapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adc_0_mash_bit_swap(&self) -> Adc0MashBitSwapR {
        Adc0MashBitSwapR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adc_1_src(&self) -> Adc1SrcR {
        Adc1SrcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adc_1_pdm_lvl_swap(&self) -> Adc1PdmLvlSwapR {
        Adc1PdmLvlSwapR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adc_1_mash_bit_swap(&self) -> Adc1MashBitSwapR {
        Adc1MashBitSwapR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adc_2_src(&self) -> Adc2SrcR {
        Adc2SrcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adc_2_pdm_lvl_swap(&self) -> Adc2PdmLvlSwapR {
        Adc2PdmLvlSwapR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_2_mash_bit_swap(&self) -> Adc2MashBitSwapR {
        Adc2MashBitSwapR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn mix_0_sel(&self) -> Mix0SelR {
        Mix0SelR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn mix_0_mode(&self) -> Mix0ModeR {
        Mix0ModeR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn mix_0_att_mode1(&self) -> Mix0AttMode1R {
        Mix0AttMode1R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn mix_0_att_mode2(&self) -> Mix0AttMode2R {
        Mix0AttMode2R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn adc_pdm_h(&mut self) -> AdcPdmHW<'_, Dac0Spec> {
        AdcPdmHW::new(self, 0)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn adc_pdm_l(&mut self) -> AdcPdmLW<'_, Dac0Spec> {
        AdcPdmLW::new(self, 6)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adc_0_src(&mut self) -> Adc0SrcW<'_, Dac0Spec> {
        Adc0SrcW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_0_pdm_lvl_swap(&mut self) -> Adc0PdmLvlSwapW<'_, Dac0Spec> {
        Adc0PdmLvlSwapW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adc_0_mash_bit_swap(&mut self) -> Adc0MashBitSwapW<'_, Dac0Spec> {
        Adc0MashBitSwapW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adc_1_src(&mut self) -> Adc1SrcW<'_, Dac0Spec> {
        Adc1SrcW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adc_1_pdm_lvl_swap(&mut self) -> Adc1PdmLvlSwapW<'_, Dac0Spec> {
        Adc1PdmLvlSwapW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adc_1_mash_bit_swap(&mut self) -> Adc1MashBitSwapW<'_, Dac0Spec> {
        Adc1MashBitSwapW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adc_2_src(&mut self) -> Adc2SrcW<'_, Dac0Spec> {
        Adc2SrcW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adc_2_pdm_lvl_swap(&mut self) -> Adc2PdmLvlSwapW<'_, Dac0Spec> {
        Adc2PdmLvlSwapW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adc_2_mash_bit_swap(&mut self) -> Adc2MashBitSwapW<'_, Dac0Spec> {
        Adc2MashBitSwapW::new(self, 20)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn mix_0_sel(&mut self) -> Mix0SelW<'_, Dac0Spec> {
        Mix0SelW::new(self, 21)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn mix_0_mode(&mut self) -> Mix0ModeW<'_, Dac0Spec> {
        Mix0ModeW::new(self, 23)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn mix_0_att_mode1(&mut self) -> Mix0AttMode1W<'_, Dac0Spec> {
        Mix0AttMode1W::new(self, 25)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn mix_0_att_mode2(&mut self) -> Mix0AttMode2W<'_, Dac0Spec> {
        Mix0AttMode2W::new(self, 28)
    }
}
#[doc = "DAC Configuration 0: Digital-to-Analog conversion parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`dac_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac0Spec;
impl crate::RegisterSpec for Dac0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_0::R`](R) reader structure"]
impl crate::Readable for Dac0Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_0::W`](W) writer structure"]
impl crate::Writable for Dac0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets dac_0 to value 0"]
impl crate::Resettable for Dac0Spec {}
