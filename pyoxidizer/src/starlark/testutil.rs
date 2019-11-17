// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use codemap::CodeMap;
use codemap_diagnostic::Diagnostic;
use starlark::eval;
use starlark::values::Value;

use super::env::global_environment;
use crate::app_packaging::environment::EnvironmentContext;

pub fn starlark_eval(snippet: &str) -> Result<Value, Diagnostic> {
    let build_target = crate::app_packaging::repackage::HOST;

    let cwd = std::env::current_dir().expect("unable to determine CWD");
    let config_path = cwd.join("dummy");

    let context = EnvironmentContext::new(&config_path, build_target)
        .expect("unable to create EnvironmentContext");

    let mut env = global_environment(&context).expect("unable to get global environment");

    let map = std::sync::Arc::new(std::sync::Mutex::new(CodeMap::new()));
    eval::simple::eval(&map, "<test>", snippet, false, &mut env)
}

pub fn starlark_ok(snippet: &str) -> Value {
    let res = starlark_eval(snippet);
    assert!(res.is_ok());

    res.unwrap()
}

pub fn starlark_nok(snippet: &str) -> Diagnostic {
    let res = starlark_eval(snippet);
    assert!(res.is_err());

    res.unwrap_err()
}
