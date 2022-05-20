fn main() {
    let mut engine = rhai::Engine::new();
    engine.set_module_resolver(rhai::module_resolvers::FileModuleResolver::new_with_path(
        "user",
    ));

    let mut ast = engine
        .compile_scripts_with_scope(
            &rhai::Scope::new(),
            [
                include_str!("api/script1.rhai"),
                include_str!("api/script2.rhai"),
            ],
        )
        .expect("failed to compile api");

    ast += engine
        .compile_into_self_contained(
            &rhai::Scope::new(),
            &std::fs::read_to_string("user/script3.rhai").expect("failed to load user script"),
        )
        .expect("failed to compile user script");

    engine
        .eval_ast::<()>(&ast)
        .expect("failed to evaluate script.");

    println!("evaluated");
}
