use json5 as j5;
use mlua::prelude::*;
use serde_json;

fn decode<'lua>(lua: &'lua Lua, s: mlua::Value<'lua>) -> LuaResult<mlua::Value<'lua>> {
    let s = match s {
        mlua::Value::String(ref s) => s.to_str(),
        _ => Err(format!("invalid input type: {}", s.type_name())).into_lua_err(),
    }?;
    let val: serde_json::Value = j5::from_str(s).map_err(LuaError::external)?;
    lua.to_value_with(
        &val,
        mlua::SerializeOptions::new()
            .serialize_unit_to_null(false)
            .detect_serde_json_arbitrary_precision(true),
    )
}

fn encode<'lua>(lua: &'lua Lua, v: mlua::Value<'lua>) -> LuaResult<mlua::Value<'lua>> {
    let val: serde_json::Value = lua.from_value(v)?;
    lua.create_string(j5::to_string(&val).map_err(LuaError::external)?)
        .map(mlua::Value::String)
}

#[mlua::lua_module]
fn json5(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("decode", lua.create_function(decode)?)?;
    exports.set("encode", lua.create_function(encode)?)?;
    Ok(exports)
}
