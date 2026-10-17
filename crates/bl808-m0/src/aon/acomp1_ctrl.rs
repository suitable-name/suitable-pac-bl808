#[doc = "Register `ACOMP1_CTRL` reader"]
pub type R = crate::R<Acomp1CtrlSpec>;
#[doc = "Register `ACOMP1_CTRL` writer"]
pub type W = crate::W<Acomp1CtrlSpec>;
#[doc = "Field `acomp1_en` reader - "]
pub type Acomp1EnR = crate::BitReader;
#[doc = "Field `acomp1_en` writer - "]
pub type Acomp1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp1_hyst_seln` reader - "]
pub type Acomp1HystSelnR = crate::FieldReader;
#[doc = "Field `acomp1_hyst_seln` writer - "]
pub type Acomp1HystSelnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `acomp1_hyst_selp` reader - "]
pub type Acomp1HystSelpR = crate::FieldReader;
#[doc = "Field `acomp1_hyst_selp` writer - "]
pub type Acomp1HystSelpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `acomp1_bias_prog` reader - "]
pub type Acomp1BiasProgR = crate::FieldReader;
#[doc = "Field `acomp1_bias_prog` writer - "]
pub type Acomp1BiasProgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `acomp1_level_sel` reader - "]
pub type Acomp1LevelSelR = crate::FieldReader;
#[doc = "Field `acomp1_level_sel` writer - "]
pub type Acomp1LevelSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `acomp1_neg_sel` reader - "]
pub type Acomp1NegSelR = crate::FieldReader;
#[doc = "Field `acomp1_neg_sel` writer - "]
pub type Acomp1NegSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `acomp1_pos_sel` reader - "]
pub type Acomp1PosSelR = crate::FieldReader;
#[doc = "Field `acomp1_pos_sel` writer - "]
pub type Acomp1PosSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `acomp1_muxen` reader - "]
pub type Acomp1MuxenR = crate::BitReader;
#[doc = "Field `acomp1_muxen` writer - "]
pub type Acomp1MuxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_en(&self) -> Acomp1EnR {
        Acomp1EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn acomp1_hyst_seln(&self) -> Acomp1HystSelnR {
        Acomp1HystSelnR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn acomp1_hyst_selp(&self) -> Acomp1HystSelpR {
        Acomp1HystSelpR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_bias_prog(&self) -> Acomp1BiasProgR {
        Acomp1BiasProgR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn acomp1_level_sel(&self) -> Acomp1LevelSelR {
        Acomp1LevelSelR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn acomp1_neg_sel(&self) -> Acomp1NegSelR {
        Acomp1NegSelR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn acomp1_pos_sel(&self) -> Acomp1PosSelR {
        Acomp1PosSelR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn acomp1_muxen(&self) -> Acomp1MuxenR {
        Acomp1MuxenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_en(&mut self) -> Acomp1EnW<'_, Acomp1CtrlSpec> {
        Acomp1EnW::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn acomp1_hyst_seln(&mut self) -> Acomp1HystSelnW<'_, Acomp1CtrlSpec> {
        Acomp1HystSelnW::new(self, 4)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn acomp1_hyst_selp(&mut self) -> Acomp1HystSelpW<'_, Acomp1CtrlSpec> {
        Acomp1HystSelpW::new(self, 7)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_bias_prog(&mut self) -> Acomp1BiasProgW<'_, Acomp1CtrlSpec> {
        Acomp1BiasProgW::new(self, 10)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn acomp1_level_sel(&mut self) -> Acomp1LevelSelW<'_, Acomp1CtrlSpec> {
        Acomp1LevelSelW::new(self, 12)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn acomp1_neg_sel(&mut self) -> Acomp1NegSelW<'_, Acomp1CtrlSpec> {
        Acomp1NegSelW::new(self, 18)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn acomp1_pos_sel(&mut self) -> Acomp1PosSelW<'_, Acomp1CtrlSpec> {
        Acomp1PosSelW::new(self, 22)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn acomp1_muxen(&mut self) -> Acomp1MuxenW<'_, Acomp1CtrlSpec> {
        Acomp1MuxenW::new(self, 26)
    }
}
#[doc = "Analog Comparator 1 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`acomp1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acomp1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Acomp1CtrlSpec;
impl crate::RegisterSpec for Acomp1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acomp1_ctrl::R`](R) reader structure"]
impl crate::Readable for Acomp1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`acomp1_ctrl::W`](W) writer structure"]
impl crate::Writable for Acomp1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACOMP1_CTRL to value 0"]
impl crate::Resettable for Acomp1CtrlSpec {}
