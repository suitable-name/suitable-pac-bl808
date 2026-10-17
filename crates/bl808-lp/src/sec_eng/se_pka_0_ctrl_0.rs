#[doc = "Register `SE_PKA_0_CTRL_0` reader"]
pub type R = crate::R<SePka0Ctrl0Spec>;
#[doc = "Register `SE_PKA_0_CTRL_0` writer"]
pub type W = crate::W<SePka0Ctrl0Spec>;
#[doc = "Field `se_pka_0_done` reader - "]
pub type SePka0DoneR = crate::BitReader;
#[doc = "Field `se_pka_0_done_clr_1t` writer - "]
pub type SePka0DoneClr1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_pka_0_busy` reader - "]
pub type SePka0BusyR = crate::BitReader;
#[doc = "Field `se_pka_0_en` reader - "]
pub type SePka0EnR = crate::BitReader;
#[doc = "Field `se_pka_0_en` writer - "]
pub type SePka0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_pka_0_prot_md` reader - "]
pub type SePka0ProtMdR = crate::FieldReader;
#[doc = "Field `se_pka_0_prot_md` writer - "]
pub type SePka0ProtMdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `se_pka_0_int` reader - "]
pub type SePka0IntR = crate::BitReader;
#[doc = "Field `se_pka_0_int_clr_1t` writer - "]
pub type SePka0IntClr1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_pka_0_int_set` reader - "]
pub type SePka0IntSetR = crate::BitReader;
#[doc = "Field `se_pka_0_int_set` writer - "]
pub type SePka0IntSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_pka_0_int_mask` reader - "]
pub type SePka0IntMaskR = crate::BitReader;
#[doc = "Field `se_pka_0_int_mask` writer - "]
pub type SePka0IntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_pka_0_endian` reader - "]
pub type SePka0EndianR = crate::BitReader;
#[doc = "Field `se_pka_0_endian` writer - "]
pub type SePka0EndianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_pka_0_ram_clr_md` reader - "]
pub type SePka0RamClrMdR = crate::BitReader;
#[doc = "Field `se_pka_0_ram_clr_md` writer - "]
pub type SePka0RamClrMdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_pka_0_status_clr_1t` writer - "]
pub type SePka0StatusClr1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `se_pka_0_status` reader - "]
pub type SePka0StatusR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_pka_0_done(&self) -> SePka0DoneR {
        SePka0DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_pka_0_busy(&self) -> SePka0BusyR {
        SePka0BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_en(&self) -> SePka0EnR {
        SePka0EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn se_pka_0_prot_md(&self) -> SePka0ProtMdR {
        SePka0ProtMdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_pka_0_int(&self) -> SePka0IntR {
        SePka0IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_pka_0_int_set(&self) -> SePka0IntSetR {
        SePka0IntSetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_pka_0_int_mask(&self) -> SePka0IntMaskR {
        SePka0IntMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_0_endian(&self) -> SePka0EndianR {
        SePka0EndianR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_0_ram_clr_md(&self) -> SePka0RamClrMdR {
        SePka0RamClrMdR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_pka_0_status(&self) -> SePka0StatusR {
        SePka0StatusR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_pka_0_done_clr_1t(&mut self) -> SePka0DoneClr1tW<'_, SePka0Ctrl0Spec> {
        SePka0DoneClr1tW::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_en(&mut self) -> SePka0EnW<'_, SePka0Ctrl0Spec> {
        SePka0EnW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn se_pka_0_prot_md(&mut self) -> SePka0ProtMdW<'_, SePka0Ctrl0Spec> {
        SePka0ProtMdW::new(self, 4)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_pka_0_int_clr_1t(&mut self) -> SePka0IntClr1tW<'_, SePka0Ctrl0Spec> {
        SePka0IntClr1tW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_pka_0_int_set(&mut self) -> SePka0IntSetW<'_, SePka0Ctrl0Spec> {
        SePka0IntSetW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_pka_0_int_mask(&mut self) -> SePka0IntMaskW<'_, SePka0Ctrl0Spec> {
        SePka0IntMaskW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_0_endian(&mut self) -> SePka0EndianW<'_, SePka0Ctrl0Spec> {
        SePka0EndianW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_0_ram_clr_md(&mut self) -> SePka0RamClrMdW<'_, SePka0Ctrl0Spec> {
        SePka0RamClrMdW::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_pka_0_status_clr_1t(&mut self) -> SePka0StatusClr1tW<'_, SePka0Ctrl0Spec> {
        SePka0StatusClr1tW::new(self, 15)
    }
}
#[doc = "PKA Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`se_pka_0_ctrl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`se_pka_0_ctrl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SePka0Ctrl0Spec;
impl crate::RegisterSpec for SePka0Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`se_pka_0_ctrl_0::R`](R) reader structure"]
impl crate::Readable for SePka0Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`se_pka_0_ctrl_0::W`](W) writer structure"]
impl crate::Writable for SePka0Ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SE_PKA_0_CTRL_0 to value 0"]
impl crate::Resettable for SePka0Ctrl0Spec {}
