use cpython::py_module_initializer;

#[macro_use]
mod macros;

mod datetime;

mod textsearcher;

pub mod prelude;

py_module_initializer!(python_comm, |python, module| {
    textsearcher::module_initializer(python, module)?;

    Ok(())
});
