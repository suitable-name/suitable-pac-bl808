#[doc = "Register `EF_CRC_CTRL_3` reader"]
pub type R = crate::R<EfCrcCtrl3Spec>;
#[doc = "Register `EF_CRC_CTRL_3` writer"]
pub type W = crate::W<EfCrcCtrl3Spec>;
#[doc = "Field `ef_crc_iv` reader - CRC initial value"]
pub type EfCrcIvR = crate::FieldReader<u32>;
#[doc = "Field `ef_crc_iv` writer - CRC initial value"]
pub type EfCrcIvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC initial value"]
    #[inline(always)]
    pub fn ef_crc_iv(&self) -> EfCrcIvR {
        EfCrcIvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC initial value"]
    #[inline(always)]
    pub fn ef_crc_iv(&mut self) -> EfCrcIvW<'_, EfCrcCtrl3Spec> {
        EfCrcIvW::new(self, 0)
    }
}
#[doc = "eFuse CRC Initial Value (IV)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCrcCtrl3Spec;
impl crate::RegisterSpec for EfCrcCtrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_crc_ctrl_3::R`](R) reader structure"]
impl crate::Readable for EfCrcCtrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_crc_ctrl_3::W`](W) writer structure"]
impl crate::Writable for EfCrcCtrl3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CRC_CTRL_3 to value 0xffff_ffff"]
impl crate::Resettable for EfCrcCtrl3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
