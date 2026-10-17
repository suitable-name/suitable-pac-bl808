#[doc = "Register `SD_CFG_FIFO_PARAM` reader"]
pub type R = crate::R<SdCfgFifoParamSpec>;
#[doc = "Register `SD_CFG_FIFO_PARAM` writer"]
pub type W = crate::W<SdCfgFifoParamSpec>;
#[doc = "Field `BOOT_ACK` reader - "]
pub type BootAckR = crate::BitReader;
#[doc = "Field `BOOT_ACK` writer - "]
pub type BootAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SQU_EMPTY_CHK` reader - "]
pub type SquEmptyChkR = crate::BitReader;
#[doc = "Field `SQU_EMPTY_CHK` writer - "]
pub type SquEmptyChkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SQU_FULL_CHK` reader - "]
pub type SquFullChkR = crate::BitReader;
#[doc = "Field `SQU_FULL_CHK` writer - "]
pub type SquFullChkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN_PAD_CLK_ON` reader - "]
pub type GenPadClkOnR = crate::BitReader;
#[doc = "Field `GEN_PAD_CLK_ON` writer - "]
pub type GenPadClkOnW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GEN_PAD_CLK_CNT` reader - "]
pub type GenPadClkCntR = crate::FieldReader;
#[doc = "Field `GEN_PAD_CLK_CNT` writer - "]
pub type GenPadClkCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn boot_ack(&self) -> BootAckR {
        BootAckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn squ_empty_chk(&self) -> SquEmptyChkR {
        SquEmptyChkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn squ_full_chk(&self) -> SquFullChkR {
        SquFullChkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gen_pad_clk_on(&self) -> GenPadClkOnR {
        GenPadClkOnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gen_pad_clk_cnt(&self) -> GenPadClkCntR {
        GenPadClkCntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn boot_ack(&mut self) -> BootAckW<'_, SdCfgFifoParamSpec> {
        BootAckW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn squ_empty_chk(&mut self) -> SquEmptyChkW<'_, SdCfgFifoParamSpec> {
        SquEmptyChkW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn squ_full_chk(&mut self) -> SquFullChkW<'_, SdCfgFifoParamSpec> {
        SquFullChkW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gen_pad_clk_on(&mut self) -> GenPadClkOnW<'_, SdCfgFifoParamSpec> {
        GenPadClkOnW::new(self, 6)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gen_pad_clk_cnt(&mut self) -> GenPadClkCntW<'_, SdCfgFifoParamSpec> {
        GenPadClkCntW::new(self, 24)
    }
}
#[doc = "Vendor specific FIFO and PAD clock configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_cfg_fifo_param::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_cfg_fifo_param::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdCfgFifoParamSpec;
impl crate::RegisterSpec for SdCfgFifoParamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sd_cfg_fifo_param::R`](R) reader structure"]
impl crate::Readable for SdCfgFifoParamSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_cfg_fifo_param::W`](W) writer structure"]
impl crate::Writable for SdCfgFifoParamSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x40;
}
#[doc = "`reset()` method sets SD_CFG_FIFO_PARAM to value 0"]
impl crate::Resettable for SdCfgFifoParamSpec {}
