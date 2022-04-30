use std::io::{stdout, Write};
use handlebars::{Handlebars, Helper, Context, RenderContext, Output, HelperResult};

pub fn get_helper_helper (h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    out.write(&format!("{:?}", h))?;
    Ok(())
}

pub fn static_string_helper (h: &Helper, _: &Handlebars, _: &Context, rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    let output_string = "haha lol get fucked !";
    println!("{}", output_string);
    stdout().flush()?;
    out.write(output_string)?;
    Ok(())
}