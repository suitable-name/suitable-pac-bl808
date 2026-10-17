#[doc = "Register `MIITX_DATA` reader"]
pub type R = crate::R<MiitxDataSpec>;
#[doc = "Register `MIITX_DATA` writer"]
pub type W = crate::W<MiitxDataSpec>;
#[doc = "Field `CTRLDATA` reader - "]
pub type CtrldataR = crate::FieldReader<u16>;
#[doc = "Field `CTRLDATA` writer - "]
pub type CtrldataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ctrldata(&self) -> CtrldataR {
        CtrldataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ctrldata(&mut self) -> CtrldataW<'_, MiitxDataSpec> {
        CtrldataW::new(self, 0)
    }
}
#[doc = "MII Management Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miitx_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miitx_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiitxDataSpec;
impl crate::RegisterSpec for MiitxDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miitx_data::R`](R) reader structure"]
impl crate::Readable for MiitxDataSpec {}
#[doc = "`write(|w| ..)` method takes [`miitx_data::W`](W) writer structure"]
impl crate::Writable for MiitxDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIITX_DATA to value 0"]
impl crate::Resettable for MiitxDataSpec {}
