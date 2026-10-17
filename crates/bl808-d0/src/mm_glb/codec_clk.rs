#[doc = "Register `CODEC_CLK` reader"]
pub type R = crate::R<CodecClkSpec>;
#[doc = "Register `CODEC_CLK` writer"]
pub type W = crate::W<CodecClkSpec>;
#[doc = "Field `reg_h264_clk_div_en` reader - "]
pub type RegH264ClkDivEnR = crate::BitReader;
#[doc = "Field `reg_h264_clk_div_en` writer - "]
pub type RegH264ClkDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_h264_clk_sel` reader - "]
pub type RegH264ClkSelR = crate::FieldReader;
#[doc = "Field `reg_h264_clk_sel` writer - "]
pub type RegH264ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_h264_clk_div` reader - "]
pub type RegH264ClkDivR = crate::FieldReader;
#[doc = "Field `reg_h264_clk_div` writer - "]
pub type RegH264ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_h264_clk_div_en(&self) -> RegH264ClkDivEnR {
        RegH264ClkDivEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn reg_h264_clk_sel(&self) -> RegH264ClkSelR {
        RegH264ClkSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn reg_h264_clk_div(&self) -> RegH264ClkDivR {
        RegH264ClkDivR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_h264_clk_div_en(&mut self) -> RegH264ClkDivEnW<'_, CodecClkSpec> {
        RegH264ClkDivEnW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn reg_h264_clk_sel(&mut self) -> RegH264ClkSelW<'_, CodecClkSpec> {
        RegH264ClkSelW::new(self, 9)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn reg_h264_clk_div(&mut self) -> RegH264ClkDivW<'_, CodecClkSpec> {
        RegH264ClkDivW::new(self, 12)
    }
}
#[doc = "Codec/H264 Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codec_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecClkSpec;
impl crate::RegisterSpec for CodecClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_clk::R`](R) reader structure"]
impl crate::Readable for CodecClkSpec {}
#[doc = "`write(|w| ..)` method takes [`codec_clk::W`](W) writer structure"]
impl crate::Writable for CodecClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CODEC_CLK to value 0x0100"]
impl crate::Resettable for CodecClkSpec {
    const RESET_VALUE: u32 = 0x0100;
}
