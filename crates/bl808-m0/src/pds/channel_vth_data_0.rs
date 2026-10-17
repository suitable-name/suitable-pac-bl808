#[doc = "Register `CHANNEL_VTH_DATA_0` reader"]
pub type R = crate::R<ChannelVthData0Spec>;
#[doc = "Register `CHANNEL_VTH_DATA_0` writer"]
pub type W = crate::W<ChannelVthData0Spec>;
#[doc = "Field `touch_vth_data_ch0` reader - "]
pub type TouchVthDataCh0R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch0` writer - "]
pub type TouchVthDataCh0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `touch_vth_data_ch1` reader - "]
pub type TouchVthDataCh1R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch1` writer - "]
pub type TouchVthDataCh1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `touch_vth_data_ch2` reader - "]
pub type TouchVthDataCh2R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch2` writer - "]
pub type TouchVthDataCh2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `touch_vth_data_ch3` reader - "]
pub type TouchVthDataCh3R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch3` writer - "]
pub type TouchVthDataCh3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn touch_vth_data_ch0(&self) -> TouchVthDataCh0R {
        TouchVthDataCh0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn touch_vth_data_ch1(&self) -> TouchVthDataCh1R {
        TouchVthDataCh1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn touch_vth_data_ch2(&self) -> TouchVthDataCh2R {
        TouchVthDataCh2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn touch_vth_data_ch3(&self) -> TouchVthDataCh3R {
        TouchVthDataCh3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn touch_vth_data_ch0(&mut self) -> TouchVthDataCh0W<'_, ChannelVthData0Spec> {
        TouchVthDataCh0W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn touch_vth_data_ch1(&mut self) -> TouchVthDataCh1W<'_, ChannelVthData0Spec> {
        TouchVthDataCh1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn touch_vth_data_ch2(&mut self) -> TouchVthDataCh2W<'_, ChannelVthData0Spec> {
        TouchVthDataCh2W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn touch_vth_data_ch3(&mut self) -> TouchVthDataCh3W<'_, ChannelVthData0Spec> {
        TouchVthDataCh3W::new(self, 24)
    }
}
#[doc = "Touch Channel 0-3 Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_vth_data_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel_vth_data_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChannelVthData0Spec;
impl crate::RegisterSpec for ChannelVthData0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel_vth_data_0::R`](R) reader structure"]
impl crate::Readable for ChannelVthData0Spec {}
#[doc = "`write(|w| ..)` method takes [`channel_vth_data_0::W`](W) writer structure"]
impl crate::Writable for ChannelVthData0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHANNEL_VTH_DATA_0 to value 0x3f3f_3f3f"]
impl crate::Resettable for ChannelVthData0Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
