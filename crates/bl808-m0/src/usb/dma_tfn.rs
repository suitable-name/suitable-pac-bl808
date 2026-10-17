#[doc = "Register `DMA_TFN` reader"]
pub type R = crate::R<DmaTfnSpec>;
#[doc = "Register `DMA_TFN` writer"]
pub type W = crate::W<DmaTfnSpec>;
#[doc = "Field `ACC_F0_HOV` reader - Access FIFO 0 (Host Overwrite)"]
pub type AccF0HovR = crate::BitReader;
#[doc = "Field `ACC_F0_HOV` writer - Access FIFO 0 (Host Overwrite)"]
pub type AccF0HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F1_HOV` reader - "]
pub type AccF1HovR = crate::BitReader;
#[doc = "Field `ACC_F1_HOV` writer - "]
pub type AccF1HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F2_HOV` reader - "]
pub type AccF2HovR = crate::BitReader;
#[doc = "Field `ACC_F2_HOV` writer - "]
pub type AccF2HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F3_HOV` reader - "]
pub type AccF3HovR = crate::BitReader;
#[doc = "Field `ACC_F3_HOV` writer - "]
pub type AccF3HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_CXF_HOV` reader - Access Control FIFO (Host Overwrite)"]
pub type AccCxfHovR = crate::BitReader;
#[doc = "Field `ACC_CXF_HOV` writer - Access Control FIFO (Host Overwrite)"]
pub type AccCxfHovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F4_HOV` reader - "]
pub type AccF4HovR = crate::BitReader;
#[doc = "Field `ACC_F4_HOV` writer - "]
pub type AccF4HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F5_HOV` reader - "]
pub type AccF5HovR = crate::BitReader;
#[doc = "Field `ACC_F5_HOV` writer - "]
pub type AccF5HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F6_HOV` reader - "]
pub type AccF6HovR = crate::BitReader;
#[doc = "Field `ACC_F6_HOV` writer - "]
pub type AccF6HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F7_HOV` reader - "]
pub type AccF7HovR = crate::BitReader;
#[doc = "Field `ACC_F7_HOV` writer - "]
pub type AccF7HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F8_HOV` reader - "]
pub type AccF8HovR = crate::BitReader;
#[doc = "Field `ACC_F8_HOV` writer - "]
pub type AccF8HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F9_HOV` reader - "]
pub type AccF9HovR = crate::BitReader;
#[doc = "Field `ACC_F9_HOV` writer - "]
pub type AccF9HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F10_HOV` reader - "]
pub type AccF10HovR = crate::BitReader;
#[doc = "Field `ACC_F10_HOV` writer - "]
pub type AccF10HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F11_HOV` reader - "]
pub type AccF11HovR = crate::BitReader;
#[doc = "Field `ACC_F11_HOV` writer - "]
pub type AccF11HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F12_HOV` reader - "]
pub type AccF12HovR = crate::BitReader;
#[doc = "Field `ACC_F12_HOV` writer - "]
pub type AccF12HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F13_HOV` reader - "]
pub type AccF13HovR = crate::BitReader;
#[doc = "Field `ACC_F13_HOV` writer - "]
pub type AccF13HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F14_HOV` reader - "]
pub type AccF14HovR = crate::BitReader;
#[doc = "Field `ACC_F14_HOV` writer - "]
pub type AccF14HovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC_F15_HOV` reader - "]
pub type AccF15HovR = crate::BitReader;
#[doc = "Field `ACC_F15_HOV` writer - "]
pub type AccF15HovW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Access FIFO 0 (Host Overwrite)"]
    #[inline(always)]
    pub fn acc_f0_hov(&self) -> AccF0HovR {
        AccF0HovR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acc_f1_hov(&self) -> AccF1HovR {
        AccF1HovR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn acc_f2_hov(&self) -> AccF2HovR {
        AccF2HovR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn acc_f3_hov(&self) -> AccF3HovR {
        AccF3HovR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Access Control FIFO (Host Overwrite)"]
    #[inline(always)]
    pub fn acc_cxf_hov(&self) -> AccCxfHovR {
        AccCxfHovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn acc_f4_hov(&self) -> AccF4HovR {
        AccF4HovR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn acc_f5_hov(&self) -> AccF5HovR {
        AccF5HovR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn acc_f6_hov(&self) -> AccF6HovR {
        AccF6HovR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn acc_f7_hov(&self) -> AccF7HovR {
        AccF7HovR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn acc_f8_hov(&self) -> AccF8HovR {
        AccF8HovR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn acc_f9_hov(&self) -> AccF9HovR {
        AccF9HovR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn acc_f10_hov(&self) -> AccF10HovR {
        AccF10HovR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn acc_f11_hov(&self) -> AccF11HovR {
        AccF11HovR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn acc_f12_hov(&self) -> AccF12HovR {
        AccF12HovR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn acc_f13_hov(&self) -> AccF13HovR {
        AccF13HovR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn acc_f14_hov(&self) -> AccF14HovR {
        AccF14HovR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn acc_f15_hov(&self) -> AccF15HovR {
        AccF15HovR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access FIFO 0 (Host Overwrite)"]
    #[inline(always)]
    pub fn acc_f0_hov(&mut self) -> AccF0HovW<'_, DmaTfnSpec> {
        AccF0HovW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acc_f1_hov(&mut self) -> AccF1HovW<'_, DmaTfnSpec> {
        AccF1HovW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn acc_f2_hov(&mut self) -> AccF2HovW<'_, DmaTfnSpec> {
        AccF2HovW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn acc_f3_hov(&mut self) -> AccF3HovW<'_, DmaTfnSpec> {
        AccF3HovW::new(self, 3)
    }
    #[doc = "Bit 4 - Access Control FIFO (Host Overwrite)"]
    #[inline(always)]
    pub fn acc_cxf_hov(&mut self) -> AccCxfHovW<'_, DmaTfnSpec> {
        AccCxfHovW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn acc_f4_hov(&mut self) -> AccF4HovW<'_, DmaTfnSpec> {
        AccF4HovW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn acc_f5_hov(&mut self) -> AccF5HovW<'_, DmaTfnSpec> {
        AccF5HovW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn acc_f6_hov(&mut self) -> AccF6HovW<'_, DmaTfnSpec> {
        AccF6HovW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn acc_f7_hov(&mut self) -> AccF7HovW<'_, DmaTfnSpec> {
        AccF7HovW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn acc_f8_hov(&mut self) -> AccF8HovW<'_, DmaTfnSpec> {
        AccF8HovW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn acc_f9_hov(&mut self) -> AccF9HovW<'_, DmaTfnSpec> {
        AccF9HovW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn acc_f10_hov(&mut self) -> AccF10HovW<'_, DmaTfnSpec> {
        AccF10HovW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn acc_f11_hov(&mut self) -> AccF11HovW<'_, DmaTfnSpec> {
        AccF11HovW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn acc_f12_hov(&mut self) -> AccF12HovW<'_, DmaTfnSpec> {
        AccF12HovW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn acc_f13_hov(&mut self) -> AccF13HovW<'_, DmaTfnSpec> {
        AccF13HovW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn acc_f14_hov(&mut self) -> AccF14HovW<'_, DmaTfnSpec> {
        AccF14HovW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn acc_f15_hov(&mut self) -> AccF15HovW<'_, DmaTfnSpec> {
        AccF15HovW::new(self, 16)
    }
}
#[doc = "DMA Toggle/Frame Number\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_tfn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_tfn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaTfnSpec;
impl crate::RegisterSpec for DmaTfnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_tfn::R`](R) reader structure"]
impl crate::Readable for DmaTfnSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_tfn::W`](W) writer structure"]
impl crate::Writable for DmaTfnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_TFN to value 0"]
impl crate::Resettable for DmaTfnSpec {}
