#[doc = "Register `TOUCH2` reader"]
pub type R = crate::R<Touch2Spec>;
#[doc = "Register `TOUCH2` writer"]
pub type W = crate::W<Touch2Spec>;
#[doc = "Field `touch_channel_sel` reader - "]
pub type TouchChannelSelR = crate::FieldReader;
#[doc = "Field `touch_channel_sel` writer - "]
pub type TouchChannelSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `touch_channel0_highz_en` reader - "]
pub type TouchChannel0HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel0_highz_en` writer - "]
pub type TouchChannel0HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel1_highz_en` reader - "]
pub type TouchChannel1HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel1_highz_en` writer - "]
pub type TouchChannel1HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel2_highz_en` reader - "]
pub type TouchChannel2HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel2_highz_en` writer - "]
pub type TouchChannel2HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel3_highz_en` reader - "]
pub type TouchChannel3HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel3_highz_en` writer - "]
pub type TouchChannel3HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel4_highz_en` reader - "]
pub type TouchChannel4HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel4_highz_en` writer - "]
pub type TouchChannel4HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel5_highz_en` reader - "]
pub type TouchChannel5HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel5_highz_en` writer - "]
pub type TouchChannel5HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel6_highz_en` reader - "]
pub type TouchChannel6HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel6_highz_en` writer - "]
pub type TouchChannel6HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel7_highz_en` reader - "]
pub type TouchChannel7HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel7_highz_en` writer - "]
pub type TouchChannel7HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel8_highz_en` reader - "]
pub type TouchChannel8HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel8_highz_en` writer - "]
pub type TouchChannel8HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel9_highz_en` reader - "]
pub type TouchChannel9HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel9_highz_en` writer - "]
pub type TouchChannel9HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel10_highz_en` reader - "]
pub type TouchChannel10HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel10_highz_en` writer - "]
pub type TouchChannel10HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_channel11_highz_en` reader - "]
pub type TouchChannel11HighzEnR = crate::BitReader;
#[doc = "Field `touch_channel11_highz_en` writer - "]
pub type TouchChannel11HighzEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn touch_channel_sel(&self) -> TouchChannelSelR {
        TouchChannelSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn touch_channel0_highz_en(&self) -> TouchChannel0HighzEnR {
        TouchChannel0HighzEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn touch_channel1_highz_en(&self) -> TouchChannel1HighzEnR {
        TouchChannel1HighzEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn touch_channel2_highz_en(&self) -> TouchChannel2HighzEnR {
        TouchChannel2HighzEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn touch_channel3_highz_en(&self) -> TouchChannel3HighzEnR {
        TouchChannel3HighzEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_channel4_highz_en(&self) -> TouchChannel4HighzEnR {
        TouchChannel4HighzEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_channel5_highz_en(&self) -> TouchChannel5HighzEnR {
        TouchChannel5HighzEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_channel6_highz_en(&self) -> TouchChannel6HighzEnR {
        TouchChannel6HighzEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_channel7_highz_en(&self) -> TouchChannel7HighzEnR {
        TouchChannel7HighzEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_channel8_highz_en(&self) -> TouchChannel8HighzEnR {
        TouchChannel8HighzEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_channel9_highz_en(&self) -> TouchChannel9HighzEnR {
        TouchChannel9HighzEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_channel10_highz_en(&self) -> TouchChannel10HighzEnR {
        TouchChannel10HighzEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_channel11_highz_en(&self) -> TouchChannel11HighzEnR {
        TouchChannel11HighzEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn touch_channel_sel(&mut self) -> TouchChannelSelW<'_, Touch2Spec> {
        TouchChannelSelW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn touch_channel0_highz_en(&mut self) -> TouchChannel0HighzEnW<'_, Touch2Spec> {
        TouchChannel0HighzEnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn touch_channel1_highz_en(&mut self) -> TouchChannel1HighzEnW<'_, Touch2Spec> {
        TouchChannel1HighzEnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn touch_channel2_highz_en(&mut self) -> TouchChannel2HighzEnW<'_, Touch2Spec> {
        TouchChannel2HighzEnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn touch_channel3_highz_en(&mut self) -> TouchChannel3HighzEnW<'_, Touch2Spec> {
        TouchChannel3HighzEnW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_channel4_highz_en(&mut self) -> TouchChannel4HighzEnW<'_, Touch2Spec> {
        TouchChannel4HighzEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_channel5_highz_en(&mut self) -> TouchChannel5HighzEnW<'_, Touch2Spec> {
        TouchChannel5HighzEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_channel6_highz_en(&mut self) -> TouchChannel6HighzEnW<'_, Touch2Spec> {
        TouchChannel6HighzEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_channel7_highz_en(&mut self) -> TouchChannel7HighzEnW<'_, Touch2Spec> {
        TouchChannel7HighzEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_channel8_highz_en(&mut self) -> TouchChannel8HighzEnW<'_, Touch2Spec> {
        TouchChannel8HighzEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_channel9_highz_en(&mut self) -> TouchChannel9HighzEnW<'_, Touch2Spec> {
        TouchChannel9HighzEnW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_channel10_highz_en(&mut self) -> TouchChannel10HighzEnW<'_, Touch2Spec> {
        TouchChannel10HighzEnW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_channel11_highz_en(&mut self) -> TouchChannel11HighzEnW<'_, Touch2Spec> {
        TouchChannel11HighzEnW::new(self, 15)
    }
}
#[doc = "Touch Configuration 2 (Channel High-Z)\n\nYou can [`read`](crate::Reg::read) this register and get [`touch2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Touch2Spec;
impl crate::RegisterSpec for Touch2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch2::R`](R) reader structure"]
impl crate::Readable for Touch2Spec {}
#[doc = "`write(|w| ..)` method takes [`touch2::W`](W) writer structure"]
impl crate::Writable for Touch2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH2 to value 0xfff0"]
impl crate::Resettable for Touch2Spec {
    const RESET_VALUE: u32 = 0xfff0;
}
