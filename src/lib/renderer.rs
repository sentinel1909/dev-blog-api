// src/lib/renderer.rs

// dependencies
use crate::error::ApiError;
use once_cell::sync::OnceCell;
use tera::Tera;

// initializer for the compiled template item, uses a static memory location
static COMPILED_TEMPLATE: OnceCell<Tera> = OnceCell::new();

// build the Tera template
pub fn get_templates() -> Result<&'static Tera, ApiError> {
    COMPILED_TEMPLATE.get_or_try_init(|| {
        let mut base_template =
            Tera::new("templates/**/*").map_err(|err| ApiError::Internal(err.to_string()))?;
        base_template
            .add_template_file("templates/base.html", None)
            .map_err(|err| ApiError::Internal(err.to_string()))?;
        Ok(base_template)
    })
}
