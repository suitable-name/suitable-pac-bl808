#[doc = "Register `CHANNEL_VTH_DATA_1` reader"]
pub type R = crate::R<ChannelVthData1Spec>;
#[doc = "Register `CHANNEL_VTH_DATA_1` writer"]
pub type W = crate::W<ChannelVthData1Spec>;
#[doc = "Field `touch_vth_data_ch4` reader - "]
pub type TouchVthDataCh4R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch4` writer - "]
pub type TouchVthDataCh4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `touch_vth_data_ch5` reader - "]
pub type TouchVthDataCh5R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch5` writer - "]
pub type TouchVthDataCh5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `touch_vth_data_ch6` reader - "]
pub type TouchVthDataCh6R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch6` writer - "]
pub type TouchVthDataCh6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `touch_vth_data_ch7` reader - "]
pub type TouchVthDataCh7R = crate::FieldReader;
#[doc = "Field `touch_vth_data_ch7` writer - "]
pub type TouchVthDataCh7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn touch_vth_data_ch4(&self) -> TouchVthDataCh4R {
        TouchVthDataCh4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn touch_vth_data_ch5(&self) -> TouchVthDataCh5R {
        TouchVthDataCh5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn touch_vth_data_ch6(&self) -> TouchVthDataCh6R {
        TouchVthDataCh6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn touch_vth_data_ch7(&self) -> TouchVthDataCh7R {
        TouchVthDataCh7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn touch_vth_data_ch4(&mut self) -> TouchVthDataCh4W<'_, ChannelVthData1Spec> {
        TouchVthDataCh4W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn touch_vth_data_ch5(&mut self) -> TouchVthDataCh5W<'_, ChannelVthData1Spec> {
        TouchVthDataCh5W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn touch_vth_data_ch6(&mut self) -> TouchVthDataCh6W<'_, ChannelVthData1Spec> {
        TouchVthDataCh6W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn touch_vth_data_ch7(&mut self) -> TouchVthDataCh7W<'_, ChannelVthData1Spec> {
        TouchVthDataCh7W::new(self, 24)
    }
}
#[doc = "Touch Channel 4-7 Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`channel_vth_data_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel_vth_data_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChannelVthData1Spec;
impl crate::RegisterSpec for ChannelVthData1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel_vth_data_1::R`](R) reader structure"]
impl crate::Readable for ChannelVthData1Spec {}
#[doc = "`write(|w| ..)` method takes [`channel_vth_data_1::W`](W) writer structure"]
impl crate::Writable for ChannelVthData1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHANNEL_VTH_DATA_1 to value 0x3f1f_1f1f"]
impl crate::Resettable for ChannelVthData1Spec {
    const RESET_VALUE: u32 = 0x3f1f_1f1f;
}
