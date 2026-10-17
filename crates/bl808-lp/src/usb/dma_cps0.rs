#[doc = "Register `DMA_CPS0` reader"]
pub type R = crate::R<DmaCps0Spec>;
#[doc = "Register `DMA_CPS0` writer"]
pub type W = crate::W<DmaCps0Spec>;
#[doc = "Field `AWCHACHE_HOV` reader - AXI Write Cache control (Host Overwrite)"]
pub type AwchacheHovR = crate::FieldReader;
#[doc = "Field `AWCHACHE_HOV` writer - AXI Write Cache control (Host Overwrite)"]
pub type AwchacheHovW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AWPORT_HOV` reader - AXI Write Port control (Host Overwrite)"]
pub type AwportHovR = crate::FieldReader;
#[doc = "Field `AWPORT_HOV` writer - AXI Write Port control (Host Overwrite)"]
pub type AwportHovW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AWLOCK_HOV` reader - AXI Write Lock control (Host Overwrite)"]
pub type AwlockHovR = crate::FieldReader;
#[doc = "Field `AWLOCK_HOV` writer - AXI Write Lock control (Host Overwrite)"]
pub type AwlockHovW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARCACHE_HOV` reader - AXI Read Cache control (Host Overwrite)"]
pub type ArcacheHovR = crate::FieldReader;
#[doc = "Field `ARCACHE_HOV` writer - AXI Read Cache control (Host Overwrite)"]
pub type ArcacheHovW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ARPORT_HOV` reader - AXI Read Port control (Host Overwrite)"]
pub type ArportHovR = crate::FieldReader;
#[doc = "Field `ARPORT_HOV` writer - AXI Read Port control (Host Overwrite)"]
pub type ArportHovW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ARLOCK_HOV` reader - AXI Read Lock control (Host Overwrite)"]
pub type ArlockHovR = crate::FieldReader;
#[doc = "Field `ARLOCK_HOV` writer - AXI Read Lock control (Host Overwrite)"]
pub type ArlockHovW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BUF_LD_EN_HOV` reader - Buffer Load Enable (Host Overwrite)"]
pub type BufLdEnHovR = crate::BitReader;
#[doc = "Field `BUF_LD_EN_HOV` writer - Buffer Load Enable (Host Overwrite)"]
pub type BufLdEnHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_WD_HOV` reader - Destination Width (Host Overwrite)"]
pub type DstWdHovR = crate::BitReader;
#[doc = "Field `DST_WD_HOV` writer - Destination Width (Host Overwrite)"]
pub type DstWdHovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - AXI Write Cache control (Host Overwrite)"]
    #[inline(always)]
    pub fn awchache_hov(&self) -> AwchacheHovR {
        AwchacheHovR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - AXI Write Port control (Host Overwrite)"]
    #[inline(always)]
    pub fn awport_hov(&self) -> AwportHovR {
        AwportHovR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - AXI Write Lock control (Host Overwrite)"]
    #[inline(always)]
    pub fn awlock_hov(&self) -> AwlockHovR {
        AwlockHovR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - AXI Read Cache control (Host Overwrite)"]
    #[inline(always)]
    pub fn arcache_hov(&self) -> ArcacheHovR {
        ArcacheHovR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - AXI Read Port control (Host Overwrite)"]
    #[inline(always)]
    pub fn arport_hov(&self) -> ArportHovR {
        ArportHovR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - AXI Read Lock control (Host Overwrite)"]
    #[inline(always)]
    pub fn arlock_hov(&self) -> ArlockHovR {
        ArlockHovR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Buffer Load Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn buf_ld_en_hov(&self) -> BufLdEnHovR {
        BufLdEnHovR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Destination Width (Host Overwrite)"]
    #[inline(always)]
    pub fn dst_wd_hov(&self) -> DstWdHovR {
        DstWdHovR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - AXI Write Cache control (Host Overwrite)"]
    #[inline(always)]
    pub fn awchache_hov(&mut self) -> AwchacheHovW<'_, DmaCps0Spec> {
        AwchacheHovW::new(self, 0)
    }
    #[doc = "Bits 4:6 - AXI Write Port control (Host Overwrite)"]
    #[inline(always)]
    pub fn awport_hov(&mut self) -> AwportHovW<'_, DmaCps0Spec> {
        AwportHovW::new(self, 4)
    }
    #[doc = "Bits 7:8 - AXI Write Lock control (Host Overwrite)"]
    #[inline(always)]
    pub fn awlock_hov(&mut self) -> AwlockHovW<'_, DmaCps0Spec> {
        AwlockHovW::new(self, 7)
    }
    #[doc = "Bits 9:12 - AXI Read Cache control (Host Overwrite)"]
    #[inline(always)]
    pub fn arcache_hov(&mut self) -> ArcacheHovW<'_, DmaCps0Spec> {
        ArcacheHovW::new(self, 9)
    }
    #[doc = "Bits 13:15 - AXI Read Port control (Host Overwrite)"]
    #[inline(always)]
    pub fn arport_hov(&mut self) -> ArportHovW<'_, DmaCps0Spec> {
        ArportHovW::new(self, 13)
    }
    #[doc = "Bits 16:17 - AXI Read Lock control (Host Overwrite)"]
    #[inline(always)]
    pub fn arlock_hov(&mut self) -> ArlockHovW<'_, DmaCps0Spec> {
        ArlockHovW::new(self, 16)
    }
    #[doc = "Bit 18 - Buffer Load Enable (Host Overwrite)"]
    #[inline(always)]
    pub fn buf_ld_en_hov(&mut self) -> BufLdEnHovW<'_, DmaCps0Spec> {
        BufLdEnHovW::new(self, 18)
    }
    #[doc = "Bit 19 - Destination Width (Host Overwrite)"]
    #[inline(always)]
    pub fn dst_wd_hov(&mut self) -> DstWdHovW<'_, DmaCps0Spec> {
        DstWdHovW::new(self, 19)
    }
}
#[doc = "DMA Control Packet Set 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cps0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cps0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCps0Spec;
impl crate::RegisterSpec for DmaCps0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cps0::R`](R) reader structure"]
impl crate::Readable for DmaCps0Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cps0::W`](W) writer structure"]
impl crate::Writable for DmaCps0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CPS0 to value 0"]
impl crate::Resettable for DmaCps0Spec {}
