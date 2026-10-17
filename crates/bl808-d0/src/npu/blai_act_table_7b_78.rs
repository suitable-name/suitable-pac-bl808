#[doc = "Register `BLAI_ACT_TABLE_7B_78` reader"]
pub type R = crate::R<BlaiActTable7b78Spec>;
#[doc = "Register `BLAI_ACT_TABLE_7B_78` writer"]
pub type W = crate::W<BlaiActTable7b78Spec>;
#[doc = "Field `reg_act_table_78` reader - 8-bit activation value (Index 0x78)"]
pub type RegActTable78R = crate::FieldReader;
#[doc = "Field `reg_act_table_78` writer - 8-bit activation value (Index 0x78)"]
pub type RegActTable78W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_79` reader - 8-bit activation value (Index 0x79)"]
pub type RegActTable79R = crate::FieldReader;
#[doc = "Field `reg_act_table_79` writer - 8-bit activation value (Index 0x79)"]
pub type RegActTable79W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_7A` reader - 8-bit activation value (Index 0x7A)"]
pub type RegActTable7aR = crate::FieldReader;
#[doc = "Field `reg_act_table_7A` writer - 8-bit activation value (Index 0x7A)"]
pub type RegActTable7aW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_7B` reader - 8-bit activation value (Index 0x7B)"]
pub type RegActTable7bR = crate::FieldReader;
#[doc = "Field `reg_act_table_7B` writer - 8-bit activation value (Index 0x7B)"]
pub type RegActTable7bW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x78)"]
    #[inline(always)]
    pub fn reg_act_table_78(&self) -> RegActTable78R {
        RegActTable78R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x79)"]
    #[inline(always)]
    pub fn reg_act_table_79(&self) -> RegActTable79R {
        RegActTable79R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x7A)"]
    #[inline(always)]
    pub fn reg_act_table_7a(&self) -> RegActTable7aR {
        RegActTable7aR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x7B)"]
    #[inline(always)]
    pub fn reg_act_table_7b(&self) -> RegActTable7bR {
        RegActTable7bR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x78)"]
    #[inline(always)]
    pub fn reg_act_table_78(&mut self) -> RegActTable78W<'_, BlaiActTable7b78Spec> {
        RegActTable78W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x79)"]
    #[inline(always)]
    pub fn reg_act_table_79(&mut self) -> RegActTable79W<'_, BlaiActTable7b78Spec> {
        RegActTable79W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x7A)"]
    #[inline(always)]
    pub fn reg_act_table_7a(&mut self) -> RegActTable7aW<'_, BlaiActTable7b78Spec> {
        RegActTable7aW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x7B)"]
    #[inline(always)]
    pub fn reg_act_table_7b(&mut self) -> RegActTable7bW<'_, BlaiActTable7b78Spec> {
        RegActTable7bW::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x78-0x7B)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_7b_78::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_7b_78::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable7b78Spec;
impl crate::RegisterSpec for BlaiActTable7b78Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_7b_78::R`](R) reader structure"]
impl crate::Readable for BlaiActTable7b78Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_7b_78::W`](W) writer structure"]
impl crate::Writable for BlaiActTable7b78Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_7B_78 to value 0"]
impl crate::Resettable for BlaiActTable7b78Spec {}
