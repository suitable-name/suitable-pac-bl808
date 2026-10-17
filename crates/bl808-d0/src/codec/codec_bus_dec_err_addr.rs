#[doc = "Register `CODEC_BUS_DEC_ERR_ADDR` reader"]
pub type R = crate::R<CodecBusDecErrAddrSpec>;
#[doc = "Field `CODEC_HS_DEC_ERR_ADDR` reader - Address that caused the decode error"]
pub type CodecHsDecErrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address that caused the decode error"]
    #[inline(always)]
    pub fn codec_hs_dec_err_addr(&self) -> CodecHsDecErrAddrR {
        CodecHsDecErrAddrR::new(self.bits)
    }
}
#[doc = "Codec Bus Decode Error Address\n\nYou can [`read`](crate::Reg::read) this register and get [`codec_bus_dec_err_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecBusDecErrAddrSpec;
impl crate::RegisterSpec for CodecBusDecErrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_bus_dec_err_addr::R`](R) reader structure"]
impl crate::Readable for CodecBusDecErrAddrSpec {}
#[doc = "`reset()` method sets CODEC_BUS_DEC_ERR_ADDR to value 0"]
impl crate::Resettable for CodecBusDecErrAddrSpec {}
