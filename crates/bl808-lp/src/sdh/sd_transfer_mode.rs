#[doc = "Register `SD_TRANSFER_MODE` reader"]
pub type R = crate::R<SdTransferModeSpec>;
#[doc = "Register `SD_TRANSFER_MODE` writer"]
pub type W = crate::W<SdTransferModeSpec>;
#[doc = "Field `DMA_EN` reader - DMA Enable"]
pub type DmaEnR = crate::BitReader;
#[doc = "Field `DMA_EN` writer - DMA Enable"]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_CNT_EN` reader - Block Count Enable"]
pub type BlkCntEnR = crate::BitReader;
#[doc = "Field `BLK_CNT_EN` writer - Block Count Enable"]
pub type BlkCntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD_EN` reader - Auto Command Enable (CMD12/CMD23)"]
pub type AutoCmdEnR = crate::FieldReader;
#[doc = "Field `AUTO_CMD_EN` writer - Auto Command Enable (CMD12/CMD23)"]
pub type AutoCmdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TO_HOST_DIR` reader - Data Transfer Direction (0: Write, 1: Read)"]
pub type ToHostDirR = crate::BitReader;
#[doc = "Field `TO_HOST_DIR` writer - Data Transfer Direction (0: Write, 1: Read)"]
pub type ToHostDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTI_BLK_SEL` reader - Multi Block Select (0: Single Block, 1: Multiple Blocks)"]
pub type MultiBlkSelR = crate::BitReader;
#[doc = "Field `MULTI_BLK_SEL` writer - Multi Block Select (0: Single Block, 1: Multiple Blocks)"]
pub type MultiBlkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn blk_cnt_en(&self) -> BlkCntEnR {
        BlkCntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Auto Command Enable (CMD12/CMD23)"]
    #[inline(always)]
    pub fn auto_cmd_en(&self) -> AutoCmdEnR {
        AutoCmdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction (0: Write, 1: Read)"]
    #[inline(always)]
    pub fn to_host_dir(&self) -> ToHostDirR {
        ToHostDirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi Block Select (0: Single Block, 1: Multiple Blocks)"]
    #[inline(always)]
    pub fn multi_blk_sel(&self) -> MultiBlkSelR {
        MultiBlkSelR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<'_, SdTransferModeSpec> {
        DmaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn blk_cnt_en(&mut self) -> BlkCntEnW<'_, SdTransferModeSpec> {
        BlkCntEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Auto Command Enable (CMD12/CMD23)"]
    #[inline(always)]
    pub fn auto_cmd_en(&mut self) -> AutoCmdEnW<'_, SdTransferModeSpec> {
        AutoCmdEnW::new(self, 2)
    }
    #[doc = "Bit 4 - Data Transfer Direction (0: Write, 1: Read)"]
    #[inline(always)]
    pub fn to_host_dir(&mut self) -> ToHostDirW<'_, SdTransferModeSpec> {
        ToHostDirW::new(self, 4)
    }
    #[doc = "Bit 5 - Multi Block Select (0: Single Block, 1: Multiple Blocks)"]
    #[inline(always)]
    pub fn multi_blk_sel(&mut self) -> MultiBlkSelW<'_, SdTransferModeSpec> {
        MultiBlkSelW::new(self, 5)
    }
}
#[doc = "Transfer Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_transfer_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_transfer_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdTransferModeSpec;
impl crate::RegisterSpec for SdTransferModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_transfer_mode::R`](R) reader structure"]
impl crate::Readable for SdTransferModeSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_transfer_mode::W`](W) writer structure"]
impl crate::Writable for SdTransferModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_TRANSFER_MODE to value 0"]
impl crate::Resettable for SdTransferModeSpec {}
