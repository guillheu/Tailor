use std::io::{stdout, Write};
use handlebars::{Handlebars, Helper, Context, RenderContext, Output, HelperResult};
use std::time::{SystemTime, UNIX_EPOCH};




pub fn static_string_helper (_h: &Helper, _: &Handlebars, _: &Context, _rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let output_string = "foo bar";
    println!("{}", output_string);
    stdout().flush()?;
    out.write(output_string)?;
    Ok(())
}

pub fn get_remote_timestamp_seconds(h: &Helper, _: &Handlebars, _: &Context, _rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    out.write(&format!("{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()))?;
    Ok(())
}