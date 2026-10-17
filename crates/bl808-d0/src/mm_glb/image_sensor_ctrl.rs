#[doc = "Register `IMAGE_SENSOR_CTRL` reader"]
pub type R = crate::R<ImageSensorCtrlSpec>;
#[doc = "Register `IMAGE_SENSOR_CTRL` writer"]
pub type W = crate::W<ImageSensorCtrlSpec>;
#[doc = "Field `rg_is_rst_n` reader - Image sensor hardware reset (Active Low)"]
pub type RgIsRstNR = crate::BitReader;
#[doc = "Field `rg_is_rst_n` writer - Image sensor hardware reset (Active Low)"]
pub type RgIsRstNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Image sensor hardware reset (Active Low)"]
    #[inline(always)]
    pub fn rg_is_rst_n(&self) -> RgIsRstNR {
        RgIsRstNR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Image sensor hardware reset (Active Low)"]
    #[inline(always)]
    pub fn rg_is_rst_n(&mut self) -> RgIsRstNW<'_, ImageSensorCtrlSpec> {
        RgIsRstNW::new(self, 0)
    }
}
#[doc = "Image Sensor Reset Control\n\nYou can [`read`](crate::Reg::read) this register and get [`image_sensor_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`image_sensor_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImageSensorCtrlSpec;
impl crate::RegisterSpec for ImageSensorCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`image_sensor_ctrl::R`](R) reader structure"]
impl crate::Readable for ImageSensorCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`image_sensor_ctrl::W`](W) writer structure"]
impl crate::Writable for ImageSensorCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMAGE_SENSOR_CTRL to value 0x01"]
impl crate::Resettable for ImageSensorCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
