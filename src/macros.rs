#![allow(non_camel_case_types)]

use crate::bindings::*;

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
    lua_tolstring($L, $i, NULL)
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
