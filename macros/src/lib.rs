#![feature(trace_macros)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Expr, Ident, Pat, Token};

#[proc_macro]
pub fn action_internal(input: TokenStream) -> TokenStream {
	action_inner(input)
}

struct Action {
	default: Option<Expr>,
	action_expr: Vec<Expr>,
	action_arms: Vec<(Pat, Expr)>,
}

impl Parse for Action {
	fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
		enum CaseKind {
			Default,
			Regular(Pat, Expr),
		}

		let mut action = Action {
			default: None,
			action_expr: vec![],
			action_arms: vec![],
		};

		while !input.is_empty() {
			let case_kind = if input.peek(Token![default]) {
				// `default` => ...
				if action.default.is_some() {
					return Err(input.error("multiple `default` cases found, only one permitted"));
				}
				input.parse::<Ident>()?;
				CaseKind::Default
			} else {
				// `<pat> = <expr>` => ...
				let pat = input.parse()?;
				input.parse::<Token![=]>()?;
				let expr = input.parse()?;
				CaseKind::Regular(pat, expr)
			};

			// `=> <expr>`
			input.parse::<Token![=>]>()?;
			let expr = input.parse::<Expr>()?;

			// We only need a comma if the expr wasn't a block or this is the last arm
			let is_block = match expr {
				Expr::Block(_) => true,
				_ => false,
			};
			if is_block || input.is_empty() {
				input.parse::<Option<Token![,]>>()?;
			} else {
				input.parse::<Token![,]>()?;
			}

			match case_kind {
				CaseKind::Default => action.default = Some(expr),
				CaseKind::Regular(pat, act) => {
					action.action_expr.push(act);
					action.action_arms.push((pat, expr));
				}
			}
		}

		Ok(action)
	}
}

fn create_result_enum(
	result_ident: Ident,
	variants: usize,
	span: Span,
) -> (Vec<Ident>, syn::ItemEnum) {
	let variant_names: Vec<Ident> = (0..variants)
		.map(|i| format_ident!("_{}", i, span = span))
		.collect();
	let type_parameters = &variant_names;
	let variants = &variant_names;

	let enum_item = parse_quote! {
		enum #result_ident<#(#type_parameters,)*> {
			#(
				#variants(#type_parameters),
			)*
			None
		}
	};

	(variant_names, enum_item)
}

fn action_inner(input: TokenStream) -> TokenStream {
	let parsed = syn::parse_macro_input!(input as Action);

	let span = Span::call_site();

	// // Create enum
	let enum_ident = Ident::new("__PrivResult", span);
	let (variant_names, enum_item) =
		create_result_enum(enum_ident.clone(), parsed.action_arms.len(), span);

	// Create actions
	let actions_ident = Ident::new("__actions", span);
	let actions_exprs = parsed.action_expr.iter();
	let actions_item = quote! {
		let mut #actions_ident = ( #(#actions_exprs),*, );
	};

	// Create poll matches
	let poll_match = variant_names.iter().enumerate().map(|(i, v)| {
		let i = syn::Index::from(i);
		quote! {
			match ::pros::rtos::action::Action::poll(&mut #actions_ident.#i) {
				::pros::rtos::action::Poll::Complete(x) => return #enum_ident::#v(x),
				_ => ()
			}
		}
	});

	let i = syn::Index::from(parsed.action_arms.len() - 1);
	let sleep_match = (0..parsed.action_arms.len() - 1)
		.rev()
		.map(syn::Index::from)
		.fold(
			quote! {
				match ::pros::rtos::action::Action::next(&mut #actions_ident.#i) {
					::pros::rtos::action::NextSleep::Never => return #enum_ident::None,
					n => ::pros::rtos::action::NextSleep::sleep(n),
				}
			},
			|prev, i| {
				quote! {
					match ::pros::rtos::action::Action::next(&mut #actions_ident.#i) {
						::pros::rtos::action::NextSleep::Never => #prev
						n => ::pros::rtos::action::NextSleep::sleep(n)
					}
				}
			},
		);

	// Result branches
	let branches =
		parsed
			.action_arms
			.iter()
			.zip(variant_names.iter())
			.map(|((pat, expr), variant)| {
				quote! {
					#enum_ident::#variant(#pat) => #expr,
				}
			});
	let branches = quote! { #( #branches )* };

	// Final match of results
	let default_expr = parsed.default.unwrap_or(syn::parse_str("()").unwrap());
	let match_result = quote! {
		match __result() {
			#branches
			_ => #default_expr
		}
	};

	TokenStream::from(quote! {
		#enum_item
		#actions_item
		let mut __result = || {
			#( #poll_match )*
			#sleep_match
			#enum_ident::None
		};
		#match_result
	})
}
