#[doc = "Register `DIG_CLK_CFG0` reader"]
pub type R = crate::R<DigClkCfg0Spec>;
#[doc = "Register `DIG_CLK_CFG0` writer"]
pub type W = crate::W<DigClkCfg0Spec>;
#[doc = "Field `dig_32k_div` reader - "]
pub type Dig32kDivR = crate::FieldReader<u16>;
#[doc = "Field `dig_32k_div` writer - "]
pub type Dig32kDivW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `dig_32k_en` reader - "]
pub type Dig32kEnR = crate::BitReader;
#[doc = "Field `dig_32k_en` writer - "]
pub type Dig32kEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dig_32k_comp` reader - "]
pub type Dig32kCompR = crate::BitReader;
#[doc = "Field `dig_32k_comp` writer - "]
pub type Dig32kCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dig_512k_div` reader - "]
pub type Dig512kDivR = crate::FieldReader;
#[doc = "Field `dig_512k_div` writer - "]
pub type Dig512kDivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dig_512k_en` reader - "]
pub type Dig512kEnR = crate::BitReader;
#[doc = "Field `dig_512k_en` writer - "]
pub type Dig512kEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dig_512k_comp` reader - "]
pub type Dig512kCompR = crate::BitReader;
#[doc = "Field `dig_512k_comp` writer - "]
pub type Dig512kCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dig_clk_src_sel` reader - "]
pub type DigClkSrcSelR = crate::FieldReader;
#[doc = "Field `dig_clk_src_sel` writer - "]
pub type DigClkSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_en_platform_wakeup` reader - "]
pub type RegEnPlatformWakeupR = crate::BitReader;
#[doc = "Field `reg_en_platform_wakeup` writer - "]
pub type RegEnPlatformWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dig_32k_div(&self) -> Dig32kDivR {
        Dig32kDivR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dig_32k_en(&self) -> Dig32kEnR {
        Dig32kEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dig_32k_comp(&self) -> Dig32kCompR {
        Dig32kCompR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn dig_512k_div(&self) -> Dig512kDivR {
        Dig512kDivR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dig_512k_en(&self) -> Dig512kEnR {
        Dig512kEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dig_512k_comp(&self) -> Dig512kCompR {
        Dig512kCompR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dig_clk_src_sel(&self) -> DigClkSrcSelR {
        DigClkSrcSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_en_platform_wakeup(&self) -> RegEnPlatformWakeupR {
        RegEnPlatformWakeupR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dig_32k_div(&mut self) -> Dig32kDivW<'_, DigClkCfg0Spec> {
        Dig32kDivW::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dig_32k_en(&mut self) -> Dig32kEnW<'_, DigClkCfg0Spec> {
        Dig32kEnW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dig_32k_comp(&mut self) -> Dig32kCompW<'_, DigClkCfg0Spec> {
        Dig32kCompW::new(self, 13)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn dig_512k_div(&mut self) -> Dig512kDivW<'_, DigClkCfg0Spec> {
        Dig512kDivW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dig_512k_en(&mut self) -> Dig512kEnW<'_, DigClkCfg0Spec> {
        Dig512kEnW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dig_512k_comp(&mut self) -> Dig512kCompW<'_, DigClkCfg0Spec> {
        Dig512kCompW::new(self, 25)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dig_clk_src_sel(&mut self) -> DigClkSrcSelW<'_, DigClkCfg0Spec> {
        DigClkSrcSelW::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_en_platform_wakeup(&mut self) -> RegEnPlatformWakeupW<'_, DigClkCfg0Spec> {
        RegEnPlatformWakeupW::new(self, 31)
    }
}
#[doc = "Digital Clock Config 0 (32K/512K)\n\nYou can [`read`](crate::Reg::read) this register and get [`dig_clk_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dig_clk_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DigClkCfg0Spec;
impl crate::RegisterSpec for DigClkCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dig_clk_cfg0::R`](R) reader structure"]
impl crate::Readable for DigClkCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`dig_clk_cfg0::W`](W) writer structure"]
impl crate::Writable for DigClkCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIG_CLK_CFG0 to value 0"]
impl crate::Resettable for DigClkCfg0Spec {}
