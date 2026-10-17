#[doc = "Register `TOUCH3` reader"]
pub type R = crate::R<Touch3Spec>;
#[doc = "Register `TOUCH3` writer"]
pub type W = crate::W<Touch3Spec>;
#[doc = "Field `touch_channel_cal_en` reader - "]
pub type TouchChannelCalEnR = crate::BitReader;
#[doc = "Field `touch_channel_cal_en` writer - "]
pub type TouchChannelCalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_force_value_en` reader - "]
pub type TouchForceValueEnR = crate::BitReader;
#[doc = "Field `touch_force_value_en` writer - "]
pub type TouchForceValueEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_data_hys_en` reader - "]
pub type TouchDataHysEnR = crate::BitReader;
#[doc = "Field `touch_data_hys_en` writer - "]
pub type TouchDataHysEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_lta_en` reader - "]
pub type TouchLtaEnR = crate::BitReader;
#[doc = "Field `touch_lta_en` writer - "]
pub type TouchLtaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_lta_order` reader - "]
pub type TouchLtaOrderR = crate::FieldReader;
#[doc = "Field `touch_lta_order` writer - "]
pub type TouchLtaOrderW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `touch_flt_en` reader - "]
pub type TouchFltEnR = crate::BitReader;
#[doc = "Field `touch_flt_en` writer - "]
pub type TouchFltEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_flt_order` reader - "]
pub type TouchFltOrderR = crate::FieldReader;
#[doc = "Field `touch_flt_order` writer - "]
pub type TouchFltOrderW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `touch_self_mutual_sel` reader - "]
pub type TouchSelfMutualSelR = crate::BitReader;
#[doc = "Field `touch_self_mutual_sel` writer - "]
pub type TouchSelfMutualSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `touch_vldo_ccsel` reader - "]
pub type TouchVldoCcselR = crate::FieldReader;
#[doc = "Field `touch_vldo_ccsel` writer - "]
pub type TouchVldoCcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ten_touch` reader - "]
pub type TenTouchR = crate::BitReader;
#[doc = "Field `ten_touch` writer - "]
pub type TenTouchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn touch_channel_cal_en(&self) -> TouchChannelCalEnR {
        TouchChannelCalEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn touch_force_value_en(&self) -> TouchForceValueEnR {
        TouchForceValueEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn touch_data_hys_en(&self) -> TouchDataHysEnR {
        TouchDataHysEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn touch_lta_en(&self) -> TouchLtaEnR {
        TouchLtaEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn touch_lta_order(&self) -> TouchLtaOrderR {
        TouchLtaOrderR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_flt_en(&self) -> TouchFltEnR {
        TouchFltEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn touch_flt_order(&self) -> TouchFltOrderR {
        TouchFltOrderR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_self_mutual_sel(&self) -> TouchSelfMutualSelR {
        TouchSelfMutualSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn touch_vldo_ccsel(&self) -> TouchVldoCcselR {
        TouchVldoCcselR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_touch(&self) -> TenTouchR {
        TenTouchR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn touch_channel_cal_en(&mut self) -> TouchChannelCalEnW<'_, Touch3Spec> {
        TouchChannelCalEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn touch_force_value_en(&mut self) -> TouchForceValueEnW<'_, Touch3Spec> {
        TouchForceValueEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn touch_data_hys_en(&mut self) -> TouchDataHysEnW<'_, Touch3Spec> {
        TouchDataHysEnW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn touch_lta_en(&mut self) -> TouchLtaEnW<'_, Touch3Spec> {
        TouchLtaEnW::new(self, 4)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn touch_lta_order(&mut self) -> TouchLtaOrderW<'_, Touch3Spec> {
        TouchLtaOrderW::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_flt_en(&mut self) -> TouchFltEnW<'_, Touch3Spec> {
        TouchFltEnW::new(self, 8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn touch_flt_order(&mut self) -> TouchFltOrderW<'_, Touch3Spec> {
        TouchFltOrderW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_self_mutual_sel(&mut self) -> TouchSelfMutualSelW<'_, Touch3Spec> {
        TouchSelfMutualSelW::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn touch_vldo_ccsel(&mut self) -> TouchVldoCcselW<'_, Touch3Spec> {
        TouchVldoCcselW::new(self, 13)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_touch(&mut self) -> TenTouchW<'_, Touch3Spec> {
        TenTouchW::new(self, 18)
    }
}
#[doc = "Touch Configuration 3 (Data Process)\n\nYou can [`read`](crate::Reg::read) this register and get [`touch3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Touch3Spec;
impl crate::RegisterSpec for Touch3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch3::R`](R) reader structure"]
impl crate::Readable for Touch3Spec {}
#[doc = "`write(|w| ..)` method takes [`touch3::W`](W) writer structure"]
impl crate::Writable for Touch3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH3 to value 0x0660"]
impl crate::Resettable for Touch3Spec {
    const RESET_VALUE: u32 = 0x0660;
}
