#[doc = "Register `DEV_ISG1` reader"]
pub type R = crate::R<DevIsg1Spec>;
#[doc = "Field `F0_OUT_INT` reader - "]
pub type F0OutIntR = crate::BitReader;
#[doc = "Field `F0_SPK_INT` reader - "]
pub type F0SpkIntR = crate::BitReader;
#[doc = "Field `F1_OUT_INT` reader - "]
pub type F1OutIntR = crate::BitReader;
#[doc = "Field `F1_SPK_INT` reader - "]
pub type F1SpkIntR = crate::BitReader;
#[doc = "Field `F2_OUT_INT` reader - "]
pub type F2OutIntR = crate::BitReader;
#[doc = "Field `F2_SPK_INT` reader - "]
pub type F2SpkIntR = crate::BitReader;
#[doc = "Field `F3_OUT_INT` reader - "]
pub type F3OutIntR = crate::BitReader;
#[doc = "Field `F3_SPK_INT` reader - "]
pub type F3SpkIntR = crate::BitReader;
#[doc = "Field `F4_OUT_INT` reader - "]
pub type F4OutIntR = crate::BitReader;
#[doc = "Field `F4_SPK_INT` reader - "]
pub type F4SpkIntR = crate::BitReader;
#[doc = "Field `F5_OUT_INT` reader - "]
pub type F5OutIntR = crate::BitReader;
#[doc = "Field `F5_SPK_INT` reader - "]
pub type F5SpkIntR = crate::BitReader;
#[doc = "Field `F6_OUT_INT` reader - "]
pub type F6OutIntR = crate::BitReader;
#[doc = "Field `F6_SPK_INT` reader - "]
pub type F6SpkIntR = crate::BitReader;
#[doc = "Field `F7_OUT_INT` reader - "]
pub type F7OutIntR = crate::BitReader;
#[doc = "Field `F7_SPK_INT` reader - "]
pub type F7SpkIntR = crate::BitReader;
#[doc = "Field `F0_IN_INT` reader - "]
pub type F0InIntR = crate::BitReader;
#[doc = "Field `F1_IN_INT` reader - "]
pub type F1InIntR = crate::BitReader;
#[doc = "Field `F2_IN_INT` reader - "]
pub type F2InIntR = crate::BitReader;
#[doc = "Field `F3_IN_INT` reader - "]
pub type F3InIntR = crate::BitReader;
#[doc = "Field `F4_IN_INT` reader - "]
pub type F4InIntR = crate::BitReader;
#[doc = "Field `F5_IN_INT` reader - "]
pub type F5InIntR = crate::BitReader;
#[doc = "Field `F6_IN_INT` reader - "]
pub type F6InIntR = crate::BitReader;
#[doc = "Field `F7_IN_INT` reader - "]
pub type F7InIntR = crate::BitReader;
#[doc = "Field `F8_IN_INT` reader - "]
pub type F8InIntR = crate::BitReader;
#[doc = "Field `F9_IN_INT` reader - "]
pub type F9InIntR = crate::BitReader;
#[doc = "Field `F10_IN_INT` reader - "]
pub type F10InIntR = crate::BitReader;
#[doc = "Field `F11_IN_INT` reader - "]
pub type F11InIntR = crate::BitReader;
#[doc = "Field `F12_IN_INT` reader - "]
pub type F12InIntR = crate::BitReader;
#[doc = "Field `F13_IN_INT` reader - "]
pub type F13InIntR = crate::BitReader;
#[doc = "Field `F14_IN_INT` reader - "]
pub type F14InIntR = crate::BitReader;
#[doc = "Field `F15_IN_INT` reader - "]
pub type F15InIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn f0_out_int(&self) -> F0OutIntR {
        F0OutIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn f0_spk_int(&self) -> F0SpkIntR {
        F0SpkIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn f1_out_int(&self) -> F1OutIntR {
        F1OutIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn f1_spk_int(&self) -> F1SpkIntR {
        F1SpkIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn f2_out_int(&self) -> F2OutIntR {
        F2OutIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn f2_spk_int(&self) -> F2SpkIntR {
        F2SpkIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn f3_out_int(&self) -> F3OutIntR {
        F3OutIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn f3_spk_int(&self) -> F3SpkIntR {
        F3SpkIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn f4_out_int(&self) -> F4OutIntR {
        F4OutIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn f4_spk_int(&self) -> F4SpkIntR {
        F4SpkIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn f5_out_int(&self) -> F5OutIntR {
        F5OutIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn f5_spk_int(&self) -> F5SpkIntR {
        F5SpkIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn f6_out_int(&self) -> F6OutIntR {
        F6OutIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn f6_spk_int(&self) -> F6SpkIntR {
        F6SpkIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn f7_out_int(&self) -> F7OutIntR {
        F7OutIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn f7_spk_int(&self) -> F7SpkIntR {
        F7SpkIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn f0_in_int(&self) -> F0InIntR {
        F0InIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn f1_in_int(&self) -> F1InIntR {
        F1InIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn f2_in_int(&self) -> F2InIntR {
        F2InIntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn f3_in_int(&self) -> F3InIntR {
        F3InIntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn f4_in_int(&self) -> F4InIntR {
        F4InIntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn f5_in_int(&self) -> F5InIntR {
        F5InIntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn f6_in_int(&self) -> F6InIntR {
        F6InIntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn f7_in_int(&self) -> F7InIntR {
        F7InIntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn f8_in_int(&self) -> F8InIntR {
        F8InIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn f9_in_int(&self) -> F9InIntR {
        F9InIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn f10_in_int(&self) -> F10InIntR {
        F10InIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn f11_in_int(&self) -> F11InIntR {
        F11InIntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn f12_in_int(&self) -> F12InIntR {
        F12InIntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn f13_in_int(&self) -> F13InIntR {
        F13InIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn f14_in_int(&self) -> F14InIntR {
        F14InIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn f15_in_int(&self) -> F15InIntR {
        F15InIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Device Interrupt Status Group 1 (FIFO 0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_isg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevIsg1Spec;
impl crate::RegisterSpec for DevIsg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_isg1::R`](R) reader structure"]
impl crate::Readable for DevIsg1Spec {}
#[doc = "`reset()` method sets DEV_ISG1 to value 0"]
impl crate::Resettable for DevIsg1Spec {}
