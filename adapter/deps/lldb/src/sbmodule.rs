use super::*;

cpp_class!(pub unsafe struct SBModule as "SBModule");

unsafe impl Send for SBModule {}

impl SBModule {
    pub fn uuid_string(&self) -> Option<&str> {
        let ptr = cpp!(unsafe [self as "SBModule*"] -> *const c_char as "const char*" {
            return self->GetUUIDString();
        });
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(get_str(ptr)) }
        }
    }
    pub fn filespec(&self) -> SBFileSpec {
        cpp!(unsafe [self as "SBModule*"] -> SBFileSpec as "SBFileSpec" {
            return self->GetFileSpec();
        })
    }
    pub fn platform_filespec(&self) -> SBFileSpec {
        cpp!(unsafe [self as "SBModule*"] -> SBFileSpec as "SBFileSpec" {
            return self->GetPlatformFileSpec();
        })
    }
    pub fn remote_install_filespec(&self) -> SBFileSpec {
        cpp!(unsafe [self as "SBModule*"] -> SBFileSpec as "SBFileSpec" {
            return self->GetRemoteInstallFileSpec();
        })
    }
    pub fn symbol_filespec(&self) -> SBFileSpec {
        cpp!(unsafe [self as "SBModule*"] -> SBFileSpec as "SBFileSpec" {
            return self->GetSymbolFileSpec();
        })
    }
    pub fn object_header_address(&self) -> SBAddress {
        cpp!(unsafe [self as "SBModule*"] -> SBAddress as "SBAddress" {
            return self->GetObjectFileHeaderAddress();
        })
    }
}

impl IsValid for SBModule {
    fn is_valid(&self) -> bool {
        cpp!(unsafe [self as "SBModule*"] -> bool as "bool" {
            return self->IsValid();
        })
    }
}

impl fmt::Debug for SBModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_descr(f, |descr| {
            cpp!(unsafe [self as "SBModule*", descr as "SBStream*"] -> bool as "bool" {
                return self->GetDescription(*descr);
            })
        })
    }
}
