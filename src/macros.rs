#![allow(non_camel_case_types)]

use crate::bindings::*;

// lua.h

// ===============================================================
// some useful macros
// ===============================================================

#[macro_export]
macro_rules! lua_pop {
    ($L:expr, $n:expr) => {
        lua_settop($L, -($n) - 1)
    };
}

#[macro_export]
macro_rules! lua_newtable {
    ($L:expr) => {
        lua_createtable($L, 0, 0)
    };
}

#[macro_export]
macro_rules! lua_register {
    ($L:expr, $n:expr, $f:expr) => {
        lua_pushcfunction($L, $f);
        lua_setglobal($L, $n);
    };
}

#[macro_export]
macro_rules! lua_pushcfunction {
    ($L:expr, $f:expr) => {
        lua_pushcclosure($L, $f, 0)
    };
}

#[macro_export]
macro_rules! lua_strlen {
    ($L:expr, $i:expr) => {
        lua_objlen($L, $i)
    };
}

#[macro_export]
macro_rules! lua_isfunction {
    ($L:expr, $n:expr) => {
        lua_type($L, $n) == LUA_TFUNCTION
    };
}
#[macro_export]
macro_rules! lua_istable {
    ($L:expr, $n:expr) => {
        lua_type($L, $n) == LUA_TTABLE
    };
}
#[macro_export]
macro_rules! lua_islightuserdata {
    ($L:expr, $n:expr) => {
        lua_type($L, $n) == LUA_TLIGHTUSERDATA
    };
}
#[macro_export]
macro_rules! lua_isnil {
    ($L:expr, $n:expr) => {
        lua_type($L, $n) == LUA_TNIL
    };
}
#[macro_export]
macro_rules! lua_isboolean {
    ($L:expr, $n:expr) => {
        lua_type($L, $n) == LUA_TBOOLEAN
    };
}
#[macro_export]
macro_rules! lua_isthread {
    ($L:expr, $n:expr) => {
        lua_type($L, $n) == LUA_TTHREAD
    };
}
#[macro_export]
macro_rules! lua_isnone {
    ($L:expr, $n:expr) => {
        lua_type($L, $n) == LUA_TNONE
    };
}
#[macro_export]
macro_rules! lua_isnoneornil {
    ($L:expr, $n:expr) => {
        lua_type($L, $n) <= 0
    };
}

// #[macro_export]
// macro_rules! lua_pushliteral {
//   ($L:expr, $s:expr) => { lua_pushlstring($L, "" $s, (sizeof($s)/sizeof(char))-1) }

#[macro_export]
macro_rules! lua_setglobal {
    ($L:expr, $s:expr) => {
        lua_setfield($L, LUA_GLOBALSINDEX, $s)
    };
}

#[macro_export]
macro_rules! lua_getglobal {
    ($L:expr, $s:expr) => {
        lua_getfield($L, LUA_GLOBALSINDEX, $s)
    };
}

#[macro_export]
macro_rules! lua_tostring {
    ($L:expr, $i:expr) => {
        lua_tolstring($L, $i, ::std::ptr::null_mut())
    };
}

// compatibility macros and functions

#[macro_export]
macro_rules! lua_open {
    () => {
        luaL_newstate()
    };
}

#[macro_export]
macro_rules! lua_getregistry {
    ($L:expr) => {
        lua_pushvalue($L, LUA_REGISTRYINDEX)
    };
}

#[macro_export]
macro_rules! lua_getgccount {
    ($L:expr) => {
        lua_gc($L, LUA_GCCOUNT, 0)
    };
}

pub type lua_Chunkreader = lua_Reader;
pub type lua_Chunkwriter = lua_Writer;

// lauxlib.h

#[macro_export]
macro_rules! luaL_argcheck {
    ($L:expr, $cond:expr, $numarg:expr, $extramsg:expr) => {
        if !$cond {
            luaL_argerror($L, $numarg, $extramsg)
        }
    };
}

#[macro_export]
macro_rules! luaL_checkstring {
    ($L:expr, $n:expr) => {
        luaL_checklstring($L, $n, ::std::ptr::null_mut())
    };
}

#[macro_export]
macro_rules! luaL_optstring {
    ($L:expr, $n:expr, $d:expr) => {
        luaL_optlstring($L, $n, $d, ::std::ptr::null_mut())
    };
}

#[macro_export]
macro_rules! luaL_checkint {
    ($L:expr, $n:expr) => {
        luaL_checkinteger($L, $n)
    };
}

#[macro_export]
macro_rules! luaL_optint {
    ($L:expr, $n:expr, $d:expr) => {
        luaL_optinteger($L, $n, $d)
    };
}

#[macro_export]
macro_rules! luaL_checklong {
    ($L:expr, $n:expr) => {
        luaL_checkinteger($L, $n)
    };
}

#[macro_export]
macro_rules! luaL_optlong {
    ($L:expr, $n:expr, $d:expr) => {
        luaL_optinteger($L, $n, $d)
    };
}

#[macro_export]
macro_rules! luaL_typename {
    ($L:expr, $i:expr) => {
        lua_typename($L, lua_type($L, $i))
    };
}

#[macro_export]
macro_rules! luaL_dofile {
    ($L:expr, $fn:expr) => {
        if luaL_loadfile($L, $fn) == 0 {
            lua_pcall($L, 0, LUA_MULTRET, 0);
        }
    };
}

#[macro_export]
macro_rules! luaL_dostring {
    ($L:expr, $s:expr) => {
        if luaL_loadstring($L, $s) == 0 {
            lua_pcall($L, 0, LUA_MULTRET, 0);
        }
    };
}

#[macro_export]
macro_rules! luaL_getmetatable {
    ($L:expr, $n:expr) => {
        lua_getfield($L, LUA_REGISTRYINDEX, $n)
    };
}

#[macro_export]
macro_rules! luaL_opt {
    ($L:expr, $f:expr, $n:expr, $d:expr) => {
        if lua_isnoneornil($L, $n) {
            $d
        } else {
            f($L, $n)
        }
    };
}

// From Lua 5.2.

#[macro_export]
macro_rules! luaL_newlibtable {
    ($L:expr, $l:expr) => {
        lua_createtable($L, 0, sizeof($l) / sizeof(($l)[0]) - 1)
    };
}

#[macro_export]
macro_rules! luaL_newlib {
    ($L:expr, $l:expr) => {
        luaL_newlibtable($L, $l);
        luaL_setfuncs($L, $l, 0)
    };
}

// {======================================================
// Generic Buffer manipulation
// =======================================================

#[macro_export]
macro_rules! luaL_addchar {
    ($B:expr,$c:expr) => {
        if $B.p >= $B.buffer + LUAL_BUFFERSIZE {
            luaL_prepbuffer(B);
        }
        $B.p += 1;
        *$B.p = c;
    };
}

// compatibility only

#[macro_export]
macro_rules! luaL_putchar {
    ($B:expr,$c:expr) => {
        luaL_addchar($B, $c)
    };
}

#[macro_export]
macro_rules! luaL_addsize {
    ($B:expr,$n:expr) => {
        $B.p += $n
    };
}
