// automatically generated by rust-bindgen

#![allow(non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

#[macro_use]
extern crate enum_primitive;
extern crate libc;
extern crate num;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct nlmsghdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}


pub mod socket {
    use std::os::raw::{c_int, c_uint, c_void};

    pub enum socket { }
    pub type socklen_t = c_uint;
    pub type pid_t = c_int;
    pub const NETLINK_NETFILTER: c_int = 12;
    pub const BUFFER_SIZE: usize = 8192;

    pub const NLM_F_REQUEST: u16 = 1;
    pub const NLM_F_MULTI: u16 = 2;
    pub const NLM_F_ACK: u16 = 4;
    pub const NLM_F_ECHO: u16 = 8;
    pub const NLM_F_DUMP_INTR: u16 = 16;
    pub const NLM_F_DUMP_FILTERED: u16 = 32;

    // Modifiers to GET request
    pub const NLM_F_ROOT: u16 = 0x100;
    pub const NLM_F_MATCH: u16 = 0x200;
    pub const NLM_F_ATOMIC: u16 = 0x400;
    pub const NLM_F_DUMP: u16 = NLM_F_ROOT | NLM_F_MATCH;

    // Modifiers to NEW request
    pub const NLM_F_REPLACE: u16 = 0x100;
    pub const NLM_F_EXCL: u16 = 0x200;
    pub const NLM_F_CREATE: u16 = 0x400;
    pub const NLM_F_APPEND: u16 = 0x800;

    #[link(name = "mnl")]
    extern "C" {
        #[link_name = "mnl_socket_open"]
        pub fn open(bus: c_int) -> *mut socket;
        #[link_name = "mnl_socket_open2"]
        pub fn open2(bus: c_int, flags: c_int) -> *mut socket;
        #[link_name = "mnl_socket_fdopen"]
        pub fn fdopen(fd: c_int) -> *mut socket;
        #[link_name = "mnl_socket_bind"]
        pub fn bind(nl: *mut socket, groups: c_uint, pid: pid_t) -> c_int;
        #[link_name = "mnl_socket_close"]
        pub fn close(nl: *mut socket) -> c_int;
        #[link_name = "mnl_socket_get_fd"]
        pub fn get_fd(nl: *const socket) -> c_int;
        #[link_name = "mnl_socket_get_portid"]
        pub fn get_portid(nl: *const socket) -> c_uint;
        #[link_name = "mnl_socket_sendto"]
        pub fn sendto(nl: *const socket, req: *const c_void, siz: usize) -> isize;
        #[link_name = "mnl_socket_recvfrom"]
        pub fn recvfrom(nl: *const socket, buf: *mut c_void, siz: usize) -> isize;
        #[link_name = "mnl_socket_setsockopt"]
        pub fn setsockopt(nl: *const socket,
                          type_: c_int,
                          buf: *mut c_void,
                          len: socklen_t)
                          -> c_int;
        #[link_name = "mnl_socket_getsockopt"]
        pub fn getsockopt(nl: *const socket,
                          type_: c_int,
                          buf: *mut c_void,
                          len: *mut socklen_t)
                          -> c_int;
    }
}

pub mod nlmsg {
    use std::os::raw::{c_int, c_uint, c_void};
    use libc;

    use nlmsghdr;
    pub enum batch { }

