#[doc = "Register `DEV_CXCFE` reader"]
pub type R = crate::R<DevCxcfeSpec>;
#[doc = "Register `DEV_CXCFE` writer"]
pub type W = crate::W<DevCxcfeSpec>;
#[doc = "Field `CX_DONE` reader - Control endpoint transaction done"]
pub type CxDoneR = crate::BitReader;
#[doc = "Field `TST_PKDONE` reader - Test packet done"]
pub type TstPkdoneR = crate::BitReader;
#[doc = "Field `CX_STL` reader - Control endpoint stall status"]
pub type CxStlR = crate::BitReader;
#[doc = "Field `CX_CLR` reader - Clear control endpoint FIFO"]
pub type CxClrR = crate::BitReader;
#[doc = "Field `CX_CLR` writer - Clear control endpoint FIFO"]
pub type CxClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CX_FUL` reader - Control FIFO full"]
pub type CxFulR = crate::BitReader;
#[doc = "Field `CX_EMP` reader - Control FIFO empty"]
pub type CxEmpR = crate::BitReader;
#[doc = "Field `F0_EMP` reader - FIFO 0 empty"]
pub type F0EmpR = crate::BitReader;
#[doc = "Field `F1_EMP` reader - FIFO 1 empty"]
pub type F1EmpR = crate::BitReader;
#[doc = "Field `F2_EMP` reader - FIFO 2 empty"]
pub type F2EmpR = crate::BitReader;
#[doc = "Field `F3_EMP` reader - FIFO 3 empty"]
pub type F3EmpR = crate::BitReader;
#[doc = "Field `F4_EMP` reader - "]
pub type F4EmpR = crate::BitReader;
#[doc = "Field `F5_EMP` reader - "]
pub type F5EmpR = crate::BitReader;
#[doc = "Field `F6_EMP` reader - "]
pub type F6EmpR = crate::BitReader;
#[doc = "Field `F7_EMP` reader - "]
pub type F7EmpR = crate::BitReader;
#[doc = "Field `F8_EMP` reader - "]
pub type F8EmpR = crate::BitReader;
#[doc = "Field `F9_EMP` reader - "]
pub type F9EmpR = crate::BitReader;
#[doc = "Field `F10_EMP` reader - "]
pub type F10EmpR = crate::BitReader;
#[doc = "Field `F11_EMP` reader - "]
pub type F11EmpR = crate::BitReader;
#[doc = "Field `F12_EMP` reader - "]
pub type F12EmpR = crate::BitReader;
#[doc = "Field `F13_EMP` reader - "]
pub type F13EmpR = crate::BitReader;
#[doc = "Field `F14_EMP` reader - "]
pub type F14EmpR = crate::BitReader;
#[doc = "Field `F15_EMP` reader - "]
pub type F15EmpR = crate::BitReader;
#[doc = "Field `CX_FNT` reader - Control Endpoint FIFO byte count"]
pub type CxFntR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Control endpoint transaction done"]
    #[inline(always)]
    pub fn cx_done(&self) -> CxDoneR {
        CxDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test packet done"]
    #[inline(always)]
    pub fn tst_pkdone(&self) -> TstPkdoneR {
        TstPkdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control endpoint stall status"]
    #[inline(always)]
    pub fn cx_stl(&self) -> CxStlR {
        CxStlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear control endpoint FIFO"]
    #[inline(always)]
    pub fn cx_clr(&self) -> CxClrR {
        CxClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control FIFO full"]
    #[inline(always)]
    pub fn cx_ful(&self) -> CxFulR {
        CxFulR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control FIFO empty"]
    #[inline(always)]
    pub fn cx_emp(&self) -> CxEmpR {
        CxEmpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO 0 empty"]
    #[inline(always)]
    pub fn f0_emp(&self) -> F0EmpR {
        F0EmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO 1 empty"]
    #[inline(always)]
    pub fn f1_emp(&self) -> F1EmpR {
        F1EmpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FIFO 2 empty"]
    #[inline(always)]
    pub fn f2_emp(&self) -> F2EmpR {
        F2EmpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO 3 empty"]
    #[inline(always)]
    pub fn f3_emp(&self) -> F3EmpR {
        F3EmpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn f4_emp(&self) -> F4EmpR {
        F4EmpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn f5_emp(&self) -> F5EmpR {
        F5EmpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn f6_emp(&self) -> F6EmpR {
        F6EmpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn f7_emp(&self) -> F7EmpR {
        F7EmpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn f8_emp(&self) -> F8EmpR {
        F8EmpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn f9_emp(&self) -> F9EmpR {
        F9EmpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn f10_emp(&self) -> F10EmpR {
        F10EmpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn f11_emp(&self) -> F11EmpR {
        F11EmpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn f12_emp(&self) -> F12EmpR {
        F12EmpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn f13_emp(&self) -> F13EmpR {
        F13EmpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn f14_emp(&self) -> F14EmpR {
        F14EmpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn f15_emp(&self) -> F15EmpR {
        F15EmpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Control Endpoint FIFO byte count"]
    #[inline(always)]
    pub fn cx_fnt(&self) -> CxFntR {
        CxFntR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Clear control endpoint FIFO"]
    #[inline(always)]
    pub fn cx_clr(&mut self) -> CxClrW<'_, DevCxcfeSpec> {
        CxClrW::new(self, 3)
    }
}
#[doc = "Device Control Endpoint FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_cxcfe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_cxcfe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevCxcfeSpec;
impl crate::RegisterSpec for DevCxcfeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_cxcfe::R`](R) reader structure"]
impl crate::Readable for DevCxcfeSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_cxcfe::W`](W) writer structure"]
impl crate::Writable for DevCxcfeSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x08;
}
#[doc = "`reset()` method sets DEV_CXCFE to value 0"]
impl crate::Resettable for DevCxcfeSpec {}
