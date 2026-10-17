#[doc = "Register `EF_CRC_CTRL_2` reader"]
pub type R = crate::R<EfCrcCtrl2Spec>;
#[doc = "Register `EF_CRC_CTRL_2` writer"]
pub type W = crate::W<EfCrcCtrl2Spec>;
#[doc = "Field `ef_crc_data_1_en` reader - Enable data bits 63:32 for CRC check"]
pub type EfCrcData1EnR = crate::FieldReader<u32>;
#[doc = "Field `ef_crc_data_1_en` writer - Enable data bits 63:32 for CRC check"]
pub type EfCrcData1EnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enable data bits 63:32 for CRC check"]
    #[inline(always)]
    pub fn ef_crc_data_1_en(&self) -> EfCrcData1EnR {
        EfCrcData1EnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable data bits 63:32 for CRC check"]
    #[inline(always)]
    pub fn ef_crc_data_1_en(&mut self) -> EfCrcData1EnW<'_, EfCrcCtrl2Spec> {
        EfCrcData1EnW::new(self, 0)
    }
}
#[doc = "eFuse CRC Data Enable Mask 1 (Bits 63:32)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCrcCtrl2Spec;
impl crate::RegisterSpec for EfCrcCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_crc_ctrl_2::R`](R) reader structure"]
impl crate::Readable for EfCrcCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_crc_ctrl_2::W`](W) writer structure"]
impl crate::Writable for EfCrcCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CRC_CTRL_2 to value 0xffff_ffff"]
impl crate::Resettable for EfCrcCtrl2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