    #[link(name = "mnl")]
    extern "C" {
        #[link_name = "mnl_nlmsg_size"]
        pub fn size(len: usize) -> usize;
        #[link_name = "mnl_nlmsg_get_payload_len"]
        pub fn get_payload_len(nlh: *const nlmsghdr) -> usize;
        #[link_name = "mnl_nlmsg_put_header"]
        pub fn put_header(buf: *mut c_void) -> *mut nlmsghdr;
        #[link_name = "mnl_nlmsg_put_extra_header"]
        pub fn put_extra_header(nlh: *mut nlmsghdr, size: usize) -> *mut c_void;
        #[link_name = "mnl_nlmsg_ok"]
        pub fn ok(nlh: *const nlmsghdr, len: c_int) -> u8;
        #[link_name = "mnl_nlmsg_next"]
        pub fn next(nlh: *const nlmsghdr, len: *mut c_int) -> *mut nlmsghdr;
        #[link_name = "mnl_nlmsg_seq_ok"]
        pub fn seq_ok(nlh: *const nlmsghdr, seq: c_uint) -> u8;
        #[link_name = "mnl_nlmsg_portid_ok"]
        pub fn portid_ok(nlh: *const nlmsghdr, portid: c_uint) -> u8;
        #[link_name = "mnl_nlmsg_get_payload"]
        pub fn get_payload(nlh: *const nlmsghdr) -> *mut c_void;
        #[link_name = "mnl_nlmsg_get_payload_offset"]
        pub fn get_payload_offset(nlh: *const nlmsghdr, offset: usize) -> *mut c_void;
        #[link_name = "mnl_nlmsg_get_payload_tail"]
        pub fn get_payload_tail(nlh: *const nlmsghdr) -> *mut c_void;
        #[link_name = "mnl_nlmsg_fprintf"]
        pub fn fprintf(fd: *mut libc::FILE,
                       data: *const c_void,
                       datalen: usize,
                       extra_header_size: usize);
        #[link_name = "mnl_nlmsg_batch_start"]
        pub fn batch_start(buf: *mut c_void, bufsiz: usize) -> *mut batch;
        #[link_name = "mnl_nlmsg_batch_next"]
        pub fn batch_next(b: *mut batch) -> u8;
        #[link_name = "mnl_nlmsg_batch_stop"]
        pub fn batch_stop(b: *mut batch);
        #[link_name = "mnl_nlmsg_batch_size"]
        pub fn batch_size(b: *mut batch) -> usize;
        #[link_name = "mnl_nlmsg_batch_reset"]
        pub fn batch_reset(b: *mut batch);
        #[link_name = "mnl_nlmsg_batch_head"]
        pub fn batch_head(b: *mut batch) -> *mut c_void;
        #[link_name = "mnl_nlmsg_batch_current"]
        pub fn batch_current(b: *mut batch) -> *mut c_void;
        #[link_name = "mnl_nlmsg_batch_is_empty"]
        pub fn batch_is_empty(b: *mut batch) -> u8;
    }
}

pub mod attr {
    use std::os::raw::{c_int, c_uint, c_void, c_char};

    use nlmsghdr;

    #[repr(C)]
    #[derive(Copy, Clone)]
    #[derive(Debug)]
    pub struct nlattr {
        nla_len: u16,
        nla_type: u16,
    }
    #[derive(Copy, Clone)]
    #[repr(u32)]
    #[derive(Debug)]
    pub enum data_type {
        TYPE_UNSPEC = 0,
        TYPE_U8 = 1,
        TYPE_U16 = 2,
        TYPE_U32 = 3,
        TYPE_U64 = 4,
        TYPE_STRING = 5,
        TYPE_FLAG = 6,
        TYPE_MSECS = 7,
        TYPE_NESTED = 8,
        TYPE_NESTED_COMPAT = 9,
        TYPE_NUL_STRING = 10,
        TYPE_BINARY = 11,
        TYPE_MAX = 12,
    }
    pub type attr_cb_t = ::std::option::Option<unsafe extern "C" fn(attr: *const nlattr,
                                                                      data: *mut c_void)
                                                                      -> c_int>;

