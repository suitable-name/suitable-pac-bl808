#[doc = "Register `BLAI_ACT_TABLE_BB_B8` reader"]
pub type R = crate::R<BlaiActTableBbB8Spec>;
#[doc = "Register `BLAI_ACT_TABLE_BB_B8` writer"]
pub type W = crate::W<BlaiActTableBbB8Spec>;
#[doc = "Field `reg_act_table_B8` reader - 8-bit activation value (Index 0xB8)"]
pub type RegActTableB8R = crate::FieldReader;
#[doc = "Field `reg_act_table_B8` writer - 8-bit activation value (Index 0xB8)"]
pub type RegActTableB8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_B9` reader - 8-bit activation value (Index 0xB9)"]
pub type RegActTableB9R = crate::FieldReader;
#[doc = "Field `reg_act_table_B9` writer - 8-bit activation value (Index 0xB9)"]
pub type RegActTableB9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_BA` reader - 8-bit activation value (Index 0xBA)"]
pub type RegActTableBaR = crate::FieldReader;
#[doc = "Field `reg_act_table_BA` writer - 8-bit activation value (Index 0xBA)"]
pub type RegActTableBaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_BB` reader - 8-bit activation value (Index 0xBB)"]
pub type RegActTableBbR = crate::FieldReader;
#[doc = "Field `reg_act_table_BB` writer - 8-bit activation value (Index 0xBB)"]
pub type RegActTableBbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xB8)"]
    #[inline(always)]
    pub fn reg_act_table_b8(&self) -> RegActTableB8R {
        RegActTableB8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xB9)"]
    #[inline(always)]
    pub fn reg_act_table_b9(&self) -> RegActTableB9R {
        RegActTableB9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xBA)"]
    #[inline(always)]
    pub fn reg_act_table_ba(&self) -> RegActTableBaR {
        RegActTableBaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xBB)"]
    #[inline(always)]
    pub fn reg_act_table_bb(&self) -> RegActTableBbR {
        RegActTableBbR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0xB8)"]
    #[inline(always)]
    pub fn reg_act_table_b8(&mut self) -> RegActTableB8W<'_, BlaiActTableBbB8Spec> {
        RegActTableB8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0xB9)"]
    #[inline(always)]
    pub fn reg_act_table_b9(&mut self) -> RegActTableB9W<'_, BlaiActTableBbB8Spec> {
        RegActTableB9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0xBA)"]
    #[inline(always)]
    pub fn reg_act_table_ba(&mut self) -> RegActTableBaW<'_, BlaiActTableBbB8Spec> {
        RegActTableBaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0xBB)"]
    #[inline(always)]
    pub fn reg_act_table_bb(&mut self) -> RegActTableBbW<'_, BlaiActTableBbB8Spec> {
        RegActTableBbW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0xB8-0xBB)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_bb_b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_bb_b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTableBbB8Spec;
impl crate::RegisterSpec for BlaiActTableBbB8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_bb_b8::R`](R) reader structure"]
impl crate::Readable for BlaiActTableBbB8Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_bb_b8::W`](W) writer structure"]
impl crate::Writable for BlaiActTableBbB8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_BB_B8 to value 0"]
impl crate::Resettable for BlaiActTableBbB8Spec {}
