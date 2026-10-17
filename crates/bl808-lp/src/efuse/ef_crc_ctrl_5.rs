#[doc = "Register `EF_CRC_CTRL_5` reader"]
pub type R = crate::R<EfCrcCtrl5Spec>;
#[doc = "Register `EF_CRC_CTRL_5` writer"]
pub type W = crate::W<EfCrcCtrl5Spec>;
#[doc = "Field `ef_crc_dout` reader - Calculated CRC result"]
pub type EfCrcDoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Calculated CRC result"]
    #[inline(always)]
    pub fn ef_crc_dout(&self) -> EfCrcDoutR {
        EfCrcDoutR::new(self.bits)
    }
}
impl W {}
#[doc = "eFuse CRC Output Data\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCrcCtrl5Spec;
impl crate::RegisterSpec for EfCrcCtrl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_crc_ctrl_5::R`](R) reader structure"]
impl crate::Readable for EfCrcCtrl5Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_crc_ctrl_5::W`](W) writer structure"]
impl crate::Writable for EfCrcCtrl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CRC_CTRL_5 to value 0xffff_ffff"]
impl crate::Resettable for EfCrcCtrl5Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
