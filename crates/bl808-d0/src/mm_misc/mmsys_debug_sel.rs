#[doc = "Register `MMSYS_DEBUG_SEL` reader"]
pub type R = crate::R<MmsysDebugSelSpec>;
#[doc = "Register `MMSYS_DEBUG_SEL` writer"]
pub type W = crate::W<MmsysDebugSelSpec>;
#[doc = "Field `mmsys_debug_sel` reader - Select which debug signal group to route to the MM system debug bus."]
pub type MmsysDebugSelR = crate::FieldReader;
#[doc = "Field `mmsys_debug_sel` writer - Select which debug signal group to route to the MM system debug bus."]
pub type MmsysDebugSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Select which debug signal group to route to the MM system debug bus."]
    #[inline(always)]
    pub fn mmsys_debug_sel(&self) -> MmsysDebugSelR {
        MmsysDebugSelR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select which debug signal group to route to the MM system debug bus."]
    #[inline(always)]
    pub fn mmsys_debug_sel(&mut self) -> MmsysDebugSelW<'_, MmsysDebugSelSpec> {
        MmsysDebugSelW::new(self, 0)
    }
}
#[doc = "Multimedia System Debug Bus Select\n\nYou can [`read`](crate::Reg::read) this register and get [`mmsys_debug_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmsys_debug_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmsysDebugSelSpec;
impl crate::RegisterSpec for MmsysDebugSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmsys_debug_sel::R`](R) reader structure"]
impl crate::Readable for MmsysDebugSelSpec {}
#[doc = "`write(|w| ..)` method takes [`mmsys_debug_sel::W`](W) writer structure"]
impl crate::Writable for MmsysDebugSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMSYS_DEBUG_SEL to value 0"]
impl crate::Resettable for MmsysDebugSelSpec {}
