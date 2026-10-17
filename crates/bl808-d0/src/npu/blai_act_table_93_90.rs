#[doc = "Register `BLAI_ACT_TABLE_93_90` reader"]
pub type R = crate::R<BlaiActTable93_90Spec>;
#[doc = "Register `BLAI_ACT_TABLE_93_90` writer"]
pub type W = crate::W<BlaiActTable93_90Spec>;
#[doc = "Field `reg_act_table_90` reader - 8-bit activation value (Index 0x90)"]
pub type RegActTable90R = crate::FieldReader;
#[doc = "Field `reg_act_table_90` writer - 8-bit activation value (Index 0x90)"]
pub type RegActTable90W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_91` reader - 8-bit activation value (Index 0x91)"]
pub type RegActTable91R = crate::FieldReader;
#[doc = "Field `reg_act_table_91` writer - 8-bit activation value (Index 0x91)"]
pub type RegActTable91W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_92` reader - 8-bit activation value (Index 0x92)"]
pub type RegActTable92R = crate::FieldReader;
#[doc = "Field `reg_act_table_92` writer - 8-bit activation value (Index 0x92)"]
pub type RegActTable92W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_93` reader - 8-bit activation value (Index 0x93)"]
pub type RegActTable93R = crate::FieldReader;
#[doc = "Field `reg_act_table_93` writer - 8-bit activation value (Index 0x93)"]
pub type RegActTable93W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x90)"]
    #[inline(always)]
    pub fn reg_act_table_90(&self) -> RegActTable90R {
        RegActTable90R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x91)"]
    #[inline(always)]
    pub fn reg_act_table_91(&self) -> RegActTable91R {
        RegActTable91R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x92)"]
    #[inline(always)]
    pub fn reg_act_table_92(&self) -> RegActTable92R {
        RegActTable92R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x93)"]
    #[inline(always)]
    pub fn reg_act_table_93(&self) -> RegActTable93R {
        RegActTable93R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x90)"]
    #[inline(always)]
    pub fn reg_act_table_90(&mut self) -> RegActTable90W<'_, BlaiActTable93_90Spec> {
        RegActTable90W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x91)"]
    #[inline(always)]
    pub fn reg_act_table_91(&mut self) -> RegActTable91W<'_, BlaiActTable93_90Spec> {
        RegActTable91W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x92)"]
    #[inline(always)]
    pub fn reg_act_table_92(&mut self) -> RegActTable92W<'_, BlaiActTable93_90Spec> {
        RegActTable92W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x93)"]
    #[inline(always)]
    pub fn reg_act_table_93(&mut self) -> RegActTable93W<'_, BlaiActTable93_90Spec> {
        RegActTable93W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x90-0x93)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_93_90::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_93_90::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable93_90Spec;
impl crate::RegisterSpec for BlaiActTable93_90Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_93_90::R`](R) reader structure"]
impl crate::Readable for BlaiActTable93_90Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_93_90::W`](W) writer structure"]
impl crate::Writable for BlaiActTable93_90Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_93_90 to value 0"]
impl crate::Resettable for BlaiActTable93_90Spec {}
