#[doc = "Register `EF_CRC_CTRL_1` reader"]
pub type R = crate::R<EfCrcCtrl1Spec>;
#[doc = "Register `EF_CRC_CTRL_1` writer"]
pub type W = crate::W<EfCrcCtrl1Spec>;
#[doc = "Field `ef_crc_data_0_en` reader - Enable data bits 31:0 for CRC check"]
pub type EfCrcData0EnR = crate::FieldReader<u32>;
#[doc = "Field `ef_crc_data_0_en` writer - Enable data bits 31:0 for CRC check"]
pub type EfCrcData0EnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enable data bits 31:0 for CRC check"]
    #[inline(always)]
    pub fn ef_crc_data_0_en(&self) -> EfCrcData0EnR {
        EfCrcData0EnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable data bits 31:0 for CRC check"]
    #[inline(always)]
    pub fn ef_crc_data_0_en(&mut self) -> EfCrcData0EnW<'_, EfCrcCtrl1Spec> {
        EfCrcData0EnW::new(self, 0)
    }
}
#[doc = "eFuse CRC Data Enable Mask 0 (Bits 31:0)\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_crc_ctrl_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_crc_ctrl_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCrcCtrl1Spec;
impl crate::RegisterSpec for EfCrcCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_crc_ctrl_1::R`](R) reader structure"]
impl crate::Readable for EfCrcCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_crc_ctrl_1::W`](W) writer structure"]
impl crate::Writable for EfCrcCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CRC_CTRL_1 to value 0xffff_ffff"]
impl crate::Resettable for EfCrcCtrl1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
