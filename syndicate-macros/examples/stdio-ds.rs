syndicate::stdio_service!{
    ($v: AnyValue) => |t| {
        t.on_stop(enclose!((v) move |_t| { tracing::info!("- {v:?}"); Ok(()) }));
        tracing::info!("+ {v:?}");
        Ok(())
    };
}
