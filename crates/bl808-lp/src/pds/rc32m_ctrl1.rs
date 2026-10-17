#[doc = "Register `RC32M_CTRL1` reader"]
pub type R = crate::R<Rc32mCtrl1Spec>;
#[doc = "Register `RC32M_CTRL1` writer"]
pub type W = crate::W<Rc32mCtrl1Spec>;
#[doc = "Field `rc32m_test_en` reader - "]
pub type Rc32mTestEnR = crate::BitReader;
#[doc = "Field `rc32m_test_en` writer - "]
pub type Rc32mTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_soft_rst` reader - "]
pub type Rc32mSoftRstR = crate::BitReader;
#[doc = "Field `rc32m_soft_rst` writer - "]
pub type Rc32mSoftRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_clk_soft_rst` reader - "]
pub type Rc32mClkSoftRstR = crate::BitReader;
#[doc = "Field `rc32m_clk_soft_rst` writer - "]
pub type Rc32mClkSoftRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_clk_inv` reader - "]
pub type Rc32mClkInvR = crate::BitReader;
#[doc = "Field `rc32m_clk_inv` writer - "]
pub type Rc32mClkInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_clk_force_on` reader - "]
pub type Rc32mClkForceOnR = crate::BitReader;
#[doc = "Field `rc32m_clk_force_on` writer - "]
pub type Rc32mClkForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_reserved` reader - "]
pub type Rc32mReservedR = crate::FieldReader;
#[doc = "Field `rc32m_reserved` writer - "]
pub type Rc32mReservedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_test_en(&self) -> Rc32mTestEnR {
        Rc32mTestEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_soft_rst(&self) -> Rc32mSoftRstR {
        Rc32mSoftRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_clk_soft_rst(&self) -> Rc32mClkSoftRstR {
        Rc32mClkSoftRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rc32m_clk_inv(&self) -> Rc32mClkInvR {
        Rc32mClkInvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rc32m_clk_force_on(&self) -> Rc32mClkForceOnR {
        Rc32mClkForceOnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rc32m_reserved(&self) -> Rc32mReservedR {
        Rc32mReservedR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_test_en(&mut self) -> Rc32mTestEnW<'_, Rc32mCtrl1Spec> {
        Rc32mTestEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_soft_rst(&mut self) -> Rc32mSoftRstW<'_, Rc32mCtrl1Spec> {
        Rc32mSoftRstW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_clk_soft_rst(&mut self) -> Rc32mClkSoftRstW<'_, Rc32mCtrl1Spec> {
        Rc32mClkSoftRstW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rc32m_clk_inv(&mut self) -> Rc32mClkInvW<'_, Rc32mCtrl1Spec> {
        Rc32mClkInvW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rc32m_clk_force_on(&mut self) -> Rc32mClkForceOnW<'_, Rc32mCtrl1Spec> {
        Rc32mClkForceOnW::new(self, 4)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rc32m_reserved(&mut self) -> Rc32mReservedW<'_, Rc32mCtrl1Spec> {
        Rc32mReservedW::new(self, 24)
    }
}
#[doc = "RC 32MHz oscillator software reset and clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`rc32m_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rc32m_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rc32mCtrl1Spec;
impl crate::RegisterSpec for Rc32mCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc32m_ctrl1::R`](R) reader structure"]
impl crate::Readable for Rc32mCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`rc32m_ctrl1::W`](W) writer structure"]
impl crate::Writable for Rc32mCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RC32M_CTRL1 to value 0x0f00_0000"]
impl crate::Resettable for Rc32mCtrl1Spec {
    const RESET_VALUE: u32 = 0x0f00_0000;
}
