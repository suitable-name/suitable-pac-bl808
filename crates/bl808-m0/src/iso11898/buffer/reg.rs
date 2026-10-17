#[doc = "Register `REG` reader"]
pub type R = crate::R<RegSpec>;
#[doc = "Register `REG` writer"]
pub type W = crate::W<RegSpec>;
#[doc = "Field `data` reader - 8-bit data/mask/filter word"]
pub type DataR = crate::FieldReader;
#[doc = "Field `data` writer - 8-bit data/mask/filter word"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit data/mask/filter word"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit data/mask/filter word"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, RegSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "CAN Data/Filter Access Register. Maps to RX/TX data in Operating Mode or Acceptance Filter parameters in Reset Mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSpec;
impl crate::RegisterSpec for RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg::R`](R) reader structure"]
impl crate::Readable for RegSpec {}
#[doc = "`write(|w| ..)` method takes [`reg::W`](W) writer structure"]
impl crate::Writable for RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG to value 0"]
impl crate::Resettable for RegSpec {}
