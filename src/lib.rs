use mlua::prelude::*;
use serde_json::Value;
use serde_json5 as j5;

fn decode<'lua>(lua: &'lua Lua, s: LuaValue<'lua>) -> LuaResult<LuaValue<'lua>> {
    let s = match s {
        LuaValue::String(ref s) => Ok(s.as_bytes()),
        _ => Err(format!("invalid input type: {}", s.type_name())).into_lua_err(),
    }?;
    let val: Value = j5::from_slice(s).map_err(LuaError::external)?;
    lua.to_value_with(
        &val,
        LuaSerializeOptions::new()
            .serialize_unit_to_null(false)
            .detect_serde_json_arbitrary_precision(true),
    )
}

fn encode<'lua>(lua: &'lua Lua, v: LuaValue<'lua>) -> LuaResult<LuaValue<'lua>> {
    let val: Value = lua.from_value(v)?;
    lua.create_string(j5::to_string(&val).map_err(LuaError::external)?)
        .map(LuaValue::String)
}

#[mlua::lua_module]
fn json5(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("decode", lua.create_function(decode)?)?;
    exports.set("encode", lua.create_function(encode)?)?;
    Ok(exports)
}
