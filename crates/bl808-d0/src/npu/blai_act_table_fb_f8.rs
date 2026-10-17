#[doc = "Register `BLAI_ACT_TABLE_FB_F8` reader"]
pub type R = crate::R<BlaiActTableFbF8Spec>;
#[doc = "Register `BLAI_ACT_TABLE_FB_F8` writer"]
pub type W = crate::W<BlaiActTableFbF8Spec>;
#[doc = "Field `reg_act_table_F8` reader - 8-bit activation value (Index 0xF8)"]
pub type RegActTableF8R = crate::FieldReader;
#[doc = "Field `reg_act_table_F8` writer - 8-bit activation value (Index 0xF8)"]
pub type RegActTableF8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_F9` reader - 8-bit activation value (Index 0xF9)"]
pub type RegActTableF9R = crate::FieldReader;
#[doc = "Field `reg_act_table_F9` writer - 8-bit activation value (Index 0xF9)"]
pub type RegActTableF9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_FA` reader - 8-bit activation value (Index 0xFA)"]
pub type RegActTableFaR = crate::FieldReader;
#[doc = "Field `reg_act_table_FA` writer - 8-bit activation value (Index 0xFA)"]
pub type RegActTableFaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_FB` reader - 8-bit activation value (Index 0xFB)"]
pub type RegActTableFbR = crate::FieldReader;
#[doc = "Field `reg_act_table_FB` writer - 8-bit activation value (Index 0xFB)"]
pub type RegActTableFbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xF8)"]
    #[inline(always)]
    pub fn reg_act_table_f8(&self) -> RegActTableF8R {
        RegActTableF8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xF9)"]
    #[inline(always)]
    pub fn reg_act_table_f9(&self) -> RegActTableF9R {
        RegActTableF9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xFA)"]
    #[inline(always)]
    pub fn reg_act_table_fa(&self) -> RegActTableFaR {
        RegActTableFaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xFB)"]
    #[inline(always)]
    pub fn reg_act_table_fb(&self) -> RegActTableFbR {
        RegActTableFbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xF8)"]
    #[inline(always)]
    pub fn reg_act_table_f8(&mut self) -> RegActTableF8W<'_, BlaiActTableFbF8Spec> {
        RegActTableF8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xF9)"]
    #[inline(always)]
    pub fn reg_act_table_f9(&mut self) -> RegActTableF9W<'_, BlaiActTableFbF8Spec> {
        RegActTableF9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xFA)"]
    #[inline(always)]
    pub fn reg_act_table_fa(&mut self) -> RegActTableFaW<'_, BlaiActTableFbF8Spec> {
        RegActTableFaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xFB)"]
    #[inline(always)]
    pub fn reg_act_table_fb(&mut self) -> RegActTableFbW<'_, BlaiActTableFbF8Spec> {
        RegActTableFbW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xF8-0xFB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_fb_f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_fb_f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableFbF8Spec;
impl crate::RegisterSpec for BlaiActTableFbF8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_fb_f8::R`](R) reader structure"]
impl crate::Readable for BlaiActTableFbF8Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_fb_f8::W`](W) writer structure"]
impl crate::Writable for BlaiActTableFbF8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_FB_F8 to value 0"]
impl crate::Resettable for BlaiActTableFbF8Spec {}
