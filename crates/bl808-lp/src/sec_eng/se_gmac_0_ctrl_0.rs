#[doc = "Register `SE_GMAC_0_CTRL_0` reader"]
pub type R = crate::R<SeGmac0Ctrl0Spec>;
#[doc = "Register `SE_GMAC_0_CTRL_0` writer"]
pub type W = crate::W<SeGmac0Ctrl0Spec>;
#[doc = "Field `se_gmac_0_busy` reader - "]
pub type SeGmac0BusyR = crate::BitReader;
#[doc = "Field `se_gmac_0_trig_1t` writer - "]
pub type SeGmac0Trig1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_gmac_0_en` reader - "]
pub type SeGmac0EnR = crate::BitReader;
#[doc = "Field `se_gmac_0_en` writer - "]
pub type SeGmac0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_gmac_0_int` reader - "]
pub type SeGmac0IntR = crate::BitReader;
#[doc = "Field `se_gmac_0_int_clr_1t` writer - "]
pub type SeGmac0IntClr1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_gmac_0_int_set_1t` writer - "]
pub type SeGmac0IntSet1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_gmac_0_int_mask` reader - "]
pub type SeGmac0IntMaskR = crate::BitReader;
#[doc = "Field `se_gmac_0_int_mask` writer - "]
pub type SeGmac0IntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_gmac_0_t_endian` reader - "]
pub type SeGmac0TEndianR = crate::BitReader;
#[doc = "Field `se_gmac_0_t_endian` writer - "]
pub type SeGmac0TEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_gmac_0_h_endian` reader - "]
pub type SeGmac0HEndianR = crate::BitReader;
#[doc = "Field `se_gmac_0_h_endian` writer - "]
pub type SeGmac0HEndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_gmac_0_x_endian` reader - "]
pub type SeGmac0XEndianR = crate::BitReader;
#[doc = "Field `se_gmac_0_x_endian` writer - "]
pub type SeGmac0XEndianW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_gmac_0_busy(&self) -> SeGmac0BusyR {
        SeGmac0BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_gmac_0_en(&self) -> SeGmac0EnR {
        SeGmac0EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_gmac_0_int(&self) -> SeGmac0IntR {
        SeGmac0IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_gmac_0_int_mask(&self) -> SeGmac0IntMaskR {
        SeGmac0IntMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_gmac_0_t_endian(&self) -> SeGmac0TEndianR {
        SeGmac0TEndianR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_gmac_0_h_endian(&self) -> SeGmac0HEndianR {
        SeGmac0HEndianR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_gmac_0_x_endian(&self) -> SeGmac0XEndianR {
        SeGmac0XEndianR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_gmac_0_trig_1t(&mut self) -> SeGmac0Trig1tW<'_, SeGmac0Ctrl0Spec> {
        SeGmac0Trig1tW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_gmac_0_en(&mut self) -> SeGmac0EnW<'_, SeGmac0Ctrl0Spec> {
        SeGmac0EnW::new(self, 2)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_gmac_0_int_clr_1t(&mut self) -> SeGmac0IntClr1tW<'_, SeGmac0Ctrl0Spec> {
        SeGmac0IntClr1tW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_gmac_0_int_set_1t(&mut self) -> SeGmac0IntSet1tW<'_, SeGmac0Ctrl0Spec> {
        SeGmac0IntSet1tW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_gmac_0_int_mask(&mut self) -> SeGmac0IntMaskW<'_, SeGmac0Ctrl0Spec> {
        SeGmac0IntMaskW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_gmac_0_t_endian(&mut self) -> SeGmac0TEndianW<'_, SeGmac0Ctrl0Spec> {
        SeGmac0TEndianW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_gmac_0_h_endian(&mut self) -> SeGmac0HEndianW<'_, SeGmac0Ctrl0Spec> {
        SeGmac0HEndianW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_gmac_0_x_endian(&mut self) -> SeGmac0XEndianW<'_, SeGmac0Ctrl0Spec> {
        SeGmac0XEndianW::new(self, 14)
    }
}
#[doc = "GMAC Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`se_gmac_0_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_gmac_0_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeGmac0Ctrl0Spec;
impl crate::RegisterSpec for SeGmac0Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_gmac_0_ctrl_0::R`](R) reader structure"]
impl crate::Readable for SeGmac0Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`se_gmac_0_ctrl_0::W`](W) writer structure"]
impl crate::Writable for SeGmac0Ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_GMAC_0_CTRL_0 to value 0x7000"]
impl crate::Resettable for SeGmac0Ctrl0Spec {
    const RESET_VALUE: u32 = 0x7000;
}
