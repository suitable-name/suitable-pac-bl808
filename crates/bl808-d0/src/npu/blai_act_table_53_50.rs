#[doc = "Register `BLAI_ACT_TABLE_53_50` reader"]
pub type R = crate::R<BlaiActTable53_50Spec>;
#[doc = "Register `BLAI_ACT_TABLE_53_50` writer"]
pub type W = crate::W<BlaiActTable53_50Spec>;
#[doc = "Field `reg_act_table_50` reader - 8-bit activation value (Index 0x50)"]
pub type RegActTable50R = crate::FieldReader;
#[doc = "Field `reg_act_table_50` writer - 8-bit activation value (Index 0x50)"]
pub type RegActTable50W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_51` reader - 8-bit activation value (Index 0x51)"]
pub type RegActTable51R = crate::FieldReader;
#[doc = "Field `reg_act_table_51` writer - 8-bit activation value (Index 0x51)"]
pub type RegActTable51W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_52` reader - 8-bit activation value (Index 0x52)"]
pub type RegActTable52R = crate::FieldReader;
#[doc = "Field `reg_act_table_52` writer - 8-bit activation value (Index 0x52)"]
pub type RegActTable52W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_act_table_53` reader - 8-bit activation value (Index 0x53)"]
pub type RegActTable53R = crate::FieldReader;
#[doc = "Field `reg_act_table_53` writer - 8-bit activation value (Index 0x53)"]
pub type RegActTable53W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x50)"]
    #[inline(always)]
    pub fn reg_act_table_50(&self) -> RegActTable50R {
        RegActTable50R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x51)"]
    #[inline(always)]
    pub fn reg_act_table_51(&self) -> RegActTable51R {
        RegActTable51R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x52)"]
    #[inline(always)]
    pub fn reg_act_table_52(&self) -> RegActTable52R {
        RegActTable52R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x53)"]
    #[inline(always)]
    pub fn reg_act_table_53(&self) -> RegActTable53R {
        RegActTable53R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit activation value (Index 0x50)"]
    #[inline(always)]
    pub fn reg_act_table_50(&mut self) -> RegActTable50W<'_, BlaiActTable53_50Spec> {
        RegActTable50W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8-bit activation value (Index 0x51)"]
    #[inline(always)]
    pub fn reg_act_table_51(&mut self) -> RegActTable51W<'_, BlaiActTable53_50Spec> {
        RegActTable51W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8-bit activation value (Index 0x52)"]
    #[inline(always)]
    pub fn reg_act_table_52(&mut self) -> RegActTable52W<'_, BlaiActTable53_50Spec> {
        RegActTable52W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 8-bit activation value (Index 0x53)"]
    #[inline(always)]
    pub fn reg_act_table_53(&mut self) -> RegActTable53W<'_, BlaiActTable53_50Spec> {
        RegActTable53W::new(self, 24)
    }
}
#[doc = "BLAI Activation Table (Indices 0x50-0x53)\n\nYou can [`read`](crate::Reg::read) this register and get [`blai_act_table_53_50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blai_act_table_53_50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlaiActTable53_50Spec;
impl crate::RegisterSpec for BlaiActTable53_50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blai_act_table_53_50::R`](R) reader structure"]
impl crate::Readable for BlaiActTable53_50Spec {}
#[doc = "`write(|w| ..)` method takes [`blai_act_table_53_50::W`](W) writer structure"]
impl crate::Writable for BlaiActTable53_50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLAI_ACT_TABLE_53_50 to value 0"]
impl crate::Resettable for BlaiActTable53_50Spec {}
