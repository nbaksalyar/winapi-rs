// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! IPv6 Internet address
use shared::minwindef::{ULONG, UCHAR, USHORT};

STRUCT!{struct in6_addr_u32 {
    Long: [ULONG; 4],
}}
STRUCT!{struct in6_addr_u16 {
    Word: [USHORT; 8],
}}
STRUCT!{struct in6_addr {
    s6_addr: [UCHAR; 16],
}}
UNION!(in6_addr, s6_addr, in6_addr_u32, in6_addr_u32_mut, in6_addr_u32);
UNION!(in6_addr, s6_addr, in6_addr_u16, in6_addr_u16_mut, in6_addr_u16);
UNION!(in6_addr, s6_addr, in6_addr, in6_addr_mut, [UCHAR; 16]);
pub type IN6_ADDR = in6_addr;
pub type PIN6_ADDR = *mut in6_addr;
pub type LPIN6_ADDR = *mut in6_addr;
