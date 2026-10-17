#[doc = "Register `UTX_RS485_CFG` reader"]
pub type R = crate::R<UtxRs485CfgSpec>;
#[doc = "Register `UTX_RS485_CFG` writer"]
pub type W = crate::W<UtxRs485CfgSpec>;
#[doc = "Field `cr_utx_rs485_en` reader - "]
pub type CrUtxRs485EnR = crate::BitReader;
#[doc = "Field `cr_utx_rs485_en` writer - "]
pub type CrUtxRs485EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_utx_rs485_pol` reader - "]
pub type CrUtxRs485PolR = crate::BitReader;
#[doc = "Field `cr_utx_rs485_pol` writer - "]
pub type CrUtxRs485PolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_rs485_en(&self) -> CrUtxRs485EnR {
        CrUtxRs485EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_rs485_pol(&self) -> CrUtxRs485PolR {
        CrUtxRs485PolR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_rs485_en(&mut self) -> CrUtxRs485EnW<'_, UtxRs485CfgSpec> {
        CrUtxRs485EnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_rs485_pol(&mut self) -> CrUtxRs485PolW<'_, UtxRs485CfgSpec> {
        CrUtxRs485PolW::new(self, 1)
    }
}
#[doc = "UART RS-485 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`utx_rs485_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`utx_rs485_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtxRs485CfgSpec;
impl crate::RegisterSpec for UtxRs485CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utx_rs485_cfg::R`](R) reader structure"]
impl crate::Readable for UtxRs485CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`utx_rs485_cfg::W`](W) writer structure"]
impl crate::Writable for UtxRs485CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UTX_RS485_CFG to value 0x02"]
impl crate::Resettable for UtxRs485CfgSpec {
    const RESET_VALUE: u32 = 0x02;
}
