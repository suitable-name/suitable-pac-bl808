#[doc = "Register `CHANNEL_VTH_DATA_2` reader"]
pub type R = crate::R<ChannelVthData2Spec>;
#[doc = "Register `CHANNEL_VTH_DATA_2` writer"]
pub type W = crate::W<ChannelVthData2Spec>;
#[doc = "Field `touch_vth_data_ch8` reader - "]
pub type TouchVthDataCh8R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch8` writer - "]
pub type TouchVthDataCh8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `touch_vth_data_ch9` reader - "]
pub type TouchVthDataCh9R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch9` writer - "]
pub type TouchVthDataCh9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `touch_vth_data_ch10` reader - "]
pub type TouchVthDataCh10R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch10` writer - "]
pub type TouchVthDataCh10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `touch_vth_data_ch11` reader - "]
pub type TouchVthDataCh11R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch11` writer - "]
pub type TouchVthDataCh11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn touch_vth_data_ch8(&self) -> TouchVthDataCh8R {
        TouchVthDataCh8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn touch_vth_data_ch9(&self) -> TouchVthDataCh9R {
        TouchVthDataCh9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn touch_vth_data_ch10(&self) -> TouchVthDataCh10R {
        TouchVthDataCh10R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn touch_vth_data_ch11(&self) -> TouchVthDataCh11R {
        TouchVthDataCh11R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn touch_vth_data_ch8(&mut self) -> TouchVthDataCh8W<'_, ChannelVthData2Spec> {
        TouchVthDataCh8W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn touch_vth_data_ch9(&mut self) -> TouchVthDataCh9W<'_, ChannelVthData2Spec> {
        TouchVthDataCh9W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn touch_vth_data_ch10(&mut self) -> TouchVthDataCh10W<'_, ChannelVthData2Spec> {
        TouchVthDataCh10W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn touch_vth_data_ch11(&mut self) -> TouchVthDataCh11W<'_, ChannelVthData2Spec> {
        TouchVthDataCh11W::new(self, 24)
    }
}
#[doc = "Touch Channel 8-11 Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_vth_data_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel_vth_data_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChannelVthData2Spec;
impl crate::RegisterSpec for ChannelVthData2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel_vth_data_2::R`](R) reader structure"]
impl crate::Readable for ChannelVthData2Spec {}
#[doc = "`write(|w| ..)` method takes [`channel_vth_data_2::W`](W) writer structure"]
impl crate::Writable for ChannelVthData2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHANNEL_VTH_DATA_2 to value 0x3f3f_3f3f"]
impl crate::Resettable for ChannelVthData2Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