    #[link(name = "mnl")]
    extern "C" {
        #[link_name = "mnl_attr_get_type"]
        pub fn get_type(attr: *const nlattr) -> u16;
        #[link_name = "mnl_attr_get_len"]
        pub fn get_len(attr: *const nlattr) -> u16;
        #[link_name = "mnl_attr_get_payload_len"]
        pub fn get_payload_len(attr: *const nlattr) -> u16;
        #[link_name = "mnl_attr_get_payload"]
        pub fn get_payload(attr: *const nlattr) -> *mut c_void;
        #[link_name = "mnl_attr_get_u8"]
        pub fn get_u8(attr: *const nlattr) -> u8;
        #[link_name = "mnl_attr_get_u16"]
        pub fn get_u16(attr: *const nlattr) -> u16;
        #[link_name = "mnl_attr_get_u32"]
        pub fn get_u32(attr: *const nlattr) -> u32;
        #[link_name = "mnl_attr_get_u64"]
        pub fn get_u64(attr: *const nlattr) -> u64;
        #[link_name = "mnl_attr_get_str"]
        pub fn get_str(attr: *const nlattr) -> *const c_char;
        #[link_name = "mnl_attr_put"]
        pub fn put(nlh: *mut nlmsghdr, type_: u16, len: usize, data: *const c_void);
        #[link_name = "mnl_attr_put_u8"]
        pub fn put_u8(nlh: *mut nlmsghdr, type_: u16, data: u8);
        #[link_name = "mnl_attr_put_u16"]
        pub fn put_u16(nlh: *mut nlmsghdr, type_: u16, data: u16);
        #[link_name = "mnl_attr_put_u32"]
        pub fn put_u32(nlh: *mut nlmsghdr, type_: u16, data: u32);
        #[link_name = "mnl_attr_put_u64"]
        pub fn put_u64(nlh: *mut nlmsghdr, type_: u16, data: u64);
        #[link_name = "mnl_attr_put_str"]
        pub fn put_str(nlh: *mut nlmsghdr, type_: u16, data: *const c_char);
        #[link_name = "mnl_attr_put_strz"]
        pub fn put_strz(nlh: *mut nlmsghdr, type_: u16, data: *const c_char);
        #[link_name = "mnl_attr_put_check"]
        pub fn put_check(nlh: *mut nlmsghdr,
                         buflen: usize,
                         type_: u16,
                         len: usize,
                         data: *const c_void)
                         -> u8;
        #[link_name = "mnl_attr_put_u8_check"]
        pub fn put_u8_check(nlh: *mut nlmsghdr, buflen: usize, type_: u16, data: u8) -> u8;
        #[link_name = "mnl_attr_put_u16_check"]
        pub fn put_u16_check(nlh: *mut nlmsghdr, buflen: usize, type_: u16, data: u16) -> u8;
        #[link_name = "mnl_attr_put_u32_check"]
        pub fn put_u32_check(nlh: *mut nlmsghdr, buflen: usize, type_: u16, data: u32) -> u8;
        #[link_name = "mnl_attr_put_u64_check"]
        pub fn put_u64_check(nlh: *mut nlmsghdr, buflen: usize, type_: u16, data: u64) -> u8;
        #[link_name = "mnl_attr_put_str_check"]
        pub fn put_str_check(nlh: *mut nlmsghdr,
                             buflen: usize,
                             type_: u16,
                             data: *const c_char)
                             -> u8;
        #[link_name = "mnl_attr_put_strz_check"]
        pub fn put_strz_check(nlh: *mut nlmsghdr,
                              buflen: usize,
                              type_: u16,
                              data: *const c_char)
                              -> u8;
        #[link_name = "mnl_attr_nest_start"]
        pub fn nest_start(nlh: *mut nlmsghdr, type_: u16) -> *mut nlattr;
        #[link_name = "mnl_attr_nest_start_check"]
        pub fn nest_start_check(nlh: *mut nlmsghdr, buflen: usize, type_: u16) -> *mut nlattr;
        #[link_name = "mnl_attr_nest_end"]
        pub fn nest_end(nlh: *mut nlmsghdr, start: *mut nlattr);
        #[link_name = "mnl_attr_nest_cancel"]
        pub fn nest_cancel(nlh: *mut nlmsghdr, start: *mut nlattr);
        #[link_name = "mnl_attr_type_valid"]
        pub fn type_valid(attr: *const nlattr, maxtype: u16) -> c_int;
        #[link_name = "mnl_attr_validate"]
        pub fn validate(attr: *const nlattr, type_: data_type) -> c_int;
        #[link_name = "mnl_attr_validate2"]
        pub fn validate2(attr: *const nlattr, type_: data_type, len: usize) -> c_int;
        #[link_name = "mnl_attr_ok"]
        pub fn ok(attr: *const nlattr, len: c_int) -> u8;
        #[link_name = "mnl_attr_next"]
        pub fn next(attr: *const nlattr) -> *mut nlattr;
        #[link_name = "mnl_attr_parse"]
        pub fn parse(nlh: *const nlmsghdr,
                     offset: c_uint,
                     cb: attr_cb_t,
                     data: *mut c_void)
                     -> c_int;
        #[link_name = "mnl_attr_parse_nested"]
        pub fn parse_nested(attr: *const nlattr, cb: attr_cb_t, data: *mut c_void) -> c_int;
        #[link_name = "mnl_attr_parse_payload"]
        pub fn parse_payload(payload: *const c_void,
                             payload_len: usize,
                             cb: attr_cb_t,
                             data: *mut c_void)
                             -> c_int;
    }
}

pub mod callback {
    use std::os::raw::{c_int, c_uint, c_void};

    use nlmsghdr;

    pub type cb_t = ::std::option::Option<unsafe extern "C" fn(nlh: *const nlmsghdr,
                                                                 data: *mut c_void)
                                                                 -> c_int>;

    enum_from_primitive! {
        #[derive(Debug, Eq, PartialEq)]
        pub enum CallbackResult {
            MNL_CB_ERROR = -1,
            MNL_CB_STOP = 0,
            MNL_CB_OK = 1,
        }
    }

    #[link(name = "mnl")]
    extern "C" {
        #[link_name = "mnl_cb_run"]
        pub fn run(buf: *const c_void,
                   numbytes: usize,
                   seq: c_uint,
                   portid: c_uint,
                   cb_data: cb_t,
                   data: *mut c_void)
                   -> c_int;
        #[link_name = "mnl_cb_run2"]
        pub fn run2(buf: *const c_void,
                    numbytes: usize,
                    seq: c_uint,
                    portid: c_uint,
                    cb_data: cb_t,
                    data: *mut c_void,
                    cb_ctl_array: *mut cb_t,
                    cb_ctl_array_len: c_uint)
                    -> c_int;
    }
}
