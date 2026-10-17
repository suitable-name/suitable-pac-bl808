#[doc = "Register `TOUCH_INT_SETTING` reader"]
pub type R = crate::R<TouchIntSettingSpec>;
#[doc = "Register `TOUCH_INT_SETTING` writer"]
pub type W = crate::W<TouchIntSettingSpec>;
#[doc = "Field `touch_int_clr` reader - "]
pub type TouchIntClrR = crate::FieldReader<u16>;
#[doc = "Field `touch_int_clr` writer - "]
pub type TouchIntClrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `touch_int_mask` reader - "]
pub type TouchIntMaskR = crate::FieldReader<u16>;
#[doc = "Field `touch_int_mask` writer - "]
pub type TouchIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `touch_int_en` reader - "]
pub type TouchIntEnR = crate::BitReader;
#[doc = "Field `touch_int_en` writer - "]
pub type TouchIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn touch_int_clr(&self) -> TouchIntClrR {
        TouchIntClrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn touch_int_mask(&self) -> TouchIntMaskR {
        TouchIntMaskR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn touch_int_en(&self) -> TouchIntEnR {
        TouchIntEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn touch_int_clr(&mut self) -> TouchIntClrW<'_, TouchIntSettingSpec> {
        TouchIntClrW::new(self, 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn touch_int_mask(&mut self) -> TouchIntMaskW<'_, TouchIntSettingSpec> {
        TouchIntMaskW::new(self, 16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn touch_int_en(&mut self) -> TouchIntEnW<'_, TouchIntSettingSpec> {
        TouchIntEnW::new(self, 31)
    }
}
#[doc = "Touch sensor interrupt configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_int_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_int_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TouchIntSettingSpec;
impl crate::RegisterSpec for TouchIntSettingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_int_setting::R`](R) reader structure"]
impl crate::Readable for TouchIntSettingSpec {}
#[doc = "`write(|w| ..)` method takes [`touch_int_setting::W`](W) writer structure"]
impl crate::Writable for TouchIntSettingSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0fff;
}
#[doc = "`reset()` method sets TOUCH_INT_SETTING to value 0x8000_0000"]
impl crate::Resettable for TouchIntSettingSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
