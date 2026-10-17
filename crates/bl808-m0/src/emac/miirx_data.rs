#[doc = "Register `MIIRX_DATA` reader"]
pub type R = crate::R<MiirxDataSpec>;
#[doc = "Field `PRSD` reader - "]
pub type PrsdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn prsd(&self) -> PrsdR {
        PrsdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MII Management Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`miirx_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiirxDataSpec;
impl crate::RegisterSpec for MiirxDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miirx_data::R`](R) reader structure"]
impl crate::Readable for MiirxDataSpec {}
#[doc = "`reset()` method sets MIIRX_DATA to value 0"]
impl crate::Resettable for MiirxDataSpec {}
