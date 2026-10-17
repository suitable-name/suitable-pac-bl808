#[doc = "Register `DP_CLK` reader"]
pub type R = crate::R<DpClkSpec>;
#[doc = "Register `DP_CLK` writer"]
pub type W = crate::W<DpClkSpec>;
#[doc = "Field `reg_dp_clk_div_en` reader - Display Port clock divider enable"]
pub type RegDpClkDivEnR = crate::BitReader;
#[doc = "Field `reg_dp_clk_div_en` writer - Display Port clock divider enable"]
pub type RegDpClkDivEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dp_clk_sel` reader - Display Port clock selection"]
pub type RegDpClkSelR = crate::BitReader;
#[doc = "Field `reg_dp_clk_sel` writer - Display Port clock selection"]
pub type RegDpClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_dp_clk_div` reader - Display Port clock divider factor"]
pub type RegDpClkDivR = crate::FieldReader;
#[doc = "Field `reg_dp_clk_div` writer - Display Port clock divider factor"]
pub type RegDpClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 16 - Display Port clock divider enable"]
    #[inline(always)]
    pub fn reg_dp_clk_div_en(&self) -> RegDpClkDivEnR {
        RegDpClkDivEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Display Port clock selection"]
    #[inline(always)]
    pub fn reg_dp_clk_sel(&self) -> RegDpClkSelR {
        RegDpClkSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Display Port clock divider factor"]
    #[inline(always)]
    pub fn reg_dp_clk_div(&self) -> RegDpClkDivR {
        RegDpClkDivR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Display Port clock divider enable"]
    #[inline(always)]
    pub fn reg_dp_clk_div_en(&mut self) -> RegDpClkDivEnW<'_, DpClkSpec> {
        RegDpClkDivEnW::new(self, 16)
    }
    #[doc = "Bit 17 - Display Port clock selection"]
    #[inline(always)]
    pub fn reg_dp_clk_sel(&mut self) -> RegDpClkSelW<'_, DpClkSpec> {
        RegDpClkSelW::new(self, 17)
    }
    #[doc = "Bits 20:23 - Display Port clock divider factor"]
    #[inline(always)]
    pub fn reg_dp_clk_div(&mut self) -> RegDpClkDivW<'_, DpClkSpec> {
        RegDpClkDivW::new(self, 20)
    }
}
#[doc = "Display and Pixel Clock Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dp_clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dp_clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpClkSpec;
impl crate::RegisterSpec for DpClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_clk::R`](R) reader structure"]
impl crate::Readable for DpClkSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_clk::W`](W) writer structure"]
impl crate::Writable for DpClkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DP_CLK to value 0x0001_0000"]
impl crate::Resettable for DpClkSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
