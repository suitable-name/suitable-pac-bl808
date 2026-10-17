#[doc = "Register `DEV_ICR` reader"]
pub type R = crate::R<DevIcrSpec>;
#[doc = "Register `DEV_ICR` writer"]
pub type W = crate::W<DevIcrSpec>;
#[doc = "Field `IDLE_CNT` reader - "]
pub type IdleCntR = crate::FieldReader;
#[doc = "Field `IDLE_CNT` writer - "]
pub type IdleCntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn idle_cnt(&self) -> IdleCntR {
        IdleCntR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn idle_cnt(&mut self) -> IdleCntW<'_, DevIcrSpec> {
        IdleCntW::new(self, 0)
    }
}
#[doc = "Device Idle Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevIcrSpec;
impl crate::RegisterSpec for DevIcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_icr::R`](R) reader structure"]
impl crate::Readable for DevIcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_icr::W`](W) writer structure"]
impl crate::Writable for DevIcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_ICR to value 0"]
impl crate::Resettable for DevIcrSpec {}
