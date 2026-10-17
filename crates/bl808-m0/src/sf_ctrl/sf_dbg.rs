#[doc = "Register `SF_DBG` reader"]
pub type R = crate::R<SfDbgSpec>;
#[doc = "Field `sf_autoload_st` reader - "]
pub type SfAutoloadStR = crate::FieldReader;
#[doc = "Field `sf_autoload_st_done` reader - "]
pub type SfAutoloadStDoneR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sf_autoload_st(&self) -> SfAutoloadStR {
        SfAutoloadStR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_autoload_st_done(&self) -> SfAutoloadStDoneR {
        SfAutoloadStDoneR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Autoload Status and Debug\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_dbg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfDbgSpec;
impl crate::RegisterSpec for SfDbgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_dbg::R`](R) reader structure"]
impl crate::Readable for SfDbgSpec {}
#[doc = "`reset()` method sets SF_DBG to value 0"]
impl crate::Resettable for SfDbgSpec {}
