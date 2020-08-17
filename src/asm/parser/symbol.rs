use crate::*;


pub fn parse_symbol(
    state: &mut asm::parser::State)
    -> Result<(), ()>
{
    let mut span = diagn::Span::new_dummy();
    let mut hierarchy_level = 0;
    
    while let Some(tk_dot) = state.parser.maybe_expect(syntax::TokenKind::Dot)
    {
        hierarchy_level += 1;
        span = span.join(&tk_dot.span);
    }

    let tk_name = state.parser.expect(syntax::TokenKind::Identifier)?;
    let name = tk_name.excerpt.clone().unwrap();
    span = span.join(&tk_name.span);
    
    let ctx = state.asm_state.get_ctx();
    
    let value = if state.parser.maybe_expect(syntax::TokenKind::Equal).is_some()
    {		
        let expr = expr::Expr::parse(&mut state.parser)?;
        let value = state.asm_state.eval_expr(
            state.report.clone(),
            &expr,
            &ctx,
            &mut expr::EvalContext::new(),
            true)?;

        state.parser.expect_linebreak()?;
        value
    }
    else
    {
        let tk_colon = state.parser.expect(syntax::TokenKind::Colon)?;
        
        span = span.join(&tk_colon.span);
        
        let addr = state.asm_state.get_addr(
            state.report.clone(),
            &ctx,
            &span)?;
        
        expr::Value::make_integer(addr)
    };

    state.asm_state.symbols.create(
        &ctx.symbol_ctx, 
        name, 
        hierarchy_level, 
        value, 
        state.report.clone(), 
        &span)?;

    Ok(())
}