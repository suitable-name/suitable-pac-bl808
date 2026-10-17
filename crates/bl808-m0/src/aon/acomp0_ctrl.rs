#[doc = "Register `ACOMP0_CTRL` reader"]
pub type R = crate::R<Acomp0CtrlSpec>;
#[doc = "Register `ACOMP0_CTRL` writer"]
pub type W = crate::W<Acomp0CtrlSpec>;
#[doc = "Field `acomp0_en` reader - "]
pub type Acomp0EnR = crate::BitReader;
#[doc = "Field `acomp0_en` writer - "]
pub type Acomp0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp0_hyst_seln` reader - "]
pub type Acomp0HystSelnR = crate::FieldReader;
#[doc = "Field `acomp0_hyst_seln` writer - "]
pub type Acomp0HystSelnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `acomp0_hyst_selp` reader - "]
pub type Acomp0HystSelpR = crate::FieldReader;
#[doc = "Field `acomp0_hyst_selp` writer - "]
pub type Acomp0HystSelpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `acomp0_bias_prog` reader - "]
pub type Acomp0BiasProgR = crate::FieldReader;
#[doc = "Field `acomp0_bias_prog` writer - "]
pub type Acomp0BiasProgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `acomp0_level_sel` reader - "]
pub type Acomp0LevelSelR = crate::FieldReader;
#[doc = "Field `acomp0_level_sel` writer - "]
pub type Acomp0LevelSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `acomp0_neg_sel` reader - "]
pub type Acomp0NegSelR = crate::FieldReader;
#[doc = "Field `acomp0_neg_sel` writer - "]
pub type Acomp0NegSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `acomp0_pos_sel` reader - "]
pub type Acomp0PosSelR = crate::FieldReader;
#[doc = "Field `acomp0_pos_sel` writer - "]
pub type Acomp0PosSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `acomp0_muxen` reader - "]
pub type Acomp0MuxenR = crate::BitReader;
#[doc = "Field `acomp0_muxen` writer - "]
pub type Acomp0MuxenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp0_en(&self) -> Acomp0EnR {
        Acomp0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn acomp0_hyst_seln(&self) -> Acomp0HystSelnR {
        Acomp0HystSelnR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn acomp0_hyst_selp(&self) -> Acomp0HystSelpR {
        Acomp0HystSelpR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp0_bias_prog(&self) -> Acomp0BiasProgR {
        Acomp0BiasProgR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn acomp0_level_sel(&self) -> Acomp0LevelSelR {
        Acomp0LevelSelR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn acomp0_neg_sel(&self) -> Acomp0NegSelR {
        Acomp0NegSelR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn acomp0_pos_sel(&self) -> Acomp0PosSelR {
        Acomp0PosSelR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn acomp0_muxen(&self) -> Acomp0MuxenR {
        Acomp0MuxenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp0_en(&mut self) -> Acomp0EnW<'_, Acomp0CtrlSpec> {
        Acomp0EnW::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn acomp0_hyst_seln(&mut self) -> Acomp0HystSelnW<'_, Acomp0CtrlSpec> {
        Acomp0HystSelnW::new(self, 4)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn acomp0_hyst_selp(&mut self) -> Acomp0HystSelpW<'_, Acomp0CtrlSpec> {
        Acomp0HystSelpW::new(self, 7)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp0_bias_prog(&mut self) -> Acomp0BiasProgW<'_, Acomp0CtrlSpec> {
        Acomp0BiasProgW::new(self, 10)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn acomp0_level_sel(&mut self) -> Acomp0LevelSelW<'_, Acomp0CtrlSpec> {
        Acomp0LevelSelW::new(self, 12)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn acomp0_neg_sel(&mut self) -> Acomp0NegSelW<'_, Acomp0CtrlSpec> {
        Acomp0NegSelW::new(self, 18)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn acomp0_pos_sel(&mut self) -> Acomp0PosSelW<'_, Acomp0CtrlSpec> {
        Acomp0PosSelW::new(self, 22)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn acomp0_muxen(&mut self) -> Acomp0MuxenW<'_, Acomp0CtrlSpec> {
        Acomp0MuxenW::new(self, 26)
    }
}
#[doc = "Analog Comparator 0 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`acomp0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acomp0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Acomp0CtrlSpec;
impl crate::RegisterSpec for Acomp0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acomp0_ctrl::R`](R) reader structure"]
impl crate::Readable for Acomp0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`acomp0_ctrl::W`](W) writer structure"]
impl crate::Writable for Acomp0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACOMP0_CTRL to value 0"]
impl crate::Resettable for Acomp0CtrlSpec {}
